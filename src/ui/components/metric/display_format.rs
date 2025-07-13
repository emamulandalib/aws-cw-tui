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

/// Format bytes in human-readable format
pub fn format_bytes(bytes: f64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0.0 {
        return "0 B".to_string();
    }

    let bytes = bytes.abs();
    let unit_index = (bytes.ln() / THRESHOLD.ln()).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);

    let value = bytes / THRESHOLD.powi(unit_index as i32);

    if value >= 100.0 {
        format!("{:.0} {}", value, UNITS[unit_index])
    } else if value >= 10.0 {
        format!("{:.1} {}", value, UNITS[unit_index])
    } else {
        format!("{:.2} {}", value, UNITS[unit_index])
    }
}

/// Format duration in human-readable format
pub fn format_duration(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.0} ms", seconds * 1000.0)
    } else if seconds < 60.0 {
        format!("{:.2} s", seconds)
    } else if seconds < 3600.0 {
        let minutes = seconds / 60.0;
        format!("{:.1} min", minutes)
    } else {
        let hours = seconds / 3600.0;
        format!("{:.1} h", hours)
    }
}
