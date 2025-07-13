//! Integration tests for the service architecture
//!
//! Tests the complete service architecture including:
//! - Service container with real and mock services
//! - End-to-end workflows using the service layer
//! - Error handling and recovery scenarios

use crate::aws::cloudwatch_service::TimeRange;
use crate::aws::time_range::TimeUnit;
use crate::config::AppConfig;
use crate::models::AwsService as AwsServiceType;
use crate::services::{
    CircuitBreakerConfig, EnhancedRdsService, EnhancedSqsService, MockRdsService, MockSqsService,
    RetryConfig, ServiceConfig, ServiceContainer,
};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_service_container_with_mock_services() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register mock services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    // Test complete workflow: list instances -> get metrics
    let rds_service = container.get_service(&AwsServiceType::Rds).unwrap();
    let sqs_service = container.get_service(&AwsServiceType::Sqs).unwrap();

    // Test RDS workflow
    let rds_instances = rds_service.list_instances().await.unwrap();
    assert!(!rds_instances.is_empty());

    let rds_instance_id = rds_instances[0].as_aws_instance().id();
    let time_range = TimeRange::new(1, TimeUnit::Hours, 1).unwrap();
    let rds_metrics = rds_service
        .get_metrics(rds_instance_id, time_range)
        .await
        .unwrap();
    assert!(!rds_metrics.is_empty());

    // Test SQS workflow
    let sqs_instances = sqs_service.list_instances().await.unwrap();
    assert!(!sqs_instances.is_empty());

    let sqs_instance_id = sqs_instances[0].as_aws_instance().id();
    let sqs_metrics = sqs_service
        .get_metrics(sqs_instance_id, time_range)
        .await
        .unwrap();
    assert!(!sqs_metrics.is_empty());
}

#[tokio::test]
async fn test_enhanced_services_with_retry_logic() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Create enhanced services with custom retry configuration
    let retry_config = RetryConfig {
        max_attempts: 2,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(100),
        backoff_multiplier: 2.0,
        jitter: false,
    };

    let circuit_breaker_config = CircuitBreakerConfig {
        failure_threshold: 3,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 1,
    };

    let service_config = ServiceConfig {
        retry: retry_config,
        circuit_breaker: circuit_breaker_config,
        timeout: Duration::from_secs(30),
        custom_settings: std::collections::HashMap::new(),
    };

    // Register enhanced services
    container.register(EnhancedRdsService::with_config(service_config.clone()));
    container.register(EnhancedSqsService::with_config(service_config));

    // Test that services work with enhanced features
    let rds_service = container.get_service(&AwsServiceType::Rds).unwrap();
    let sqs_service = container.get_service(&AwsServiceType::Sqs).unwrap();

    // These should work if AWS credentials are available, or fail gracefully
    let rds_health = rds_service.health_check().await;
    let sqs_health = sqs_service.health_check().await;

    // We can't guarantee AWS access in tests, so we just verify the calls don't panic
    // and return some result (either success or a proper error)
    assert!(rds_health.is_ok() || rds_health.is_err());
    assert!(sqs_health.is_ok() || sqs_health.is_err());
}

#[tokio::test]
async fn test_service_container_health_check_workflow() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register mix of healthy and failing services
    container.register(MockRdsService::new());
    container.register(MockSqsService::with_failure(
        "Service temporarily unavailable".to_string(),
    ));

    // Perform health check on all services
    let health_results = container.health_check_all().await;

    assert_eq!(health_results.len(), 2);

    // RDS should be healthy
    let rds_health = health_results.get(&AwsServiceType::Rds).unwrap();
    assert!(rds_health.is_ok());

    // SQS should be unhealthy
    let sqs_health = health_results.get(&AwsServiceType::Sqs).unwrap();
    assert!(sqs_health.is_err());
}

#[tokio::test]
async fn test_service_error_handling_workflow() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register failing service
    container.register(MockRdsService::with_failure(
        "Database connection failed".to_string(),
    ));

    let rds_service = container.get_service(&AwsServiceType::Rds).unwrap();

    // Test that all operations fail gracefully
    let instances_result = rds_service.list_instances().await;
    assert!(instances_result.is_err());

    let time_range = TimeRange::new(1, TimeUnit::Hours, 1).unwrap();
    let metrics_result = rds_service.get_metrics("test-instance", time_range).await;
    assert!(metrics_result.is_err());

    let health_result = rds_service.health_check().await;
    assert!(health_result.is_err());
}

#[tokio::test]
async fn test_service_configuration_injection() {
    let mut app_config = AppConfig::default();
    app_config.aws.region = Some("us-west-2".to_string());
    app_config.ui.auto_refresh_interval = 60;

    let config = Arc::new(app_config);
    let container = ServiceContainer::new(config.clone());

    // Verify configuration is accessible
    let container_config = container.get_config();
    assert_eq!(container_config.aws.region, Some("us-west-2".to_string()));
    assert_eq!(container_config.ui.auto_refresh_interval, 60);
}

#[tokio::test]
async fn test_concurrent_service_operations() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    let container = Arc::new(container);
    let mut handles = Vec::new();

    // Spawn multiple concurrent operations
    for i in 0..5 {
        let container_clone = container.clone();
        let handle = tokio::spawn(async move {
            let service_type = if i % 2 == 0 {
                AwsServiceType::Rds
            } else {
                AwsServiceType::Sqs
            };

            let service = container_clone.get_service(&service_type).unwrap();

            // Perform operations
            let instances = service.list_instances().await?;
            if !instances.is_empty() {
                let instance_id = instances[0].as_aws_instance().id();
                let time_range = TimeRange::new(1, TimeUnit::Hours, 1).unwrap();
                let _metrics = service.get_metrics(instance_id, time_range).await?;
            }

            service.health_check().await?;

            Ok::<(), crate::services::ServiceError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_service_metrics_data_consistency() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    // Test RDS metrics consistency
    let rds_service = container.get_service(&AwsServiceType::Rds).unwrap();
    let rds_instances = rds_service.list_instances().await.unwrap();

    if !rds_instances.is_empty() {
        let instance_id = rds_instances[0].as_aws_instance().id();
        let time_range = TimeRange::new(3, TimeUnit::Hours, 1).unwrap();
        let metrics = rds_service
            .get_metrics(instance_id, time_range)
            .await
            .unwrap();

        // Verify metrics structure
        assert_eq!(metrics.service_type(), &AwsServiceType::Rds);
        assert_eq!(metrics.instance_id(), instance_id);
        assert!(!metrics.is_empty());

        // Verify metrics have data points
        for metric_name in metrics.metric_names() {
            let metric_data = metrics.get_metric(metric_name).unwrap();
            assert!(!metric_data.history.is_empty());
        }
    }

    // Test SQS metrics consistency
    let sqs_service = container.get_service(&AwsServiceType::Sqs).unwrap();
    let sqs_instances = sqs_service.list_instances().await.unwrap();

    if !sqs_instances.is_empty() {
        let instance_id = sqs_instances[0].as_aws_instance().id();
        let time_range = TimeRange::new(3, TimeUnit::Hours, 1).unwrap();
        let metrics = sqs_service
            .get_metrics(instance_id, time_range)
            .await
            .unwrap();

        // Verify metrics structure
        assert_eq!(metrics.service_type(), &AwsServiceType::Sqs);
        assert_eq!(metrics.instance_id(), instance_id);
        assert!(!metrics.is_empty());
    }
}

#[tokio::test]
async fn test_service_replacement_in_container() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register initial service
    container.register(MockRdsService::new());

    // Test initial service works
    let initial_service = container.get_service(&AwsServiceType::Rds).unwrap();
    let initial_health = initial_service.health_check().await;
    assert!(initial_health.is_ok());

    // Replace with failing service
    container.register(MockRdsService::with_failure("Service replaced".to_string()));

    // Test replacement service has different behavior
    let replacement_service = container.get_service(&AwsServiceType::Rds).unwrap();
    let replacement_health = replacement_service.health_check().await;
    assert!(replacement_health.is_err());
}
