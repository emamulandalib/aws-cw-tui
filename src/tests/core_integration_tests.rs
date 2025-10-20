//! Integration tests for core application modules
//!
//! These tests verify that all core modules work together properly and that
//! the Phase 5 integration objectives are met.

use crate::models::{App, AppState, AwsService, FocusedPanel, TimeRangeMode, Timezone};
use std::time::Duration;

/// Test that application initialization works with core modules
#[cfg(test)]
mod initialization_tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        // Ensure environment variables don't interfere with test
        std::env::remove_var("AWSCW_AUTO_REFRESH");

        // Clean up any existing config file to ensure test isolation
        if let Some(config_path) = dirs::config_dir() {
            let config_file = config_path.join("awscw").join("config.json");
            if config_file.exists() {
                std::fs::remove_file(&config_file).ok();
            }
        }

        // Also check Application Support directory (macOS)
        if let Some(app_support_path) = dirs::data_dir() {
            let config_file = app_support_path.join("awscw").join("config.json");
            if config_file.exists() {
                std::fs::remove_file(&config_file).ok();
            }
        }

        // Test that App::new() properly initializes using core modules
        let app = App::new();

        // Verify initial state
        assert_eq!(app.state, AppState::ServiceList);
        assert_eq!(app.focused_panel, FocusedPanel::Timezone);
        assert_eq!(app.available_services.len(), 2);
        assert!(app.available_services.contains(&AwsService::Rds));
        assert!(app.available_services.contains(&AwsService::Sqs));

        // Verify loading state
        assert!(!app.loading);
        assert!(!app.metrics_loading);
        assert!(app.error_message.is_none());

        // Verify auto-refresh is enabled
        assert!(app.auto_refresh_enabled);
        assert!(app.last_refresh.is_none());
    }

    #[test]
    fn test_app_helper_methods() {
        // Ensure environment variables don't interfere with test
        std::env::remove_var("AWSCW_AUTO_REFRESH");

        // Clean up any existing config file to ensure test isolation
        if let Some(config_path) = dirs::config_dir() {
            let config_file = config_path.join("awscw").join("config.json");
            if config_file.exists() {
                std::fs::remove_file(&config_file).ok();
            }
        }

        // Also check Application Support directory (macOS)
        if let Some(app_support_path) = dirs::data_dir() {
            let config_file = app_support_path.join("awscw").join("config.json");
            if config_file.exists() {
                std::fs::remove_file(&config_file).ok();
            }
        }

        let app = App::new();

        // Test helper methods work correctly
        assert!(!app.is_loading());
        assert!(!app.has_error());
        assert!(app.get_error().is_none());
        assert!(!app.can_navigate_back()); // Should be false for ServiceList
        assert!(!app.has_available_instances());
        assert!(!app.has_available_metrics());
        assert_eq!(app.get_current_state(), &AppState::ServiceList);
        assert!(app.is_focused_on(&FocusedPanel::Timezone));
    }
}

/// Test that state management works with core modules
#[cfg(test)]
mod state_management_tests {
    use super::*;

    #[test]
    fn test_state_management_functions() {
        // Ensure environment variables don't interfere with test
        std::env::remove_var("AWSCW_AUTO_REFRESH");

        // Clean up any existing config file to ensure test isolation
        if let Some(config_path) = dirs::config_dir() {
            let config_file = config_path.join("awscw").join("config.json");
            if config_file.exists() {
                std::fs::remove_file(&config_file).ok();
            }
        }

        // Also check Application Support directory (macOS)
        if let Some(app_support_path) = dirs::data_dir() {
            let config_file = app_support_path.join("awscw").join("config.json");
            if config_file.exists() {
                std::fs::remove_file(&config_file).ok();
            }
        }

        let mut app = App::new();

        // Test error handling
        assert!(!app.has_error());
        app.set_error("Test error");
        assert!(app.has_error());
        assert_eq!(app.get_error(), Some(&"Test error".to_string()));

        app.clear_error();
        assert!(!app.has_error());

        // Test loading state
        assert!(!app.is_loading());
        app.start_loading();
        assert!(app.is_loading());

        app.stop_loading();
        assert!(!app.is_loading());

        // Test refresh state
        assert!(app.needs_refresh()); // Should be true initially (no last refresh)
        app.mark_refreshed();
        assert!(!app.needs_refresh()); // Should be false immediately after refresh
    }

    #[test]
    fn test_loading_timeout_handling() {
        let mut app = App::new();

        // Start loading
        app.start_loading();
        assert!(app.is_loading());
        assert!(app.loading_start_time.is_some());

        // Should not timeout immediately
        assert!(!app.check_loading_timeout());
        assert!(app.is_loading());

        // Stop loading
        app.stop_loading();
        assert!(!app.is_loading());
        assert!(app.loading_start_time.is_none());
    }
}

/// Test that UI state management works with core modules
#[cfg(test)]
mod ui_state_tests {
    use super::*;

    #[test]
    fn test_panel_switching() {
        let mut app = App::new();

        // Test initial state
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone);

        // Test panel switching cycle
        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Period);

        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::TimeRanges);

        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::SparklineGrid);

        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone); // Should cycle back
    }

    #[test]
    fn test_panel_focus_management() {
        let mut app = App::new();

        // Test is_panel_focused
        assert!(app.is_panel_focused(&FocusedPanel::Timezone));
        assert!(!app.is_panel_focused(&FocusedPanel::Period));

        // Test set_focused_panel
        app.set_focused_panel(FocusedPanel::SparklineGrid);
        assert!(app.is_panel_focused(&FocusedPanel::SparklineGrid));
        assert!(!app.is_panel_focused(&FocusedPanel::Timezone));
    }

    #[test]
    fn test_scroll_state_reset() {
        let mut app = App::new();

        // Change some state
        app.set_focused_panel(FocusedPanel::SparklineGrid);
        app.sparkline_grid_selected_index = 5;

        // Reset scroll
        app.reset_scroll();

        // Verify reset
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone);
        assert_eq!(app.sparkline_grid_selected_index, 0);
    }
}

/// Test that navigation works with core modules
#[cfg(test)]
mod navigation_tests {
    use super::*;

    #[test]
    fn test_service_navigation() {
        let mut app = App::new();

        // Test initial selection
        assert_eq!(app.service_list_state.selected(), Some(0));

        // Test service navigation
        app.service_next();
        assert_eq!(app.service_list_state.selected(), Some(1));

        app.service_next();
        assert_eq!(app.service_list_state.selected(), Some(0)); // Should wrap around

        app.service_previous();
        assert_eq!(app.service_list_state.selected(), Some(1));

        app.service_previous();
        assert_eq!(app.service_list_state.selected(), Some(0)); // Should wrap around
    }

    #[test]
    fn test_instance_navigation() {
        let mut app = App::new();

        // Add some mock instances
        app.instances = vec![
            crate::models::ServiceInstance::Rds(crate::models::RdsInstance {
                id: "test-1".to_string(),
                name: "Test 1".to_string(),
                identifier: "test-1".to_string(),
                engine: "mysql".to_string(),
                status: "available".to_string(),
                instance_class: "db.t3.micro".to_string(),
                endpoint: Some("test-1.amazonaws.com".to_string()),
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
            }),
            crate::models::ServiceInstance::Rds(crate::models::RdsInstance {
                id: "test-2".to_string(),
                name: "Test 2".to_string(),
                identifier: "test-2".to_string(),
                engine: "postgres".to_string(),
                status: "available".to_string(),
                instance_class: "db.t3.micro".to_string(),
                endpoint: Some("test-2.amazonaws.com".to_string()),
                port: Some(5432),
                engine_version: Some("13.7".to_string()),
                allocated_storage: Some(20),
                storage_type: Some("gp2".to_string()),
                availability_zone: Some("us-east-1a".to_string()),
                backup_retention_period: Some(7),
                multi_az: Some(false),
                storage_encrypted: Some(false),
                performance_insights_enabled: Some(false),
                deletion_protection: Some(false),
                creation_time: None,
            }),
        ];

        // Test initial selection
        app.list_state.select(Some(0));
        assert_eq!(app.list_state.selected(), Some(0));

        // Test instance navigation
        app.next();
        assert_eq!(app.list_state.selected(), Some(1));

        app.next();
        assert_eq!(app.list_state.selected(), Some(0)); // Should wrap around

        app.previous();
        assert_eq!(app.list_state.selected(), Some(1));

        app.previous();
        assert_eq!(app.list_state.selected(), Some(0)); // Should wrap around
    }
}

/// Test that screen navigation works with core modules
#[cfg(test)]
mod screen_navigation_tests {
    use super::*;

    #[test]
    fn test_screen_transitions() {
        let mut app = App::new();

        // Test initial state
        assert_eq!(app.get_current_state(), &AppState::ServiceList);
        assert!(!app.can_navigate_back());

        // Test service selection
        app.select_service();
        assert_eq!(app.get_current_state(), &AppState::InstanceList);
        assert!(app.can_navigate_back());

        // Test entering metrics summary
        app.enter_metrics_summary();
        assert_eq!(app.get_current_state(), &AppState::MetricsSummary);
        assert!(app.can_navigate_back());

        // Test entering instance details
        app.enter_instance_details();
        assert_eq!(app.get_current_state(), &AppState::InstanceDetails);
        assert!(app.can_navigate_back());

        // Test backward navigation
        app.back_to_metrics_summary();
        assert_eq!(app.get_current_state(), &AppState::MetricsSummary);

        app.back_to_list();
        assert_eq!(app.get_current_state(), &AppState::InstanceList);

        app.back_to_service_list();
        assert_eq!(app.get_current_state(), &AppState::ServiceList);
        assert!(!app.can_navigate_back());
    }

    #[test]
    fn test_context_aware_navigation() {
        let mut app = App::new();

        // Test go_back from different states
        app.state = AppState::ServiceList;
        app.go_back(); // Should do nothing
        assert_eq!(app.get_current_state(), &AppState::ServiceList);

        app.state = AppState::InstanceList;
        app.go_back();
        assert_eq!(app.get_current_state(), &AppState::ServiceList);

        app.state = AppState::MetricsSummary;
        app.go_back();
        assert_eq!(app.get_current_state(), &AppState::InstanceList);

        app.state = AppState::InstanceDetails;
        app.go_back();
        assert_eq!(app.get_current_state(), &AppState::MetricsSummary);
    }
}

/// Test that time range management works with core modules
#[cfg(test)]
mod time_range_management_tests {
    use super::*;

    #[test]
    fn test_time_range_options() {
        let options = App::get_time_range_options();
        assert!(!options.is_empty());

        // Test that default option exists
        let default_option = options.get(8); // Default to "3 hours"
        assert!(default_option.is_some());
        if let Some((name, value, unit, _)) = default_option {
            assert_eq!(*name, "3 hours");
            assert_eq!(*value, 3);
            assert_eq!(*unit, crate::aws::time_range::TimeUnit::Hours);
        }
    }

    #[test]
    fn test_time_range_selection() {
        let mut app = App::new();

        // Test selecting different time ranges
        let result = app.select_time_range(0);
        assert!(result.is_ok());

        let result = app.select_time_range(8); // 3 hours
        assert!(result.is_ok());

        // Test invalid index
        let result = app.select_time_range(999);
        assert!(result.is_ok()); // Should not fail, just not change anything
    }

    #[test]
    fn test_time_range_navigation() {
        let mut app = App::new();

        // Test initial selection
        let initial_selection = app.get_current_time_range_index();

        // Test scroll down
        app.time_range_scroll_down();
        let after_down = app.get_current_time_range_index();
        assert_ne!(initial_selection, after_down);

        // Test scroll up
        app.time_range_scroll_up();
        let after_up = app.get_current_time_range_index();
        assert_eq!(initial_selection, after_up);
    }

    #[test]
    fn test_timezone_management() {
        let mut app = App::new();

        // Test timezone options
        let options = App::get_timezone_options();
        assert_eq!(options.len(), 2);
        assert!(options.contains(&Timezone::Utc));
        assert!(options.contains(&Timezone::Local));

        // Test current timezone
        assert_eq!(app.get_current_timezone(), &Timezone::Utc);

        // Test timezone navigation
        app.timezone_scroll_down();
        // Should change selection in list state

        app.timezone_scroll_up();
        // Should change selection back
    }

    #[test]
    fn test_time_range_mode_toggle() {
        let mut app = App::new();

        // Test initial mode
        assert_eq!(app.get_time_range_mode(), &TimeRangeMode::Relative);

        // Test toggle
        app.toggle_time_range_mode();
        assert_eq!(app.get_time_range_mode(), &TimeRangeMode::Absolute);

        app.toggle_time_range_mode();
        assert_eq!(app.get_time_range_mode(), &TimeRangeMode::Relative);
    }
}

/// Test that instance access works with core modules
#[cfg(test)]
mod instance_access_tests {
    use super::*;

    #[test]
    fn test_instance_access_empty() {
        let app = App::new();

        // Test with no instances
        assert!(app.get_current_instances().is_empty());
        assert!(app.get_selected_instance().is_none());
        assert!(app.get_selected_instance_id().is_none());
        assert!(app.get_selected_rds_instance().is_none());
        assert!(app.get_selected_sqs_queue().is_none());
        assert!(!app.has_instances());
        assert_eq!(app.instance_count(), 0);
    }

    #[test]
    fn test_instance_access_with_data() {
        let mut app = App::new();

        // Add a mock RDS instance
        app.instances = vec![crate::models::ServiceInstance::Rds(
            crate::models::RdsInstance {
                id: "test-instance".to_string(),
                name: "Test Instance".to_string(),
                identifier: "test-instance".to_string(),
                engine: "mysql".to_string(),
                status: "available".to_string(),
                endpoint: Some("test.amazonaws.com".to_string()),
                port: Some(3306),
                engine_version: Some("8.0.28".to_string()),
                instance_class: "db.t3.micro".to_string(),
                allocated_storage: Some(20),
                storage_type: Some("gp2".to_string()),
                availability_zone: Some("us-east-1a".to_string()),
                backup_retention_period: Some(7),
                multi_az: Some(false),
                storage_encrypted: Some(false),
                performance_insights_enabled: Some(false),
                deletion_protection: Some(false),
                creation_time: None,
            },
        )];

        // Set selection
        app.list_state.select(Some(0));

        // Test instance access
        assert!(!app.get_current_instances().is_empty());
        assert!(app.get_selected_instance().is_some());
        assert_eq!(
            app.get_selected_instance_id(),
            Some("test-instance".to_string())
        );
        assert!(app.get_selected_rds_instance().is_some());
        assert!(app.get_selected_sqs_queue().is_none()); // Should be None for RDS instance
        assert!(app.has_instances());
        assert_eq!(app.instance_count(), 1);
    }
}

/// Test that metrics management works with core modules
#[cfg(test)]
mod metrics_management_tests {
    use super::*;

    #[test]
    fn test_metrics_state_management() {
        let mut app = App::new();

        // Test initial state
        assert!(!app.is_metrics_loading());
        assert!(!app.has_metrics());
        assert!(app.get_available_metrics().is_empty());

        // Test metrics loading state
        app.metrics_loading = true;
        assert!(app.is_metrics_loading());

        // Test clear metrics
        app.clear_metrics();
        assert!(!app.is_metrics_loading());
        assert!(app.dynamic_metrics.is_none());
        assert!(app.selected_metric_name.is_none());
        assert_eq!(app.sparkline_grid_selected_index, 0);
    }

    #[test]
    fn test_sparkline_grid_initialization() {
        let mut app = App::new();

        // Test initialize sparkline grid
        app.initialize_sparkline_grid();

        // Should not fail even with no metrics
        assert_eq!(app.sparkline_grid_selected_index, 0);
    }
}

/// Integration test that verifies all core modules work together
#[cfg(test)]
mod full_integration_tests {
    use super::*;

    #[test]
    fn test_complete_application_flow() {
        let mut app = App::new();

        // 1. Start with service list
        assert_eq!(app.get_current_state(), &AppState::ServiceList);
        assert!(!app.can_navigate_back());

        // 2. Select a service
        app.service_list_state.select(Some(0));
        app.select_service();
        assert_eq!(app.get_current_state(), &AppState::InstanceList);
        assert!(app.can_navigate_back());

        // 3. Add mock instances and select one
        app.instances = vec![crate::models::ServiceInstance::Rds(
            crate::models::RdsInstance {
                id: "test-instance".to_string(),
                name: "Test Instance".to_string(),
                identifier: "test-instance".to_string(),
                engine: "mysql".to_string(),
                status: "available".to_string(),
                endpoint: Some("test.amazonaws.com".to_string()),
                port: Some(3306),
                engine_version: Some("8.0.28".to_string()),
                instance_class: "db.t3.micro".to_string(),
                allocated_storage: Some(20),
                storage_type: Some("gp2".to_string()),
                availability_zone: Some("us-east-1a".to_string()),
                backup_retention_period: Some(7),
                multi_az: Some(false),
                storage_encrypted: Some(false),
                performance_insights_enabled: Some(false),
                deletion_protection: Some(false),
                creation_time: None,
            },
        )];
        app.list_state.select(Some(0));

        // 4. Enter metrics summary
        app.enter_metrics_summary();
        assert_eq!(app.get_current_state(), &AppState::MetricsSummary);
        assert_eq!(app.get_focused_panel(), &FocusedPanel::SparklineGrid);

        // 5. Test panel switching
        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone);

        // 6. Enter instance details
        app.enter_instance_details();
        assert_eq!(app.get_current_state(), &AppState::InstanceDetails);

        // 7. Test complete backward navigation
        app.go_back(); // Should go to MetricsSummary
        assert_eq!(app.get_current_state(), &AppState::MetricsSummary);

        app.go_back(); // Should go to InstanceList
        assert_eq!(app.get_current_state(), &AppState::InstanceList);

        app.go_back(); // Should go to ServiceList
        assert_eq!(app.get_current_state(), &AppState::ServiceList);

        // 8. Test that we can't go back further
        app.go_back(); // Should stay at ServiceList
        assert_eq!(app.get_current_state(), &AppState::ServiceList);
    }

    #[test]
    fn test_error_handling_integration() {
        let mut app = App::new();

        // Test error handling across different states
        app.set_error("Test error");
        assert!(app.has_error());

        // Error should persist across state changes
        app.state = AppState::InstanceList;
        assert!(app.has_error());

        // Clear error
        app.clear_error();
        assert!(!app.has_error());

        // Test loading state with error
        app.start_loading();
        assert!(app.is_loading());

        app.set_error("Loading error");
        assert!(!app.is_loading()); // set_error should stop loading
        assert!(app.has_error());
    }

    #[test]
    fn test_state_consistency() {
        let mut app = App::new();

        // Test that state changes are consistent
        app.state = AppState::MetricsSummary;
        app.focused_panel = FocusedPanel::SparklineGrid;

        // Test that resetting scroll maintains state consistency
        app.reset_scroll();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone);
        assert_eq!(app.sparkline_grid_selected_index, 0);

        // Test that panel switching works correctly
        app.switch_panel();
        app.switch_panel();
        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::SparklineGrid);
    }
}
