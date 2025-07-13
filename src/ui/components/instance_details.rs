// Updated imports for focused rendering modules
use crate::models::App;
use crate::ui::charts::chart_data::MetricChartData;
use crate::ui::charts::validation::validate_metric_data;
use crate::ui::charts::rendering::metric_charts::render_metric_chart;
use crate::ui::charts::rendering::dynamic_charts::render_dynamic_metric_chart;
use crate::ui::charts::error_display::render_error_message;
use log::{debug, info, warn};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// Include debug logging macros
use crate::log_metric_operation;

pub fn render_instance_details(f: &mut Frame, app: &mut App) {
    debug!("INSTANCE_DETAILS: Starting render");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header - increased height to show the endpoint
            Constraint::Min(0),    // Content (metrics chart will handle its own controls)
        ])
        .split(f.area());

    let instance_id = app
        .get_selected_instance_id()
        .unwrap_or_else(|| "unknown".to_string());
    debug!("INSTANCE_DETAILS: Selected instance ID: {}", instance_id);

    // Handle both RDS and SQS instances
    if let Some(selected_instance) = app.get_selected_instance() {
        match selected_instance {
            crate::models::ServiceInstance::Rds(instance) => {
                render_rds_instance_info(f, chunks[0], instance);
            }
            crate::models::ServiceInstance::Sqs(queue) => {
                render_sqs_instance_info(f, chunks[0], queue);
            }
        }
    } else {
        // Generic header if no instance selected
        let header_block = Paragraph::new("Instance Details")
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("AWS CloudWatch TUI")
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        f.render_widget(header_block, chunks[0]);
    }

    if app.metrics_loading {
        info!("INSTANCE_DETAILS: Metrics are loading, showing loading state");
        render_metrics_loading(f, chunks[1]);
    } else {
        // Use focused rendering modules instead of monolithic renderer
        render_detailed_metric_view(f, chunks[1], app);
    }
}

/// Render detailed metric view using focused rendering modules
fn render_detailed_metric_view(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    // Get the selected metric index from the same source of truth that navigation methods use
    let selected_metric_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
    
    // Check if we have dynamic metrics available (preferred)
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        let available_metrics = dynamic_metrics.get_available_metric_names();
        
        if available_metrics.is_empty() {
            render_error_message(f, area, "No dynamic metrics available");
            return;
        }
        
        info!("INSTANCE_DETAILS: Displaying detail view for metric index: {} out of {} available metrics", 
              selected_metric_index, available_metrics.len());

        if let Some(metric_name) = available_metrics.get(selected_metric_index) {
            log_metric_operation!(
                "Display detailed view",
                metric_name,
                format!("Instance: {}, Mode: Detail", app.get_selected_instance_id().unwrap_or_else(|| "unknown".to_string()))
            );
            info!(
                "INSTANCE_DETAILS: Selected metric for detail view: '{}'",
                metric_name
            );

            // Find the metric data for the selected metric
            if let Some(metric_data) = dynamic_metrics
                .metrics
                .iter()
                .find(|m| &m.display_name == metric_name)
            {
                debug!("INSTANCE_DETAILS: Detailed metric data - Name: '{}', Unit: '{:?}', Current Value: {}, History Points: {}, Has Timestamps: {}", 
                       metric_data.display_name,
                       metric_data.unit,
                       metric_data.current_value,
                       metric_data.history.len(),
                       !metric_data.timestamps.is_empty());

                // Validate metric data using focused validation module
                if let Err(validation_error) = validate_metric_data(&metric_data.history, &metric_data.timestamps) {
                    warn!("INSTANCE_DETAILS: Metric data validation failed: {}", validation_error);
                    render_error_message(f, area, &format!("Data validation failed: {}", validation_error));
                    return;
                }

                // Log recent values for debugging (last 5 points)
                if !metric_data.history.is_empty() {
                    let recent_values: Vec<f64> =
                        metric_data.history.iter().rev().take(5).cloned().collect();
                    debug!(
                        "INSTANCE_DETAILS: Recent metric values (last 5): {:?}",
                        recent_values
                    );
                }

                // Log timestamp range
                if !metric_data.timestamps.is_empty() {
                    debug!(
                        "INSTANCE_DETAILS: Timestamp range - From: {:?} to {:?}",
                        metric_data.timestamps.first(),
                        metric_data.timestamps.last()
                    );
                }

                // Use focused rendering module for dynamic metrics
                render_dynamic_metric_chart(f, area, metric_data, true);
                info!("INSTANCE_DETAILS: Completed rendering detailed dynamic metric view");
            } else {
                warn!("INSTANCE_DETAILS: Could not find metric data for selected metric: '{}'", metric_name);
                render_error_message(f, area, &format!("Metric data not found for: {}", metric_name));
            }
        } else {
            warn!("INSTANCE_DETAILS: Selected metric index {} is out of bounds for {} available metrics", 
                  selected_metric_index, available_metrics.len());
            render_error_message(f, area, "Selected metric index is out of bounds");
        }
    } else {
        // Fallback to legacy system for RDS
        if let Some(service) = app.selected_service.as_ref() {
            match service {
                crate::models::AwsService::Rds => {
                    render_legacy_rds_detailed_view(f, area, app, selected_metric_index);
                }
                crate::models::AwsService::Sqs => {
                    warn!("INSTANCE_DETAILS: SQS service should have dynamic metrics");
                    render_error_message(f, area, "SQS metrics not available. Press 'r' to refresh.");
                }
            }
        } else {
            warn!("INSTANCE_DETAILS: No service selected");
            render_error_message(f, area, "No service selected");
        }
    }
}

/// Render detailed view for legacy RDS metrics
fn render_legacy_rds_detailed_view(f: &mut Frame, area: ratatui::layout::Rect, app: &App, selected_metric_index: usize) {
    let available_metrics = app.metrics.get_available_metrics_with_data();
    
    if available_metrics.is_empty() {
        render_error_message(f, area, "No legacy RDS metrics available");
        return;
    }
    
    let metric_type = &available_metrics[selected_metric_index];
    
    // CRITICAL: Bounds check to prevent crashes
    if selected_metric_index >= available_metrics.len() {
        warn!("INSTANCE_DETAILS: Selected index {} out of bounds for {} metrics", selected_metric_index, available_metrics.len());
        render_error_message(f, area, "Metric index out of bounds");
        return;
    }
    
            // Create chart data from app state
        if let Some(chart_data) = MetricChartData::from_app(app, metric_type.clone()) {
        // Validate the chart data before rendering
        if let Err(validation_error) = validate_metric_data(&chart_data.history, &chart_data.timestamps) {
            warn!("INSTANCE_DETAILS: Legacy chart data validation failed: {}", validation_error);
            render_error_message(f, area, &format!("Data validation failed: {}", validation_error));
            return;
        }
        
        debug!("INSTANCE_DETAILS: Rendering legacy RDS metric: {:?}", metric_type);
        
        // Render the chart using focused rendering modules
        render_metric_chart(f, area, &chart_data, true);
    } else {
        warn!("INSTANCE_DETAILS: Failed to create chart data for metric: {:?}", metric_type);
        render_error_message(f, area, "Failed to create chart data");
    }
}

fn render_rds_instance_info(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    instance: &crate::models::RdsInstance,
) {
    debug!(
        "INSTANCE_DETAILS: Rendering RDS instance info for: {}",
        instance.identifier
    );

    let na_string = "N/A".to_string();
    let info_text = vec![
        Line::from(vec![
            Span::styled("Engine: ", Style::default().fg(Color::White)),
            Span::styled(&instance.engine, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(&instance.status, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Class: ", Style::default().fg(Color::White)),
            Span::styled(&instance.instance_class, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Endpoint: ", Style::default().fg(Color::White)),
            Span::styled(
                instance.endpoint.as_ref().unwrap_or(&na_string),
                Style::default().fg(Color::Cyan),
            ),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Instance Information")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(info, area);
}

fn render_sqs_instance_info(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    queue: &crate::models::SqsQueue,
) {
    debug!(
        "INSTANCE_DETAILS: Rendering SQS queue info for: {}",
        queue.name
    );

    // Get queue attributes for display
    let retention_period = queue
        .attributes
        .get("MessageRetentionPeriod")
        .and_then(|p| p.parse::<u64>().ok())
        .map(|p| format!("{}d", p / 86400))
        .unwrap_or_else(|| "Unknown".to_string());

    let visibility_timeout = queue
        .attributes
        .get("VisibilityTimeout")
        .unwrap_or(&"30".to_string())
        .clone();

    let max_receive_count = queue
        .attributes
        .get("ApproximateNumberOfMessages")
        .unwrap_or(&"0".to_string())
        .clone();

    let visibility_timeout_str = format!("{}s", visibility_timeout);

    debug!(
        "INSTANCE_DETAILS: SQS attributes - Retention: {}, Timeout: {}, Messages: {}",
        retention_period, visibility_timeout_str, max_receive_count
    );

    let info_text = vec![
        Line::from(vec![
            Span::styled("Type: ", Style::default().fg(Color::White)),
            Span::styled(&queue.queue_type, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Messages: ", Style::default().fg(Color::White)),
            Span::styled(&max_receive_count, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Retention: ", Style::default().fg(Color::White)),
            Span::styled(&retention_period, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("URL: ", Style::default().fg(Color::White)),
            Span::styled(&queue.url, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Visibility Timeout: ", Style::default().fg(Color::White)),
            Span::styled(&visibility_timeout_str, Style::default().fg(Color::Yellow)),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Queue Information")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(info, area);
}

pub fn render_metrics_loading(f: &mut Frame, area: ratatui::layout::Rect) {
    debug!("INSTANCE_DETAILS: Rendering loading state");
    let loading_msg = Paragraph::new("Loading metrics...")
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("CloudWatch Metrics")
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(loading_msg, area);
}
