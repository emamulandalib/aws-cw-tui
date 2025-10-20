use crate::aws::cloudwatch_service::TimeRange;
use crate::models::aws_services::{AwsService, RdsInstance, ServiceInstance, SqsQueue};
use crate::models::metrics::{DynamicMetrics, MetricData, SqsMetricData};
use ratatui::widgets::ListState;
use std::time::Instant;

/// Application state enumeration
///
/// Represents the different screens/views in the application.
/// State transitions are managed by the core::app::screen_navigation module.
#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AppState {
    ServiceList,     // Show list of available AWS services
    InstanceList,    // Show instances for selected service
    MetricsSummary,  // Show metrics summary for selected instance
    InstanceDetails, // Show detailed metrics for selected instance
}

/// Panel focus state for navigation
///
/// Represents which panel is currently focused in the MetricsSummary view.
/// Focus management is handled by the core::app::ui_state module.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum FocusedPanel {
    Timezone,
    Period,
    TimeRanges,
    SparklineGrid,
}

impl std::fmt::Display for FocusedPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FocusedPanel::Timezone => write!(f, "Timezone"),
            FocusedPanel::Period => write!(f, "Period"),
            FocusedPanel::TimeRanges => write!(f, "TimeRanges"),
            FocusedPanel::SparklineGrid => write!(f, "SparklineGrid"),
        }
    }
}

/// Time range display mode
///
/// Time range management is handled by the core::app::time_range_management module.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TimeRangeMode {
    Absolute,
    Relative,
}

/// Timezone selection
///
/// Timezone management is handled by the core::app::time_range_management module.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Timezone {
    Utc,
    Local,
}

impl Timezone {
    pub fn display_name(&self) -> &'static str {
        match self {
            Timezone::Utc => "UTC timezone",
            Timezone::Local => "Local timezone",
        }
    }

    pub fn get_timezone_options() -> Vec<Timezone> {
        vec![Timezone::Local, Timezone::Utc]
    }
}

/// Main application state structure
///
/// This struct contains all the state needed for the AWS CloudWatch TUI application.
/// Different aspects of the state are managed by different core modules:
///
/// - Service management: core::app::service_management
/// - Instance access: core::app::instance_access  
/// - Navigation: core::app::navigation
/// - Screen transitions: core::app::screen_navigation
/// - UI state: core::app::ui_state
/// - State management: core::app::state_management
/// - Time range: core::app::time_range_management
/// - Metrics: core::app::metrics_management
/// - Initialization: core::app::initialization
pub struct App {
    // === Theme Configuration ===
    // Managed by core::app::theme_management module
    pub current_theme: crate::ui::themes::ThemeVariant,

    // === Service Selection State ===
    // Managed by core::app::service_management module
    pub available_services: Vec<AwsService>,
    pub service_list_state: ListState,
    pub selected_service: Option<AwsService>,

    // === Instance Management State ===
    // Managed by core::app::instance_access and core::app::service_management modules
    pub instances: Vec<ServiceInstance>,
    pub rds_instances: Vec<RdsInstance>, // Keep for backward compatibility during transition
    pub sqs_queues: Vec<SqsQueue>,       // SQS queues for the selected service
    pub list_state: ListState,
    pub selected_instance: Option<usize>,

    // === Application State and Navigation ===
    // Managed by core::app::screen_navigation and core::app::ui_state modules
    pub state: AppState,
    pub focused_panel: FocusedPanel,
    pub saved_focused_panel: FocusedPanel,

    // === Metrics Data ===
    // Managed by core::app::metrics_management module
    pub dynamic_metrics: Option<DynamicMetrics>,
    pub metrics: MetricData, // Legacy hardcoded metrics for backward compatibility
    pub sqs_metrics: SqsMetricData, // SQS-specific metrics
    pub selected_metric_name: Option<String>,
    pub sparkline_grid_selected_index: usize,
    pub saved_sparkline_grid_selected_index: usize,
    pub sparkline_grid_list_state: ListState,

    // === Time Range Configuration ===
    // Managed by core::app::time_range_management module
    pub time_range: TimeRange,
    pub time_range_mode: TimeRangeMode,
    pub timezone: Timezone,
    pub selected_period: Option<i32>, // Manual period override in seconds (None = auto-calculate)
    pub time_range_list_state: ListState,
    pub period_list_state: ListState,
    pub timezone_list_state: ListState,

    // === Loading and Error State ===
    // Managed by core::app::state_management module
    pub loading: bool,
    pub metrics_loading: bool,
    pub error_message: Option<String>,
    pub loading_start_time: Option<Instant>,

    // === Auto-refresh State ===
    // Managed by core::app::state_management module
    pub last_refresh: Option<Instant>,
    pub auto_refresh_enabled: bool,

    // === Help System ===
    // Managed by help system module
    pub help_system: crate::ui::components::help_system::HelpSystem,

    // === Error Feedback System ===
    // Enhanced error handling and user feedback
    pub error_feedback: crate::ui::components::error_feedback::ErrorFeedback,

    // === Workflow Memory System ===
    // Tracks user patterns and optimizes workflows
    pub workflow_memory: crate::core::workflow_memory::WorkflowMemory,

    // === Performance Optimization ===
    // UI responsiveness and performance monitoring
    pub performance_optimizer: crate::ui::performance::PerformanceOptimizer,
}

impl App {
    // Note: The new() method is implemented in core::app::initialization
    // This ensures all defaults are properly set up through the core modules

    /// Get the current theme
    pub fn get_current_theme(&self) -> crate::ui::themes::UnifiedTheme {
        self.current_theme.to_theme()
    }

    /// Switch to the next theme in the cycle
    pub fn next_theme(&mut self) {
        self.current_theme = match self.current_theme {
            crate::ui::themes::ThemeVariant::Default => crate::ui::themes::ThemeVariant::WarmSunset,
            crate::ui::themes::ThemeVariant::WarmSunset => {
                crate::ui::themes::ThemeVariant::BlueGold
            }
            crate::ui::themes::ThemeVariant::BlueGold => {
                crate::ui::themes::ThemeVariant::HighContrast
            }
            crate::ui::themes::ThemeVariant::HighContrast => {
                crate::ui::themes::ThemeVariant::Monochrome
            }
            crate::ui::themes::ThemeVariant::Monochrome => {
                crate::ui::themes::ThemeVariant::TerminalCyan
            }
            crate::ui::themes::ThemeVariant::TerminalCyan => {
                crate::ui::themes::ThemeVariant::Default
            }
        };
    }

    /// Get the current theme name for display
    pub fn get_current_theme_name(&self) -> &'static str {
        match self.current_theme {
            crate::ui::themes::ThemeVariant::Default => "Default",
            crate::ui::themes::ThemeVariant::WarmSunset => "Warm Sunset",
            crate::ui::themes::ThemeVariant::BlueGold => "Blue Gold",
            crate::ui::themes::ThemeVariant::HighContrast => "High Contrast",
            crate::ui::themes::ThemeVariant::Monochrome => "Monochrome",
            crate::ui::themes::ThemeVariant::TerminalCyan => "Terminal Cyan",
        }
    }

    /// Check if the application is in a loading state
    ///
    /// Uses core::app::state_management for consistent state checking.
    pub fn is_loading(&self) -> bool {
        self.loading || self.metrics_loading
    }

    /// Check if the application has any errors
    ///
    /// Uses core::app::state_management for consistent error checking.
    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    /// Get the current error message if any
    ///
    /// Uses core::app::state_management for consistent error handling.
    pub fn get_error(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    /// Check if the application can navigate back from the current state
    ///
    /// Uses core::app::screen_navigation for consistent navigation logic.
    pub fn can_navigate_back(&self) -> bool {
        !matches!(self.state, AppState::ServiceList)
    }

    /// Check if the application has instances available
    ///
    /// Uses core::app::instance_access for consistent instance checking.
    pub fn has_available_instances(&self) -> bool {
        !self.instances.is_empty()
    }

    /// Check if the application has metrics available
    ///
    /// Uses core::app::metrics_management for consistent metrics checking.
    pub fn has_available_metrics(&self) -> bool {
        self.dynamic_metrics.is_some() && self.dynamic_metrics.as_ref().unwrap().len() > 0
    }

    // Note: The get_current_state() method is implemented in core::app::screen_navigation
    // This provides consistent state access through the core modules

    /// Check if a specific panel is currently focused
    ///
    /// Uses core::app::ui_state for consistent focus management.
    pub fn is_focused_on(&self, panel: &FocusedPanel) -> bool {
        &self.focused_panel == panel
    }

    /// Save current theme preference to configuration
    pub fn save_theme_preference(&self) -> anyhow::Result<()> {
        use crate::config::ConfigManager;

        let config_manager = ConfigManager::new()?;
        config_manager.set_theme(self.current_theme.clone())?;

        tracing::info!("Theme preference saved: {:?}", self.current_theme);
        Ok(())
    }

    /// Save auto-refresh settings to configuration
    pub fn save_auto_refresh_settings(&self) -> anyhow::Result<()> {
        use crate::config::ConfigManager;

        let config_manager = ConfigManager::new()?;
        config_manager.set_auto_refresh_enabled(self.auto_refresh_enabled)?;

        tracing::info!("Auto-refresh settings saved: {}", self.auto_refresh_enabled);
        Ok(())
    }

    /// Save time range preference to configuration
    pub fn save_time_range_preference(&self) -> anyhow::Result<()> {
        use crate::aws::time_range::TimeUnit;
        use crate::config::ConfigManager;

        let time_range_str = match self.time_range.unit {
            TimeUnit::Hours => format!("{}h", self.time_range.value),
            TimeUnit::Days => format!("{}d", self.time_range.value),
            TimeUnit::Weeks => format!("{}w", self.time_range.value),
            TimeUnit::Minutes => format!("{}m", self.time_range.value),
            TimeUnit::Months => format!("{}M", self.time_range.value),
        };

        let config_manager = ConfigManager::new()?;
        config_manager.set_default_time_range(time_range_str.clone())?;

        tracing::info!("Time range preference saved: {}", time_range_str);
        Ok(())
    }

    /// Load and apply configuration updates
    pub fn reload_configuration(&mut self) -> anyhow::Result<()> {
        use crate::config::ConfigManager;

        let config_manager = ConfigManager::new()?;
        let config = config_manager.get_config();

        // Apply configuration updates
        self.current_theme = config.theme.current_theme;
        self.auto_refresh_enabled = config.ui.auto_refresh_enabled;

        // Parse and apply time range
        let (time_value, time_unit) = crate::core::app::initialization::parse_time_range_config(
            &config.ui.default_time_range,
        );
        if let Ok(new_time_range) = TimeRange::new(time_value, time_unit, 1) {
            self.time_range = new_time_range;
        }

        tracing::info!("Configuration reloaded and applied");
        Ok(())
    }
}
