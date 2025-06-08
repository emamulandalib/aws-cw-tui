//! Modular CloudWatch metrics system
//! 
//! This module provides a trait-based architecture for fetching CloudWatch metrics
//! from different AWS services. It supports extensible service providers while
//! maintaining backward compatibility with existing RDS functionality.

pub mod types;
pub mod providers;
pub mod fetcher;
pub mod factory;

// Re-export commonly used types
// Types are imported directly where needed
