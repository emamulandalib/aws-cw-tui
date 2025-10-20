//! Instance-related UI components

pub mod rds_instance;
pub mod sqs_queue;

// Re-export main functions for backward compatibility
pub use rds_instance::*;
pub use sqs_queue::*;
