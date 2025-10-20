use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUnit {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
}

#[derive(Debug, Clone, Copy)]
pub struct TimeRange {
    pub value: u32,
    pub unit: TimeUnit,
    pub period_days: u32,
}

impl TimeRange {
    pub fn new(value: u32, unit: TimeUnit, period_days: u32) -> Result<Self> {
        // Validate input values
        match unit {
            TimeUnit::Minutes if value < 1 => {
                return Err(anyhow::anyhow!("Minutes must be at least 1"));
            }
            TimeUnit::Months if value > 15 => {
                return Err(anyhow::anyhow!("Months must not exceed 15"));
            }
            _ => {}
        }

        if !(1..=30).contains(&period_days) {
            return Err(anyhow::anyhow!("Period must be between 1 and 30 days"));
        }

        Ok(Self {
            value,
            unit,
            period_days,
        })
    }

    pub fn duration(self) -> Duration {
        let seconds = match self.unit {
            TimeUnit::Minutes => self.value as u64 * 60,
            TimeUnit::Hours => self.value as u64 * 3600,
            TimeUnit::Days => self.value as u64 * 86400,
            TimeUnit::Weeks => self.value as u64 * 604800,
            TimeUnit::Months => self.value as u64 * 2592000, // Approximate: 30 days per month
        };
        Duration::from_secs(seconds)
    }
}

pub fn calculate_period_seconds(time_range: &TimeRange) -> i32 {
    // Calculate appropriate period based on time range duration and period_days
    let duration_seconds = time_range.duration().as_secs();

    // Use period_days to influence the granularity
    // Shorter period_days means finer granularity, longer means coarser
    let base_period = match duration_seconds {
        0..=3600 => 60,         // 1 minute for <= 1 hour
        3601..=21600 => 300,    // 5 minutes for <= 6 hours
        21601..=86400 => 900,   // 15 minutes for <= 1 day
        86401..=604800 => 3600, // 1 hour for <= 1 week
        _ => {
            // For longer periods, use period_days to calculate appropriate granularity
            let target_points = 100; // Target ~100 data points
            let calculated_period = (duration_seconds / target_points).max(3600) as i32;

            // Ensure period aligns with CloudWatch supported periods
            match calculated_period {
                0..=300 => 300,
                301..=900 => 900,
                901..=3600 => 3600,
                _ => ((calculated_period / 3600) * 3600).min(86400), // Round to hours, max 1 day
            }
        }
    };

    // Adjust period based on period_days setting
    // Lower period_days = finer granularity, higher period_days = coarser granularity
    let period_multiplier = match time_range.period_days {
        1..=3 => 1,  // Fine granularity for short periods
        4..=7 => 2,  // Medium granularity
        8..=14 => 3, // Coarser granularity for medium periods
        _ => 4,      // Coarsest granularity for long periods
    };

    let adjusted_period = base_period * period_multiplier;

    // Ensure the period doesn't exceed CloudWatch limits (max 1 day = 86400 seconds)
    adjusted_period.min(86400)
}
