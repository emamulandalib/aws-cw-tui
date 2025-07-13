use crate::aws::cloudwatch_service::TimeRange;
use crate::models::aws_services::{AwsService, RdsInstance, ServiceInstance, SqsQueue};
use crate::models::metrics::{DynamicMetrics, MetricData, SqsMetricData};
use ratatui::widgets::ListState;
use std::time::Instant;

/// Application state enumeration
#[derive(Debug, PartialEq)]
pub enum AppState {
    ServiceList,     // Show list of available AWS services
    InstanceList,    // Show instances for selected service
    MetricsSummary,  // Show metrics summary for selected instance
    InstanceDetails, // Show detailed metrics for selected instance
}

/// Panel focus state for navigation
#[derive(Debug, PartialEq, Clone)]
pub enum FocusedPanel {
    Timezone,
    Period,
    TimeRanges,
    SparklineGrid,
}

/// Time range display mode
#[derive(Debug, Clone, PartialEq)]
pub enum TimeRangeMode {
    Absolute,
    Relative,
}

/// Timezone selection
#[derive(Debug, Clone, PartialEq)]
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

/// Main application state container
pub struct App {
    // Service selection state
    pub available_services: Vec<AwsService>,
    pub service_list_state: ListState,
    pub selected_service: Option<AwsService>,

    // Instance list state (generic for all services)
    pub instances: Vec<ServiceInstance>,
    pub rds_instances: Vec<RdsInstance>, // Keep for backward compatibility during transition
    pub sqs_queues: Vec<SqsQueue>,       // SQS queues for the selected service
    pub list_state: ListState,
    pub loading: bool,
    pub state: AppState,
    pub selected_instance: Option<usize>,

    // Dynamic metrics that work for any service
    pub dynamic_metrics: Option<DynamicMetrics>,

    // Legacy hardcoded metrics for backward compatibility during UI transition
    pub metrics: MetricData,
    pub sqs_metrics: SqsMetricData,

    // Metrics and refresh state
    pub metrics_loading: bool,
    pub last_refresh: Option<Instant>,
    pub auto_refresh_enabled: bool,
    pub focused_panel: FocusedPanel,
    pub saved_focused_panel: FocusedPanel,
    pub time_range: TimeRange,

    // Sparkline grid state
    pub selected_metric_name: Option<String>,
    pub sparkline_grid_selected_index: usize,
    pub saved_sparkline_grid_selected_index: usize,
    pub sparkline_grid_list_state: ListState,

    // UI state for different components
    pub time_range_list_state: ListState,
    pub period_list_state: ListState,
    pub timezone_list_state: ListState,

    // Error handling
    pub error_message: Option<String>,

    // Loading timeout management
    pub loading_start_time: Option<Instant>,

    // Time range display mode
    pub time_range_mode: TimeRangeMode,

    // Timezone selection
    pub timezone: Timezone,
}
