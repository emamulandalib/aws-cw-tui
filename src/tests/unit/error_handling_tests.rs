use crate::tests::helpers::*;

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_error_setting_and_clearing() {
        let mut app = create_test_app();

        // Test initial error state
        assert!(!app.has_error());
        assert!(app.get_error().is_none());

        // Set error using App methods
        app.set_error("Test error");

        assert!(app.has_error());
        assert_eq!(app.get_error(), Some(&"Test error".to_string()));

        // Clear error using App methods
        app.clear_error();

        assert!(!app.has_error());
        assert!(app.get_error().is_none());
    }

    #[test]
    fn test_error_stops_loading() {
        let mut app = create_test_app();

        // Start loading
        app.start_loading();
        assert!(app.is_loading());

        // Set error - should stop loading
        app.set_error("Error during loading");

        assert!(!app.is_loading());
        assert!(app.has_error());
        assert_error_state_consistent(&app);
    }

    #[test]
    fn test_loading_timeout() {
        let mut app = create_test_app();

        // Test loading timeout using App methods
        app.start_loading();
        assert!(app.is_loading());

        // Simulate timeout by manipulating start time
        app.loading_start_time =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(35));

        let timed_out = app.check_loading_timeout();
        assert!(timed_out);
        assert!(!app.is_loading());
    }

    #[tokio::test]
    async fn test_mock_service_errors() {
        let mock_service = MockService::new().with_failure(MockError::NetworkTimeout);

        let result = mock_service.simulate_operation().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Network timeout"));
    }

    #[tokio::test]
    async fn test_mock_service_delays() {
        let mock_service = MockService::new().with_delay(100); // 100ms delay

        let start = std::time::Instant::now();
        let result = mock_service.simulate_operation().await;
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        assert!(elapsed >= std::time::Duration::from_millis(100));
    }
}
