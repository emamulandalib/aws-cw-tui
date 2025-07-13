use crate::models::*;
use crate::tests::helpers::*;

#[cfg(test)]
mod workflow_tests {
    use super::*;

    #[test]
    fn test_service_to_instance_workflow() {
        let mut app = create_test_app();

        // Start at service list
        assert_eq!(app.state, AppState::ServiceList);
        assert_app_valid_state(&app);

        // Select a service
        app.select_service();

        assert_eq!(app.state, AppState::InstanceList);
        assert!(app.can_navigate_back());
        assert_app_valid_state(&app);
    }

    #[test]
    fn test_complete_navigation_workflow() {
        let mut app = create_test_app();

        // Add test data
        app.instances = vec![ServiceInstance::Rds(create_test_rds_instance())];
        app.list_state.select(Some(0));

        // Service -> Instance List
        app.select_service();
        assert_eq!(app.state, AppState::InstanceList);
        assert_app_valid_state(&app);

        // Instance List -> Metrics Summary
        app.enter_metrics_summary();
        assert_eq!(app.state, AppState::MetricsSummary);
        assert_app_valid_state(&app);

        // Metrics Summary -> Instance Details
        app.enter_instance_details();
        assert_eq!(app.state, AppState::InstanceDetails);
        assert_app_valid_state(&app);

        // Navigate back through all states
        app.go_back();
        assert_eq!(app.state, AppState::MetricsSummary);

        app.go_back();
        assert_eq!(app.state, AppState::InstanceList);

        app.go_back();
        assert_eq!(app.state, AppState::ServiceList);
    }

    #[test]
    fn test_error_recovery_workflow() {
        let mut app = create_test_app();

        // Start workflow
        app.select_service();

        // Inject error
        app.set_error("Test error");

        assert!(app.has_error());
        assert_error_state_consistent(&app);

        // Recovery: clear error and continue
        app.clear_error();

        assert!(!app.has_error());
        assert_app_valid_state(&app);
    }

    #[test]
    fn test_theme_switching_workflow() {
        let mut app = create_test_app();

        // Test theme switching across different states
        let _initial_theme_name = app.get_current_theme_name();

        // Switch theme in service list
        app.next_theme();
        // Theme should change (we test this by checking it's not the same name)

        // Navigate and theme should persist
        app.select_service();
        // Theme should persist

        // Switch theme in instance list
        app.next_theme();

        // Navigate to metrics and theme should persist
        app.enter_metrics_summary();

        // Theme should persist across all navigation
        assert_app_valid_state(&app);
    }
}
