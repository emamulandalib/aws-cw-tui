// Integration tests for sparkline interface
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use crate::models::{App, MetricData, MetricType, FocusedPanel};

    fn create_test_app_with_metrics() -> App {
        let mut app = App::new();
        
        // Simulate some test metric data
        let mut metrics = MetricData::default();
        
        // Add some sample data for testing
        let now = SystemTime::now();
        for i in 0..10 {
            metrics.timestamps.push(now);
            metrics.cpu_history.push(50.0 + (i as f64 * 5.0));
            metrics.connections_history.push(100.0 + (i as f64 * 10.0));
            metrics.read_iops_history.push(200.0 + (i as f64 * 20.0));
            metrics.write_iops_history.push(150.0 + (i as f64 * 15.0));
        }
        
        app.metrics = metrics;
        app.initialize_sparkline_grid();
        
        app
    }

    #[test]
    fn test_two_panel_navigation() {
        let mut app = create_test_app_with_metrics();
        
        // Test 1: Initial state should be time ranges panel
        assert_eq!(app.get_focused_panel(), &FocusedPanel::TimeRanges);
        
        // Test 2: Switch to sparkline grid panel
        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::SparklineGrid);
        
        // Test 3: Switch back to time ranges panel (complete cycle)
        app.switch_panel();
        assert_eq!(app.get_focused_panel(), &FocusedPanel::TimeRanges);
    }

    #[test]
    fn test_sparkline_grid_navigation() {
        let mut app = create_test_app_with_metrics();
        
        // Test 1: Initial sparkline grid state
        let initial_index = app.get_sparkline_grid_selected_index();
        assert_eq!(initial_index, 0);
        
        // Test 2: Navigate down in sparkline grid
        app.sparkline_grid_scroll_down();
        let after_down = app.get_sparkline_grid_selected_index();
        assert_eq!(after_down, 1);
        
        // Test 3: Navigate up in sparkline grid
        app.sparkline_grid_scroll_up();
        let after_up = app.get_sparkline_grid_selected_index();
        assert_eq!(after_up, 0);
        
        // Test 4: Verify selected metric updates
        let available_metrics = app.get_available_metrics();
        if !available_metrics.is_empty() {
            let selected_metric = app.get_selected_metric();
            assert!(selected_metric.is_some());
            assert_eq!(selected_metric.unwrap(), &available_metrics[0]);
        }
    }

    #[test]
    fn test_metric_data_validation() {
        let mut app = create_test_app_with_metrics();
        
        // Test 1: Verify available metrics
        let available_metrics = app.get_available_metrics();
        assert!(!available_metrics.is_empty());
        assert!(available_metrics.contains(&MetricType::CpuUtilization));
        assert!(available_metrics.contains(&MetricType::DatabaseConnections));
        
        // Test 2: Verify metric history retrieval
        let cpu_history = app.get_metric_history(&MetricType::CpuUtilization);
        assert!(!cpu_history.is_empty());
        assert_eq!(cpu_history.len(), 10);
        
        // Test 3: Verify metric count
        let count = app.metrics.count_available_metrics();
        assert!(count >= 4); // We added at least 4 metrics
    }

    #[test]
    fn test_edge_cases() {
        // Test 1: Empty metrics
        let mut empty_app = App::new();
        empty_app.initialize_sparkline_grid();
        assert!(empty_app.get_available_metrics().is_empty());
        assert!(empty_app.get_selected_metric().is_none());
        
        // Test 2: Single metric
        let mut single_metric_app = App::new();
        single_metric_app.metrics.cpu_history.push(50.0);
        single_metric_app.metrics.timestamps.push(SystemTime::now());
        single_metric_app.initialize_sparkline_grid();
        
        let available = single_metric_app.get_available_metrics();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0], MetricType::CpuUtilization);
        
        // Test 3: Boundary navigation
        let mut app = create_test_app_with_metrics();
        let available_count = app.get_available_metrics().len();
        
        // Navigate to last item
        for _ in 0..available_count {
            app.sparkline_grid_scroll_down();
        }
        let final_index = app.get_sparkline_grid_selected_index();
        assert_eq!(final_index, available_count - 1);
        
        // Try to navigate beyond - should stay at last item
        app.sparkline_grid_scroll_down();
        assert_eq!(app.get_sparkline_grid_selected_index(), available_count - 1);
    }

    #[test]
    fn test_time_range_functionality() {
        let mut app = create_test_app_with_metrics();
        
        // Test 1: Initial time range
        let initial_index = app.get_current_time_range_index();
        assert_eq!(initial_index, 2); // Should be "3 hours" as per app.rs line 28
        
        // Test 2: Time range options
        let options = App::get_time_range_options();
        assert!(!options.is_empty());
        assert_eq!(options.len(), 10); // As defined in app.rs
        
        // Test 3: Time range navigation
        app.time_range_scroll_down();
        let after_down = app.get_current_time_range_index();
        assert_eq!(after_down, 3);
        
        app.time_range_scroll_up();
        let after_up = app.get_current_time_range_index();
        assert_eq!(after_up, 2);
    }
}