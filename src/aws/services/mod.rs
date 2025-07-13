//! AWS services module
//!
//! Contains refactored AWS service implementations with proper separation of concerns.
//! Each service has its own module with dedicated fetchers, validators, and mappers.

pub mod sqs;

// Note: Re-exports removed as they were unused

// TODO: Add RDS service module when ready
// pub mod rds;
// pub use rds::{RdsMetricsFetcher, fetch_rds_metrics};
