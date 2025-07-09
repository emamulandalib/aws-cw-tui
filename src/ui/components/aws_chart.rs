use super::metric_definitions::{MetricDefinition, MetricRegistry};
use crate::models::{App, MetricType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};
use std::time::SystemTime;

/// AWS Console-style metric chart data
#[derive(Debug, Clone)]
pub struct MetricChartData {
    pub metric_type: MetricType,
    pub current_value: f64,
    pub history: Vec<f64>,
    pub timestamps: Vec<SystemTime>,
}

impl MetricChartData {
    /// Create chart data from app state
    pub fn from_app(app: &App, metric_type: MetricType) -> Option<Self> {
        let service = app.selected_service.as_ref()?;
        
        match service {
            crate::models::AwsService::Rds => {
                let metrics = &app.metrics;
                let current_value = get_rds_current_value(metrics, &metric_type)?;
                let history = get_rds_history(metrics, &metric_type)?;
                
                Some(MetricChartData {
                    metric_type,
                    current_value,
                    history: history.clone(),
                    timestamps: metrics.timestamps.clone(),
                })
            }
            crate::models::AwsService::Sqs => {
                let metrics = &app.sqs_metrics;
                let current_value = get_sqs_current_value(metrics, &metric_type)?;
                let history = get_sqs_history(metrics, &metric_type)?;
                
                Some(MetricChartData {
                    metric_type,
                    current_value,
                    history: history.clone(),
                    timestamps: metrics.timestamps.clone(),
                })
            }
        }
    }
}

/// AWS Console-style metrics grid renderer
pub struct AwsMetricsGrid;

impl AwsMetricsGrid {
    /// Render metrics in AWS console-style grid layout with scrolling support
    pub fn render(f: &mut Frame, app: &App, area: Rect) {
        let service = match app.selected_service.as_ref() {
            Some(service) => service,
            None => return,
        };

        // Get available metrics for the service using data-based filtering (only metrics with actual data)
        let available_metrics = match service {
            crate::models::AwsService::Rds => app.metrics.get_available_metrics_with_data(),
            crate::models::AwsService::Sqs => app.sqs_metrics.get_available_metrics(),
        };
        
        if available_metrics.is_empty() {
            Self::render_no_metrics(f, area);
            return;
        }

        // Create metric chart data for all available metrics
        let all_chart_data: Vec<MetricChartData> = available_metrics
            .into_iter()
            .filter_map(|metric_type| MetricChartData::from_app(app, metric_type))
            .collect();

        if all_chart_data.is_empty() {
            Self::render_loading_state(f, area);
            return;
        }

        // Force 2x2 grid layout (4 metrics per screen)
        let metrics_per_row = 2;
        let metrics_per_screen = 4; // Always show 4 metrics in 2x2 grid
        
        let total_metrics = all_chart_data.len();
        let selected_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
        
        // CRITICAL: Bounds check to prevent crashes - clamp selected_index to available data
        let safe_selected_index = selected_index.min(total_metrics.saturating_sub(1));
        
        // Calculate which metrics to show in the current 2x2 grid
        let current_page = safe_selected_index / metrics_per_screen;
        let start_index = current_page * metrics_per_screen;
        let end_index = (start_index + metrics_per_screen).min(total_metrics);
        
        // Get the metrics to display in current 2x2 grid
        let visible_metrics: Vec<MetricChartData> = all_chart_data[start_index..end_index].to_vec();
        
        // Always use 2x2 grid layout
        let grid_layout = calculate_scrollable_grid_layout(visible_metrics.len(), metrics_per_row);
        
        // Render the 2x2 grid of metrics
        Self::render_metrics_grid(f, area, &visible_metrics, grid_layout, app, safe_selected_index - start_index);
    }

    /// Render individual metric chart in AWS console style
    pub fn render_metric_chart(
        f: &mut Frame,
        area: Rect,
        chart_data: &MetricChartData,
        is_focused: bool,
    ) {
        let definition = MetricRegistry::get_definition(&chart_data.metric_type);
        
        // Get health color based on current value
        let health_color = definition.get_health_color(chart_data.current_value);
        let border_color = if is_focused { Color::Yellow } else { Color::White };

        // Calculate layout for title and chart
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title area
                Constraint::Min(8),    // Chart area
            ])
            .split(area);

        // Render title with current value
        Self::render_metric_title(f, chunks[0], &definition, chart_data, health_color, border_color);

        // Render chart if we have enough space and data
        if chunks[1].height >= 8 && !chart_data.history.is_empty() {
            Self::render_time_series_chart(f, chunks[1], chart_data, &definition, health_color, border_color);
        } else {
            Self::render_simple_metric(f, chunks[1], chart_data, &definition, health_color, border_color);
        }
    }

    /// Render metric title with current value
    fn render_metric_title(
        f: &mut Frame,
        area: Rect,
        definition: &MetricDefinition,
        chart_data: &MetricChartData,
        health_color: Color,
        border_color: Color,
    ) {
        let title_text = format!(
            "{}: {}",
            definition.name,
            definition.format_value(chart_data.current_value)
        );

        let title_widget = Paragraph::new(title_text)
            .style(Style::default().fg(health_color).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
            );

        f.render_widget(title_widget, area);
    }

    /// Render time series chart
    fn render_time_series_chart(
        f: &mut Frame,
        area: Rect,
        chart_data: &MetricChartData,
        definition: &MetricDefinition,
        health_color: Color,
        border_color: Color,
    ) {
        if chart_data.history.is_empty() || chart_data.timestamps.is_empty() {
            Self::render_no_data_chart(f, area, border_color);
            return;
        }

        // Convert timestamps to seconds for x-axis
        let start_time = chart_data.timestamps[0];
        let data_points: Vec<(f64, f64)> = chart_data.timestamps
            .iter()
            .zip(chart_data.history.iter())
            .map(|(timestamp, value)| {
                let seconds = timestamp.duration_since(start_time)
                    .unwrap_or_default()
                    .as_secs() as f64;
                (seconds, *value)
            })
            .collect();

        // Calculate bounds
        let x_bounds = [0.0, data_points.last().map(|p| p.0).unwrap_or(1.0)];
        let y_bounds = calculate_y_bounds(&chart_data.history);

        // Create labels
        let x_labels = create_time_labels(&chart_data.timestamps);
        let y_labels = create_value_labels(y_bounds, definition);

        // Create dataset
        let dataset = Dataset::default()
            .name("")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(health_color))
            .data(&data_points);

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
            )
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds(x_bounds)
                    .labels(x_labels)
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds(y_bounds)
                    .labels(y_labels)
            );

        f.render_widget(chart, area);
    }

    /// Render simple metric display when chart space is limited
    fn render_simple_metric(
        f: &mut Frame,
        area: Rect,
        chart_data: &MetricChartData,
        definition: &MetricDefinition,
        health_color: Color,
        border_color: Color,
    ) {
        let trend_indicator = if chart_data.history.len() > 1 {
            let first = chart_data.history[0];
            let last = chart_data.current_value;
            if last > first * 1.1 {
                " ↗"
            } else if last < first * 0.9 {
                " ↘"
            } else {
                " >"
            }
        } else {
            ""
        };

        let content = format!("{}{}", definition.description, trend_indicator);

        let simple_widget = Paragraph::new(content)
            .style(Style::default().fg(health_color))
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
            );

        f.render_widget(simple_widget, area);
    }

    /// Render loading state
    fn render_loading_state(f: &mut Frame, area: Rect) {
        let loading_widget = Paragraph::new("Loading metrics...")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Metrics")
                    .border_style(Style::default().fg(Color::White))
            );

        f.render_widget(loading_widget, area);
    }

    /// Render no metrics available state
    fn render_no_metrics(f: &mut Frame, area: Rect) {
        let no_metrics_widget = Paragraph::new("No metrics available for this service")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Metrics")
                    .border_style(Style::default().fg(Color::White))
            );

        f.render_widget(no_metrics_widget, area);
    }

    /// Render no data chart
    fn render_no_data_chart(f: &mut Frame, area: Rect, border_color: Color) {
        let no_data_widget = Paragraph::new("No data available")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
            );

        f.render_widget(no_data_widget, area);
    }

    /// Render metrics in grid layout
    fn render_metrics_grid(
        f: &mut Frame,
        area: Rect,
        chart_data: &[MetricChartData],
        grid_layout: GridLayout,
        app: &App,
        selected_metric_index: usize,
    ) {
        // Calculate the height for each row to fit within available area
        let min_row_height = 12u16;
        let available_height = area.height;
        let rows_to_render = grid_layout.rows;
        
        // Calculate actual row height - distribute available space evenly
        // but ensure minimum height if possible
        let row_height = if rows_to_render == 0 {
            available_height
        } else {
            let calculated_height = available_height / rows_to_render as u16;
            calculated_height.max(min_row_height).min(available_height / rows_to_render.max(1) as u16)
        };
        
        // Create row constraints that respect the container bounds
        let row_constraints: Vec<Constraint> = if rows_to_render * min_row_height as usize <= available_height as usize {
            // If we have enough space, use minimum height
            (0..rows_to_render)
                .map(|_| Constraint::Min(min_row_height))
                .collect()
        } else {
            // Otherwise, use percentage to fit within bounds
            (0..rows_to_render)
                .map(|_| Constraint::Percentage(100 / rows_to_render as u16))
                .collect()
        };

        let row_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area);

        // Render each row
        for (row_idx, row_area) in row_chunks.iter().enumerate() {
            // Create column constraints for this row (always 2 columns)
            let col_constraints: Vec<Constraint> = (0..grid_layout.cols)
                .map(|_| Constraint::Percentage(100 / grid_layout.cols as u16))
                .collect();

            let col_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints)
                .split(*row_area);

            // Render metrics in this row
            for (col_idx, col_area) in col_chunks.iter().enumerate() {
                let metric_idx = row_idx * grid_layout.cols + col_idx;
                if let Some(chart) = chart_data.get(metric_idx) {
                    // Check if this metric is the currently selected one in the 2x2 grid
                    let is_focused = metric_idx == selected_metric_index;
                    Self::render_metric_chart(f, *col_area, chart, is_focused);
                }
            }
        }
    }
}

/// Grid layout configuration
#[derive(Debug, Clone)]
struct GridLayout {
    rows: usize,
    cols: usize,
}

/// Calculate scrollable grid layout (always 2 columns for readability)
fn calculate_scrollable_grid_layout(visible_metric_count: usize, metrics_per_row: usize) -> GridLayout {
    if visible_metric_count == 0 {
        return GridLayout { rows: 1, cols: 1 };
    }

    let cols = metrics_per_row; // Always 2 columns for better readability
    let rows = (visible_metric_count + cols - 1) / cols; // Ceiling division

    GridLayout { rows, cols }
}

/// Calculate Y axis bounds for chart
fn calculate_y_bounds(history: &[f64]) -> [f64; 2] {
    if history.is_empty() {
        return [0.0, 1.0];
    }

    if history.len() == 1 {
        let val = history[0];
        let margin = if val.abs() > 1.0 { val.abs() * 0.1 } else { 1.0 };
        [val - margin, val + margin]
    } else {
        let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
        
        if min_val.is_finite() && max_val.is_finite() && min_val != max_val {
            let range = max_val - min_val;
            let padding = range * 0.1;
            let y_min = if min_val >= 0.0 { (min_val - padding).max(0.0) } else { min_val - padding };
            [y_min, max_val + padding]
        } else {
            [0.0, 1.0]
        }
    }
}

/// Create time labels for X axis
fn create_time_labels(timestamps: &[SystemTime]) -> Vec<Line<'_>> {
    use chrono::{DateTime, Utc};

    if timestamps.is_empty() {
        return vec![Line::from("No data")];
    }

    let num_labels = 4.min(timestamps.len());
    
    (0..num_labels)
        .map(|i| {
            let idx = if num_labels == 1 {
                0
            } else {
                (i * (timestamps.len() - 1)) / (num_labels - 1)
            };

            let timestamp = timestamps[idx];
            let dt: DateTime<Utc> = timestamp.into();
            let local_time: chrono::DateTime<chrono::Local> = dt.into();

            Line::from(Span::styled(
                format!("{}", local_time.format("%H:%M")),
                Style::default().fg(Color::DarkGray),
            ))
        })
        .collect()
}

/// Create value labels for Y axis
fn create_value_labels(y_bounds: [f64; 2], definition: &MetricDefinition) -> Vec<Line<'_>> {
    let num_labels = 5;
    let range = y_bounds[1] - y_bounds[0];
    
    (0..num_labels)
        .map(|i| {
            let ratio = i as f64 / (num_labels - 1) as f64;
            let value = y_bounds[0] + ratio * range;
            
            Line::from(Span::styled(
                definition.format_value(value),
                Style::default().fg(Color::DarkGray),
            ))
        })
        .collect()
}

// Helper functions to extract current values and history from different metric types
fn get_rds_current_value(metrics: &crate::models::MetricData, metric_type: &MetricType) -> Option<f64> {
    match metric_type {
        MetricType::CpuUtilization => Some(metrics.cpu_utilization),
        MetricType::DatabaseConnections => Some(metrics.database_connections),
        MetricType::FreeStorageSpace => Some(metrics.free_storage_space),
        MetricType::ReadLatency => Some(metrics.read_latency),
        MetricType::WriteLatency => Some(metrics.write_latency),
        MetricType::ReadIops => Some(metrics.read_iops),
        MetricType::WriteIops => Some(metrics.write_iops),
        MetricType::FreeableMemory => Some(metrics.freeable_memory),
        MetricType::ReadThroughput => Some(metrics.read_throughput),
        MetricType::WriteThroughput => Some(metrics.write_throughput),
        MetricType::NetworkReceiveThroughput => Some(metrics.network_receive_throughput),
        MetricType::NetworkTransmitThroughput => Some(metrics.network_transmit_throughput),
        MetricType::SwapUsage => Some(metrics.swap_usage),
        MetricType::BurstBalance => Some(metrics.burst_balance),
        MetricType::CpuCreditUsage => Some(metrics.cpu_credit_usage),
        MetricType::CpuCreditBalance => Some(metrics.cpu_credit_balance),
        MetricType::QueueDepth => Some(metrics.queue_depth),
        MetricType::BinLogDiskUsage => Some(metrics.bin_log_disk_usage),
        MetricType::ReplicaLag => Some(metrics.replica_lag),
        MetricType::TransactionLogsGeneration => Some(metrics.transaction_logs_generation),
        MetricType::TransactionLogsDiskUsage => Some(metrics.transaction_logs_disk_usage),
        MetricType::MaximumUsedTransactionIds => Some(metrics.maximum_used_transaction_ids),
        MetricType::OldestReplicationSlotLag => Some(metrics.oldest_replication_slot_lag),
        MetricType::ReplicationSlotDiskUsage => Some(metrics.replication_slot_disk_usage),
        MetricType::FailedSqlServerAgentJobsCount => Some(metrics.failed_sql_server_agent_jobs_count),
        MetricType::CheckpointLag => Some(metrics.checkpoint_lag),
        MetricType::ConnectionAttempts => Some(metrics.connection_attempts),
        _ => None,
    }
}

fn get_rds_history<'a>(metrics: &'a crate::models::MetricData, metric_type: &MetricType) -> Option<&'a Vec<f64>> {
    match metric_type {
        MetricType::CpuUtilization => Some(&metrics.cpu_history),
        MetricType::DatabaseConnections => Some(&metrics.connections_history),
        MetricType::FreeStorageSpace => Some(&metrics.free_storage_space_history),
        MetricType::ReadLatency => Some(&metrics.read_latency_history),
        MetricType::WriteLatency => Some(&metrics.write_latency_history),
        MetricType::ReadIops => Some(&metrics.read_iops_history),
        MetricType::WriteIops => Some(&metrics.write_iops_history),
        MetricType::FreeableMemory => Some(&metrics.freeable_memory_history),
        MetricType::ReadThroughput => Some(&metrics.read_throughput_history),
        MetricType::WriteThroughput => Some(&metrics.write_throughput_history),
        MetricType::NetworkReceiveThroughput => Some(&metrics.network_receive_history),
        MetricType::NetworkTransmitThroughput => Some(&metrics.network_transmit_history),
        MetricType::SwapUsage => Some(&metrics.swap_usage_history),
        MetricType::BurstBalance => Some(&metrics.burst_balance_history),
        MetricType::CpuCreditUsage => Some(&metrics.cpu_credit_usage_history),
        MetricType::CpuCreditBalance => Some(&metrics.cpu_credit_balance_history),
        MetricType::QueueDepth => Some(&metrics.queue_depth_history),
        MetricType::BinLogDiskUsage => Some(&metrics.bin_log_disk_usage_history),
        MetricType::ReplicaLag => Some(&metrics.replica_lag_history),
        MetricType::TransactionLogsGeneration => Some(&metrics.transaction_logs_generation_history),
        MetricType::TransactionLogsDiskUsage => Some(&metrics.transaction_logs_disk_usage_history),
        MetricType::MaximumUsedTransactionIds => Some(&metrics.maximum_used_transaction_ids_history),
        MetricType::OldestReplicationSlotLag => Some(&metrics.oldest_replication_slot_lag_history),
        MetricType::ReplicationSlotDiskUsage => Some(&metrics.replication_slot_disk_usage_history),
        MetricType::FailedSqlServerAgentJobsCount => Some(&metrics.failed_sql_server_agent_jobs_count_history),
        MetricType::CheckpointLag => Some(&metrics.checkpoint_lag_history),
        MetricType::ConnectionAttempts => Some(&metrics.connection_attempts_history),
        _ => None,
    }
}

fn get_sqs_current_value(metrics: &crate::models::SqsMetricData, metric_type: &MetricType) -> Option<f64> {
    match metric_type {
        MetricType::ApproximateNumberOfMessages => Some(metrics.approximate_number_of_messages),
        MetricType::ApproximateNumberOfMessagesVisible => Some(metrics.approximate_number_of_messages_visible),
        MetricType::ApproximateNumberOfMessagesNotVisible => Some(metrics.approximate_number_of_messages_not_visible),
        MetricType::ApproximateAgeOfOldestMessage => Some(metrics.approximate_age_of_oldest_message),
        MetricType::ApproximateNumberOfMessagesDelayed => Some(metrics.approximate_number_of_messages_delayed),
        MetricType::NumberOfMessagesSent => Some(metrics.number_of_messages_sent),
        MetricType::NumberOfMessagesReceived => Some(metrics.number_of_messages_received),
        MetricType::NumberOfMessagesDeleted => Some(metrics.number_of_messages_deleted),
        MetricType::NumberOfMessagesInDlq => Some(metrics.number_of_messages_in_dlq),
        MetricType::NumberOfEmptyReceives => Some(metrics.number_of_empty_receives),
        MetricType::SentMessageSize => Some(metrics.sent_message_size),
        MetricType::ApproximateNumberOfGroupsWithInflightMessages => Some(metrics.approximate_number_of_groups_with_inflight_messages),
        MetricType::NumberOfDeduplicatedSentMessages => Some(metrics.number_of_deduplicated_sent_messages),
        _ => None,
    }
}

fn get_sqs_history<'a>(metrics: &'a crate::models::SqsMetricData, metric_type: &MetricType) -> Option<&'a Vec<f64>> {
    match metric_type {
        MetricType::ApproximateNumberOfMessages => Some(&metrics.queue_depth_history),
        MetricType::ApproximateNumberOfMessagesVisible => Some(&metrics.messages_visible_history),
        MetricType::ApproximateNumberOfMessagesNotVisible => Some(&metrics.messages_not_visible_history),
        MetricType::ApproximateAgeOfOldestMessage => Some(&metrics.oldest_message_age_history),
        MetricType::ApproximateNumberOfMessagesDelayed => Some(&metrics.messages_delayed_history),
        MetricType::NumberOfMessagesSent => Some(&metrics.messages_sent_history),
        MetricType::NumberOfMessagesReceived => Some(&metrics.messages_received_history),
        MetricType::NumberOfMessagesDeleted => Some(&metrics.messages_deleted_history),
        MetricType::NumberOfMessagesInDlq => Some(&metrics.dlq_messages_history),
        MetricType::NumberOfEmptyReceives => Some(&metrics.empty_receives_history),
        MetricType::SentMessageSize => Some(&metrics.sent_message_size_history),
        MetricType::ApproximateNumberOfGroupsWithInflightMessages => Some(&metrics.groups_with_inflight_messages_history),
        MetricType::NumberOfDeduplicatedSentMessages => Some(&metrics.deduplicated_sent_messages_history),
        _ => None,
    }
} 