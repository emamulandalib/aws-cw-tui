//! SQS service module with focused responsibilities
//!
//! This module provides a clean, maintainable structure for SQS CloudWatch metrics:
//! - `utils`: Time range calculation and queue utilities
//! - `validator`: Data validation and sanitization
//! - `mapper`: CloudWatch metrics to SqsMetricData mapping
//! - `fetcher`: Core metric fetching logic

pub mod fetcher;
pub mod mapper;
pub mod utils;
pub mod validator;

// Note: Re-exports removed as they were unused

// Note: Convenience function removed as it was unused
