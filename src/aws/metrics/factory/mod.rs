//! Factory for creating metric service providers

pub mod provider_factory;

// Re-export the main factory
pub use provider_factory::MetricServiceFactory;
