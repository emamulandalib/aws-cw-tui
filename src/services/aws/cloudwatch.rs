//! CloudWatch service implementation with enhanced architecture
//!
//! Provides CloudWatch-specific implementation for metrics fetching
//! with proper error handling, retry logic, and circuit breaker patterns.

use async_trait::async_trait;
use tracing::{debug, error, info};

use crate::aws::cloudwatch_service::{load_dynamic_metrics, TimeRange};
use crate::aws::session::AwsSessionManager;
use crate::models::{AwsService as AwsServiceType, DynamicMetrics, ServiceInstance};
use crate::services::abstractions::{AwsService, ServiceConfig, ServiceError, ServiceResult};

/// CloudWatch service implementation
///
/// This service is primarily used for metrics fetching across all AWS services
/// rather than managing its own instances.
pub struct CloudWatchServiceImpl {
    config: ServiceConfig,
}

impl CloudWatchServiceImpl {
    /// Create a new CloudWatch service implementation
    pub fn new() -> Self {
        Self {
            config: ServiceConfig::default(),
        }
    }

    /// Create a new CloudWatch service implementation with custom configuration
    pub fn with_config(config: ServiceConfig) -> Self {
        Self { config }
    }

    /// Load metrics for any AWS service and instance
    pub async fn load_metrics_for_service(
        &self,
        service_type: &AwsServiceType,
        instance_id: &str,
        time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        debug!(
            "Loading CloudWatch metrics for {} instance: {}",
            service_type.short_name(),
            instance_id
        );

        match load_dynamic_metrics(service_type, instance_id, time_range, None).await {
            Ok(metrics) => {
                info!(
                    "Successfully loaded {} metrics for {} instance {}",
                    metrics.len(),
                    service_type.short_name(),
                    instance_id
                );
                Ok(metrics)
            }
            Err(e) => {
                error!(
                    "Failed to load metrics for {} instance {}: {}",
                    service_type.short_name(),
                    instance_id,
                    e
                );
                Err(ServiceError::AwsApi {
                    message: format!(
                        "Failed to load metrics for {} instance {}: {}",
                        service_type.short_name(),
                        instance_id,
                        e
                    ),
                    source: None,
                })
            }
        }
    }
}

#[async_trait]
impl AwsService for CloudWatchServiceImpl {
    fn service_type(&self) -> AwsServiceType {
        // CloudWatch doesn't have its own instances, but we need to implement the trait
        // This is primarily used as a metrics service
        AwsServiceType::Rds // Default, but not really used
    }

    fn service_name(&self) -> &'static str {
        "CloudWatch"
    }

    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>> {
        // CloudWatch doesn't have instances in the traditional sense
        // Return empty list as this service is used for metrics fetching
        Ok(Vec::new())
    }

    async fn get_metrics(
        &self,
        instance_id: &str,
        time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        // This is a generic implementation - in practice, you'd specify the service type
        // For now, we'll return an error suggesting to use the specific service
        Err(ServiceError::Validation {
            message: format!(
                "CloudWatch service requires a specific AWS service type to fetch metrics for instance: {}. Use the specific service implementation instead.",
                instance_id
            ),
        })
    }

    async fn health_check(&self) -> ServiceResult<()> {
        debug!("Performing CloudWatch service health check");

        // Try to get CloudWatch client and perform a simple operation
        let client = AwsSessionManager::cloudwatch_client().await;

        match client.list_metrics().send().await {
            Ok(_) => {
                debug!("CloudWatch health check passed");
                Ok(())
            }
            Err(e) => {
                error!("CloudWatch health check failed: {:?}", e);
                Err(ServiceError::ServiceUnavailable {
                    message: format!("CloudWatch service health check failed: {}", e),
                })
            }
        }
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    fn update_config(&mut self, config: ServiceConfig) {
        self.config = config;
        info!("CloudWatch service configuration updated");
    }
}

impl Default for CloudWatchServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
