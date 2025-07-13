//! Tests for service trait implementations
//!
//! Tests the basic functionality of all service implementations
//! to ensure they correctly implement the AwsService trait.

use crate::aws::cloudwatch_service::TimeRange;
use crate::aws::time_range::TimeUnit;
use crate::models::{AwsService as AwsServiceType, ServiceInstance};
use crate::services::{
    AwsService, CloudWatchServiceImpl, MockRdsService, MockSqsService, RdsServiceImpl,
    ServiceError, SqsServiceImpl,
};

#[tokio::test]
async fn test_rds_service_trait_implementation() {
    let service = RdsServiceImpl::new();

    // Test service metadata
    assert_eq!(service.service_type(), AwsServiceType::Rds);
    assert_eq!(service.service_name(), "RDS");

    // Test configuration access
    let config = service.get_config();
    assert!(config.retry.max_attempts > 0);
    assert!(config.timeout.as_secs() > 0);
}

#[tokio::test]
async fn test_sqs_service_trait_implementation() {
    let service = SqsServiceImpl::new();

    // Test service metadata
    assert_eq!(service.service_type(), AwsServiceType::Sqs);
    assert_eq!(service.service_name(), "SQS");

    // Test configuration access
    let config = service.get_config();
    assert!(config.retry.max_attempts > 0);
    assert!(config.timeout.as_secs() > 0);
}

#[tokio::test]
async fn test_cloudwatch_service_trait_implementation() {
    let service = CloudWatchServiceImpl::new();

    // Test service metadata
    assert_eq!(service.service_name(), "CloudWatch");

    // Test configuration access
    let config = service.get_config();
    assert!(config.retry.max_attempts > 0);
    assert!(config.timeout.as_secs() > 0);
}

#[tokio::test]
async fn test_mock_rds_service_basic_functionality() {
    let service = MockRdsService::new();

    // Test service metadata
    assert_eq!(service.service_type(), AwsServiceType::Rds);
    assert_eq!(service.service_name(), "MockRDS");

    // Test list instances
    let instances = service.list_instances().await.unwrap();
    assert!(!instances.is_empty());

    // Verify instances are RDS instances
    for instance in &instances {
        match instance {
            ServiceInstance::Rds(_) => {} // Expected
            _ => panic!("Expected RDS instance"),
        }
    }

    // Test health check
    let health_result = service.health_check().await;
    assert!(health_result.is_ok());
}

#[tokio::test]
async fn test_mock_sqs_service_basic_functionality() {
    let service = MockSqsService::new();

    // Test service metadata
    assert_eq!(service.service_type(), AwsServiceType::Sqs);
    assert_eq!(service.service_name(), "MockSQS");

    // Test list instances
    let instances = service.list_instances().await.unwrap();
    assert!(!instances.is_empty());

    // Verify instances are SQS queues
    for instance in &instances {
        match instance {
            ServiceInstance::Sqs(_) => {} // Expected
            _ => panic!("Expected SQS queue"),
        }
    }

    // Test health check
    let health_result = service.health_check().await;
    assert!(health_result.is_ok());
}

#[tokio::test]
async fn test_mock_service_with_failure() {
    let service = MockRdsService::with_failure("Test failure".to_string());

    // Test that operations fail as expected
    let instances_result = service.list_instances().await;
    assert!(instances_result.is_err());

    let health_result = service.health_check().await;
    assert!(health_result.is_err());

    // Test error message
    if let Err(ServiceError::AwsApi { message, .. }) = instances_result {
        assert_eq!(message, "Test failure");
    } else {
        panic!("Expected AwsApi error");
    }
}

#[tokio::test]
async fn test_mock_service_get_metrics() {
    let service = MockRdsService::new();

    // Get instances first
    let instances = service.list_instances().await.unwrap();
    assert!(!instances.is_empty());

    // Get the first instance ID
    let instance_id = instances[0].as_aws_instance().id();

    // Test get metrics
    let time_range = TimeRange::new(3, TimeUnit::Hours, 1).unwrap();
    let metrics = service.get_metrics(instance_id, time_range).await.unwrap();

    assert!(!metrics.is_empty());
    assert_eq!(metrics.service_type(), &AwsServiceType::Rds);
    assert_eq!(metrics.instance_id(), instance_id);
}

#[tokio::test]
async fn test_mock_service_get_metrics_invalid_instance() {
    let service = MockRdsService::new();

    // Test with non-existent instance
    let time_range = TimeRange::new(3, TimeUnit::Hours, 1).unwrap();
    let result = service
        .get_metrics("non-existent-instance", time_range)
        .await;

    assert!(result.is_err());
    if let Err(ServiceError::Validation { message }) = result {
        assert!(message.contains("not found"));
    } else {
        panic!("Expected Validation error");
    }
}

#[tokio::test]
async fn test_service_configuration_update() {
    let mut service = MockRdsService::new();

    // Get initial config
    let initial_config = service.get_config().clone();

    // Update configuration
    let mut new_config = initial_config.clone();
    new_config.retry.max_attempts = 10;
    new_config.timeout = std::time::Duration::from_secs(60);

    service.update_config(new_config.clone());

    // Verify configuration was updated
    let updated_config = service.get_config();
    assert_eq!(updated_config.retry.max_attempts, 10);
    assert_eq!(updated_config.timeout, std::time::Duration::from_secs(60));
}

#[tokio::test]
async fn test_service_error_categories() {
    let aws_error = ServiceError::AwsApi {
        message: "Test".to_string(),
        source: None,
    };
    assert_eq!(aws_error.category(), "aws_api");
    assert!(!aws_error.is_retryable());

    let network_error = ServiceError::Network {
        message: "Test".to_string(),
    };
    assert_eq!(network_error.category(), "network");
    assert!(network_error.is_retryable());

    let auth_error = ServiceError::Authentication {
        message: "Test".to_string(),
    };
    assert_eq!(auth_error.category(), "authentication");
    assert!(!auth_error.is_retryable());

    let timeout_error = ServiceError::Timeout {
        message: "Test".to_string(),
    };
    assert_eq!(timeout_error.category(), "timeout");
    assert!(timeout_error.is_retryable());
}
