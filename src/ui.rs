use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, 
        Paragraph, Chart, Axis, Dataset, GraphType
    },
    symbols,
    Frame,
};
use std::time::SystemTime;
use crate::models::{App, AppState, MetricData};

pub fn ui(f: &mut Frame, app: &mut App) {
    match app.state {
        AppState::RdsList => render_rds_list(f, app),
        AppState::InstanceDetails => render_instance_details(f, app),
    }
}

fn render_rds_list(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("AWS CloudWatch TUI - RDS Instances")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    if app.loading {
        let loading_msg = Paragraph::new("Loading RDS instances...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(loading_msg, chunks[1]);
        return;
    }

    if app.rds_instances.is_empty() {
        let no_instances = Paragraph::new("No RDS instances found in this account/region")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("RDS Instances"));
        f.render_widget(no_instances, chunks[1]);
        return;
    }

    // RDS Instances List
    let items: Vec<ListItem> = app
        .rds_instances
        .iter()
        .map(|instance| {
            let lines = vec![Line::from(vec![
                Span::styled(
                    format!("{}", instance.identifier),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" | "),
                Span::styled(
                    &instance.engine,
                    Style::default().fg(Color::Green),
                ),
                Span::raw(" | "),
                Span::styled(
                    &instance.status,
                    match instance.status.as_str() {
                        "available" => Style::default().fg(Color::Green),
                        "stopped" => Style::default().fg(Color::Red),
                        "starting" | "stopping" => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Gray),
                    },
                ),
                Span::raw(" | "),
                Span::styled(
                    &instance.instance_class,
                    Style::default().fg(Color::Cyan),
                ),
            ])];
            ListItem::new(lines)
        })
        .collect();

    let items = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("RDS Instances (↑/↓ to navigate, Enter to view details, q to quit)"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(items, chunks[1], &mut app.list_state);
}

fn render_instance_details(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(f.area());

    let instance = &app.rds_instances[app.selected_instance.unwrap()];

    // Instance Info
    let na_string = "N/A".to_string();
    let info_text = vec![
        Line::from(vec![
            Span::styled("Engine: ", Style::default().fg(Color::White)),
            Span::styled(&instance.engine, Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(&instance.status, match instance.status.as_str() {
                "available" => Style::default().fg(Color::Green),
                "stopped" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Yellow),
            }),
            Span::raw("  "),
            Span::styled("Class: ", Style::default().fg(Color::White)),
            Span::styled(&instance.instance_class, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Endpoint: ", Style::default().fg(Color::White)),
            Span::styled(
                instance.endpoint.as_ref().unwrap_or(&na_string),
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Instance Information"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(info, chunks[0]);

    if app.metrics_loading {
        let loading_msg = Paragraph::new("Loading metrics...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("CloudWatch Metrics"));
        f.render_widget(loading_msg, chunks[1]);
        return;
    }

    // Metrics
    render_metrics(f, chunks[1], &app.metrics, app.scroll_offset, app.metrics_per_screen);
}

fn render_metrics(f: &mut Frame, area: ratatui::layout::Rect, metrics: &MetricData, scroll_offset: usize, metrics_per_screen: usize) {
    // Enhanced layout with larger charts for better Braille resolution
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),  // Main metrics area - takes most space
            Constraint::Length(3),  // Instructions at bottom
        ])
        .split(area);

    // Helper function to check if a metric has actual data
    fn is_metric_available(history: &Vec<f64>) -> bool {
        !history.is_empty() // If we have data points, the metric is available regardless of values
    }



    // Define all 27 comprehensive RDS metrics with availability checking
    let mut individual_metrics = vec![];
    
    // Core Performance Metrics (14) - only add if they have data
    let core_metrics = vec![
        ("CPU Utilization", format!("{:.1}%", metrics.cpu_utilization), &metrics.cpu_history, Color::Red, 100.0),
        ("DB Connections", format!("{:.0}", metrics.database_connections), &metrics.connections_history, Color::Blue, 200.0),
        ("Read IOPS", format!("{:.0}", metrics.read_iops), &metrics.read_iops_history, Color::Green, 1000.0),
        ("Write IOPS", format!("{:.0}", metrics.write_iops), &metrics.write_iops_history, Color::Yellow, 1000.0),
        ("Read Latency", format!("{:.2} ms", metrics.read_latency * 1000.0), &metrics.read_latency_history, Color::Red, 0.1),
        ("Write Latency", format!("{:.2} ms", metrics.write_latency * 1000.0), &metrics.write_latency_history, Color::Magenta, 0.1),
        ("Free Storage", format!("{:.1} GB", metrics.free_storage_space / 1024.0 / 1024.0 / 1024.0), &metrics.free_storage_space_history, Color::White, 1000.0),
        ("Read Throughput", format!("{:.1} MB/s", metrics.read_throughput / 1024.0 / 1024.0), &metrics.read_throughput_history, Color::Cyan, 100_000_000.0),
        ("Write Throughput", format!("{:.1} MB/s", metrics.write_throughput / 1024.0 / 1024.0), &metrics.write_throughput_history, Color::LightYellow, 100_000_000.0),
        ("Network RX", format!("{:.1} MB/s", metrics.network_receive_throughput / 1024.0 / 1024.0), &metrics.network_receive_history, Color::LightBlue, 100_000_000.0),
        ("Network TX", format!("{:.1} MB/s", metrics.network_transmit_throughput / 1024.0 / 1024.0), &metrics.network_transmit_history, Color::LightGreen, 100_000_000.0),
        ("Freeable Memory", format!("{:.1} GB", metrics.freeable_memory / 1024.0 / 1024.0 / 1024.0), &metrics.freeable_memory_history, Color::LightMagenta, 10_000_000_000.0),
        ("Swap Usage", format!("{:.1} MB", metrics.swap_usage / 1024.0 / 1024.0), &metrics.swap_usage_history, Color::Gray, 1_000_000_000.0),
        ("Queue Depth", format!("{:.2}", metrics.queue_depth), &metrics.queue_depth_history, Color::DarkGray, 100.0),
    ];

    // Process core metrics and only add those with data
    for (name, value, history, color, max_val) in core_metrics {
        let available = is_metric_available(history);
        if available {
            individual_metrics.push((name, value, history, color, max_val, available));
        }
    }

    // Advanced RDS Metrics (13 additional) - Check availability for these
    let advanced_metrics = vec![
        ("Burst Balance", metrics.burst_balance, &metrics.burst_balance_history, "%", Color::LightCyan, 100.0, "gp2 storage only"),
        ("CPU Credit Usage", metrics.cpu_credit_usage, &metrics.cpu_credit_usage_history, "", Color::LightRed, 1000.0, "T-series instances only"),
        ("CPU Credit Balance", metrics.cpu_credit_balance, &metrics.cpu_credit_balance_history, "", Color::LightYellow, 1000.0, "T-series instances only"),
        ("Bin Log Usage", metrics.bin_log_disk_usage / 1024.0 / 1024.0, &metrics.bin_log_disk_usage_history, " MB", Color::Cyan, 100_000_000.0, "MySQL only"),
        ("Replica Lag", metrics.replica_lag, &metrics.replica_lag_history, " s", Color::Red, 60.0, "read replicas only"),
        ("Max Transaction IDs", metrics.maximum_used_transaction_ids, &metrics.maximum_used_transaction_ids_history, "", Color::Yellow, 2_000_000_000.0, "PostgreSQL"),
        ("Replication Slot Lag", metrics.oldest_replication_slot_lag / 1024.0 / 1024.0, &metrics.oldest_replication_slot_lag_history, " MB", Color::Magenta, 100_000_000.0, "PostgreSQL"),
        ("Replication Slot Usage", metrics.replication_slot_disk_usage / 1024.0 / 1024.0, &metrics.replication_slot_disk_usage_history, " MB", Color::LightMagenta, 100_000_000.0, "PostgreSQL"),
        ("Transaction Logs Usage", metrics.transaction_logs_disk_usage / 1024.0 / 1024.0, &metrics.transaction_logs_disk_usage_history, " MB", Color::Green, 100_000_000.0, "PostgreSQL"),
        ("Transaction Log Gen", metrics.transaction_logs_generation / 1024.0 / 1024.0, &metrics.transaction_logs_generation_history, " MB/s", Color::LightGreen, 10_000_000.0, "PostgreSQL"),
        ("Failed SQL Agent Jobs", metrics.failed_sql_server_agent_jobs_count, &metrics.failed_sql_server_agent_jobs_count_history, "", Color::Red, 10.0, "SQL Server only"),
        ("Checkpoint Lag", metrics.checkpoint_lag, &metrics.checkpoint_lag_history, " s", Color::Blue, 60.0, "PostgreSQL"),
        ("Connection Attempts", metrics.connection_attempts, &metrics.connection_attempts_history, "", Color::LightBlue, 1000.0, "some engines"),
    ];

    // Process advanced metrics and only add those with data
    for (name, value, history, unit, color, max_val, _requirement) in advanced_metrics {
        let available = is_metric_available(history);
        if available {
            let formatted_value = format!("{:.1}{}", value, unit);
            individual_metrics.push((name, formatted_value, history, color, max_val, available));
        }
    }

    // All metrics in the list now have data
    let available_count = individual_metrics.len();

    render_scrollable_individual_metrics(f, main_chunks[0], &metrics.timestamps, &individual_metrics, scroll_offset, metrics_per_screen);

    // Enhanced instructions showing only available metrics with scroll position
    let instructions = Paragraph::new(format!("↑/↓ scroll ({} metrics with data, showing {}/{}) • r refresh • b back • q quit", 
        available_count, 
        scroll_offset + 1,
        available_count))
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(instructions, main_chunks[1]);
}

// New function to render scrollable individual metrics (1 per row)
fn render_scrollable_individual_metrics(
    f: &mut Frame, 
    area: ratatui::layout::Rect,
    timestamps: &Vec<SystemTime>,
    individual_metrics: &[(&str, String, &Vec<f64>, Color, f64, bool)], // Added availability bool
    scroll_offset: usize,
    metrics_per_screen: usize
) {
    // For individual metrics, we want to show exactly metrics_per_screen (usually 1) 
    // to maximize chart size, regardless of terminal height
    let metrics_to_show = metrics_per_screen;
    
    // Apply scroll offset to get visible metrics
    let start_idx = scroll_offset;
    let end_idx = (start_idx + metrics_to_show).min(individual_metrics.len());
    let visible_metrics: Vec<_> = individual_metrics[start_idx..end_idx].iter().collect();
    
    if visible_metrics.is_empty() {
        let no_data = Paragraph::new("No metrics to display")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(no_data, area);
        return;
    }
    
    // Create constraints for each visible metric (equal height distribution)
    let constraints: Vec<Constraint> = (0..visible_metrics.len())
        .map(|_| Constraint::Percentage((100 / visible_metrics.len()) as u16))
        .collect();
    
    let metric_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    
    // Render each individual metric with full-width chart
    for (i, &metric) in visible_metrics.iter().enumerate() {
        let (name, value, history, color, max_val, available) = metric;
        render_large_metric_chart(f, metric_chunks[i], timestamps, (name, value.clone(), history, *color, *max_val, *available));
    }
}



// New large metric chart widget with enhanced Braille plotting
fn render_large_metric_chart(
    f: &mut Frame, 
    area: ratatui::layout::Rect, 
    timestamps: &Vec<SystemTime>,
    metric: (&str, String, &Vec<f64>, Color, f64, bool) // Added availability bool
) {
    let (name, value, history, color, max_val, available) = metric;
    
    // Ensure minimum area for meaningful charts
    if area.width < 20 || area.height < 6 {
        let simple_widget = Paragraph::new(format!("{}: {}", name, value))
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(simple_widget, area);
        return;
    }
    
    // Create layout: title + large chart area
    let widget_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Metric title
            Constraint::Min(12),     // Larger chart area for better high-resolution display
        ])
        .split(area);

    // Enhanced metric header with value integrated and availability indication
    let title_style = if available {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
    };
    
    let title_widget = Paragraph::new(format!("{}: {}", name, value))
        .style(title_style)
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(title_widget, widget_chunks[0]);

    // Render high-resolution chart if we have data and metric is available
    if available && !history.is_empty() && widget_chunks[1].height >= 5 {
        render_high_resolution_chart(f, widget_chunks[1], timestamps, history, color, max_val, name);
    } else {
        // Show status message with availability info
        let status_msg = if !available {
            "Metric not available for this DB engine/instance type".to_string()
        } else if history.is_empty() { 
            if name == "DB Connections" {
                format!("Loading data... ({})", history.len())
            } else {
                "Loading data...".to_string()
            }
        } else { 
            "Area too small for chart".to_string() 
        };
        
        let status_color = if !available { Color::DarkGray } else { Color::Gray };
        let status_widget = Paragraph::new(status_msg)
            .style(Style::default().fg(status_color))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("3hr"));
        f.render_widget(status_widget, widget_chunks[1]);
    }
}

// Simplified chart rendering using ratatui's built-in coordinate mapping
fn render_high_resolution_chart(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    timestamps: &Vec<SystemTime>,
    history: &Vec<f64>,
    color: Color,
    _max_val: f64,
    metric_name: &str
) {
    use chrono::{DateTime, Local, Utc};
    
    // Early return if no data
    if history.is_empty() || timestamps.is_empty() {
        let no_data_chart = Chart::new(vec![])
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Gray)))
            .x_axis(Axis::default().style(Style::default().fg(Color::Gray)).bounds([0.0, 180.0]))
            .y_axis(Axis::default().style(Style::default().fg(Color::Gray)).bounds([0.0, 1.0]));
        f.render_widget(no_data_chart, area);
        return;
    }
    
    // Verify data alignment
    if timestamps.len() != history.len() {
        return; // Skip rendering if data is misaligned
    }

    // Convert timestamps to Unix epoch seconds for consistent time handling
    let start_time: DateTime<Utc> = timestamps[0].into();
    let start_epoch = start_time.timestamp() as f64;
    
    // Create data points using Unix timestamps as X coordinates
    let data_points: Vec<(f64, f64)> = timestamps
        .iter()
        .zip(history.iter())
        .map(|(timestamp, &value)| {
            let dt: DateTime<Utc> = (*timestamp).into();
            let epoch_seconds = dt.timestamp() as f64;
            (epoch_seconds, value)
        })
        .collect();

    // Calculate time bounds in epoch seconds
    let end_epoch = data_points.last().map(|(x, _)| *x).unwrap_or(start_epoch + 3600.0 * 3.0);
    let time_bounds = [start_epoch, end_epoch];
    
    // Calculate Y-axis bounds with proper padding
    let (y_min, y_max) = if history.len() == 1 {
        let val = history[0];
        let margin = if val.abs() > 1.0 { val.abs() * 0.1 } else { 1.0 };
        (val - margin, val + margin)
    } else {
        let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
        if min_val.is_finite() && max_val.is_finite() && min_val != max_val {
            let range = max_val - min_val;
            let padding = range * 0.1;
            let y_min = if min_val >= 0.0 { (min_val - padding).max(0.0) } else { min_val - padding };
            (y_min, max_val + padding)
        } else {
            (0.0, 1.0)
        }
    };

    // Ensure valid Y bounds
    let y_bounds = if y_max <= y_min { [y_min, y_min + 1.0] } else { [y_min, y_max] };

    // Create dataset
    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&data_points);

    // Create X-axis labels based on actual data points for proper alignment
    let num_x_labels = 8.min(timestamps.len()); // Reasonable number of labels, but not more than data points
    
    let x_labels: Vec<Line> = if timestamps.len() <= 1 {
        // Handle single data point case
        vec![Line::from(Span::styled(
            {
                let dt: DateTime<Utc> = timestamps[0].into();
                let local_time: DateTime<Local> = dt.into();
                format!("{}", local_time.format("%H:%M"))
            },
            Style::default().fg(Color::DarkGray)
        ))]
    } else {
        // Select evenly spaced timestamps from actual data
        (0..num_x_labels)
            .map(|i| {
                let idx = if num_x_labels == 1 {
                    0
                } else {
                    (i * (timestamps.len() - 1)) / (num_x_labels - 1)
                };
                
                let timestamp = timestamps[idx];
                let dt: DateTime<Utc> = timestamp.into();
                let local_time: DateTime<Local> = dt.into();
                
                Line::from(Span::styled(
                    format!("{}", local_time.format("%H:%M")),
                    Style::default().fg(Color::DarkGray)
                ))
            })
            .collect()
    };

    // Create Y-axis labels with smart formatting
    let format_value = |v: f64| -> String {
        if metric_name.contains("Memory") || metric_name.contains("Storage") {
            let gb_value = v / (1024.0 * 1024.0 * 1024.0);
            if gb_value >= 1.0 {
                format!("{:.1}G", gb_value)
            } else {
                let mb_value = v / (1024.0 * 1024.0);
                format!("{:.0}M", mb_value)
            }
        } else if metric_name.contains("Throughput") || metric_name.contains("Network") {
            let mb_value = v / (1024.0 * 1024.0);
            if mb_value >= 1.0 {
                format!("{:.1}M", mb_value)
            } else {
                let kb_value = v / 1024.0;
                format!("{:.0}K", kb_value)
            }
        } else if v.abs() >= 1000000.0 {
            format!("{:.1}M", v / 1000000.0)
        } else if v.abs() >= 1000.0 {
            format!("{:.1}K", v / 1000.0)
        } else if v.abs() >= 1.0 {
            format!("{:.1}", v)
        } else {
            format!("{:.2}", v)
        }
    };

    let y_range = y_bounds[1] - y_bounds[0];
    let num_y_labels = if y_range <= 1.0 { 12 } else if y_range <= 10.0 { 15 } else { 12 };
    
    let y_labels: Vec<Line> = (0..num_y_labels)
        .map(|i| {
            let ratio = i as f64 / (num_y_labels - 1) as f64;
            let value = y_bounds[0] + ratio * y_range;
            Line::from(Span::styled(
                format_value(value),
                Style::default().fg(Color::DarkGray)
            ))
        })
        .collect();

    // Create and render chart
    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray))
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(time_bounds)
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


