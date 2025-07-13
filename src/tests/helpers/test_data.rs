use crate::models::*;
use std::time::SystemTime;

/// Create a test RDS instance with minimal required fields
pub fn create_test_rds_instance() -> RdsInstance {
    RdsInstance {
        id: "test-db-1".to_string(),
        name: "Test DB 1".to_string(),
        identifier: "test-db-1".to_string(),
        engine: "mysql".to_string(),
        status: "available".to_string(),
        instance_class: "db.t3.micro".to_string(),
        endpoint: Some("test-db-1.cluster-abc123.us-east-1.rds.amazonaws.com".to_string()),
        port: Some(3306),
        engine_version: Some("8.0.28".to_string()),
        allocated_storage: Some(20),
        storage_type: Some("gp2".to_string()),
        availability_zone: Some("us-east-1a".to_string()),
        backup_retention_period: Some(7),
        multi_az: Some(false),
        storage_encrypted: Some(false),
        performance_insights_enabled: Some(false),
        deletion_protection: Some(false),
        creation_time: None,
    }
}

/// Create a test SQS queue with minimal required fields
pub fn create_test_sqs_queue() -> SqsQueue {
    SqsQueue {
        url: "https://sqs.us-east-1.amazonaws.com/123456789012/test-queue".to_string(),
        name: "test-queue".to_string(),
        queue_type: "Standard".to_string(),
        attributes: std::collections::HashMap::new(),
    }
}

/// Create test metric data with sample values
pub fn create_test_metric_data() -> MetricData {
    let now = SystemTime::now();
    let mut data = MetricData::default();

    data.timestamps = vec![now; 10];
    data.cpu_utilization = 45.0;
    data.cpu_history = vec![40.0, 42.0, 44.0, 45.0, 46.0, 44.0, 43.0, 45.0, 47.0, 45.0];
    data.database_connections = 25.0;
    data.connections_history = vec![20.0, 22.0, 24.0, 25.0, 26.0, 24.0, 23.0, 25.0, 27.0, 25.0];

    data
}

/// Create test SQS metric data with sample values
pub fn create_test_sqs_metric_data() -> SqsMetricData {
    let now = SystemTime::now();
    let mut data = SqsMetricData::default();

    data.timestamps = vec![now; 10];
    data.approximate_number_of_messages = 5.0;
    data.queue_depth_history = vec![3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 5.0, 7.0, 5.0];
    data.number_of_messages_sent = 100.0;
    data.messages_sent_history = vec![
        90.0, 95.0, 100.0, 105.0, 100.0, 95.0, 90.0, 100.0, 110.0, 100.0,
    ];

    data
}

/// Create a test app with reasonable defaults
pub fn create_test_app() -> App {
    // Ensure environment variables don't interfere with test
    std::env::remove_var("AWSCW_AUTO_REFRESH");

    let mut app = App::new();

    // Set up default selected service for tests that need it
    app.selected_service = Some(AwsService::Rds);

    // Set up some default instances for tests
    app.instances = vec![
        ServiceInstance::Rds(create_test_rds_instance()),
        ServiceInstance::Sqs(create_test_sqs_queue()),
    ];

    // Set up selected index for tests that need it
    app.list_state.select(Some(0));

    // Ensure auto_refresh_enabled is true for tests
    app.auto_refresh_enabled = true;

    app
}
