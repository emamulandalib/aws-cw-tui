use crate::models::*;

/// Assert that app is in a valid state
pub fn assert_app_valid_state(app: &App) {
    match app.state {
        AppState::ServiceList => {
            // Service list should have services available
            assert!(!app.available_services.is_empty());
        }
        AppState::InstanceList => {
            // Instance list should have a selected service
            assert!(app.selected_service.is_some());
        }
        AppState::MetricsSummary => {
            // Metrics summary should have selected service and instance
            assert!(app.selected_service.is_some());
            assert!(app.list_state.selected().is_some());
        }
        AppState::InstanceDetails => {
            // Instance details should have selected service and instance
            assert!(app.selected_service.is_some());
            assert!(app.list_state.selected().is_some());
        }
    }
}

/// Assert that app navigation state is consistent
pub fn assert_navigation_consistent(app: &App) {
    // If we have a selected instance, it should be valid
    if let Some(selected_idx) = app.list_state.selected() {
        assert!(selected_idx < app.instances.len());
    }

    // If we have a selected service, it should be in available services
    if let Some(selected_service) = &app.selected_service {
        assert!(app.available_services.contains(selected_service));
    }
}

/// Assert that app error state is consistent
pub fn assert_error_state_consistent(app: &App) {
    // If we have an error, we shouldn't be loading
    if app.has_error() {
        assert!(!app.is_loading());
    }
}
