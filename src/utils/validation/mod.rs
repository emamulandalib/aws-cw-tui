// Validation utilities

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

impl std::fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error) = &self.error_message {
            write!(f, "{}", error)
        } else {
            write!(f, "Valid")
        }
    }
}

impl std::error::Error for ValidationResult {}

/// Validate metric data arrays for consistency and validity
pub fn validate_metric_data(
    metric_name: &str,
    values: &[f64],
    timestamps: &[SystemTime],
) -> Result<(), ValidationResult> {
    let mut warnings = Vec::new();

    // Check if data is empty
    if values.is_empty() || timestamps.is_empty() {
        return Err(ValidationResult::invalid(format!(
            "Metric {} has empty data arrays",
            metric_name
        )));
    }

    // Check if arrays have matching lengths
    if values.len() != timestamps.len() {
        return Err(ValidationResult::invalid(format!(
            "Metric {} data length mismatch: values={}, timestamps={}",
            metric_name,
            values.len(),
            timestamps.len()
        )));
    }

    // Check for finite values
    let valid_count = values.iter().filter(|&&v| v.is_finite()).count();
    if valid_count == 0 {
        return Err(ValidationResult::invalid(format!(
            "Metric {} has no valid data points",
            metric_name
        )));
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

    // Check for reasonable timestamp ordering (should be mostly ascending)
    let mut out_of_order = 0;
    for i in 1..timestamps.len() {
        if timestamps[i] < timestamps[i - 1] {
            out_of_order += 1;
        }
    }
    if out_of_order > timestamps.len() / 10 {
        warnings.push(format!(
            "Metric {} has {} out-of-order timestamps out of {}",
            metric_name,
            out_of_order,
            timestamps.len()
        ));
    }

    if warnings.is_empty() {
        Ok(())
    } else {
        Err(ValidationResult::valid().with_warnings(warnings))
    }
}

pub fn validate_non_empty(s: &str) -> bool {
    !s.trim().is_empty()
}
