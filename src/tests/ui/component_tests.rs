use crate::models::*;
use crate::tests::helpers::*;

#[cfg(test)]
mod component_tests {
    use super::*;

    #[test]
    fn test_service_list_component() {
        let mut app = create_test_app();
        let _theme = app.get_current_theme();

        // Test that service list component has required data
        assert!(!app.available_services.is_empty());
        assert!(app.service_list_state.selected().is_some());

        // Test navigation
        app.service_next();

        assert_navigation_consistent(&app);
    }

    #[test]
    fn test_instance_list_component() {
        let mut app = create_test_app();
        app.instances = vec![
            ServiceInstance::Rds(create_test_rds_instance()),
            ServiceInstance::Sqs(create_test_sqs_queue()),
        ];
        app.list_state.select(Some(0));

        // Test instance access using App methods
        assert!(app.has_instances());
        assert_eq!(app.instance_count(), 2);

        let selected = app.get_selected_instance();
        assert!(selected.is_some());
    }

    #[test]
    fn test_metrics_component() {
        let mut app = create_test_app();
        app.metrics = create_test_metric_data();
        app.sqs_metrics = create_test_sqs_metric_data();

        // Test metrics data
        assert!(app.metrics.cpu_utilization > 0.0);
        assert!(!app.metrics.cpu_history.is_empty());

        assert!(app.sqs_metrics.approximate_number_of_messages > 0.0);
        assert!(!app.sqs_metrics.queue_depth_history.is_empty());
    }

    #[test]
    fn test_time_range_component() {
        let mut app = create_test_app();

        // Test time range selection using App methods
        let result = app.select_time_range(0);
        assert!(result.is_ok());

        assert_app_valid_state(&app);
    }
}
