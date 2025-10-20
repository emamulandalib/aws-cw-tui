use crate::models::*;
use crate::tests::helpers::*;

#[cfg(test)]
mod app_state_tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = create_test_app();
        assert_eq!(app.state, AppState::ServiceList);
        assert!(!app.available_services.is_empty());
        assert_app_valid_state(&app);
    }

    #[test]
    fn test_app_has_error() {
        let mut app = create_test_app();
        assert!(!app.has_error());

        // Set error using App method
        app.set_error("Test error");
        assert!(app.has_error());
        assert_eq!(app.get_error(), Some(&"Test error".to_string()));
    }

    #[test]
    fn test_app_loading_state() {
        let mut app = create_test_app();
        assert!(!app.is_loading());

        // Start loading using App method
        app.start_loading();
        assert!(app.is_loading());
    }

    #[test]
    fn test_app_navigation_state() {
        let app = create_test_app();
        assert!(!app.can_navigate_back()); // Should be false for ServiceList

        assert_navigation_consistent(&app);
    }

    #[test]
    fn test_app_theme_management() {
        let mut app = create_test_app();

        // Test theme switching without accessing current_theme directly
        app.next_theme();

        // Theme name should be valid
        let theme_name = app.get_current_theme_name();
        assert!(!theme_name.is_empty());
    }
}
