use crate::aws::time_range::{calculate_period_seconds, TimeRange, TimeUnit};
use crate::models::{App, TimeRangeMode, Timezone};
use anyhow::Result;

impl App {
    /// Update the time range configuration
    pub fn update_time_range(
        &mut self,
        value: u32,
        unit: TimeUnit,
        period_days: u32,
    ) -> Result<()> {
        self.time_range = TimeRange::new(value, unit, period_days)?;
        Ok(())
    }

    /// Get all available time range options
    pub fn get_time_range_options() -> Vec<(&'static str, u32, TimeUnit, u32)> {
        vec![
            // Minutes
            ("1 minute", 1, TimeUnit::Minutes, 1),
            ("3 minutes", 3, TimeUnit::Minutes, 1),
            ("5 minutes", 5, TimeUnit::Minutes, 1),
            ("15 minutes", 15, TimeUnit::Minutes, 1),
            ("30 minutes", 30, TimeUnit::Minutes, 1),
            ("45 minutes", 45, TimeUnit::Minutes, 1),
            // Hours
            ("1 hour", 1, TimeUnit::Hours, 1),
            ("2 hours", 2, TimeUnit::Hours, 1),
            ("3 hours", 3, TimeUnit::Hours, 1),
            ("6 hours", 6, TimeUnit::Hours, 1),
            ("8 hours", 8, TimeUnit::Hours, 1),
            ("12 hours", 12, TimeUnit::Hours, 1),
            // Days
            ("1 day", 1, TimeUnit::Days, 1),
            ("2 days", 2, TimeUnit::Days, 1),
            ("3 days", 3, TimeUnit::Days, 1),
            ("4 days", 4, TimeUnit::Days, 1),
            ("5 days", 5, TimeUnit::Days, 1),
            ("6 days", 6, TimeUnit::Days, 1),
            // Weeks
            ("1 week", 1, TimeUnit::Weeks, 7),
            ("2 weeks", 2, TimeUnit::Weeks, 14),
            ("4 weeks", 4, TimeUnit::Weeks, 28),
            ("6 weeks", 6, TimeUnit::Weeks, 42),
            // Months
            ("3 months", 3, TimeUnit::Months, 90),
            ("6 months", 6, TimeUnit::Months, 180),
            ("12 months", 12, TimeUnit::Months, 365),
            ("15 months", 15, TimeUnit::Months, 455),
        ]
    }

    /// Get current time range selection index
    pub fn get_current_time_range_index(&self) -> usize {
        self.time_range_list_state.selected().unwrap_or(0)
    }

    /// Select a specific time range by index
    pub fn select_time_range(&mut self, index: usize) -> Result<()> {
        let options = Self::get_time_range_options();
        if let Some(&(_, value, unit, period_days)) = options.get(index) {
            self.time_range_list_state.select(Some(index));
            self.time_range = TimeRange::new(value, unit, period_days)?;
            // Validate and adjust period selection for new time range
            self.validate_period_selection();
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Navigate up in time range selection
    pub fn time_range_scroll_up(&mut self) {
        let current = self.time_range_list_state.selected().unwrap_or(0);
        if current > 0 {
            let new_index = current - 1;
            self.time_range_list_state.select(Some(new_index));
            // Actually update the time_range field used for AWS API calls
            let _ = self.select_time_range(new_index);
        }
    }

    /// Navigate down in time range selection
    pub fn time_range_scroll_down(&mut self) {
        let options = Self::get_time_range_options();
        let current = self.time_range_list_state.selected().unwrap_or(0);
        if current < options.len() - 1 {
            let new_index = current + 1;
            self.time_range_list_state.select(Some(new_index));
            // Actually update the time_range field used for AWS API calls
            let _ = self.select_time_range(new_index);
        }
    }

    /// Navigate left in time range selection
    pub fn time_range_scroll_left(&mut self) {
        self.time_range_scroll_up();
    }

    /// Navigate right in time range selection
    pub fn time_range_scroll_right(&mut self) {
        self.time_range_scroll_down();
    }

    /// Toggle between absolute and relative time range modes
    pub fn toggle_time_range_mode(&mut self) {
        self.time_range_mode = match self.time_range_mode {
            TimeRangeMode::Absolute => TimeRangeMode::Relative,
            TimeRangeMode::Relative => TimeRangeMode::Absolute,
        };
    }

    /// Get current time range mode
    pub fn get_time_range_mode(&self) -> &TimeRangeMode {
        &self.time_range_mode
    }

    /// Get all available period options based on current time range (CloudWatch constraints)
    pub fn get_period_options(&self) -> Vec<(&'static str, i32)> {
        let time_range_duration = self.time_range.duration();
        let hours = time_range_duration.as_secs() / 3600;

        match hours {
            // Within 3 hours: High-resolution periods allowed
            0..=3 => vec![
                ("5 seconds", 5),
                ("10 seconds", 10),
                ("20 seconds", 20),
                ("30 seconds", 30),
                ("1 minute", 60),
                ("5 minutes", 300),
                ("15 minutes", 900),
            ],
            // 3 hours to 15 days: Standard periods only
            4..=360 => vec![
                ("1 minute", 60),
                ("5 minutes", 300),
                ("15 minutes", 900),
                ("1 hour", 3600),
            ],
            // 15+ days: Coarse periods only
            _ => vec![("1 hour", 3600), ("6 hours", 21600), ("1 day", 86400)],
        }
    }

    /// Get a static method for compatibility (used by UI components)
    pub fn get_period_options_static() -> Vec<(&'static str, i32)> {
        // Default fallback when no app context is available
        vec![
            ("1 minute", 60),
            ("5 minutes", 300),
            ("15 minutes", 900),
            ("1 hour", 3600),
            ("6 hours", 21600),
            ("1 day", 86400),
        ]
    }

    /// Get current period selection index
    pub fn get_current_period_index(&self) -> usize {
        self.period_list_state.selected().unwrap_or(0)
    }

    /// Select a specific period by index
    pub fn select_period(&mut self, index: usize) {
        let options = self.get_period_options();
        if let Some(&(_, period_seconds)) = options.get(index) {
            self.period_list_state.select(Some(index));
            self.selected_period = Some(period_seconds);
        }
    }

    /// Validate and auto-adjust period selection for current time range
    pub fn validate_period_selection(&mut self) {
        if let Some(current_period) = self.selected_period {
            let valid_periods = self.get_period_options();

            // Check if current period is still valid
            let is_valid = valid_periods
                .iter()
                .any(|(_, period)| *period == current_period);

            if !is_valid {
                // Find the closest valid period
                let closest_period = valid_periods
                    .iter()
                    .min_by_key(|(_, period)| (*period as i32 - current_period).abs())
                    .map(|(_, period)| *period);

                if let Some(new_period) = closest_period {
                    self.selected_period = Some(new_period);

                    // Update the selection index to match
                    if let Some(index) = valid_periods
                        .iter()
                        .position(|(_, period)| *period == new_period)
                    {
                        self.period_list_state.select(Some(index));
                    }

                    log::info!(
                        "Auto-adjusted period from {}s to {}s for current time range",
                        current_period,
                        new_period
                    );
                } else {
                    // Clear selection if no valid period found
                    self.selected_period = None;
                    self.period_list_state.select(Some(0));
                }
            }
        }
    }

    /// Clear manual period selection (revert to auto-calculation)
    pub fn clear_period_selection(&mut self) {
        self.selected_period = None;
    }

    /// Get the effective period in seconds (manual override or auto-calculated)
    pub fn get_effective_period(&self) -> i32 {
        self.selected_period.unwrap_or_else(|| {
            // Fall back to auto-calculation
            calculate_period_seconds(&self.time_range)
        })
    }

    /// Navigate up in period selection
    pub fn period_scroll_up(&mut self) {
        let current = self.period_list_state.selected().unwrap_or(0);
        if current > 0 {
            let new_index = current - 1;
            self.period_list_state.select(Some(new_index));
            // Actually update the selected period value
            self.select_period(new_index);
        }
    }

    /// Navigate down in period selection
    pub fn period_scroll_down(&mut self) {
        let options = self.get_period_options();
        let current = self.period_list_state.selected().unwrap_or(0);
        if current < options.len() - 1 {
            let new_index = current + 1;
            self.period_list_state.select(Some(new_index));
            // Actually update the selected period value
            self.select_period(new_index);
        }
    }

    /// Get all available timezone options
    pub fn get_timezone_options() -> Vec<Timezone> {
        Timezone::get_timezone_options()
    }

    /// Get current timezone
    pub fn get_current_timezone(&self) -> &Timezone {
        &self.timezone
    }

    /// Get current timezone selection index
    pub fn get_current_timezone_index(&self) -> usize {
        self.timezone_list_state.selected().unwrap_or(0)
    }

    /// Navigate up in timezone selection
    pub fn timezone_scroll_up(&mut self) {
        let current = self.timezone_list_state.selected().unwrap_or(0);
        if current > 0 {
            self.timezone_list_state.select(Some(current - 1));
            let options = Self::get_timezone_options();
            if let Some(timezone) = options.get(current - 1) {
                self.timezone = timezone.clone();
            }
        }
    }

    /// Navigate down in timezone selection
    pub fn timezone_scroll_down(&mut self) {
        let options = Self::get_timezone_options();
        let current = self.timezone_list_state.selected().unwrap_or(0);
        if current < options.len() - 1 {
            self.timezone_list_state.select(Some(current + 1));
            if let Some(timezone) = options.get(current + 1) {
                self.timezone = timezone.clone();
            }
        }
    }
}
