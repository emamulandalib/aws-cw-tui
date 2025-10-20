use crate::models::*;
use crate::tests::helpers::*;

#[cfg(test)]
mod navigation_tests {
    use super::*;

    #[test]
    fn test_service_navigation() {
        let mut app = create_test_app();

        // Test initial state
        assert_eq!(app.service_list_state.selected(), Some(0));

        // Test navigation using App methods
        app.service_next();

        // Should navigate to next service or wrap around
        let selected = app.service_list_state.selected();
        assert!(selected.is_some());
        assert!(selected.unwrap() < app.available_services.len());
    }

    #[test]
    fn test_instance_navigation() {
        let mut app = create_test_app();

        // Add test instances
        app.instances = vec![ServiceInstance::Rds(create_test_rds_instance())];

        app.list_state.select(Some(0));

        // Test navigation using App methods
        app.next();

        // Should handle navigation properly
        assert_navigation_consistent(&app);
    }

    #[test]
    fn test_screen_navigation() {
        let mut app = create_test_app();

        // Test screen transitions using App methods
        app.select_service();

        assert_eq!(app.state, AppState::InstanceList);
        assert!(app.can_navigate_back());
        assert_app_valid_state(&app);
    }

    #[test]
    fn test_back_navigation() {
        let mut app = create_test_app();

        // Navigate to a deeper state
        app.state = AppState::InstanceDetails;

        // Test back navigation using App methods
        app.go_back();

        assert_eq!(app.state, AppState::MetricsSummary);
        assert_app_valid_state(&app);
    }
}
