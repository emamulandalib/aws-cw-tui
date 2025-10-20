use crate::aws::time_range::{TimeRange, TimeUnit};
use crate::config::{AppConfig, ConfigManager};
use crate::models::{App, AppState, AwsService, FocusedPanel, TimeRangeMode, Timezone};
use crate::ui::themes::ThemeVariant;
use anyhow::Result;
use tracing::{info, warn};

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new application instance with proper default initialization
    pub fn new() -> App {
        // Try to load configuration, fall back to defaults if it fails
        match Self::new_with_config() {
            Ok(app) => {
                info!("Application initialized with saved configuration");
                app
            }
            Err(e) => {
                warn!("Failed to load configuration, using defaults: {}", e);
                Self::new_with_defaults()
            }
        }
    }

    /// Create a new application instance with configuration loading
    pub fn new_with_config() -> Result<App> {
        info!("Initializing application with configuration system");

        // Load configuration
        let config_manager = ConfigManager::new()?;
        let config = config_manager.get_config();

        info!("Configuration loaded successfully");
        info!("Theme: {:?}", config.theme.current_theme);
        info!("Auto-refresh enabled: {}", config.ui.auto_refresh_enabled);
        info!("Default time range: {}", config.ui.default_time_range);

        let mut app = Self::create_app_from_config(&config);

        // Store config manager reference for future updates
        // Note: In a real implementation, we'd store this in the App struct
        // For now, we'll create it as needed

        info!("Application initialized with configuration");
        Ok(app)
    }

    /// Create application instance from configuration
    fn create_app_from_config(config: &AppConfig) -> App {
        // Parse time range from config
        let (time_value, time_unit) = Self::parse_time_range_config(&config.ui.default_time_range);
        let time_range = TimeRange::new(time_value, time_unit, 1)
            .unwrap_or_else(|_| TimeRange::new(3, TimeUnit::Hours, 1).unwrap());

        let mut app = App {
            // Theme configuration from config
            current_theme: config.theme.current_theme.clone(),

            // Service selection initialization
            available_services: Self::get_available_services_from_config(config),
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
            auto_refresh_enabled: config.ui.auto_refresh_enabled,

            // UI state initialization
            focused_panel: FocusedPanel::Timezone,
            saved_focused_panel: FocusedPanel::Timezone,
            time_range,
            selected_period: None, // Start with auto-calculated period

            // Sparkline grid state
            selected_metric_name: None,
            sparkline_grid_selected_index: 0,
            saved_sparkline_grid_selected_index: 0,
            sparkline_grid_list_state: ratatui::widgets::ListState::default(),

            // Built-in list states for all components
            time_range_list_state: Self::create_time_range_list_state_from_config(config),
            period_list_state: Self::create_period_list_state(),
            timezone_list_state: Self::create_timezone_list_state(),

            // Error handling
            error_message: None,
            loading_start_time: None,

            // Time range configuration
            time_range_mode: TimeRangeMode::Relative,
            timezone: Timezone::Utc,

            // Help system initialization
            help_system: crate::ui::components::help_system::HelpSystem::new(),
            // Error feedback system initialization
            error_feedback: crate::ui::components::error_feedback::ErrorFeedback::new(),

            // Workflow memory system initialization
            workflow_memory: crate::core::workflow_memory::WorkflowMemory::new(),

            // Performance optimization initialization
            performance_optimizer: crate::ui::performance::PerformanceOptimizer::new(),
        };

        // Set initial service selection
        app.service_list_state.select(Some(0));
        app
    }

    /// Create application with default values (fallback when config fails)
    fn new_with_defaults() -> App {
        let mut app = App {
            // Theme configuration
            current_theme: ThemeVariant::WarmSunset,

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

            // Help system initialization
            help_system: crate::ui::components::help_system::HelpSystem::new(),

            // Error feedback system initialization
            error_feedback: crate::ui::components::error_feedback::ErrorFeedback::new(),

            // Workflow memory system initialization
            workflow_memory: crate::core::workflow_memory::WorkflowMemory::new(),

            // Performance optimization initialization
            performance_optimizer: crate::ui::performance::PerformanceOptimizer::new(),
        };

        // Set initial service selection
        app.service_list_state.select(Some(0));
        app
    }

    /// Parse time range configuration string (e.g., "3h", "1d", "1w")
    fn parse_time_range_config(time_range_str: &str) -> (u32, TimeUnit) {
        let time_range_str = time_range_str.to_lowercase();

        if let Some(captures) = regex::Regex::new(r"^(\d+)([hdw])$")
            .unwrap()
            .captures(&time_range_str)
        {
            let value: u32 = captures[1].parse().unwrap_or(3);
            let unit = match &captures[2] {
                "h" => TimeUnit::Hours,
                "d" => TimeUnit::Days,
                "w" => TimeUnit::Weeks,
                _ => TimeUnit::Hours,
            };
            (value, unit)
        } else {
            // Default fallback
            (3, TimeUnit::Hours)
        }
    }
    /// Get available services from configuration
    fn get_available_services_from_config(config: &AppConfig) -> Vec<AwsService> {
        let mut services = Vec::new();

        for service_name in &config.aws.default_services {
            match service_name.to_uppercase().as_str() {
                "RDS" => services.push(AwsService::Rds),
                "SQS" => services.push(AwsService::Sqs),
                _ => {
                    warn!("Unknown service in configuration: {}", service_name);
                }
            }
        }

        // Ensure we have at least the default services
        if services.is_empty() {
            services = vec![AwsService::Rds, AwsService::Sqs];
        }

        services
    }

    /// Create time range list state from configuration
    fn create_time_range_list_state_from_config(config: &AppConfig) -> ratatui::widgets::ListState {
        let mut state = ratatui::widgets::ListState::default();

        // Map config time range to list index
        let index = match config.ui.default_time_range.as_str() {
            "1h" => 6,
            "2h" => 7,
            "3h" => 8, // Default
            "6h" => 9,
            "12h" => 10,
            "1d" => 11,
            "3d" => 12,
            "1w" => 13,
            _ => 8, // Default to 3 hours
        };

        state.select(Some(index));
        state
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

/// Parse time range configuration string (e.g., "3h", "1d", "1w") - public function
pub fn parse_time_range_config(time_range_str: &str) -> (u32, TimeUnit) {
    let time_range_str = time_range_str.to_lowercase();

    if let Some(captures) = regex::Regex::new(r"^(\d+)([hdwmM])$")
        .unwrap()
        .captures(&time_range_str)
    {
        let value: u32 = captures[1].parse().unwrap_or(3);
        let unit = match &captures[2] {
            "h" => TimeUnit::Hours,
            "d" => TimeUnit::Days,
            "w" => TimeUnit::Weeks,
            "m" => TimeUnit::Minutes,
            "M" => TimeUnit::Months,
            _ => TimeUnit::Hours,
        };
        (value, unit)
    } else {
        // Default fallback
        (3, TimeUnit::Hours)
    }
}
