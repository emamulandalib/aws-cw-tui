use ratatui::{style::Color, text::Line};
use std::time::SystemTime;

/// Calculate optimal Y-axis bounds for metric data
pub fn calculate_y_bounds(history: &[f64]) -> (f64, f64) {
    if history.is_empty() {
        return (0.0, 1.0);
    }

    let min_val = history.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = history.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    if (max_val - min_val).abs() < f64::EPSILON {
        // All values are the same, add some padding
        let padding = if min_val.abs() < f64::EPSILON {
            1.0
        } else {
            min_val.abs() * 0.1
        };
        (min_val - padding, max_val + padding)
    } else {
        // Add 10% padding to the range
        let range = max_val - min_val;
        let padding = range * 0.1;
        ((min_val - padding).max(0.0), max_val + padding)
    }
}

/// Create X-axis labels from timestamps
pub fn create_x_labels(timestamps: &[SystemTime]) -> Vec<Line<'_>> {
    if timestamps.is_empty() {
        return vec![Line::from("No data")];
    }

    let len = timestamps.len();
    if len <= 5 {
        // Show all timestamps for small datasets
        timestamps
            .iter()
            .map(|&ts| {
                let duration = ts
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(duration as i64, 0)
                    .unwrap_or_default();
                Line::from(datetime.format("%H:%M").to_string())
            })
            .collect()
    } else {
        // Show first, middle, and last timestamps for larger datasets
        let indices = vec![0, len / 2, len - 1];
        indices
            .iter()
            .map(|&i| {
                let ts = timestamps[i];
                let duration = ts
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(duration as i64, 0)
                    .unwrap_or_default();
                Line::from(datetime.format("%H:%M").to_string())
            })
            .collect()
    }
}

/// Create Y-axis labels from bounds and metric name
pub fn create_y_labels(y_bounds: [f64; 2], metric_name: &str) -> Vec<Line<'_>> {
    let min_val = y_bounds[0];
    let max_val = y_bounds[1];

    // Create 3-5 labels across the range
    let num_labels = 4;
    let step = (max_val - min_val) / (num_labels - 1) as f64;

    (0..num_labels)
        .map(|i| {
            let value = min_val + (i as f64 * step);
            let formatted_value = format_y_axis_value(value, metric_name);
            Line::from(formatted_value)
        })
        .collect()
}

/// Format Y-axis values based on metric type
pub fn format_y_axis_value(value: f64, metric_name: &str) -> String {
    let metric_lower = metric_name.to_lowercase();

    // Format based on metric type
    if metric_lower.contains("bytes") || metric_lower.contains("size") {
        format_bytes(value)
    } else if metric_lower.contains("percent") || metric_lower.contains("cpu") {
        format!("{:.1}%", value)
    } else if metric_lower.contains("count") || metric_lower.contains("connections") {
        format!("{:.0}", value)
    } else if metric_lower.contains("latency") || metric_lower.contains("duration") {
        format_duration_ms(value)
    } else if value >= 1000.0 {
        format!("{:.1}k", value / 1000.0)
    } else if value >= 100.0 {
        format!("{:.0}", value)
    } else if value >= 10.0 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
    }
}

/// Format byte values with appropriate units
pub fn format_bytes(bytes: f64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes;
    let mut unit_index = 0;

    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if value >= 100.0 {
        format!("{:.0}{}", value, UNITS[unit_index])
    } else if value >= 10.0 {
        format!("{:.1}{}", value, UNITS[unit_index])
    } else {
        format!("{:.2}{}", value, UNITS[unit_index])
    }
}

/// Format duration in milliseconds
pub fn format_duration_ms(ms: f64) -> String {
    if ms >= 1000.0 {
        format!("{:.1}s", ms / 1000.0)
    } else if ms >= 1.0 {
        format!("{:.0}ms", ms)
    } else {
        format!("{:.2}ms", ms)
    }
}

/// Get color for dynamic metrics based on metric name
pub fn get_dynamic_metric_color(metric_name: &str) -> Color {
    let name_lower = metric_name.to_lowercase();

    if name_lower.contains("cpu") {
        Color::Red
    } else if name_lower.contains("memory") || name_lower.contains("ram") {
        Color::Blue
    } else if name_lower.contains("disk") || name_lower.contains("storage") {
        Color::Green
    } else if name_lower.contains("network") || name_lower.contains("bytes") {
        Color::Cyan
    } else if name_lower.contains("connection") || name_lower.contains("count") {
        Color::Yellow
    } else if name_lower.contains("latency") || name_lower.contains("duration") {
        Color::Magenta
    } else if name_lower.contains("error") || name_lower.contains("fault") {
        Color::LightRed
    } else if name_lower.contains("throughput") || name_lower.contains("rate") {
        Color::LightBlue
    } else {
        // Use a consistent hash-based color for unknown metrics
        let hash = metric_name
            .chars()
            .fold(0u32, |acc, c| acc.wrapping_mul(31).wrapping_add(c as u32));
        match hash % 8 {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Cyan,
            3 => Color::Yellow,
            4 => Color::Magenta,
            5 => Color::LightBlue,
            6 => Color::LightGreen,
            _ => Color::LightCyan,
        }
    }
}

/// Calculate maximum value for dynamic metrics
pub fn calculate_dynamic_metric_max(
    metric_data: &crate::aws::dynamic_metric_discovery::DynamicMetricData,
) -> f64 {
    if metric_data.history.is_empty() {
        return 1.0;
    }

    let max_val = metric_data
        .history
        .iter()
        .filter(|&&v| v.is_finite())
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    if max_val.is_finite() && max_val > 0.0 {
        // Add 10% padding
        max_val * 1.1
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_y_bounds_single_value() {
        let history = vec![5.0];
        let (min, max) = calculate_y_bounds(&history);
        assert!(min < 5.0);
        assert!(max > 5.0);
    }

    #[test]
    fn test_calculate_y_bounds_multiple_values() {
        let history = vec![1.0, 5.0, 3.0, 8.0, 2.0];
        let (min, max) = calculate_y_bounds(&history);
        assert!(min <= 1.0);
        assert!(max >= 8.0);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512.0), "512B");
        assert_eq!(format_bytes(1536.0), "1.5KB");
        assert_eq!(format_bytes(1048576.0), "1.0MB");
    }

    #[test]
    fn test_format_duration_ms() {
        assert_eq!(format_duration_ms(0.5), "0.50ms");
        assert_eq!(format_duration_ms(150.0), "150ms");
        assert_eq!(format_duration_ms(2500.0), "2.5s");
    }

    #[test]
    fn test_get_dynamic_metric_color() {
        assert_eq!(get_dynamic_metric_color("CPUUtilization"), Color::Red);
        assert_eq!(get_dynamic_metric_color("MemoryUtilization"), Color::Blue);
        assert_eq!(get_dynamic_metric_color("NetworkIn"), Color::Cyan);
    }
}
