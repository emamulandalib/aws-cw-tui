// AWS module exports
pub mod cloudwatch_service;
pub mod dynamic_metric_discovery;
pub mod rds_service;
pub mod sqs_metrics;
pub mod sqs_service;

// Error handling utilities
pub mod error_utils;

// AWS Session Management - centralized config and client management
pub mod session;

// New refactored modules
pub mod metric_fetcher;
pub mod metric_types;
pub mod time_range;

// New modular metrics system - removed (unused)

// RDS-focused service organization
pub mod rds;

// Keep existing exports for backward compatibility
pub use rds_service::load_rds_instances;
