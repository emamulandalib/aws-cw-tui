use std::time::SystemTime;

/// Validates metric data for rendering
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

/// Sanitizes metric data by removing invalid values
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
    use std::time::SystemTime;

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
}
