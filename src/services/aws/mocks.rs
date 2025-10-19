//! Mock service implementations for testing
//!
//! Provides mock implementations of AWS services that can be used
//! in tests without making actual AWS API calls.

use async_trait::async_trait;
use std::collections::HashMap;
use std::time::SystemTime;
use tracing::{debug, info};

use crate::aws::cloudwatch_service::TimeRange;
use crate::models::{
    AwsService as AwsServiceType, DynamicMetrics, MetricType, RdsInstance, ServiceInstance,
    SqsQueue,
};
use crate::services::abstractions::{AwsService, ServiceConfig, ServiceError, ServiceResult};

/// Mock RDS service for testing
pub struct MockRdsService {
    config: ServiceConfig,
    instances: Vec<RdsInstance>,
    should_fail: bool,
    failure_message: Option<String>,
}

impl MockRdsService {
    /// Create a new mock RDS service with default test data
    pub fn new() -> Self {
        let instances = vec![
            RdsInstance {
                id: "test-db-1".to_string(),
                name: "test-db-1".to_string(),
                identifier: "test-db-1".to_string(),
                engine: "mysql".to_string(),
                status: "available".to_string(),
                instance_class: "db.t3.micro".to_string(),
                endpoint: Some("test-db-1.cluster-xyz.us-east-1.rds.amazonaws.com".to_string()),
                port: Some(3306),
                engine_version: Some("8.0.35".to_string()),
                allocated_storage: Some(20),
                storage_type: Some("gp2".to_string()),
                availability_zone: Some("us-east-1a".to_string()),
                backup_retention_period: Some(7),
                multi_az: Some(false),
                storage_encrypted: Some(true),
                performance_insights_enabled: Some(false),
                deletion_protection: Some(false),
                creation_time: Some(SystemTime::now()),
            },
            RdsInstance {
                id: "test-db-2".to_string(),
                name: "test-db-2".to_string(),
                identifier: "test-db-2".to_string(),
                engine: "postgres".to_string(),
                status: "available".to_string(),
                instance_class: "db.t3.small".to_string(),
                endpoint: Some("test-db-2.cluster-abc.us-east-1.rds.amazonaws.com".to_string()),
                port: Some(5432),
                engine_version: Some("15.4".to_string()),
                allocated_storage: Some(100),
                storage_type: Some("gp3".to_string()),
                availability_zone: Some("us-east-1b".to_string()),
                backup_retention_period: Some(14),
                multi_az: Some(true),
                storage_encrypted: Some(true),
                performance_insights_enabled: Some(true),
                deletion_protection: Some(true),
                creation_time: Some(SystemTime::now()),
            },
        ];

        Self {
            config: ServiceConfig::default(),
            instances,
            should_fail: false,
            failure_message: None,
        }
    }

    /// Create a mock service that will fail operations
    pub fn with_failure(message: String) -> Self {
        let mut service = Self::new();
        service.should_fail = true;
        service.failure_message = Some(message);
        service
    }

    /// Add a custom instance to the mock service
    pub fn with_instance(mut self, instance: RdsInstance) -> Self {
        self.instances.push(instance);
        self
    }

    /// Set custom instances for the mock service
    pub fn with_instances(mut self, instances: Vec<RdsInstance>) -> Self {
        self.instances = instances;
        self
    }

    /// Create mock metrics for testing
    fn create_mock_metrics(&self, instance_id: &str) -> DynamicMetrics {
        let mut metrics = DynamicMetrics::new(AwsServiceType::Rds, instance_id.to_string());

        // Add some mock metric data
        let mock_data = vec![
            (chrono::Utc::now().timestamp() as f64, 75.5),
            (chrono::Utc::now().timestamp() as f64 - 300.0, 80.2),
            (chrono::Utc::now().timestamp() as f64 - 600.0, 72.1),
        ];

        metrics.add_metric(
            "CPUUtilization".to_string(),
            MetricType::Percentage,
            mock_data.clone(),
        );
        metrics.add_metric(
            "DatabaseConnections".to_string(),
            MetricType::Count,
            mock_data,
        );

        metrics
    }
}

#[async_trait]
impl AwsService for MockRdsService {
    fn service_type(&self) -> AwsServiceType {
        AwsServiceType::Rds
    }

    fn service_name(&self) -> &'static str {
        "MockRDS"
    }

    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>> {
        debug!("Mock RDS: listing instances");

        if self.should_fail {
            return Err(ServiceError::AwsApi {
                message: self
                    .failure_message
                    .clone()
                    .unwrap_or_else(|| "Mock failure".to_string()),
                source: None,
            });
        }

        let service_instances: Vec<ServiceInstance> = self
            .instances
            .clone()
            .into_iter()
            .map(ServiceInstance::Rds)
            .collect();

        info!("Mock RDS: returning {} instances", service_instances.len());
        Ok(service_instances)
    }

    async fn get_metrics(
        &self,
        instance_id: &str,
        _time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        debug!("Mock RDS: getting metrics for instance {}", instance_id);

        if self.should_fail {
            return Err(ServiceError::AwsApi {
                message: self
                    .failure_message
                    .clone()
                    .unwrap_or_else(|| "Mock failure".to_string()),
                source: None,
            });
        }

        // Check if instance exists
        if !self.instances.iter().any(|i| i.id == instance_id) {
            return Err(ServiceError::Validation {
                message: format!("Instance {} not found", instance_id),
            });
        }

        let metrics = self.create_mock_metrics(instance_id);
        info!(
            "Mock RDS: returning {} metrics for instance {}",
            metrics.len(),
            instance_id
        );
        Ok(metrics)
    }

    async fn health_check(&self) -> ServiceResult<()> {
        debug!("Mock RDS: performing health check");

        if self.should_fail {
            return Err(ServiceError::ServiceUnavailable {
                message: self
                    .failure_message
                    .clone()
                    .unwrap_or_else(|| "Mock failure".to_string()),
            });
        }

        info!("Mock RDS: health check passed");
        Ok(())
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    fn update_config(&mut self, config: ServiceConfig) {
        self.config = config;
        info!("Mock RDS: configuration updated");
    }
}

/// Mock SQS service for testing
pub struct MockSqsService {
    config: ServiceConfig,
    queues: Vec<SqsQueue>,
    should_fail: bool,
    failure_message: Option<String>,
}

impl MockSqsService {
    /// Create a new mock SQS service with default test data
    pub fn new() -> Self {
        let mut attributes1 = HashMap::new();
        attributes1.insert(
            "QueueArn".to_string(),
            "arn:aws:sqs:us-east-1:123456789012:test-queue-1".to_string(),
        );
        attributes1.insert("ApproximateNumberOfMessages".to_string(), "5".to_string());

        let mut attributes2 = HashMap::new();
        attributes2.insert(
            "QueueArn".to_string(),
            "arn:aws:sqs:us-east-1:123456789012:test-queue-2.fifo".to_string(),
        );
        attributes2.insert("ApproximateNumberOfMessages".to_string(), "12".to_string());
        attributes2.insert("FifoQueue".to_string(), "true".to_string());

        let queues = vec![
            SqsQueue {
                url: "https://sqs.us-east-1.amazonaws.com/123456789012/test-queue-1".to_string(),
                name: "test-queue-1".to_string(),
                queue_type: "Standard".to_string(),
                attributes: attributes1,
            },
            SqsQueue {
                url: "https://sqs.us-east-1.amazonaws.com/123456789012/test-queue-2.fifo"
                    .to_string(),
                name: "test-queue-2.fifo".to_string(),
                queue_type: "FIFO".to_string(),
                attributes: attributes2,
            },
        ];

        Self {
            config: ServiceConfig::default(),
            queues,
            should_fail: false,
            failure_message: None,
        }
    }

    /// Create a mock service that will fail operations
    pub fn with_failure(message: String) -> Self {
        let mut service = Self::new();
        service.should_fail = true;
        service.failure_message = Some(message);
        service
    }

    /// Add a custom queue to the mock service
    pub fn with_queue(mut self, queue: SqsQueue) -> Self {
        self.queues.push(queue);
        self
    }

    /// Set custom queues for the mock service
    pub fn with_queues(mut self, queues: Vec<SqsQueue>) -> Self {
        self.queues = queues;
        self
    }

    /// Create mock metrics for testing
    fn create_mock_metrics(&self, queue_name: &str) -> DynamicMetrics {
        let mut metrics = DynamicMetrics::new(AwsServiceType::Sqs, queue_name.to_string());

        // Add some mock metric data
        let mock_data = vec![
            (chrono::Utc::now().timestamp() as f64, 25.0),
            (chrono::Utc::now().timestamp() as f64 - 300.0, 30.0),
            (chrono::Utc::now().timestamp() as f64 - 600.0, 20.0),
        ];

        metrics.add_metric(
            "ApproximateNumberOfMessages".to_string(),
            MetricType::Count,
            mock_data.clone(),
        );
        metrics.add_metric(
            "NumberOfMessagesSent".to_string(),
            MetricType::Count,
            mock_data,
        );

        metrics
    }
}

#[async_trait]
impl AwsService for MockSqsService {
    fn service_type(&self) -> AwsServiceType {
        AwsServiceType::Sqs
    }

    fn service_name(&self) -> &'static str {
        "MockSQS"
    }

    async fn list_instances(&self) -> ServiceResult<Vec<ServiceInstance>> {
        debug!("Mock SQS: listing queues");

        if self.should_fail {
            return Err(ServiceError::AwsApi {
                message: self
                    .failure_message
                    .clone()
                    .unwrap_or_else(|| "Mock failure".to_string()),
                source: None,
            });
        }

        let service_instances: Vec<ServiceInstance> = self
            .queues
            .clone()
            .into_iter()
            .map(ServiceInstance::Sqs)
            .collect();

        info!("Mock SQS: returning {} queues", service_instances.len());
        Ok(service_instances)
    }

    async fn get_metrics(
        &self,
        instance_id: &str,
        _time_range: TimeRange,
    ) -> ServiceResult<DynamicMetrics> {
        debug!("Mock SQS: getting metrics for queue {}", instance_id);

        if self.should_fail {
            return Err(ServiceError::AwsApi {
                message: self
                    .failure_message
                    .clone()
                    .unwrap_or_else(|| "Mock failure".to_string()),
                source: None,
            });
        }

        // Check if queue exists
        if !self.queues.iter().any(|q| q.name == instance_id) {
            return Err(ServiceError::Validation {
                message: format!("Queue {} not found", instance_id),
            });
        }

        let metrics = self.create_mock_metrics(instance_id);
        info!(
            "Mock SQS: returning {} metrics for queue {}",
            metrics.len(),
            instance_id
        );
        Ok(metrics)
    }

    async fn health_check(&self) -> ServiceResult<()> {
        debug!("Mock SQS: performing health check");

        if self.should_fail {
            return Err(ServiceError::ServiceUnavailable {
                message: self
                    .failure_message
                    .clone()
                    .unwrap_or_else(|| "Mock failure".to_string()),
            });
        }

        info!("Mock SQS: health check passed");
        Ok(())
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    fn update_config(&mut self, config: ServiceConfig) {
        self.config = config;
        info!("Mock SQS: configuration updated");
    }
}
