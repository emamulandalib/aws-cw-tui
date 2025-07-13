use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use ratatui::text::Line;
use std::time::SystemTime;

/// Format metric value for display based on dynamic metric data
pub fn format_dynamic_metric_value(metric_data: &DynamicMetricData) -> String {
    let value = metric_data.current_value;

    // Smart formatting based on metric name patterns
    if metric_data.metric_name.contains("Percent")
        || metric_data.metric_name.contains("Utilization")
    {
        format!("{:.1}%", value)
    } else if metric_data.metric_name.contains("Bytes")
        || metric_data.metric_name.contains("Storage")
    {
        format_bytes(value)
    } else if metric_data.metric_name.contains("Latency")
        || metric_data.metric_name.contains("Duration")
    {
        format_duration(value)
    } else if value >= 1000.0 {
        format!("{:.2}K", value / 1000.0)
    } else if value >= 1.0 {
        format!("{:.2}", value)
    } else {
        format!("{:.3}", value)
    }
}

/// Format bytes value with appropriate units
pub fn format_bytes(bytes: f64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = bytes;
    let mut unit_index = 0;

    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if value >= 100.0 {
        format!("{:.0} {}", value, UNITS[unit_index])
    } else if value >= 10.0 {
        format!("{:.1} {}", value, UNITS[unit_index])
    } else {
        format!("{:.2} {}", value, UNITS[unit_index])
    }
}

/// Format duration in seconds with appropriate units
pub fn format_duration(seconds: f64) -> String {
    if seconds >= 1.0 {
        if seconds >= 60.0 {
            let minutes = seconds / 60.0;
            if minutes >= 60.0 {
                format!("{:.1}h", minutes / 60.0)
            } else {
                format!("{:.1}m", minutes)
            }
        } else {
            format!("{:.1}s", seconds)
        }
    } else {
        format!("{:.0}ms", seconds * 1000.0)
    }
}

/// Create X-axis labels from timestamps
pub fn create_x_labels(timestamps: &[SystemTime]) -> Vec<Line<'_>> {
    if timestamps.is_empty() {
        return vec![Line::from("No data")];
    }

    let now = SystemTime::now();
    let labels: Vec<String> = timestamps
        .iter()
        .step_by(timestamps.len().max(6) / 6) // Show about 6 labels
        .map(|&timestamp| match now.duration_since(timestamp) {
            Ok(duration) => {
                let hours = duration.as_secs() / 3600;
                if hours == 0 {
                    "now".to_string()
                } else {
                    format!("-{}h", hours)
                }
            }
            Err(_) => "future".to_string(),
        })
        .collect();

    labels.into_iter().map(Line::from).collect()
}

/// Create Y-axis labels with proper formatting
pub fn create_y_labels(y_bounds: [f64; 2], metric_name: &str) -> Vec<Line<'_>> {
    let min_val = y_bounds[0];
    let max_val = y_bounds[1];

    // Generate 5 evenly spaced labels
    let step = (max_val - min_val) / 4.0;
    let mut labels = Vec::new();

    for i in 0..5 {
        let value = min_val + (step * i as f64);
        let formatted = if metric_name.contains("Percent") || metric_name.contains("Utilization") {
            format!("{:.1}%", value)
        } else if metric_name.contains("Bytes") || metric_name.contains("Storage") {
            format_bytes(value)
        } else if metric_name.contains("Latency") || metric_name.contains("Duration") {
            format_duration(value)
        } else if value >= 1000.0 {
            format!("{:.1}K", value / 1000.0)
        } else {
            format!("{:.1}", value)
        };
        labels.push(Line::from(formatted));
    }

    labels
}

/// Format percentage with appropriate precision
pub fn format_percentage(value: f64) -> String {
    if value >= 10.0 {
        format!("{:.1}%", value)
    } else {
        format!("{:.2}%", value)
    }
}

/// Format count/integer values with appropriate scaling
pub fn format_count(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else if value >= 1.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

/// Format IOPS (Input/Output Operations Per Second)
pub fn format_iops(value: f64) -> String {
    format_count(value)
}

/// Format throughput (bytes per second)
pub fn format_throughput(value: f64) -> String {
    format!("{}/s", format_bytes(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512.0), "512 B");
        assert_eq!(format_bytes(1536.0), "1.50 KB");
        assert_eq!(format_bytes(1_048_576.0), "1.00 MB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.5), "500ms");
        assert_eq!(format_duration(1.5), "1.5s");
        assert_eq!(format_duration(90.0), "1.5m");
        assert_eq!(format_duration(3665.0), "1.0h");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(1.234), "1.23%");
        assert_eq!(format_percentage(12.34), "12.3%");
        assert_eq!(format_percentage(99.9), "99.9%");
    }

    #[test]
    fn test_format_count() {
        assert_eq!(format_count(123.0), "123");
        assert_eq!(format_count(1234.0), "1.2K");
        assert_eq!(format_count(1_234_567.0), "1.2M");
    }
}
