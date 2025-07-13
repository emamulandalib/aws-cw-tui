//! Tests specifically for mock service implementations
//!
//! Tests the mock services to ensure they provide consistent
//! behavior for testing scenarios.

use crate::models::{RdsInstance, ServiceInstance, SqsQueue};
use crate::services::{AwsService, MockRdsService, MockSqsService};
use std::collections::HashMap;

#[tokio::test]
async fn test_mock_rds_service_default_instances() {
    let service = MockRdsService::new();
    let instances = service.list_instances().await.unwrap();

    assert_eq!(instances.len(), 2);

    // Verify instance types
    for instance in instances {
        match instance {
            ServiceInstance::Rds(rds_instance) => {
                assert!(!rds_instance.id.is_empty());
                assert!(!rds_instance.engine.is_empty());
            }
            _ => panic!("Expected RDS instance"),
        }
    }
}

#[tokio::test]
async fn test_mock_sqs_service_default_queues() {
    let service = MockSqsService::new();
    let instances = service.list_instances().await.unwrap();

    assert_eq!(instances.len(), 2);

    // Verify queue types
    for instance in instances {
        match instance {
            ServiceInstance::Sqs(sqs_queue) => {
                assert!(!sqs_queue.name.is_empty());
                assert!(!sqs_queue.url.is_empty());
                assert!(sqs_queue.queue_type == "Standard" || sqs_queue.queue_type == "FIFO");
            }
            _ => panic!("Expected SQS queue"),
        }
    }
}

#[tokio::test]
async fn test_mock_service_custom_instances() {
    let custom_instance = RdsInstance {
        id: "custom-db".to_string(),
        name: "custom-db".to_string(),
        identifier: "custom-db".to_string(),
        engine: "mysql".to_string(),
        status: "available".to_string(),
        instance_class: "db.t3.micro".to_string(),
        endpoint: Some("custom-db.amazonaws.com".to_string()),
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
        creation_time: Some(std::time::SystemTime::now()),
    };

    let service = MockRdsService::new().with_instance(custom_instance.clone());
    let instances = service.list_instances().await.unwrap();

    // Should have default instances plus the custom one
    assert_eq!(instances.len(), 3);

    // Find our custom instance
    let found_custom = instances.iter().any(|instance| match instance {
        ServiceInstance::Rds(rds) => rds.id == "custom-db",
        _ => false,
    });

    assert!(found_custom);
}

#[tokio::test]
async fn test_mock_service_custom_queues() {
    let mut attributes = HashMap::new();
    attributes.insert(
        "QueueArn".to_string(),
        "arn:aws:sqs:us-east-1:123456789012:custom-queue".to_string(),
    );

    let custom_queue = SqsQueue {
        url: "https://sqs.us-east-1.amazonaws.com/123456789012/custom-queue".to_string(),
        name: "custom-queue".to_string(),
        queue_type: "Standard".to_string(),
        attributes,
    };

    let service = MockSqsService::new().with_queue(custom_queue.clone());
    let instances = service.list_instances().await.unwrap();

    // Should have default queues plus the custom one
    assert_eq!(instances.len(), 3);

    // Find our custom queue
    let found_custom = instances.iter().any(|instance| match instance {
        ServiceInstance::Sqs(queue) => queue.name == "custom-queue",
        _ => false,
    });

    assert!(found_custom);
}

#[tokio::test]
async fn test_mock_service_replace_instances() {
    let custom_instance = RdsInstance {
        id: "only-db".to_string(),
        name: "only-db".to_string(),
        identifier: "only-db".to_string(),
        engine: "postgres".to_string(),
        status: "available".to_string(),
        instance_class: "db.t3.small".to_string(),
        endpoint: Some("only-db.amazonaws.com".to_string()),
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
        creation_time: Some(std::time::SystemTime::now()),
    };

    let service = MockRdsService::new().with_instances(vec![custom_instance]);
    let instances = service.list_instances().await.unwrap();

    // Should only have our custom instance
    assert_eq!(instances.len(), 1);

    match &instances[0] {
        ServiceInstance::Rds(rds) => {
            assert_eq!(rds.id, "only-db");
            assert_eq!(rds.engine, "postgres");
        }
        _ => panic!("Expected RDS instance"),
    }
}
