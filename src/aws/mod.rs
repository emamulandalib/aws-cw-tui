// AWS module exports
pub mod cloudwatch_service;
pub mod rds_service;

// New refactored modules
pub mod metric_builder;
pub mod metric_fetcher;
pub mod metric_types;
pub mod time_range;

// Removed unused exports - only keeping what's needed
pub use rds_service::load_rds_instances;
