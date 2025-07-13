use std::time::SystemTime;

/// Validation result for metric data
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            error_message: None,
            warnings: Vec::new(),
        }
    }

    pub fn invalid(message: String) -> Self {
        Self {
            is_valid: false,
            error_message: Some(message),
            warnings: Vec::new(),
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
}

/// Validate metric data arrays for consistency and validity
pub fn validate_metric_data(
    metric_name: &str,
    values: &[f64],
    timestamps: &[SystemTime],
) -> ValidationResult {
    let mut warnings = Vec::new();

    // Check if data is empty
    if values.is_empty() || timestamps.is_empty() {
        return ValidationResult::invalid(format!("Metric {} has empty data arrays", metric_name));
    }

    // Check if arrays have matching lengths
    if values.len() != timestamps.len() {
        return ValidationResult::invalid(format!(
            "Metric {} data length mismatch: values={}, timestamps={}",
            metric_name,
            values.len(),
            timestamps.len()
        ));
    }

    // Check for finite values
    let valid_count = values.iter().filter(|&&v| v.is_finite()).count();
    if valid_count == 0 {
        return ValidationResult::invalid(format!(
            "Metric {} has no valid data points",
            metric_name
        ));
    }

    // Warn about invalid values but continue processing
    if valid_count < values.len() {
        warnings.push(format!(
            "Metric {} has {} invalid values out of {}",
            metric_name,
            values.len() - valid_count,
            values.len()
        ));
    }

    // Validate timestamps are chronologically ordered
    for i in 1..timestamps.len() {
        if timestamps[i] < timestamps[i - 1] {
            return ValidationResult::invalid(format!(
                "Metric {} has unordered timestamps at index {}",
                metric_name, i
            ));
        }
    }

    ValidationResult::valid().with_warnings(warnings)
}

/// Sanitize metric values by replacing invalid values with interpolated ones
pub fn sanitize_metric_values(values: &[f64]) -> Vec<f64> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut sanitized = values.to_vec();

    // Replace infinite and NaN values with interpolated values
    for i in 0..sanitized.len() {
        if !sanitized[i].is_finite() {
            sanitized[i] = interpolate_value(&sanitized, i);
        }
    }

    sanitized
}

/// Interpolate a value at the given index based on surrounding valid values
fn interpolate_value(values: &[f64], index: usize) -> f64 {
    // Find the nearest valid values before and after the index
    let mut prev_valid = None;
    let mut next_valid = None;

    // Look backwards for a valid value
    for i in (0..index).rev() {
        if values[i].is_finite() {
            prev_valid = Some((i, values[i]));
            break;
        }
    }

    // Look forwards for a valid value
    for i in (index + 1)..values.len() {
        if values[i].is_finite() {
            next_valid = Some((i, values[i]));
            break;
        }
    }

    match (prev_valid, next_valid) {
        (Some((_, prev_val)), Some((next_idx, next_val))) => {
            // Linear interpolation
            let prev_idx = prev_valid.unwrap().0;
            let weight = (index - prev_idx) as f64 / (next_idx - prev_idx) as f64;
            prev_val + weight * (next_val - prev_val)
        }
        (Some((_, prev_val)), None) => prev_val, // Use previous value
        (None, Some((_, next_val))) => next_val, // Use next value
        (None, None) => 0.0,                     // Fallback to zero if no valid values found
    }
}

/// Validate SQS-specific metric constraints
pub fn validate_sqs_metric_constraints(metric_name: &str, value: f64) -> ValidationResult {
    let mut warnings = Vec::new();

    match metric_name {
        "NumberOfMessagesSent" | "NumberOfMessagesReceived" | "NumberOfMessagesDeleted" => {
            if value < 0.0 {
                return ValidationResult::invalid(format!(
                    "Metric {} cannot have negative values: {}",
                    metric_name, value
                ));
            }
        }
        "ApproximateNumberOfMessages"
        | "ApproximateNumberOfMessagesVisible"
        | "ApproximateNumberOfMessagesNotVisible"
        | "ApproximateNumberOfMessagesDelayed" => {
            if value < 0.0 {
                return ValidationResult::invalid(format!(
                    "Queue depth metric {} cannot be negative: {}",
                    metric_name, value
                ));
            }
            if value > 1_000_000.0 {
                warnings.push(format!(
                    "Queue depth metric {} is unusually high: {}",
                    metric_name, value
                ));
            }
        }
        "ApproximateAgeOfOldestMessage" => {
            if value < 0.0 {
                return ValidationResult::invalid(format!(
                    "Message age cannot be negative: {}",
                    value
                ));
            }
            if value > 1_209_600.0 {
                // 14 days in seconds
                warnings.push(format!(
                    "Message age is very high ({}s), possible retention issue",
                    value
                ));
            }
        }
        "SentMessageSize" => {
            if value < 0.0 {
                return ValidationResult::invalid(format!(
                    "Message size cannot be negative: {}",
                    value
                ));
            }
            if value > 262_144.0 {
                // 256KB max message size
                warnings.push(format!("Message size ({} bytes) exceeds SQS limit", value));
            }
        }
        _ => {} // No specific validation for other metrics
    }

    ValidationResult::valid().with_warnings(warnings)
}

/// Validate timestamp ordering and reasonable time ranges
pub fn validate_timestamps(timestamps: &[SystemTime]) -> ValidationResult {
    if timestamps.is_empty() {
        return ValidationResult::invalid("Empty timestamp array".to_string());
    }

    let now = SystemTime::now();
    let mut warnings = Vec::new();

    // Check for reasonable time range (not more than 1 year old)
    let one_year_ago = now - std::time::Duration::from_secs(365 * 24 * 3600);

    for (i, &timestamp) in timestamps.iter().enumerate() {
        if timestamp < one_year_ago {
            warnings.push(format!("Timestamp at index {} is more than 1 year old", i));
        }
        if timestamp > now {
            warnings.push(format!("Timestamp at index {} is in the future", i));
        }
    }

    // Check chronological ordering
    for i in 1..timestamps.len() {
        if timestamps[i] < timestamps[i - 1] {
            return ValidationResult::invalid(format!(
                "Timestamps are not in chronological order at index {}",
                i
            ));
        }
    }

    ValidationResult::valid().with_warnings(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_validate_metric_data_valid() {
        let values = vec![1.0, 2.0, 3.0];
        let timestamps = create_test_timestamps(3);

        let result = validate_metric_data("TestMetric", &values, &timestamps);
        assert!(result.is_valid);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_validate_metric_data_empty() {
        let values = vec![];
        let timestamps = vec![];

        let result = validate_metric_data("TestMetric", &values, &timestamps);
        assert!(!result.is_valid);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_validate_metric_data_length_mismatch() {
        let values = vec![1.0, 2.0];
        let timestamps = create_test_timestamps(3);

        let result = validate_metric_data("TestMetric", &values, &timestamps);
        assert!(!result.is_valid);
        assert!(result.error_message.unwrap().contains("length mismatch"));
    }

    #[test]
    fn test_sanitize_metric_values() {
        let values = vec![1.0, f64::NAN, 3.0, f64::INFINITY, 5.0];
        let sanitized = sanitize_metric_values(&values);

        assert_eq!(sanitized.len(), 5);
        assert!(sanitized.iter().all(|&v| v.is_finite()));
        assert_eq!(sanitized[0], 1.0);
        assert_eq!(sanitized[2], 3.0);
        assert_eq!(sanitized[4], 5.0);
    }

    #[test]
    fn test_validate_sqs_metric_constraints() {
        // Test valid message count
        let result = validate_sqs_metric_constraints("NumberOfMessagesSent", 100.0);
        assert!(result.is_valid);

        // Test negative message count (invalid)
        let result = validate_sqs_metric_constraints("NumberOfMessagesSent", -1.0);
        assert!(!result.is_valid);

        // Test very high queue depth (warning)
        let result = validate_sqs_metric_constraints("ApproximateNumberOfMessages", 2_000_000.0);
        assert!(result.is_valid);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_interpolate_value() {
        let values = vec![1.0, f64::NAN, 3.0];
        let interpolated = interpolate_value(&values, 1);
        assert_eq!(interpolated, 2.0); // Linear interpolation between 1.0 and 3.0
    }

    fn create_test_timestamps(count: usize) -> Vec<SystemTime> {
        let now = SystemTime::now();
        (0..count)
            .map(|i| now - Duration::from_secs((count - 1 - i) as u64 * 60))
            .collect()
    }
}
