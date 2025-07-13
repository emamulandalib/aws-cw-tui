use crate::utils::formatting::{format_bytes, format_duration};

/// Display format configuration for metrics
#[derive(Debug, Clone)]
pub enum DisplayFormat {
    Integer,
    Decimal(u8), // number of decimal places
    Bytes,
    Duration,
    Percentage,
}

impl DisplayFormat {
    /// Format value according to display format
    pub fn format_value(&self, value: f64) -> String {
        match self {
            DisplayFormat::Integer => format!("{:.0}", value),
            DisplayFormat::Decimal(places) => format!("{:.1$}", value, *places as usize),
            DisplayFormat::Bytes => format_bytes(value),
            DisplayFormat::Duration => format_duration(value),
            DisplayFormat::Percentage => format!("{:.1}%", value),
        }
    }
}

// format_bytes and format_duration now imported from utils::formatting
