//! AWS Metrics module with focused responsibilities
//!
//! This module provides a clean, maintainable structure for AWS CloudWatch metrics:
//! - `discovery`: Metric discovery and listing from CloudWatch
//! - `units`: Unit determination and formatting for different metric types
//! - `statistics`: Statistic type determination for optimal data collection
//! - `fetcher`: Core metric data fetching and processing logic
//! - `formatter`: Display name and value formatting utilities

pub mod discovery;
pub mod fetcher;
pub mod formatter;
pub mod statistics;
pub mod units;

// Re-export main types and functions for easy access
pub use discovery::{discover_rds_metrics, discover_sqs_metrics};
pub use fetcher::{fetch_discovered_metrics, DynamicMetricData};

// Note: Backward compatibility aliases removed as they were unused
