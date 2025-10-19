//! SQS service implementation with enhanced architecture
//!
//! Provides SQS-specific implementation of the AwsService trait with
//! proper error handling, retry logic, and circuit breaker patterns.

use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

use crate::aws::cloudwatch_service::{load_dynamic_metrics, TimeRange};
use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use crate::models::{AwsService as AwsServiceType, DynamicMetrics, ServiceInstance, SqsQueue};
use crate::services::abstractions::{AwsService, ServiceConfig, ServiceError, ServiceResult};

/// SQS service implementation
pub struct SqsServiceImpl {
    config: ServiceConfig,
}

impl SqsServiceImpl {
    /// Create a new SQS service implementation
    pub fn new() -> Self {
        Self {
            config: ServiceConfig::default(),
        }
    }

    /// Create a new SQS service implementation with custom configuration
    pub fn with_config(config: ServiceConfig) -> Self {
        Self { config }
    }

    /// Load SQS queues with enhanced error handling
    async fn load_queues_internal(&self) -> ServiceResult<Vec<SqsQueue>> {
        debug!("Starting SQS queue loading via AWS SDK...");

        // Use shared AWS session manager for SQS client
        let client = AwsSessionManager::sqs_client().await;
        debug!("SQS client obtained from session manager");

        debug!("Sending list_queues API call...");
        let resp = match client.list_queues().send().await {
            Ok(resp) => {
                debug!("Successfully received SQS API response");
                resp
            }
            Err(e) => {
                error!("SQS API call failed: {:?}", e);
                return Err(ServiceError::AwsApi {
                    message: format!("Failed to fetch SQS queues: {}", e),
                    source: None,
                });
            }
        };

        let mut queues = Vec::new();

        if let Some(queue_urls) = resp.queue_urls {
            info!(
                "Processing {} SQS queues from API response",
                queue_urls.len()
            );

            for (index, url) in queue_urls.iter().enumerate() {
                debug!("Processing SQS queue {}: {}", index + 1, url);

                // Extract queue name from URL
                let name = url.split('/').next_back().unwrap_or("unknown").to_string();

                // Determine queue type based on name (.fifo suffix)
                let queue_type = if name.ends_with(".fifo") {
                    "FIFO".to_string()
                } else {
                    "Standard".to_string()
                };

                // Get queue attributes for additional info
                let mut attributes = HashMap::new();
                match client
                    .get_queue_attributes()
                    .queue_url(url)
                    .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
                    .send()
                    .await
                {
                    Ok(attr_resp) => {
                        if let Some(attrs) = attr_resp.attributes {
                            for (key, value) in attrs {
                                attributes.insert(key.as_str().to_string(), value);
                            }
                        }
                        debug!(
                            "Retrieved {} attributes for queue {}",
                            attributes.len(),
                            name
                        );
                    }
                    Err(e) => {
                        warn!("Failed to get attributes for queue {}: {}", name, e);
                        // Continue without attributes rather than failing completely
                    }
                }

                let sqs_queue = SqsQueue {
                    url: url.clone(),
                    name,
                    queue_type,
                    attributes,
                };
                debug!("Created SQS queue: {:?}", sqs_queue);
                queues.push(sqs_queue);
            }
        } else {
            warn!("No SQS queues found in API response");
        }

        info!("Successfully loaded {} SQS queues", queues.len());
        Ok(queues)
    }
}

#[async_trait]
impl AwsService for SqsServiceImpl {
    fn service_type(&self) -> AwsServiceType {
        AwsServiceType::Sqs
    }

    fn service_name(&self) -> &'static str {
        "SQS"
    }

    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>> {
        let sqs_queues = self.load_queues_internal().await?;
        let service_instances = sqs_queues.into_iter().map(ServiceInstance::Sqs).collect();

        Ok(service_instances)
    }

    async fn get_metrics(
        &self,
        instance_id: &str,
        time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        debug!("Loading metrics for SQS queue: {}", instance_id);

        match load_dynamic_metrics(&AwsServiceType::Sqs, instance_id, time_range, None).await {
            Ok(metrics) => {
                info!(
                    "Successfully loaded {} metrics for SQS queue {}",
                    metrics.len(),
                    instance_id
                );
                Ok(metrics)
            }
            Err(e) => {
                error!(
                    "Failed to load metrics for SQS queue {}: {}",
                    instance_id, e
                );
                Err(ServiceError::AwsApi {
                    message: format!(
                        "Failed to load metrics for SQS queue {}: {}",
                        instance_id, e
                    ),
                    source: None,
                })
            }
        }
    }

    async fn health_check(&self) -> ServiceResult<()> {
        debug!("Performing SQS service health check");

        // Try to get SQS client and perform a simple operation
        let client = AwsSessionManager::sqs_client().await;

        match client.list_queues().max_results(1).send().await {
            Ok(_) => {
                debug!("SQS health check passed");
                Ok(())
            }
            Err(e) => {
                error!("SQS health check failed: {:?}", e);
                Err(ServiceError::ServiceUnavailable {
                    message: format!("SQS service health check failed: {}", e),
                })
            }
        }
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    fn update_config(&mut self, config: ServiceConfig) {
        self.config = config;
        info!("SQS service configuration updated");
    }
}

impl Default for SqsServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
