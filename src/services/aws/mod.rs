//! AWS service implementations
//!
//! This module contains concrete implementations of the AwsService trait
//! for different AWS services like RDS, SQS, and CloudWatch.

pub mod cloudwatch;
pub mod enhanced_rds;
pub mod enhanced_sqs;
pub mod mocks;
pub mod rds;
pub mod sqs;

// Re-export implementations
pub use cloudwatch::CloudWatchServiceImpl;
pub use rds::RdsServiceImpl;
pub use sqs::SqsServiceImpl;

// Re-export enhanced implementations
pub use enhanced_rds::EnhancedRdsService;
pub use enhanced_sqs::EnhancedSqsService;

// Re-export mock implementations
pub use mocks::{MockRdsService, MockSqsService};
