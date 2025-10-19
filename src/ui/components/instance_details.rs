// Updated imports for focused rendering modules
use crate::models::App;
use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::charts::error_display::render_error_message;
use crate::ui::charts::rendering::dynamic_charts::render_dynamic_metric_chart;
use crate::ui::charts::rendering::metric_charts::render_metric_chart;
use crate::ui::components::universal_box::UniversalBox;
use crate::ui::components::{render_rds_instance_details, render_sqs_queue_details};
use crate::ui::themes::UnifiedTheme;
use log::{debug, info, warn};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// Include debug logging macros
use crate::log_metric_operation;

pub fn render_instance_details(f: &mut Frame, app: &mut App, theme: &UnifiedTheme) {
    debug!("INSTANCE_DETAILS: Starting render");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(f.area());

    let instance_id = app
        .get_selected_instance_id()
        .unwrap_or_else(|| "unknown".to_string());
    debug!("INSTANCE_DETAILS: Selected instance ID: {}", instance_id);

    if let Some(selected_instance) = app.get_selected_instance() {
        match selected_instance {
            crate::models::ServiceInstance::Rds(instance) => {
                render_rds_instance_details(f, chunks[0], instance, true, theme);
            }
            crate::models::ServiceInstance::Sqs(queue) => {
                render_sqs_queue_details(f, chunks[0], &queue.name, true, theme);
            }
        }
    } else if app.metrics_loading {
        info!("INSTANCE_DETAILS: Metrics are loading, showing loading state");
        render_metrics_loading(f, chunks[0], theme);
    } else {
        // Use focused rendering modules instead of monolithic renderer
        render_detailed_metric_view(f, chunks[0], app, theme);
    }
}

/// Render detailed metric view using focused rendering modules
fn render_detailed_metric_view(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &App,
    theme: &UnifiedTheme,
) {
    debug!("INSTANCE_DETAILS: Rendering detailed metric view");

    if let Some(selected_metric) = &app.selected_metric_name {
        debug!("INSTANCE_DETAILS: Selected metric: {}", selected_metric);

        // Try to find the metric in dynamic metrics first
        if let Some(dynamic_metrics) = &app.dynamic_metrics {
            debug!(
                "INSTANCE_DETAILS: Checking dynamic metrics for: {}",
                selected_metric
            );
            // Search through the metrics to find the matching one
            if let Some(metric_data) = dynamic_metrics
                .metrics
                .iter()
                .find(|m| &m.display_name == selected_metric)
            {
                debug!(
                    "INSTANCE_DETAILS: Found dynamic metric data for: {}",
                    selected_metric
                );
                render_dynamic_metric_chart(f, area, metric_data, true, theme);
                return;
            }
        }

        // Fallback to legacy metrics if dynamic metrics not found
        debug!("INSTANCE_DETAILS: Dynamic metric not found, checking legacy metrics");
        if let Some(chart_data) = get_legacy_metric_data(app, selected_metric) {
            debug!(
                "INSTANCE_DETAILS: Found legacy metric data for: {}",
                selected_metric
            );
            render_metric_chart(f, area, &chart_data, true, theme);
            return;
        }

        // If no metric data found, show error
        warn!(
            "INSTANCE_DETAILS: No metric data found for: {}",
            selected_metric
        );
        render_error_message(
            f,
            area,
            &format!("No data available for metric: {}", selected_metric),
        );
    } else {
        warn!("INSTANCE_DETAILS: No metric selected");
        render_error_message(
            f,
            area,
            "No metric selected. Return to metrics summary to select a metric.",
        );
    }
}

/// Render the metrics loading indicator
pub fn render_metrics_loading(f: &mut Frame, area: ratatui::layout::Rect, theme: &UnifiedTheme) {
    debug!("LOADING: Rendering metrics loading indicator");

    UniversalBox::loading_box("Loading", "Loading metrics data...", theme.clone()).render(f, area);
}

/// Get legacy metric data for backward compatibility
fn get_legacy_metric_data(app: &App, metric_name: &str) -> Option<MetricChartData> {
    debug!(
        "LEGACY_METRIC: Getting legacy metric data for: {}",
        metric_name
    );

    // Check if the metric exists in legacy metrics
    let metric_type = match metric_name {
        "CPUUtilization" => crate::models::MetricType::CpuUtilization,
        "DatabaseConnections" => crate::models::MetricType::DatabaseConnections,
        "ReadLatency" => crate::models::MetricType::ReadLatency,
        "WriteLatency" => crate::models::MetricType::WriteLatency,
        "ReadIOPS" => crate::models::MetricType::ReadIops,
        "WriteIOPS" => crate::models::MetricType::WriteIops,
        "FreeStorageSpace" => crate::models::MetricType::FreeStorageSpace,
        "FreeableMemory" => crate::models::MetricType::FreeableMemory,
        _ => {
            debug!("LEGACY_METRIC: Unknown metric name: {}", metric_name);
            return None;
        }
    };

    // Use the existing from_app method to create chart data
    MetricChartData::from_app(app, metric_type)
}
