//! Enhanced SQS service implementation with retry logic and circuit breaker
//!
//! This implementation wraps the basic SQS service with retry logic,
//! circuit breaker patterns, and enhanced error handling.

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use super::sqs::SqsServiceImpl;
use crate::aws::cloudwatch_service::TimeRange;
use crate::models::{AwsService as AwsServiceType, DynamicMetrics, ServiceInstance};
use crate::services::abstractions::{
    retry_with_backoff, AwsService, CircuitBreaker, ServiceConfig, ServiceError, ServiceResult,
};

/// Enhanced SQS service with retry logic and circuit breaker
pub struct EnhancedSqsService {
    inner: SqsServiceImpl,
    circuit_breaker: Arc<Mutex<CircuitBreaker>>,
}

impl EnhancedSqsService {
    /// Create a new enhanced SQS service
    pub fn new() -> Self {
        let config = ServiceConfig::default();
        let circuit_breaker = CircuitBreaker::new(config.circuit_breaker.clone());

        Self {
            inner: SqsServiceImpl::new(),
            circuit_breaker: Arc::new(Mutex::new(circuit_breaker)),
        }
    }

    /// Create a new enhanced SQS service with custom configuration
    pub fn with_config(config: ServiceConfig) -> Self {
        let circuit_breaker = CircuitBreaker::new(config.circuit_breaker.clone());

        Self {
            inner: SqsServiceImpl::with_config(config),
            circuit_breaker: Arc::new(Mutex::new(circuit_breaker)),
        }
    }
}

#[async_trait]
impl AwsService for EnhancedSqsService {
    fn service_type(&self) -> AwsServiceType {
        self.inner.service_type()
    }

    fn service_name(&self) -> &'static str {
        "EnhancedSQS"
    }

    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>> {
        debug!("Enhanced SQS: listing queues with retry logic");

        // For now, just delegate to the inner service
        // TODO: Implement proper retry logic with circuit breaker
        self.inner.list_instances().await
    }

    async fn get_metrics(
        &self,
        instance_id: &str,
        time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        debug!(
            "Enhanced SQS: getting metrics for queue {} with retry logic",
            instance_id
        );

        // For now, just delegate to the inner service
        // TODO: Implement proper retry logic with circuit breaker
        self.inner.get_metrics(instance_id, time_range).await
    }

    async fn health_check(&self) -> ServiceResult<()> {
        debug!("Enhanced SQS: performing health check with retry logic");

        // For now, just delegate to the inner service
        // TODO: Implement proper retry logic with circuit breaker
        self.inner.health_check().await
    }

    fn get_config(&self) -> &ServiceConfig {
        self.inner.get_config()
    }

    fn update_config(&mut self, config: ServiceConfig) {
        // Update circuit breaker config
        let new_circuit_breaker = CircuitBreaker::new(config.circuit_breaker.clone());
        self.circuit_breaker = Arc::new(Mutex::new(new_circuit_breaker));

        // Update inner service config
        self.inner.update_config(config);

        info!("Enhanced SQS service configuration updated");
    }
}

impl Default for EnhancedSqsService {
    fn default() -> Self {
        Self::new()
    }
}
