//! RDS service implementation with enhanced architecture
//!
//! Provides RDS-specific implementation of the AwsService trait with
//! proper error handling, retry logic, and circuit breaker patterns.

use async_trait::async_trait;
use std::time::SystemTime;
use tracing::{debug, error, info, warn};

use crate::aws::cloudwatch_service::{load_dynamic_metrics, TimeRange};
use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use crate::models::{AwsService as AwsServiceType, DynamicMetrics, RdsInstance, ServiceInstance};
use crate::services::abstractions::{AwsService, ServiceConfig, ServiceError, ServiceResult};

/// RDS service implementation
pub struct RdsServiceImpl {
    config: ServiceConfig,
}

impl RdsServiceImpl {
    /// Create a new RDS service implementation
    pub fn new() -> Self {
        Self {
            config: ServiceConfig::default(),
        }
    }

    /// Create a new RDS service implementation with custom configuration
    pub fn with_config(config: ServiceConfig) -> Self {
        Self { config }
    }

    /// Load RDS instances with enhanced error handling
    async fn load_instances_internal(&self) -> ServiceResult<Vec<RdsInstance>> {
        debug!("Starting RDS instance loading via AWS SDK...");

        // Use shared AWS session manager for RDS client
        let client = AwsSessionManager::rds_client().await;
        debug!("RDS client obtained from session manager");

        debug!("Sending describe_db_instances API call...");
        let resp = match client.describe_db_instances().send().await {
            Ok(resp) => {
                debug!("Successfully received RDS API response");
                resp
            }
            Err(e) => {
                error!("RDS API call failed: {:?}", e);
                return Err(ServiceError::AwsApi {
                    message: format!("Failed to fetch RDS instances: {}", e),
                    source: None,
                });
            }
        };

        let mut instances = Vec::new();

        if let Some(db_instances) = resp.db_instances {
            info!(
                "Processing {} RDS instances from API response",
                db_instances.len()
            );

            for (index, instance) in db_instances.iter().enumerate() {
                debug!(
                    "Processing RDS instance {}: {:?}",
                    index + 1,
                    instance.db_instance_identifier
                );
                let rds_instance = RdsInstance {
                    id: instance.db_instance_identifier.clone().unwrap_or_default(),
                    name: instance.db_instance_identifier.clone().unwrap_or_default(),
                    identifier: instance.db_instance_identifier.clone().unwrap_or_default(),
                    engine: instance.engine.clone().unwrap_or_default(),
                    status: instance.db_instance_status.clone().unwrap_or_default(),
                    instance_class: instance.db_instance_class.clone().unwrap_or_default(),
                    endpoint: instance.endpoint.as_ref().and_then(|e| e.address.clone()),
                    port: instance.endpoint.as_ref().and_then(|e| e.port),
                    engine_version: instance.engine_version.clone(),
                    allocated_storage: instance.allocated_storage,
                    storage_type: instance.storage_type.clone(),
                    availability_zone: instance.availability_zone.clone(),
                    backup_retention_period: instance.backup_retention_period,
                    multi_az: instance.multi_az,
                    storage_encrypted: instance.storage_encrypted,
                    performance_insights_enabled: instance.performance_insights_enabled,
                    deletion_protection: instance.deletion_protection,
                    creation_time: instance.instance_create_time.map(|t| {
                        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(t.secs() as u64)
                    }),
                };
                debug!("Created RDS instance: {:?}", rds_instance);
                instances.push(rds_instance);
            }
        } else {
            warn!("No RDS instances found in API response");
        }

        info!("Successfully loaded {} RDS instances", instances.len());
        Ok(instances)
    }
}

#[async_trait]
impl AwsService for RdsServiceImpl {
    fn service_type(&self) -> AwsServiceType {
        AwsServiceType::Rds
    }

    fn service_name(&self) -> &'static str {
        "RDS"
    }

    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>> {
        let rds_instances = self.load_instances_internal().await?;
        let service_instances = rds_instances
            .into_iter()
            .map(ServiceInstance::Rds)
            .collect();

        Ok(service_instances)
    }

    async fn get_metrics(
        &self,
        instance_id: &str,
        time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        debug!("Loading metrics for RDS instance: {}", instance_id);

        match load_dynamic_metrics(&AwsServiceType::Rds, instance_id, time_range, None).await {
            Ok(metrics) => {
                info!(
                    "Successfully loaded {} metrics for RDS instance {}",
                    metrics.len(),
                    instance_id
                );
                Ok(metrics)
            }
            Err(e) => {
                error!(
                    "Failed to load metrics for RDS instance {}: {}",
                    instance_id, e
                );
                Err(ServiceError::AwsApi {
                    message: format!(
                        "Failed to load metrics for RDS instance {}: {}",
                        instance_id, e
                    ),
                    source: None,
                })
            }
        }
    }

    async fn health_check(&self) -> ServiceResult<()> {
        debug!("Performing RDS service health check");

        // Try to get RDS client and perform a simple operation
        let client = AwsSessionManager::rds_client().await;

        match client.describe_db_instances().max_records(1).send().await {
            Ok(_) => {
                debug!("RDS health check passed");
                Ok(())
            }
            Err(e) => {
                error!("RDS health check failed: {:?}", e);
                Err(ServiceError::ServiceUnavailable {
                    message: format!("RDS service health check failed: {}", e),
                })
            }
        }
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    fn update_config(&mut self, config: ServiceConfig) {
        self.config = config;
        info!("RDS service configuration updated");
    }
}

impl Default for RdsServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
