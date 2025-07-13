use std::time::SystemTime;

/// Validate metric data for chart rendering
pub fn validate_metric_data(history: &[f64], timestamps: &[SystemTime]) -> Result<(), String> {
    // Check if data is empty
    if history.is_empty() || timestamps.is_empty() {
        return Err("Empty data arrays".to_string());
    }

    // Check if arrays have matching lengths
    if history.len() != timestamps.len() {
        return Err(format!(
            "Data length mismatch: history has {} points, timestamps has {} points",
            history.len(),
            timestamps.len()
        ));
    }

    // Check for invalid values
    for (i, &value) in history.iter().enumerate() {
        if !value.is_finite() {
            return Err(format!("Invalid value at index {}: {}", i, value));
        }
    }

    Ok(())
}

/// Sanitize metric data by removing invalid values
pub fn sanitize_metric_data(
    history: &[f64],
    timestamps: &[SystemTime],
) -> (Vec<f64>, Vec<SystemTime>) {
    let mut clean_history = Vec::new();
    let mut clean_timestamps = Vec::new();

    for (i, (&value, &timestamp)) in history.iter().zip(timestamps.iter()).enumerate() {
        if value.is_finite() {
            clean_history.push(value);
            clean_timestamps.push(timestamp);
        } else {
            log::warn!("Skipping invalid metric value at index {}: {}", i, value);
        }
    }

    (clean_history, clean_timestamps)
}

/// Calculate appropriate Y-axis bounds for chart data
pub fn calculate_y_bounds(history: &[f64]) -> (f64, f64) {
    if history.is_empty() {
        return (0.0, 1.0);
    }

    let valid_values: Vec<f64> = history
        .iter()
        .filter(|&&v| v.is_finite())
        .copied()
        .collect();

    if valid_values.is_empty() {
        return (0.0, 1.0);
    }

    let min_val = valid_values
        .iter()
        .fold(f64::INFINITY, |acc, &x| acc.min(x));
    let max_val = valid_values
        .iter()
        .fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));

    // Handle case where all values are the same
    if (max_val - min_val).abs() < f64::EPSILON {
        if min_val == 0.0 {
            return (0.0, 1.0);
        }
        let padding = min_val.abs() * 0.1;
        return (min_val - padding, min_val + padding);
    }

    // Add 5-10% padding
    let range = max_val - min_val;
    let padding = range * 0.05;

    let adjusted_min = (min_val - padding).max(0.0);
    let adjusted_max = max_val + padding;

    (adjusted_min, adjusted_max)
}

/// Validate timestamps are in chronological order
pub fn validate_timestamps(timestamps: &[SystemTime]) -> Result<(), String> {
    for i in 1..timestamps.len() {
        if timestamps[i] < timestamps[i - 1] {
            return Err(format!(
                "Timestamps are not in chronological order at index {}",
                i
            ));
        }
    }
    Ok(())
}

/// Validates that timestamps are available for the current app state
pub fn validate_timestamps_available(app: &crate::models::App) -> bool {
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        // Dynamic metrics system: check if any dynamic metric has timestamps
        !dynamic_metrics.is_empty()
            && dynamic_metrics
                .metrics
                .iter()
                .any(|m| !m.timestamps.is_empty())
    } else {
        // Legacy system: check service-specific timestamps
        match app
            .selected_service
            .as_ref()
            .unwrap_or(&crate::models::AwsService::Rds)
        {
            crate::models::AwsService::Rds => !app.metrics.timestamps.is_empty(),
            crate::models::AwsService::Sqs => !app.sqs_metrics.timestamps.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn test_validate_metric_data_empty() {
        let result = validate_metric_data(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_metric_data_mismatch() {
        let history = vec![1.0, 2.0];
        let timestamps = vec![SystemTime::now()];
        let result = validate_metric_data(&history, &timestamps);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_metric_data_invalid_values() {
        let history = vec![1.0, f64::NAN, 3.0];
        let timestamps = vec![SystemTime::now(); 3];
        let result = validate_metric_data(&history, &timestamps);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_metric_data_valid() {
        let history = vec![1.0, 2.0, 3.0];
        let timestamps = vec![SystemTime::now(); 3];
        let result = validate_metric_data(&history, &timestamps);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sanitize_metric_data() {
        let history = vec![1.0, f64::NAN, 3.0, f64::INFINITY];
        let timestamps = vec![SystemTime::now(); 4];
        let (clean_history, clean_timestamps) = sanitize_metric_data(&history, &timestamps);
        assert_eq!(clean_history.len(), 2);
        assert_eq!(clean_timestamps.len(), 2);
        assert_eq!(clean_history, vec![1.0, 3.0]);
    }

    #[test]
    fn test_calculate_y_bounds_single_value() {
        let history = vec![5.0];
        let (min_bound, max_bound) = calculate_y_bounds(&history);
        assert!(min_bound < 5.0);
        assert!(max_bound > 5.0);
    }

    #[test]
    fn test_calculate_y_bounds_multiple_values() {
        let history = vec![1.0, 5.0, 3.0];
        let (min_bound, max_bound) = calculate_y_bounds(&history);
        assert!(min_bound <= 1.0);
        assert!(max_bound >= 5.0);
    }
}
