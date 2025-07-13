#[cfg(test)]
use crate::models::*;
use std::time::SystemTime;

/// Create a sample RDS instance for testing
pub fn create_test_rds_instance() -> RdsInstance {
    RdsInstance {
        identifier: "test-db-1".to_string(),
        engine: "mysql".to_string(),
        status: "available".to_string(),
        instance_class: "db.t3.micro".to_string(),
        endpoint: Some("test-db-1.cluster-abc123.us-east-1.rds.amazonaws.com".to_string()),
    }
}

/// Create a sample SQS queue for testing
pub fn create_test_sqs_queue() -> SqsQueue {
    SqsQueue {
        url: "https://sqs.us-east-1.amazonaws.com/123456789012/test-queue".to_string(),
        name: "test-queue".to_string(),
        queue_type: "Standard".to_string(),
        attributes: std::collections::HashMap::new(),
    }
}

/// Create sample metric data for testing
pub fn create_test_metric_data() -> MetricData {
    let now = SystemTime::now();
    let mut data = MetricData::default();

    // Add some sample data
    data.timestamps = vec![now; 10];
    data.cpu_utilization = 45.0;
    data.cpu_history = vec![40.0, 42.0, 44.0, 45.0, 46.0, 44.0, 43.0, 45.0, 47.0, 45.0];
    data.database_connections = 25.0;
    data.connections_history = vec![20.0, 22.0, 24.0, 25.0, 26.0, 24.0, 23.0, 25.0, 27.0, 25.0];

    data
}

/// Create sample SQS metric data for testing
pub fn create_test_sqs_metric_data() -> SqsMetricData {
    let now = SystemTime::now();
    let mut data = SqsMetricData::default();

    // Add some sample data
    data.timestamps = vec![now; 10];
    data.approximate_number_of_messages = 5.0;
    data.queue_depth_history = vec![3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 5.0, 7.0, 5.0];
    data.number_of_messages_sent = 100.0;
    data.messages_sent_history = vec![
        90.0, 95.0, 100.0, 105.0, 100.0, 95.0, 90.0, 100.0, 110.0, 100.0,
    ];

    data
}

/// Create a test app with sample data
pub fn create_test_app() -> App {
    App {
        available_services: vec![AwsService::Rds, AwsService::Sqs],
        service_list_state: ratatui::widgets::ListState::default(),
        selected_service: Some(AwsService::Rds),
        instances: vec![],
        rds_instances: vec![create_test_rds_instance()],
        sqs_queues: vec![create_test_sqs_queue()],
        list_state: ratatui::widgets::ListState::default(),
        loading: false,
        state: AppState::InstanceList,
        selected_instance: Some(0),
        dynamic_metrics: None,
        metrics: create_test_metric_data(),
        sqs_metrics: create_test_sqs_metric_data(),
        metrics_loading: false,
        last_refresh: None,
        auto_refresh_enabled: true,
        focused_panel: FocusedPanel::SparklineGrid,
        saved_focused_panel: FocusedPanel::SparklineGrid,
        time_range: crate::aws::cloudwatch_service::TimeRange::new(
            1,
            crate::aws::cloudwatch_service::TimeUnit::Hours,
            1,
        )
        .unwrap(),
        selected_metric_name: None,
        sparkline_grid_selected_index: 0,
        saved_sparkline_grid_selected_index: 0,
        sparkline_grid_list_state: ratatui::widgets::ListState::default(),
        time_range_list_state: ratatui::widgets::ListState::default(),
        period_list_state: ratatui::widgets::ListState::default(),
        timezone_list_state: ratatui::widgets::ListState::default(),
        error_message: None,
        loading_start_time: None,
        time_range_mode: TimeRangeMode::Relative,
        timezone: Timezone::Local,
    }
}
