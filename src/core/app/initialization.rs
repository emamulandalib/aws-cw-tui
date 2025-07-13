use crate::aws::time_range::{TimeRange, TimeUnit};
use crate::models::{App, AppState, AwsService, FocusedPanel, TimeRangeMode, Timezone};

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new application instance with proper default initialization
    pub fn new() -> App {
        let mut app = App {
            // Service selection initialization
            available_services: vec![AwsService::Rds, AwsService::Sqs],
            service_list_state: ratatui::widgets::ListState::default(),
            selected_service: None,

            // Instance list initialization
            instances: Vec::new(),
            rds_instances: Vec::new(),
            sqs_queues: Vec::new(),
            list_state: ratatui::widgets::ListState::default(),
            loading: false,
            state: AppState::ServiceList,
            selected_instance: None,

            // Metrics initialization
            dynamic_metrics: None,
            metrics: crate::models::MetricData::default(),
            sqs_metrics: crate::models::SqsMetricData::default(),
            metrics_loading: false,
            last_refresh: None,
            auto_refresh_enabled: true,

            // UI state initialization
            focused_panel: FocusedPanel::Timezone,
            saved_focused_panel: FocusedPanel::Timezone,
            time_range: TimeRange::new(3, TimeUnit::Hours, 1).unwrap(),
            selected_period: None, // Start with auto-calculated period

            // Sparkline grid state
            selected_metric_name: None,
            sparkline_grid_selected_index: 0,
            saved_sparkline_grid_selected_index: 0,
            sparkline_grid_list_state: ratatui::widgets::ListState::default(),

            // Built-in list states for all components
            time_range_list_state: Self::create_time_range_list_state(),
            period_list_state: Self::create_period_list_state(),
            timezone_list_state: Self::create_timezone_list_state(),

            // Error handling
            error_message: None,
            loading_start_time: None,

            // Time range configuration
            time_range_mode: TimeRangeMode::Relative,
            timezone: Timezone::Utc,
        };

        // Set initial service selection
        app.service_list_state.select(Some(0));
        app
    }

    /// Create properly initialized time range list state
    fn create_time_range_list_state() -> ratatui::widgets::ListState {
        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(8)); // Default to "3 hours" option
        state
    }

    /// Create properly initialized period list state
    fn create_period_list_state() -> ratatui::widgets::ListState {
        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(2)); // Default to a reasonable period option
        state
    }

    /// Create properly initialized timezone list state
    fn create_timezone_list_state() -> ratatui::widgets::ListState {
        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(1)); // Default to UTC (index 1 in the options)
        state
    }
}
