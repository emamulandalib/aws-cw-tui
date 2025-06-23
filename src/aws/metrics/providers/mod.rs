//! Service-specific metric providers

use super::types::{MetricDefinition, ServiceMetrics};
use crate::models::AwsService;
use std::any::Any;
use std::collections::HashMap;

pub mod rds_provider;

// Re-export the provider implementations
pub use rds_provider::RdsMetricProvider;

/// Core trait that all AWS service metric providers must implement
///
/// This trait abstracts the service-specific details of CloudWatch metrics,
/// allowing for extensible support of different AWS services.
pub trait MetricProvider: Send + Sync {
    /// Returns the CloudWatch namespace for this service (e.g., "AWS/RDS")
    fn get_service_namespace(&self) -> &'static str;

    /// Returns the list of metrics this provider can fetch
    fn get_metrics_config(&self) -> Vec<MetricDefinition>;

    /// Returns dimension mappings for this service
    /// Maps generic dimension names to service-specific ones
    /// (e.g., "instance_id" -> "DBInstanceIdentifier" for RDS)
    fn get_dimension_mappings(&self) -> HashMap<String, String>;

    /// Transforms raw ServiceMetrics into the legacy format for backward compatibility
    /// Returns a boxed Any that can be downcast to the appropriate service-specific type
    fn transform_raw_data(&self, data: ServiceMetrics) -> Box<dyn Any>;

    /// Returns the AWS service type this provider handles
    fn get_service_type(&self) -> AwsService;
}
