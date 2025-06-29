// AWS module exports
pub mod cloudwatch_service;
pub mod rds_service;

// Error handling utilities
pub mod error_utils;

// AWS Session Management - centralized config and client management
pub mod session;

// New refactored modules
pub mod metric_builder;
pub mod metric_fetcher;
pub mod metric_types;
pub mod time_range;

// New modular metrics system
pub mod metrics;

// RDS-focused service organization
pub mod rds;

// Keep existing exports for backward compatibility
pub use rds_service::load_rds_instances;
