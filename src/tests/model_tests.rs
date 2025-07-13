#[cfg(test)]
mod tests {
    use super::super::test_utils::*;
    use crate::models::*;

    #[test]
    fn test_rds_instance_creation() {
        let instance = create_test_rds_instance();
        assert_eq!(instance.identifier, "test-db-1");
        assert_eq!(instance.engine, "mysql");
        assert_eq!(instance.status, "available");
        assert_eq!(instance.instance_class, "db.t3.micro");
        assert!(instance.endpoint.is_some());
    }

    #[test]
    fn test_sqs_queue_creation() {
        let queue = create_test_sqs_queue();
        assert_eq!(queue.name, "test-queue");
        assert_eq!(queue.queue_type, "Standard");
        assert!(queue.url.contains("sqs.us-east-1.amazonaws.com"));
    }

    #[test]
    fn test_aws_service_display_names() {
        assert_eq!(AwsService::Rds.short_name(), "RDS");
        assert_eq!(AwsService::Sqs.short_name(), "SQS");
        assert_eq!(
            AwsService::Rds.display_name(),
            "RDS (Relational Database Service)"
        );
        assert_eq!(AwsService::Sqs.display_name(), "SQS (Simple Queue Service)");
    }

    #[test]
    fn test_metric_data_defaults() {
        let data = MetricData::default();
        assert_eq!(data.cpu_utilization, 0.0);
        assert_eq!(data.database_connections, 0.0);
        assert!(data.timestamps.is_empty());
        assert!(data.cpu_history.is_empty());
    }

    #[test]
    fn test_sqs_metric_data_defaults() {
        let data = SqsMetricData::default();
        assert_eq!(data.approximate_number_of_messages, 0.0);
        assert_eq!(data.number_of_messages_sent, 0.0);
        assert!(data.timestamps.is_empty());
        assert!(data.queue_depth_history.is_empty());
    }

    #[test]
    fn test_app_state_transitions() {
        let app = create_test_app();
        assert_eq!(app.state, AppState::InstanceList);
        assert_eq!(app.selected_service, Some(AwsService::Rds));
        assert_eq!(app.selected_instance, Some(0));
    }

    #[test]
    fn test_timezone_options() {
        let options = Timezone::get_timezone_options();
        assert_eq!(options.len(), 2);
        assert!(options.contains(&Timezone::Local));
        assert!(options.contains(&Timezone::Utc));
    }

    #[test]
    fn test_dynamic_metrics_initialization() {
        let metrics = DynamicMetrics::new(AwsService::Rds, "test-instance".to_string());
        assert!(metrics.is_empty());
        assert_eq!(metrics.len(), 0);
        assert!(metrics.get_available_metric_names().is_empty());
    }
}
