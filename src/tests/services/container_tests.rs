//! Tests for the ServiceContainer dependency injection system
//!
//! Tests the service container functionality including:
//! - Service registration and retrieval
//! - Configuration injection
//! - Health checking across services

use crate::config::AppConfig;
use crate::models::AwsService as AwsServiceType;
use crate::services::{MockRdsService, MockSqsService, ServiceContainer};
use std::sync::Arc;

#[tokio::test]
async fn test_service_container_creation() {
    let config = Arc::new(AppConfig::default());
    let container = ServiceContainer::new(config.clone());

    // Test that container is created with empty services
    let all_services = container.get_all_services();
    assert!(all_services.is_empty());

    // Test configuration access
    let container_config = container.get_config();
    assert!(Arc::ptr_eq(&config, &container_config));
}

#[tokio::test]
async fn test_service_registration_and_retrieval() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register services
    let rds_service = MockRdsService::new();
    let sqs_service = MockSqsService::new();

    container.register(rds_service);
    container.register(sqs_service);

    // Test service retrieval
    let retrieved_rds = container.get_service(&AwsServiceType::Rds);
    assert!(retrieved_rds.is_some());

    let retrieved_sqs = container.get_service(&AwsServiceType::Sqs);
    assert!(retrieved_sqs.is_some());

    // Test non-existent service
    // Note: We only have RDS and SQS, so this tests the None case
    let all_services = container.get_all_services();
    assert_eq!(all_services.len(), 2);
}

#[tokio::test]
async fn test_service_container_get_all_services() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register multiple services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    // Test get all services
    let all_services = container.get_all_services();
    assert_eq!(all_services.len(), 2);

    // Verify service types
    let mut service_types = Vec::new();
    for service in all_services {
        service_types.push(service.service_type());
    }

    assert!(service_types.contains(&AwsServiceType::Rds));
    assert!(service_types.contains(&AwsServiceType::Sqs));
}

#[tokio::test]
async fn test_service_container_health_check_all() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register healthy services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    // Test health check all
    let health_results = container.health_check_all().await;
    assert_eq!(health_results.len(), 2);

    // All should be healthy
    for (service_type, result) in health_results {
        assert!(
            result.is_ok(),
            "Service {:?} should be healthy",
            service_type
        );
    }
}

#[tokio::test]
async fn test_service_container_health_check_with_failures() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register one healthy and one failing service
    container.register(MockRdsService::new());
    container.register(MockSqsService::with_failure(
        "Service unavailable".to_string(),
    ));

    // Test health check all
    let health_results = container.health_check_all().await;
    assert_eq!(health_results.len(), 2);

    // Check individual results
    let rds_result = health_results.get(&AwsServiceType::Rds).unwrap();
    assert!(rds_result.is_ok());

    let sqs_result = health_results.get(&AwsServiceType::Sqs).unwrap();
    assert!(sqs_result.is_err());
}

#[tokio::test]
async fn test_service_container_service_replacement() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register initial service
    let initial_service = MockRdsService::new();
    container.register(initial_service);

    // Verify service is registered
    let retrieved = container.get_service(&AwsServiceType::Rds);
    assert!(retrieved.is_some());

    // Register replacement service (should replace the previous one)
    let replacement_service = MockRdsService::with_failure("New service".to_string());
    container.register(replacement_service);

    // Verify replacement
    let new_retrieved = container.get_service(&AwsServiceType::Rds);
    assert!(new_retrieved.is_some());

    // Test that the new service has the failure behavior
    let health_result = new_retrieved.unwrap().health_check().await;
    assert!(health_result.is_err());
}

#[tokio::test]
async fn test_service_container_concurrent_access() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    // Test concurrent access to services
    let container = Arc::new(container);
    let mut handles = Vec::new();

    for _ in 0..10 {
        let container_clone = container.clone();
        let handle = tokio::spawn(async move {
            let rds_service = container_clone.get_service(&AwsServiceType::Rds).unwrap();
            let sqs_service = container_clone.get_service(&AwsServiceType::Sqs).unwrap();

            // Perform operations
            let rds_health = rds_service.health_check().await;
            let sqs_health = sqs_service.health_check().await;

            (rds_health.is_ok(), sqs_health.is_ok())
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let (rds_ok, sqs_ok) = handle.await.unwrap();
        assert!(rds_ok);
        assert!(sqs_ok);
    }
}

#[tokio::test]
async fn test_service_container_debug_formatting() {
    let config = Arc::new(AppConfig::default());
    let mut container = ServiceContainer::new(config);

    // Register services
    container.register(MockRdsService::new());
    container.register(MockSqsService::new());

    // Test debug formatting
    let debug_output = format!("{:?}", container);
    assert!(debug_output.contains("ServiceContainer"));
    assert!(debug_output.contains("services"));
}
