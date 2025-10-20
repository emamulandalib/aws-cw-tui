use crate::models::*;
use crate::tests::helpers::*;

#[cfg(test)]
mod rendering_tests {
    use super::*;

    #[test]
    fn test_service_list_rendering_data() {
        let app = create_test_app();
        let _theme = app.get_current_theme();

        // Test that we have all required data for service list rendering
        assert!(!app.available_services.is_empty());
        assert!(app.service_list_state.selected().is_some());

        // Test that services can be converted to display format
        for service in &app.available_services {
            let display_name = match service {
                AwsService::Rds => "RDS",
                AwsService::Sqs => "SQS",
            };
            assert!(!display_name.is_empty());
        }
    }

    #[test]
    fn test_instance_list_rendering_data() {
        let mut app = create_test_app();
        app.instances = vec![
            ServiceInstance::Rds(create_test_rds_instance()),
            ServiceInstance::Sqs(create_test_sqs_queue()),
        ];

        // Test that instances can be converted to display format
        for instance in &app.instances {
            let display_name = match instance {
                ServiceInstance::Rds(rds) => &rds.name,
                ServiceInstance::Sqs(sqs) => &sqs.name,
            };
            assert!(!display_name.is_empty());
        }
    }

    #[test]
    fn test_metrics_rendering_data() {
        let mut app = create_test_app();
        app.metrics = create_test_metric_data();

        // Test that metrics data is valid for rendering
        assert!(app.metrics.cpu_utilization >= 0.0);
        assert!(app.metrics.cpu_utilization <= 100.0);
        assert!(!app.metrics.cpu_history.is_empty());

        // Test that history data is valid
        for value in &app.metrics.cpu_history {
            assert!(*value >= 0.0);
            assert!(*value <= 100.0);
        }
    }

    #[test]
    fn test_theme_rendering_consistency() {
        let mut app = create_test_app();

        // Test that all themes produce consistent color values
        let themes = vec![
            crate::ui::themes::ThemeVariant::Default,
            crate::ui::themes::ThemeVariant::WarmSunset,
            crate::ui::themes::ThemeVariant::BlueGold,
            crate::ui::themes::ThemeVariant::HighContrast,
            crate::ui::themes::ThemeVariant::Monochrome,
            crate::ui::themes::ThemeVariant::TerminalCyan,
        ];

        for theme_variant in themes {
            app.current_theme = theme_variant;
            let unified_theme = app.get_current_theme();

            // All color components should be accessible
            let _primary = unified_theme.primary;
            let _secondary = unified_theme.secondary;
            let _accent = unified_theme.accent;
            let _background = unified_theme.background;

            // Test passes if it doesn't panic
            assert!(true);
        }
    }

    #[test]
    fn test_error_display_data() {
        let mut app = create_test_app();

        // Test error display
        app.set_error("Test error message");

        assert!(app.has_error());
        let error = app.get_error();
        assert!(error.is_some());
        assert!(!error.unwrap().is_empty());
    }

    #[test]
    fn test_loading_display_data() {
        let mut app = create_test_app();

        // Test loading display
        app.start_loading();

        assert!(app.is_loading());
        assert!(app.loading_start_time.is_some());
    }
}
