// Type alias to simplify complex types
type MetricTuple<'a> = (&'a str, String, &'a Vec<f64>, Color, f64, bool);

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};
use std::time::SystemTime;

// Legacy render_metrics function removed - use render_metrics_unified instead

pub fn render_metrics_unified(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &crate::models::App,
    _selected_metric_index: usize, // Legacy parameter, now unused - using ListState instead
    metrics_per_screen: usize,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main metrics area
            Constraint::Length(3), // Instructions
        ])
        .split(area);

    let individual_metrics = collect_available_metrics_unified(app);
    let available_count = individual_metrics.len();

    // Get timestamps based on service type
    let timestamps = match app
        .selected_service
        .as_ref()
        .unwrap_or(&crate::models::AwsService::Rds)
    {
        crate::models::AwsService::Rds => &app.metrics.timestamps,
        crate::models::AwsService::Sqs => &app.sqs_metrics.timestamps,
    };

    // Use ListState-based selection instead of manual scroll_offset
    let selected_index = app.sparkline_grid_list_state.selected().unwrap_or(0);

    render_scrollable_individual_metrics(
        f,
        main_chunks[0],
        timestamps,
        &individual_metrics,
        selected_index,
        metrics_per_screen,
    );

    render_instructions(f, main_chunks[1], available_count, selected_index);
}

fn collect_available_metrics_unified(app: &crate::models::App) -> Vec<MetricTuple<'_>> {
    let mut individual_metrics = vec![];

    // NEW: Check if dynamic metrics are available, otherwise fall back to legacy
    if let Some(ref dynamic_metrics) = app.dynamic_metrics {
        // Use dynamic metrics system
        for metric_data in &dynamic_metrics.metrics {
            if !metric_data.history.is_empty() {
                let display_name = metric_data.display_name.as_str(); // Use AWS SDK metric name directly
                let formatted_value = format_dynamic_metric_value(metric_data);
                let color = get_dynamic_metric_color(&metric_data.metric_name);
                let max_val = calculate_dynamic_metric_max(metric_data);
                
                individual_metrics.push((
                    display_name,
                    formatted_value,
                    &metric_data.history,
                    color,
                    max_val,
                    true
                ));
            }
        }
    } else {
        // LEGACY: Fall back to hardcoded metrics (empty for now since we're transitioning)
        // This branch will be removed once dynamic metrics are fully working
    }

    individual_metrics
}

fn get_rds_metric_display_info<'a>(
    metric_type: &crate::models::MetricType,
    metrics: &'a crate::models::MetricData,
) -> (
    &'static str,
    String,
    &'a Vec<f64>,
    ratatui::style::Color,
    f64,
) {
    match metric_type {
        crate::models::MetricType::CpuUtilization => (
            "CPU Utilization",
            format!("{:.1}%", metrics.cpu_utilization),
            &metrics.cpu_history,
            ratatui::style::Color::Red,
            100.0,
        ),
        crate::models::MetricType::DatabaseConnections => (
            "DB Connections",
            format!("{:.0}", metrics.database_connections),
            &metrics.connections_history,
            ratatui::style::Color::Blue,
            200.0,
        ),
        crate::models::MetricType::FreeStorageSpace => (
            "Free Storage",
            format!(
                "{:.1} GB",
                metrics.free_storage_space / 1024.0 / 1024.0 / 1024.0
            ),
            &metrics.free_storage_space_history,
            ratatui::style::Color::White,
            1000.0,
        ),
        crate::models::MetricType::ReadIops => (
            "Read IOPS",
            format!("{:.0}", metrics.read_iops),
            &metrics.read_iops_history,
            ratatui::style::Color::Green,
            1000.0,
        ),
        crate::models::MetricType::WriteIops => (
            "Write IOPS",
            format!("{:.0}", metrics.write_iops),
            &metrics.write_iops_history,
            ratatui::style::Color::Yellow,
            1000.0,
        ),
        crate::models::MetricType::ReadLatency => (
            "Read Latency",
            format!("{:.2} ms", metrics.read_latency * 1000.0),
            &metrics.read_latency_history,
            ratatui::style::Color::Red,
            0.1,
        ),
        crate::models::MetricType::WriteLatency => (
            "Write Latency",
            format!("{:.2} ms", metrics.write_latency * 1000.0),
            &metrics.write_latency_history,
            ratatui::style::Color::Red,
            0.1,
        ),
        crate::models::MetricType::ReadThroughput => (
            "Read Throughput",
            format!("{:.1} MB/s", metrics.read_throughput / 1024.0 / 1024.0),
            &metrics.read_throughput_history,
            ratatui::style::Color::Green,
            100_000_000.0,
        ),
        crate::models::MetricType::WriteThroughput => (
            "Write Throughput",
            format!("{:.1} MB/s", metrics.write_throughput / 1024.0 / 1024.0),
            &metrics.write_throughput_history,
            ratatui::style::Color::Yellow,
            100_000_000.0,
        ),
        crate::models::MetricType::NetworkReceiveThroughput => (
            "Network Receive",
            format!(
                "{:.1} MB/s",
                metrics.network_receive_throughput / 1024.0 / 1024.0
            ),
            &metrics.network_receive_history,
            ratatui::style::Color::Cyan,
            100_000_000.0,
        ),
        crate::models::MetricType::NetworkTransmitThroughput => (
            "Network Transmit",
            format!(
                "{:.1} MB/s",
                metrics.network_transmit_throughput / 1024.0 / 1024.0
            ),
            &metrics.network_transmit_history,
            ratatui::style::Color::Cyan,
            100_000_000.0,
        ),
        crate::models::MetricType::SwapUsage => (
            "Swap Usage",
            format!("{:.1} MB", metrics.swap_usage / 1024.0 / 1024.0),
            &metrics.swap_usage_history,
            ratatui::style::Color::DarkGray,
            1_000_000_000.0,
        ),
        crate::models::MetricType::FreeableMemory => (
            "Freeable Memory",
            format!("{:.1} MB", metrics.freeable_memory / 1024.0 / 1024.0),
            &metrics.freeable_memory_history,
            ratatui::style::Color::White,
            1_000_000_000.0,
        ),
        crate::models::MetricType::QueueDepth => (
            "Queue Depth",
            format!("{:.1}", metrics.queue_depth),
            &metrics.queue_depth_history,
            ratatui::style::Color::Red,
            100.0,
        ),
        crate::models::MetricType::BurstBalance => (
            "Burst Balance",
            format!("{:.1}%", metrics.burst_balance),
            &metrics.burst_balance_history,
            ratatui::style::Color::LightGreen,
            100.0,
        ),
        crate::models::MetricType::CpuCreditUsage => (
            "CPU Credit Usage",
            format!("{:.1}", metrics.cpu_credit_usage),
            &metrics.cpu_credit_usage_history,
            ratatui::style::Color::Blue,
            100.0,
        ),
        crate::models::MetricType::CpuCreditBalance => (
            "CPU Credit Balance",
            format!("{:.1}", metrics.cpu_credit_balance),
            &metrics.cpu_credit_balance_history,
            ratatui::style::Color::LightBlue,
            1000.0,
        ),
        // Missing CPU surplus credit metrics
        crate::models::MetricType::CpuSurplusCreditBalance => (
            "CPU Surplus Credit Balance",
            format!("{:.1}", metrics.cpu_surplus_credit_balance),
            &metrics.cpu_surplus_credit_balance_history,
            ratatui::style::Color::LightBlue,
            1000.0,
        ),
        crate::models::MetricType::CpuSurplusCreditsCharged => (
            "CPU Surplus Credits Charged",
            format!("{:.1}", metrics.cpu_surplus_credits_charged),
            &metrics.cpu_surplus_credits_charged_history,
            ratatui::style::Color::LightRed,
            1000.0,
        ),
        // Missing EBS performance metrics
        crate::models::MetricType::EbsByteBalance => (
            "EBS Byte Balance",
            format!("{:.1}%", metrics.ebs_byte_balance),
            &metrics.ebs_byte_balance_history,
            ratatui::style::Color::LightGreen,
            100.0,
        ),
        crate::models::MetricType::EbsIoBalance => (
            "EBS IO Balance",
            format!("{:.1}%", metrics.ebs_io_balance),
            &metrics.ebs_io_balance_history,
            ratatui::style::Color::LightGreen,
            100.0,
        ),
        crate::models::MetricType::BinLogDiskUsage => (
            "Binary Log Usage",
            format!("{:.1} MB", metrics.bin_log_disk_usage / 1024.0 / 1024.0),
            &metrics.bin_log_disk_usage_history,
            ratatui::style::Color::Magenta,
            1_000_000_000.0,
        ),
        crate::models::MetricType::ReplicaLag => (
            "Replica Lag",
            format!("{:.2} s", metrics.replica_lag),
            &metrics.replica_lag_history,
            ratatui::style::Color::Red,
            60.0,
        ),
        crate::models::MetricType::MaximumUsedTransactionIds => (
            "Max Used Transaction IDs",
            format!("{:.0}", metrics.maximum_used_transaction_ids),
            &metrics.maximum_used_transaction_ids_history,
            ratatui::style::Color::Red,
            2_000_000_000.0,
        ),
        crate::models::MetricType::OldestReplicationSlotLag => (
            "Oldest Replication Slot Lag",
            format!(
                "{:.1} MB",
                metrics.oldest_replication_slot_lag / 1024.0 / 1024.0
            ),
            &metrics.oldest_replication_slot_lag_history,
            ratatui::style::Color::Red,
            1_000_000_000.0,
        ),
        // Missing logical replication slot lag metric
        crate::models::MetricType::OldestLogicalReplicationSlotLag => (
            "Oldest Logical Replication Slot Lag",
            format!(
                "{:.1} MB",
                metrics.oldest_logical_replication_slot_lag / 1024.0 / 1024.0
            ),
            &metrics.oldest_logical_replication_slot_lag_history,
            ratatui::style::Color::Red,
            1_000_000_000.0,
        ),
        crate::models::MetricType::ReplicationSlotDiskUsage => (
            "Replication Slot Usage",
            format!(
                "{:.1} MB",
                metrics.replication_slot_disk_usage / 1024.0 / 1024.0
            ),
            &metrics.replication_slot_disk_usage_history,
            ratatui::style::Color::Blue,
            1_000_000_000.0,
        ),
        crate::models::MetricType::TransactionLogsDiskUsage => (
            "Transaction Logs Usage",
            format!(
                "{:.1} MB",
                metrics.transaction_logs_disk_usage / 1024.0 / 1024.0
            ),
            &metrics.transaction_logs_disk_usage_history,
            ratatui::style::Color::Green,
            100_000_000.0,
        ),
        crate::models::MetricType::TransactionLogsGeneration => (
            "Transaction Log Gen",
            format!(
                "{:.1} MB/s",
                metrics.transaction_logs_generation / 1024.0 / 1024.0
            ),
            &metrics.transaction_logs_generation_history,
            ratatui::style::Color::LightGreen,
            10_000_000.0,
        ),
        crate::models::MetricType::FailedSqlServerAgentJobsCount => (
            "Failed SQL Agent Jobs",
            format!("{:.0}", metrics.failed_sql_server_agent_jobs_count),
            &metrics.failed_sql_server_agent_jobs_count_history,
            ratatui::style::Color::Red,
            10.0,
        ),
        crate::models::MetricType::CheckpointLag => (
            "Checkpoint Lag",
            format!("{:.1} s", metrics.checkpoint_lag),
            &metrics.checkpoint_lag_history,
            ratatui::style::Color::Blue,
            60.0,
        ),
        crate::models::MetricType::ConnectionAttempts => (
            "Connection Attempts",
            format!("{:.0}", metrics.connection_attempts),
            &metrics.connection_attempts_history,
            ratatui::style::Color::LightBlue,
            1000.0,
        ),
        // SQS metrics should not appear for RDS, but if they do, handle gracefully
        crate::models::MetricType::NumberOfMessagesSent
        | crate::models::MetricType::NumberOfMessagesReceived
        | crate::models::MetricType::NumberOfMessagesDeleted
        | crate::models::MetricType::ApproximateNumberOfMessages
        | crate::models::MetricType::ApproximateNumberOfMessagesVisible
        | crate::models::MetricType::ApproximateNumberOfMessagesNotVisible
        | crate::models::MetricType::ApproximateAgeOfOldestMessage
        | crate::models::MetricType::NumberOfEmptyReceives
        | crate::models::MetricType::ApproximateNumberOfMessagesDelayed
        | crate::models::MetricType::SentMessageSize
        | crate::models::MetricType::NumberOfMessagesInDlq
        | crate::models::MetricType::ApproximateNumberOfGroupsWithInflightMessages
        | crate::models::MetricType::NumberOfDeduplicatedSentMessages => {
            // These are SQS metrics that shouldn't appear for RDS
            static EMPTY_VEC: Vec<f64> = Vec::new();
            (
                "SQS Metric (Invalid)",
                "0".to_string(),
                &EMPTY_VEC,
                ratatui::style::Color::Gray,
                0.0,
            )
        }

    }
}

fn get_sqs_metric_display_info<'a>(
    metric_type: &crate::models::MetricType,
    metrics: &'a crate::models::SqsMetricData,
) -> (
    &'static str,
    String,
    &'a Vec<f64>,
    ratatui::style::Color,
    f64,
) {
    match metric_type {
        crate::models::MetricType::NumberOfMessagesSent => (
            "Messages Sent",
            format!("{:.0}", metrics.number_of_messages_sent),
            &metrics.messages_sent_history,
            ratatui::style::Color::Green,
            1000.0,
        ),
        crate::models::MetricType::NumberOfMessagesReceived => (
            "Messages Received",
            format!("{:.0}", metrics.number_of_messages_received),
            &metrics.messages_received_history,
            ratatui::style::Color::Blue,
            1000.0,
        ),
        crate::models::MetricType::NumberOfMessagesDeleted => (
            "Messages Deleted",
            format!("{:.0}", metrics.number_of_messages_deleted),
            &metrics.messages_deleted_history,
            ratatui::style::Color::Yellow,
            1000.0,
        ),
        crate::models::MetricType::ApproximateNumberOfMessages => (
            "Total Queue Depth",
            format!("{:.0}", metrics.approximate_number_of_messages),
            &metrics.queue_depth_history,
            ratatui::style::Color::Cyan,
            1000.0,
        ),
        crate::models::MetricType::ApproximateNumberOfMessagesVisible => (
            "Messages Visible",
            format!("{:.0}", metrics.approximate_number_of_messages_visible),
            &metrics.messages_visible_history,
            ratatui::style::Color::LightCyan,
            1000.0,
        ),
        crate::models::MetricType::ApproximateNumberOfMessagesNotVisible => (
            "Messages Not Visible",
            format!("{:.0}", metrics.approximate_number_of_messages_not_visible),
            &metrics.messages_not_visible_history,
            ratatui::style::Color::Magenta,
            1000.0,
        ),
        crate::models::MetricType::ApproximateAgeOfOldestMessage => (
            "Oldest Message Age",
            format!("{:.0}s", metrics.approximate_age_of_oldest_message),
            &metrics.oldest_message_age_history,
            ratatui::style::Color::Red,
            3600.0,
        ),
        crate::models::MetricType::NumberOfEmptyReceives => (
            "Empty Receives",
            format!("{:.0}", metrics.number_of_empty_receives),
            &metrics.empty_receives_history,
            ratatui::style::Color::Gray,
            1000.0,
        ),
        crate::models::MetricType::ApproximateNumberOfMessagesDelayed => (
            "Messages Delayed",
            format!("{:.0}", metrics.approximate_number_of_messages_delayed),
            &metrics.messages_delayed_history,
            ratatui::style::Color::LightRed,
            1000.0,
        ),
        crate::models::MetricType::SentMessageSize => (
            "Message Size",
            format!("{:.0} B", metrics.sent_message_size),
            &metrics.sent_message_size_history,
            ratatui::style::Color::LightGreen,
            262144.0, // 256 KB = 262144 bytes max
        ),
        crate::models::MetricType::NumberOfMessagesInDlq => (
            "DLQ Messages",
            format!("{:.0}", metrics.number_of_messages_in_dlq),
            &metrics.dlq_messages_history,
            ratatui::style::Color::DarkGray,
            1000.0,
        ),
        crate::models::MetricType::ApproximateNumberOfGroupsWithInflightMessages => (
            "Groups with In-flight Messages",
            format!(
                "{:.0}",
                metrics.approximate_number_of_groups_with_inflight_messages
            ),
            &metrics.groups_with_inflight_messages_history,
            ratatui::style::Color::LightYellow,
            100.0,
        ),
        crate::models::MetricType::NumberOfDeduplicatedSentMessages => (
            "Deduplicated Messages",
            format!("{:.0}", metrics.number_of_deduplicated_sent_messages),
            &metrics.deduplicated_sent_messages_history,
            ratatui::style::Color::LightBlue,
            1000.0,
        ),
        // RDS metrics should not appear for SQS
        _ => {
            static EMPTY_VEC: Vec<f64> = Vec::new();
            (
                "Unknown",
                "0".to_string(),
                &EMPTY_VEC,
                ratatui::style::Color::Gray,
                0.0,
            )
        }
    }
}

fn render_scrollable_individual_metrics(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    timestamps: &[SystemTime],
    individual_metrics: &[MetricTuple],
    selected_index: usize, // Now using ListState-based selection index
    _metrics_per_screen: usize,
) {
    if individual_metrics.is_empty() {
        let no_data = Paragraph::new("No metrics to display")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(no_data, area);
        return;
    }

    // For enhanced user experience: show only the selected metric in full detail
    // This leverages ListState's built-in scrolling without manual viewport calculations
    let safe_selected_index = selected_index.min(individual_metrics.len().saturating_sub(1));

    if let Some(selected_metric) = individual_metrics.get(safe_selected_index) {
        let (name, value, history, color, max_val, available) = selected_metric;

        // Render the selected metric in full detail using the entire area
        render_large_metric_chart(
            f,
            area,
            timestamps,
            (name, value.clone(), history, *color, *max_val, *available),
        );
    } else {
        let no_data = Paragraph::new("No metric selected")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(no_data, area);
    }
}

fn render_instructions(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    available_count: usize,
    selected_index: usize, // Now using ListState-based selection index
) {
    let instructions = Paragraph::new(format!(
        "Up/Down scroll ({} metrics with data, showing {}/{}) • r refresh • b back • q quit",
        available_count,
        selected_index + 1,
        available_count
    ))
    .style(Style::default().fg(Color::Gray))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Controls")
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(instructions, area);
}

fn render_large_metric_chart(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    timestamps: &[SystemTime],
    metric: MetricTuple,
) {
    let (name, value, history, color, _max_val, available) = metric;

    if area.width < 20 || area.height < 6 {
        let simple_widget = Paragraph::new(format!("{name}: {value}"))
            .style(Style::default().fg(color))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            );
        f.render_widget(simple_widget, area);
        return;
    }

    let widget_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(12)])
        .split(area);

    let title_style = if available {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM)
    };

    let title_widget = Paragraph::new(format!("{name}: {value}"))
        .style(title_style)
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(title_widget, widget_chunks[0]);

    if available && !history.is_empty() && widget_chunks[1].height >= 5 {
        render_high_resolution_chart(f, widget_chunks[1], timestamps, history, color, name);
    } else {
        let status_msg = if !available {
            "Metric not available for this DB engine/instance type".to_string()
        } else if history.is_empty() {
            "Loading data...".to_string()
        } else {
            "Area too small for chart".to_string()
        };

        let status_color = if !available {
            Color::DarkGray
        } else {
            Color::Gray
        };
        let status_widget = Paragraph::new(status_msg)
            .style(Style::default().fg(status_color))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("3hr")
                    .border_style(Style::default().fg(Color::White)),
            );
        f.render_widget(status_widget, widget_chunks[1]);
    }
}

fn render_high_resolution_chart(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    timestamps: &[SystemTime],
    history: &[f64],
    color: Color,
    metric_name: &str,
) {
    use chrono::{DateTime, Utc};

    if history.is_empty() || timestamps.is_empty() {
        let no_data_chart = Chart::new(vec![])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            )
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 180.0]),
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 1.0]),
            );
        f.render_widget(no_data_chart, area);
        return;
    }

    if timestamps.len() != history.len() {
        return;
    }

    let start_time: DateTime<Utc> = timestamps[0].into();
    let start_epoch = start_time.timestamp() as f64;

    let data_points: Vec<(f64, f64)> = timestamps
        .iter()
        .zip(history.iter())
        .map(|(timestamp, &value)| {
            let dt: DateTime<Utc> = (*timestamp).into();
            let epoch_seconds = dt.timestamp() as f64;
            (epoch_seconds, value)
        })
        .collect();

    let end_epoch = data_points
        .last()
        .map(|(x, _)| *x)
        .unwrap_or(start_epoch + 3600.0 * 3.0);
    let time_bounds = [start_epoch, end_epoch];

    let (y_min, y_max) = calculate_y_bounds(history);
    let y_bounds = if y_max <= y_min {
        [y_min, y_min + 1.0]
    } else {
        [y_min, y_max]
    };

    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&data_points);

    let x_labels = create_x_labels(timestamps);
    let y_labels = create_y_labels(y_bounds, metric_name);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(time_bounds)
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(y_bounds)
                .labels(y_labels),
        );

    f.render_widget(chart, area);
}

fn calculate_y_bounds(history: &[f64]) -> (f64, f64) {
    if history.len() == 1 {
        let val = history[0];
        let margin = if val.abs() > 1.0 {
            val.abs() * 0.1
        } else {
            1.0
        };
        (val - margin, val + margin)
    } else {
        let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
        if min_val.is_finite() && max_val.is_finite() && min_val != max_val {
            let range = max_val - min_val;
            let padding = range * 0.1;
            let y_min = if min_val >= 0.0 {
                (min_val - padding).max(0.0)
            } else {
                min_val - padding
            };
            (y_min, max_val + padding)
        } else {
            (0.0, 1.0)
        }
    }
}

fn create_x_labels(timestamps: &[SystemTime]) -> Vec<Line<'_>> {
    use chrono::{DateTime, Utc};

    let num_x_labels = 8.min(timestamps.len());

    if timestamps.len() <= 1 {
        vec![Line::from(Span::styled(
            {
                let dt: DateTime<Utc> = timestamps[0].into();
                let local_time: chrono::DateTime<chrono::Local> = dt.into();
                format!("{}", local_time.format("%H:%M"))
            },
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        (0..num_x_labels)
            .map(|i| {
                let idx = if num_x_labels == 1 {
                    0
                } else {
                    (i * (timestamps.len() - 1)) / (num_x_labels - 1)
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
}

fn create_y_labels(y_bounds: [f64; 2], metric_name: &str) -> Vec<Line<'_>> {
    let format_value = |v: f64| -> String {
        if metric_name.contains("Memory") || metric_name.contains("Storage") {
            let gb_value = v / (1024.0 * 1024.0 * 1024.0);
            if gb_value >= 1.0 {
                format!("{gb_value:.1}G")
            } else {
                let mb_value = v / (1024.0 * 1024.0);
                format!("{mb_value:.0}M")
            }
        } else if metric_name.contains("Throughput") || metric_name.contains("Network") {
            let mb_value = v / (1024.0 * 1024.0);
            if mb_value >= 1.0 {
                format!("{mb_value:.1}M")
            } else {
                let kb_value = v / 1024.0;
                format!("{kb_value:.0}K")
            }
        } else if v.abs() >= 1000000.0 {
            format!("{:.1}M", v / 1000000.0)
        } else if v.abs() >= 1000.0 {
            format!("{:.1}K", v / 1000.0)
        } else if v.abs() >= 1.0 {
            format!("{v:.1}")
        } else {
            format!("{v:.2}")
        }
    };

    let y_range = y_bounds[1] - y_bounds[0];
    let num_y_labels = if y_range <= 1.0 {
        12
    } else if y_range <= 10.0 {
        15
    } else {
        12
    };

    (0..num_y_labels)
        .map(|i| {
            let ratio = i as f64 / (num_y_labels - 1) as f64;
            let value = y_bounds[0] + ratio * y_range;
            Line::from(Span::styled(
                format_value(value),
                Style::default().fg(Color::DarkGray),
            ))
        })
        .collect()
}

// ================================
// NEW: Dynamic Metrics Helper Functions
// ================================

/// Get a user-friendly display name for a dynamic metric
fn get_dynamic_metric_display_name(metric_name: &str) -> &'static str {
    match metric_name {
        "CPUUtilization" => "CPU Utilization",
        "DatabaseConnections" => "DB Connections", 
        "FreeStorageSpace" => "Free Storage",
        "ReadIOPS" => "Read IOPS",
        "WriteIOPS" => "Write IOPS",
        "ReadLatency" => "Read Latency",
        "WriteLatency" => "Write Latency", 
        "ReadThroughput" => "Read Throughput",
        "WriteThroughput" => "Write Throughput",
        "NetworkReceiveThroughput" => "Network Receive",
        "NetworkTransmitThroughput" => "Network Transmit",
        "SwapUsage" => "Swap Usage",
        "FreeableMemory" => "Freeable Memory",
        "DiskQueueDepth" => "Queue Depth",
        "BurstBalance" => "Burst Balance",
        "CPUCreditUsage" => "CPU Credit Usage",
        "CPUCreditBalance" => "CPU Credit Balance",
        "BinLogDiskUsage" => "Binary Log Usage",
        "ReplicaLag" => "Replica Lag",
        "NumberOfMessagesSent" => "Messages Sent",
        "NumberOfMessagesReceived" => "Messages Received",
        "NumberOfMessagesDeleted" => "Messages Deleted",
        "ApproximateNumberOfMessages" => "Queue Depth",
        "ApproximateNumberOfMessagesVisible" => "Visible Messages",
        "ApproximateNumberOfMessagesNotVisible" => "Hidden Messages",
        "ApproximateAgeOfOldestMessage" => "Oldest Message Age",
        "NumberOfEmptyReceives" => "Empty Receives",
        _ => "Unknown Metric", // Fallback to static string
    }
}

/// Format the current value of a dynamic metric for display
fn format_dynamic_metric_value(metric_data: &crate::aws::dynamic_metric_discovery::DynamicMetricData) -> String {
    match metric_data.metric_name.as_str() {
        "CPUUtilization" | "BurstBalance" => format!("{:.1}%", metric_data.current_value),
        "FreeStorageSpace" | "FreeableMemory" | "SwapUsage" | "BinLogDiskUsage" => {
            format!("{:.1} GB", metric_data.current_value / 1024.0 / 1024.0 / 1024.0)
        }
        "ReadThroughput" | "WriteThroughput" | "NetworkReceiveThroughput" | "NetworkTransmitThroughput" => {
            format!("{:.1} MB/s", metric_data.current_value / 1024.0 / 1024.0)
        }
        "ReadLatency" | "WriteLatency" => format!("{:.2} ms", metric_data.current_value * 1000.0),
        "ReplicaLag" | "ApproximateAgeOfOldestMessage" => format!("{:.2} s", metric_data.current_value),
        _ => format!("{:.1}", metric_data.current_value), // Default number formatting
    }
}

/// Get the color for a dynamic metric based on its type
fn get_dynamic_metric_color(metric_name: &str) -> ratatui::style::Color {
    match metric_name {
        "CPUUtilization" | "ReadLatency" | "WriteLatency" => ratatui::style::Color::Red,
        "DatabaseConnections" | "CPUCreditUsage" => ratatui::style::Color::Blue,
        "ReadIOPS" | "ReadThroughput" | "BurstBalance" => ratatui::style::Color::Green,
        "WriteIOPS" | "WriteThroughput" => ratatui::style::Color::Yellow,
        "NetworkReceiveThroughput" | "NetworkTransmitThroughput" => ratatui::style::Color::Cyan,
        "FreeStorageSpace" | "FreeableMemory" => ratatui::style::Color::White,
        "SwapUsage" => ratatui::style::Color::DarkGray,
        "CPUCreditBalance" => ratatui::style::Color::LightBlue,
        "BinLogDiskUsage" => ratatui::style::Color::Magenta,
        "ReplicaLag" => ratatui::style::Color::Yellow,
        "NumberOfMessagesSent" => ratatui::style::Color::Green,
        "NumberOfMessagesReceived" => ratatui::style::Color::Blue,
        "NumberOfMessagesDeleted" => ratatui::style::Color::Red,
        "ApproximateNumberOfMessages" => ratatui::style::Color::Yellow,
        "ApproximateNumberOfMessagesVisible" => ratatui::style::Color::LightGreen,
        "ApproximateNumberOfMessagesNotVisible" => ratatui::style::Color::Gray,
        _ => ratatui::style::Color::White, // Default color
    }
}

/// Calculate maximum value for chart scaling
fn calculate_dynamic_metric_max(metric_data: &crate::aws::dynamic_metric_discovery::DynamicMetricData) -> f64 {
    match metric_data.metric_name.as_str() {
        "CPUUtilization" | "BurstBalance" => 100.0,
        "DatabaseConnections" => 200.0,
        "ReadIOPS" | "WriteIOPS" => 1000.0,
        "ReadLatency" | "WriteLatency" => 0.1,
        "FreeStorageSpace" | "FreeableMemory" | "SwapUsage" | "BinLogDiskUsage" => 1_000_000_000.0,
        "ReadThroughput" | "WriteThroughput" | "NetworkReceiveThroughput" | "NetworkTransmitThroughput" => 100_000_000.0,
        "DiskQueueDepth" => 100.0,
        "CPUCreditUsage" => 100.0,
        "CPUCreditBalance" => 1000.0,
        "ReplicaLag" => 3600.0, // 1 hour max
        "NumberOfMessagesSent" | "NumberOfMessagesReceived" | "NumberOfMessagesDeleted" => 1000.0,
        "ApproximateNumberOfMessages" | "ApproximateNumberOfMessagesVisible" | "ApproximateNumberOfMessagesNotVisible" => 10000.0,
        "ApproximateAgeOfOldestMessage" => 86400.0, // 1 day max
        "NumberOfEmptyReceives" => 100.0,
        _ => {
            // Dynamic calculation based on history
            if !metric_data.history.is_empty() {
                let max_in_history = metric_data.history.iter().fold(0.0_f64, |a, &b| a.max(b));
                (max_in_history * 1.2_f64).max(1.0_f64) // 20% buffer, minimum 1.0
            } else {
                100.0 // Default fallback
            }
        }
    }
}
