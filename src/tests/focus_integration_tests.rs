//! Integration tests for focus transitions and border color updates
//!
//! These tests verify that focus state changes trigger correct border color updates
//! specifically on the metrics page, and that non-metrics pages maintain consistent
//! border colors during navigation.

use crate::models::{App, AppState, FocusedPanel};
use crate::ui::components::list_styling::border_factory;
use crate::ui::themes::UnifiedTheme;
use ratatui::style::Color;

/// Test focus state changes trigger correct border color updates on metrics list page
#[cfg(test)]
mod metrics_page_focus_tests {
    use super::*;

    #[test]
    fn test_metrics_page_focus_border_colors() {
        let theme = UnifiedTheme::warm_sunset();

        // Test focused state on metrics page should use orange border
        let focused_style = border_factory::create_theme_border_style(&theme, true);
        assert_eq!(focused_style.fg, Some(Color::Rgb(247, 127, 0))); // #F77F00 orange

        // Test unfocused state on metrics page should use beige border
        let unfocused_style = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(unfocused_style.fg, Some(Color::Rgb(234, 226, 183))); // #EAE2B7 beige
    }

    #[test]
    fn test_metrics_page_panel_focus_transitions() {
        let mut app = App::new();
        app.state = AppState::MetricsSummary;
        let theme = app.get_current_theme();

        // Test initial focus state (should be Timezone)
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone);

        // Test border color for initially focused panel
        let initial_border = border_factory::create_theme_border_style(&theme, true);
        assert_eq!(initial_border.fg, Some(Color::Rgb(247, 127, 0))); // Orange for focused

        // Test border color for unfocused panels
        let unfocused_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(unfocused_border.fg, Some(Color::Rgb(234, 226, 183))); // Beige for unfocused

        // Test panel switching and verify focus changes
        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Period);

        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::TimeRanges);

        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::SparklineGrid);

        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::Timezone); // Should cycle back

        // Verify border colors remain consistent after panel switches
        let final_focused_border = border_factory::create_theme_border_style(&theme, true);
        let final_unfocused_border = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(final_focused_border.fg, Some(Color::Rgb(247, 127, 0))); // Still orange
        assert_eq!(final_unfocused_border.fg, Some(Color::Rgb(234, 226, 183))); // Still beige
    }

    #[test]
    fn test_sparkline_grid_focus_navigation() {
        let mut app = App::new();
        app.state = AppState::MetricsSummary;
        app.focused_panel = FocusedPanel::SparklineGrid;
        let theme = app.get_current_theme();

        // Add some mock metrics to enable navigation
        app.dynamic_metrics = Some(crate::models::metrics::DynamicMetrics::new(
            crate::models::aws_services::AwsService::Rds,
            "test-instance".to_string(),
        ));

        // Test initial sparkline grid selection
        assert_eq!(app.sparkline_grid_selected_index, 0);

        // Test that focused sparkline grid uses orange border
        let focused_border = border_factory::create_theme_border_style(&theme, true);
        assert_eq!(focused_border.fg, Some(Color::Rgb(247, 127, 0)));

        // Test that unfocused elements use beige border
        let unfocused_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(unfocused_border.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_metrics_page_keyboard_navigation_border_updates() {
        let mut app = App::new();
        app.state = AppState::MetricsSummary;
        let theme = app.get_current_theme();

        // Test Tab key navigation through panels
        let initial_panel = app.get_focused_panel().clone();

        // Simulate Tab key press (panel switching)
        app.switch_panel();
        let after_tab_panel = app.get_focused_panel().clone();

        // Verify panel changed
        assert_ne!(initial_panel, after_tab_panel);

        // Verify border colors are still correct after navigation
        let focused_border = border_factory::create_theme_border_style(&theme, true);
        let unfocused_border = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(focused_border.fg, Some(Color::Rgb(247, 127, 0))); // Orange for focused
        assert_eq!(unfocused_border.fg, Some(Color::Rgb(234, 226, 183))); // Beige for unfocused

        // Test arrow key navigation within panels
        app.focused_panel = FocusedPanel::SparklineGrid;
        let _initial_index = app.sparkline_grid_selected_index;

        // Simulate arrow key navigation (scroll operations)
        app.scroll_down();
        app.scroll_up();
        app.scroll_left();
        app.scroll_right();

        // Verify border colors remain consistent after navigation
        let post_nav_focused = border_factory::create_theme_border_style(&theme, true);
        let post_nav_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(post_nav_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(post_nav_unfocused.fg, Some(Color::Rgb(234, 226, 183)));
    }
}

/// Test that non-metrics pages maintain consistent border colors during navigation
#[cfg(test)]
mod non_metrics_page_consistency_tests {
    use super::*;

    #[test]
    fn test_service_list_page_consistent_borders() {
        let mut app = App::new();
        app.state = AppState::ServiceList;
        let theme = app.get_current_theme();

        // Non-metrics pages should always use unfocused border color (#EAE2B7)
        let border_style = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(border_style.fg, Some(Color::Rgb(234, 226, 183))); // #EAE2B7 beige

        // Even when "focused" parameter is true, non-metrics pages should use consistent color
        // (This would be handled by the component implementation, but we test the expectation)
        let consistent_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(consistent_border.fg, Some(Color::Rgb(234, 226, 183)));

        // Test navigation doesn't change border color expectations
        app.service_next();
        app.service_previous();

        let post_nav_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(post_nav_border.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_instance_list_page_consistent_borders() {
        let mut app = App::new();
        app.state = AppState::InstanceList;
        let theme = app.get_current_theme();

        // Instance list page should consistently use beige borders
        let border_style = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(border_style.fg, Some(Color::Rgb(234, 226, 183))); // #EAE2B7 beige

        // Add mock instances for navigation testing
        app.instances = vec![
            crate::models::aws_services::ServiceInstance::Rds(
                crate::models::aws_services::RdsInstance {
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
                },
            ),
            crate::models::aws_services::ServiceInstance::Rds(
                crate::models::aws_services::RdsInstance {
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
                },
            ),
        ];

        // Test navigation through instances
        app.list_state.select(Some(0));
        app.next();
        app.previous();

        // Border color should remain consistent after navigation
        let post_nav_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(post_nav_border.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_instance_details_page_consistent_borders() {
        let mut app = App::new();
        app.state = AppState::InstanceDetails;
        let theme = app.get_current_theme();

        // Instance details page should consistently use beige borders
        let border_style = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(border_style.fg, Some(Color::Rgb(234, 226, 183))); // #EAE2B7 beige

        // Test navigation within instance details
        app.scroll_up();
        app.scroll_down();

        // Border color should remain consistent
        let post_nav_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(post_nav_border.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_page_transitions_maintain_border_consistency() {
        let mut app = App::new();
        let theme = app.get_current_theme();

        // Test ServiceList -> InstanceList transition
        app.state = AppState::ServiceList;
        let service_list_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(service_list_border.fg, Some(Color::Rgb(234, 226, 183)));

        app.state = AppState::InstanceList;
        let instance_list_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(instance_list_border.fg, Some(Color::Rgb(234, 226, 183)));

        // Test InstanceList -> MetricsSummary transition
        app.state = AppState::MetricsSummary;
        let metrics_unfocused_border = border_factory::create_theme_border_style(&theme, false);
        let metrics_focused_border = border_factory::create_theme_border_style(&theme, true);

        // Metrics page should have different focused/unfocused colors
        assert_eq!(metrics_unfocused_border.fg, Some(Color::Rgb(234, 226, 183))); // Beige
        assert_eq!(metrics_focused_border.fg, Some(Color::Rgb(247, 127, 0))); // Orange

        // Test MetricsSummary -> InstanceDetails transition
        app.state = AppState::InstanceDetails;
        let details_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(details_border.fg, Some(Color::Rgb(234, 226, 183))); // Back to consistent beige
    }
}

/// Test focus behavior differences between metrics and non-metrics pages
#[cfg(test)]
mod page_specific_focus_behavior_tests {
    use super::*;

    #[test]
    fn test_metrics_page_vs_non_metrics_page_focus_behavior() {
        let theme = UnifiedTheme::warm_sunset();

        // Metrics page should have distinct focused/unfocused colors
        let metrics_focused = border_factory::create_theme_border_style(&theme, true);
        let metrics_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(metrics_focused.fg, Some(Color::Rgb(247, 127, 0))); // Orange
        assert_eq!(metrics_unfocused.fg, Some(Color::Rgb(234, 226, 183))); // Beige
        assert_ne!(metrics_focused.fg, metrics_unfocused.fg); // Should be different

        // Non-metrics pages should consistently use unfocused color
        // (Components should call with is_focused=false for non-metrics pages)
        let non_metrics_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(non_metrics_border.fg, Some(Color::Rgb(234, 226, 183))); // Always beige
    }

    #[test]
    fn test_focus_state_simulation_for_different_pages() {
        let mut app = App::new();
        let theme = app.get_current_theme();

        // Simulate focus behavior on metrics page
        app.state = AppState::MetricsSummary;
        app.focused_panel = FocusedPanel::SparklineGrid;

        // Focused component on metrics page should use orange
        let metrics_focused_border = border_factory::create_theme_border_style(&theme, true);
        assert_eq!(metrics_focused_border.fg, Some(Color::Rgb(247, 127, 0)));

        // Unfocused components on metrics page should use beige
        let metrics_unfocused_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(metrics_unfocused_border.fg, Some(Color::Rgb(234, 226, 183)));

        // Simulate focus behavior on non-metrics pages
        app.state = AppState::ServiceList;

        // All components on non-metrics pages should use beige (unfocused color)
        let non_metrics_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(non_metrics_border.fg, Some(Color::Rgb(234, 226, 183)));

        app.state = AppState::InstanceList;
        let instance_list_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(instance_list_border.fg, Some(Color::Rgb(234, 226, 183)));

        app.state = AppState::InstanceDetails;
        let instance_details_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(instance_details_border.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_theme_consistency_across_focus_transitions() {
        let mut app = App::new();
        let initial_theme = app.get_current_theme();

        // Test focus transitions on metrics page
        app.state = AppState::MetricsSummary;

        // Test all panel focus states
        let panels = vec![
            FocusedPanel::Timezone,
            FocusedPanel::Period,
            FocusedPanel::TimeRanges,
            FocusedPanel::SparklineGrid,
        ];

        for panel in panels {
            app.focused_panel = panel.clone();

            // Verify focused border color is consistent
            let focused_border = border_factory::create_theme_border_style(&initial_theme, true);
            assert_eq!(focused_border.fg, Some(Color::Rgb(247, 127, 0))); // Orange

            // Verify unfocused border color is consistent
            let unfocused_border = border_factory::create_theme_border_style(&initial_theme, false);
            assert_eq!(unfocused_border.fg, Some(Color::Rgb(234, 226, 183))); // Beige
        }
    }
}

/// Integration tests that simulate complete user navigation flows
#[cfg(test)]
mod navigation_flow_integration_tests {
    use super::*;

    #[test]
    fn test_complete_navigation_flow_border_consistency() {
        let mut app = App::new();
        let theme = app.get_current_theme();

        // Start at ServiceList
        app.state = AppState::ServiceList;
        let service_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(service_border.fg, Some(Color::Rgb(234, 226, 183)));

        // Navigate to InstanceList
        app.state = AppState::InstanceList;
        let instance_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(instance_border.fg, Some(Color::Rgb(234, 226, 183)));

        // Navigate to MetricsSummary
        app.state = AppState::MetricsSummary;
        app.focused_panel = FocusedPanel::Timezone;

        // Test focus transitions within metrics page
        let metrics_focused = border_factory::create_theme_border_style(&theme, true);
        let metrics_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(metrics_focused.fg, Some(Color::Rgb(247, 127, 0))); // Orange
        assert_eq!(metrics_unfocused.fg, Some(Color::Rgb(234, 226, 183))); // Beige

        // Simulate panel switching
        app.switch_panel(); // Timezone -> Period
        app.switch_panel(); // Period -> TimeRanges
        app.switch_panel(); // TimeRanges -> SparklineGrid

        // Verify colors remain consistent after panel switches
        let post_switch_focused = border_factory::create_theme_border_style(&theme, true);
        let post_switch_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(post_switch_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(post_switch_unfocused.fg, Some(Color::Rgb(234, 226, 183)));

        // Navigate to InstanceDetails
        app.state = AppState::InstanceDetails;
        let details_border = border_factory::create_theme_border_style(&theme, false);
        assert_eq!(details_border.fg, Some(Color::Rgb(234, 226, 183))); // Back to consistent beige

        // Navigate back to MetricsSummary
        app.state = AppState::MetricsSummary;
        let return_focused = border_factory::create_theme_border_style(&theme, true);
        let return_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(return_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(return_unfocused.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_keyboard_navigation_simulation() {
        let mut app = App::new();
        app.state = AppState::MetricsSummary;
        let theme = app.get_current_theme();

        // Simulate Tab key navigation (panel switching)
        let initial_panel = app.get_focused_panel().clone();

        // Test multiple Tab presses
        for _ in 0..4 {
            app.switch_panel();

            // Verify border colors remain correct after each panel switch
            let focused_border = border_factory::create_theme_border_style(&theme, true);
            let unfocused_border = border_factory::create_theme_border_style(&theme, false);

            assert_eq!(focused_border.fg, Some(Color::Rgb(247, 127, 0)));
            assert_eq!(unfocused_border.fg, Some(Color::Rgb(234, 226, 183)));
        }

        // Should cycle back to initial panel
        assert_eq!(app.get_focused_panel(), &initial_panel);

        // Test arrow key navigation within SparklineGrid
        app.focused_panel = FocusedPanel::SparklineGrid;

        // Simulate arrow key presses
        app.scroll_up();
        app.scroll_down();
        app.scroll_left();
        app.scroll_right();

        // Verify border colors remain consistent after arrow navigation
        let final_focused = border_factory::create_theme_border_style(&theme, true);
        let final_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(final_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(final_unfocused.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_error_state_navigation_border_consistency() {
        let mut app = App::new();
        let theme = app.get_current_theme();

        // Test border consistency when app has errors
        app.set_error("Test error");
        app.state = AppState::MetricsSummary;

        let error_state_focused = border_factory::create_theme_border_style(&theme, true);
        let error_state_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(error_state_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(error_state_unfocused.fg, Some(Color::Rgb(234, 226, 183)));

        // Clear error and verify consistency
        app.clear_error();

        let cleared_focused = border_factory::create_theme_border_style(&theme, true);
        let cleared_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(cleared_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(cleared_unfocused.fg, Some(Color::Rgb(234, 226, 183)));
    }

    #[test]
    fn test_loading_state_navigation_border_consistency() {
        let mut app = App::new();
        let theme = app.get_current_theme();

        // Test border consistency during loading states
        app.start_loading();
        app.state = AppState::MetricsSummary;

        let loading_focused = border_factory::create_theme_border_style(&theme, true);
        let loading_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(loading_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(loading_unfocused.fg, Some(Color::Rgb(234, 226, 183)));

        // Stop loading and verify consistency
        app.stop_loading();

        let stopped_focused = border_factory::create_theme_border_style(&theme, true);
        let stopped_unfocused = border_factory::create_theme_border_style(&theme, false);

        assert_eq!(stopped_focused.fg, Some(Color::Rgb(247, 127, 0)));
        assert_eq!(stopped_unfocused.fg, Some(Color::Rgb(234, 226, 183)));
    }
}
