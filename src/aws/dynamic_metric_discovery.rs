//! Dynamic Metric Discovery - Refactored and Modularized
//!
//! This module has been refactored into focused services under src/aws/metrics/
//! This file now serves as a compatibility wrapper for existing code.
//!
//! All functionality is now split into focused modules:
//! - discovery: Metric discovery and listing from CloudWatch
//! - units: Unit determination and formatting for different metric types
//! - statistics: Statistic type determination for optimal data collection
//! - fetcher: Core metric data fetching and processing logic
//! - formatter: Display name and value formatting utilities

// Re-export only the actively used types and functions for backward compatibility
pub use crate::aws::metrics::{
    discover_rds_metrics, discover_sqs_metrics, fetch_discovered_metrics, DynamicMetricData,
};

// Note: Legacy compatibility imports removed

// Note: Legacy functions removed as they were unused

// Note: All tests have been moved to their respective modules in src/aws/metrics/
// This provides better organization and faster compilation

#[cfg(test)]
mod tests {
    use super::*;
    // Import functions directly for testing
    use crate::aws::metrics::statistics::{
        determine_best_statistic, determine_sqs_statistic,
    };

    #[test]
    fn test_backward_compatibility() {
        // Test that functions still work in the new modules
        assert_eq!(determine_best_statistic("CPUUtilization"), "Average");
        assert_eq!(determine_sqs_statistic("NumberOfMessagesSent"), "Sum");
    }
}
