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
pub mod time_range;

// Refactored services with clean architecture
pub mod services;

// RDS-focused service organization
pub mod rds;

// New modular metrics system
pub mod metrics;

// Keep existing exports for backward compatibility
pub use rds_service::load_rds_instances;
