use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Chart, Axis, Dataset, GraphType},
    symbols,
};
use crate::models::{App, MetricData};

pub fn render_metrics_summary(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header - increased height to show the endpoint
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    // Header - Instance Information
    if let Some(selected_idx) = app.selected_instance {
        if let Some(instance) = app.rds_instances.get(selected_idx) {
            render_instance_info(f, chunks[0], app, instance);
        } else {
            render_default_header(f, chunks[0]);
        }
    } else {
        render_default_header(f, chunks[0]);
    }

    // Responsive dashboard layout: Compact time panel + Expanded trend visualization
    let time_panel_width = calculate_time_panel_width(chunks[1].width);
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(time_panel_width), // Compact Time Panel (responsive width)
            Constraint::Min(0),                   // Trend Chart (flex-grow equivalent)
        ])
        .split(chunks[1]);

    // Compact Time Range Panel
    render_compact_time_ranges(f, app, content_chunks[0]);

    // Expanded Trend Visualization
    render_expanded_trend_chart(f, app, content_chunks[1]);

    // Controls
    render_controls(f, chunks[2]);
}

fn calculate_time_panel_width(total_width: u16) -> u16 {
    // Responsive breakpoints: min 20, max 25 chars based on available space
    if total_width < 60 {
        20 // Minimum usable width for small terminals
    } else if total_width < 100 {
        22 // Medium terminals
    } else {
        25 // Large terminals - maximum width to preserve space for trend chart
    }
}


fn get_available_metrics_with_history(metrics: &MetricData) -> Vec<(&'static str, f64, &Vec<f64>, &'static str)> {
    let mut available = Vec::new();
    
    // Core metrics
    if !metrics.cpu_history.is_empty() {
        available.push(("CPU Utilization", metrics.cpu_utilization, &metrics.cpu_history, "Percent"));
    }
    if !metrics.connections_history.is_empty() {
        available.push(("Database Connections", metrics.database_connections, &metrics.connections_history, "Count"));
    }
    if !metrics.read_iops_history.is_empty() {
        available.push(("Read IOPS", metrics.read_iops, &metrics.read_iops_history, "Count/Second"));
    }
    if !metrics.write_iops_history.is_empty() {
        available.push(("Write IOPS", metrics.write_iops, &metrics.write_iops_history, "Count/Second"));
    }
    if !metrics.read_latency_history.is_empty() {
        available.push(("Read Latency", metrics.read_latency, &metrics.read_latency_history, "Seconds"));
    }
    if !metrics.write_latency_history.is_empty() {
        available.push(("Write Latency", metrics.write_latency, &metrics.write_latency_history, "Seconds"));
    }
    if !metrics.free_storage_space_history.is_empty() {
        available.push(("Free Storage Space", metrics.free_storage_space, &metrics.free_storage_space_history, "Bytes"));
    }
    if !metrics.read_throughput_history.is_empty() {
        available.push(("Read Throughput", metrics.read_throughput, &metrics.read_throughput_history, "Bytes/Second"));
    }
    if !metrics.write_throughput_history.is_empty() {
        available.push(("Write Throughput", metrics.write_throughput, &metrics.write_throughput_history, "Bytes/Second"));
    }
    if !metrics.network_receive_history.is_empty() {
        available.push(("Network Receive", metrics.network_receive_throughput, &metrics.network_receive_history, "Bytes/Second"));
    }
    if !metrics.network_transmit_history.is_empty() {
        available.push(("Network Transmit", metrics.network_transmit_throughput, &metrics.network_transmit_history, "Bytes/Second"));
    }
    if !metrics.freeable_memory_history.is_empty() {
        available.push(("Freeable Memory", metrics.freeable_memory, &metrics.freeable_memory_history, "Bytes"));
    }
    if !metrics.swap_usage_history.is_empty() {
        available.push(("Swap Usage", metrics.swap_usage, &metrics.swap_usage_history, "Bytes"));
    }
    if !metrics.queue_depth_history.is_empty() {
        available.push(("Queue Depth", metrics.queue_depth, &metrics.queue_depth_history, "Count"));
    }

    // Advanced metrics
    if !metrics.burst_balance_history.is_empty() {
        available.push(("Burst Balance", metrics.burst_balance, &metrics.burst_balance_history, "Percent"));
    }
    if !metrics.cpu_credit_usage_history.is_empty() {
        available.push(("CPU Credit Usage", metrics.cpu_credit_usage, &metrics.cpu_credit_usage_history, "Credits"));
    }
    if !metrics.cpu_credit_balance_history.is_empty() {
        available.push(("CPU Credit Balance", metrics.cpu_credit_balance, &metrics.cpu_credit_balance_history, "Credits"));
    }
    if !metrics.replica_lag_history.is_empty() {
        available.push(("Replica Lag", metrics.replica_lag, &metrics.replica_lag_history, "Seconds"));
    }

    available
}

fn get_metric_colors(metric_name: &str, current_value: f64) -> (Color, Color) {
    let (value_color, trend_color) = match metric_name {
        "CPU Utilization" => {
            if current_value > 80.0 { (Color::Red, Color::Red) }
            else if current_value > 60.0 { (Color::Yellow, Color::Yellow) }
            else { (Color::Green, Color::Green) }
        },
        "Database Connections" => {
            // Assume > 1000 is high, > 500 is moderate
            if current_value > 1000.0 { (Color::Red, Color::Red) }
            else if current_value > 500.0 { (Color::Yellow, Color::Yellow) }
            else { (Color::Green, Color::Green) }
        },
        "Read Latency" | "Write Latency" => {
            // Latency in seconds - > 0.1s is bad, > 0.05s is moderate
            if current_value > 0.1 { (Color::Red, Color::Red) }
            else if current_value > 0.05 { (Color::Yellow, Color::Yellow) }
            else { (Color::Green, Color::Green) }
        },
        "Free Storage Space" | "Freeable Memory" => {
            // For storage/memory, lower is worse (inverted logic)
            if current_value < 1024.0 * 1024.0 * 1024.0 { (Color::Red, Color::Red) } // < 1GB
            else if current_value < 5.0 * 1024.0 * 1024.0 * 1024.0 { (Color::Yellow, Color::Yellow) } // < 5GB
            else { (Color::Green, Color::Green) }
        },
        "Burst Balance" => {
            if current_value < 20.0 { (Color::Red, Color::Red) }
            else if current_value < 50.0 { (Color::Yellow, Color::Yellow) }
            else { (Color::Green, Color::Green) }
        },
        "Replica Lag" => {
            if current_value > 300.0 { (Color::Red, Color::Red) } // > 5 minutes
            else if current_value > 60.0 { (Color::Yellow, Color::Yellow) } // > 1 minute
            else { (Color::Green, Color::Green) }
        },
        _ => (Color::Cyan, Color::Cyan), // Default neutral color
    };
    
    (value_color, trend_color)
}

fn format_value(value: f64, unit: &str) -> String {
    match unit {
        "Bytes" | "Bytes/Second" => format_bytes(value),
        "Percent" => format!("{:.1}%", value),
        "Seconds" => {
            if value < 0.001 {
                format!("{:.2} μs", value * 1_000_000.0)
            } else if value < 1.0 {
                format!("{:.2} ms", value * 1000.0)
            } else {
                format!("{:.2} s", value)
            }
        },
        "Count" | "Count/Second" | "Credits" => {
            if value >= 1_000_000.0 {
                format!("{:.1}M", value / 1_000_000.0)
            } else if value >= 1_000.0 {
                format!("{:.1}K", value / 1_000.0)
            } else {
                format!("{:.1}", value)
            }
        },
        _ => format!("{:.2}", value),
    }
}

fn format_bytes(bytes: f64) -> String {
    const UNITS: &[(&str, f64)] = &[
        ("TB", 1024.0 * 1024.0 * 1024.0 * 1024.0),
        ("GB", 1024.0 * 1024.0 * 1024.0),
        ("MB", 1024.0 * 1024.0),
        ("KB", 1024.0),
    ];
    
    for &(unit, size) in UNITS {
        if bytes >= size {
            return format!("{:.1} {}", bytes / size, unit);
        }
    }
    
    format!("{:.0} B", bytes)
}

fn render_instance_info(f: &mut Frame, area: ratatui::layout::Rect, _app: &crate::models::App, instance: &crate::models::RdsInstance) {
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
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Instance Information")
            .border_style(Style::default().fg(Color::Cyan)))
        .wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(info, area);
}

fn render_default_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let header_block = Paragraph::new("Metrics Summary")
        .style(Style::default().fg(Color::White))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("RDS CloudWatch TUI")
            .border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(header_block, area);
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect) {
    let controls = Paragraph::new(
        "↑/↓: Navigate • Tab/←/→: Switch Panels • Enter: Select/View Details • r: Refresh • b/Esc: Back • q: Quit")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(controls, area);
}

fn render_compact_time_ranges(f: &mut Frame, app: &mut App, area: Rect) {
    let time_ranges = crate::models::App::get_time_range_options();
    
    // Create compact time range items with abbreviated labels for narrow space
    let items: Vec<ListItem> = time_ranges
        .iter()
        .enumerate()
        .map(|(i, &(label, _value, _unit, _period))| {
            let is_selected = i == app.get_current_time_range_index();
            let style = if is_selected {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            // Abbreviate labels for compact display
            let compact_label = match label {
                "Last 1 Hour" => "1h",
                "Last 3 Hours" => "3h",
                "Last 6 Hours" => "6h",
                "Last 12 Hours" => "12h",
                "Last 24 Hours" => "24h",
                "Last 3 Days" => "3d",
                "Last 7 Days" => "7d",
                _ => label, // Fallback to original if no abbreviation
            };
            
            // Add selection indicator
            let display_text = if is_selected {
                format!("● {}", compact_label)
            } else {
                format!("  {}", compact_label)
            };
            
            ListItem::new(Line::from(Span::styled(display_text, style)))
        })
        .collect();

    // Determine if this panel is focused
    let is_focused = matches!(app.get_focused_panel(), crate::models::FocusedPanel::TimeRanges);
    let border_color = if is_focused { Color::Green } else { Color::White };
    let title = if is_focused { "Time [F]" } else { "Time" }; // Compact title

    let time_range_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(border_color)))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(""); // Remove highlight symbol to save space

    // Create a temporary list state for this render
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.get_current_time_range_index()));

    // Render the stateful list widget
    f.render_stateful_widget(time_range_list, area, &mut list_state);
}

fn render_expanded_trend_chart(f: &mut Frame, app: &mut App, area: Rect) {
    // Determine if this panel is focused
    let is_focused = matches!(app.get_focused_panel(), crate::models::FocusedPanel::Metrics);
    let border_color = if is_focused { Color::Green } else { Color::White };
    let title = if is_focused { "Trend Visualization [FOCUSED]" } else { "Trend Visualization" };

    // Get currently selected metric for detailed chart display
    let selected_metric_idx = app.get_current_scroll_position();
    let available_metrics = get_available_metrics_with_history(&app.metrics);
    
    if available_metrics.is_empty() {
        let no_data = Paragraph::new("No metrics available")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(border_color)));
        f.render_widget(no_data, area);
        return;
    }

    // Select metric to display (bounded by available metrics)
    let metric_idx = selected_metric_idx.min(available_metrics.len().saturating_sub(1));
    let (metric_name, current_value, history, unit) = available_metrics[metric_idx];

    if history.is_empty() || app.metrics.timestamps.is_empty() {
        let loading_msg = Paragraph::new("Loading metric data...")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(border_color)));
        f.render_widget(loading_msg, area);
        return;
    }

    // Split area for metric info and chart
    let chart_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Metric info header
            Constraint::Min(0),    // Chart area
        ])
        .split(area);

    // Render metric info header
    render_metric_info_header(f, chart_chunks[0], metric_name, current_value, unit, border_color);

    // Render the enhanced trend chart
    render_enhanced_trend_chart(f, chart_chunks[1], &app.metrics.timestamps, history, metric_name, border_color);
}

fn render_metric_info_header(f: &mut Frame, area: Rect, metric_name: &str, current_value: f64, unit: &str, border_color: Color) {
    let formatted_value = format_value(current_value, unit);
    let (value_color, _) = get_metric_colors(metric_name, current_value);
    
    let info_text = vec![
        Line::from(vec![
            Span::styled(metric_name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(": ", Style::default().fg(Color::White)),
            Span::styled(formatted_value, Style::default().fg(value_color).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let info_block = Paragraph::new(info_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)))
        .alignment(ratatui::layout::Alignment::Left);
    
    f.render_widget(info_block, area);
}

fn render_enhanced_trend_chart(f: &mut Frame, area: Rect, timestamps: &Vec<std::time::SystemTime>, history: &Vec<f64>, metric_name: &str, border_color: Color) {
    use chrono::{DateTime, Utc};
    
    if area.width < 30 || area.height < 8 {
        let small_area_msg = Paragraph::new("Area too small for chart")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Chart")
                .border_style(Style::default().fg(border_color)));
        f.render_widget(small_area_msg, area);
        return;
    }

    // Prepare data points for chart
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

    let end_epoch = data_points.last().map(|(x, _)| *x).unwrap_or(start_epoch + 3600.0 * 3.0);
    let time_bounds = [start_epoch, end_epoch];
    
    // Calculate Y bounds with padding
    let (y_min, y_max) = calculate_chart_y_bounds(history);
    let y_bounds = if y_max <= y_min { [y_min, y_min + 1.0] } else { [y_min, y_max] };

    // Choose color based on metric type
    let chart_color = get_chart_color(metric_name);

    // Create enhanced dataset with thicker line appearance
    let dataset = Dataset::default()
        .name("")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(chart_color))
        .data(&data_points);

    // Create axis labels
    let x_labels = create_chart_x_labels(timestamps, area.width);
    let y_labels = create_chart_y_labels(y_bounds, metric_name, area.height);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Trend ({})", get_chart_time_range(timestamps)))
                .border_style(Style::default().fg(border_color))
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

fn calculate_chart_y_bounds(history: &Vec<f64>) -> (f64, f64) {
    if history.len() == 1 {
        let val = history[0];
        let margin = if val.abs() > 1.0 { val.abs() * 0.1 } else { 1.0 };
        (val - margin, val + margin)
    } else {
        let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
        if min_val.is_finite() && max_val.is_finite() && min_val != max_val {
            let range = max_val - min_val;
            let padding = range * 0.15; // Slightly more padding for better visibility
            let y_min = if min_val >= 0.0 { (min_val - padding).max(0.0) } else { min_val - padding };
            (y_min, max_val + padding)
        } else {
            (0.0, 1.0)
        }
    }
}

fn get_chart_color(metric_name: &str) -> Color {
    match metric_name {
        "CPU Utilization" => Color::Red,
        "Database Connections" => Color::Blue,
        "Read IOPS" | "Write IOPS" => Color::Green,
        "Read Latency" | "Write Latency" => Color::Magenta,
        "Free Storage Space" | "Freeable Memory" => Color::Cyan,
        "Network Receive" | "Network Transmit" => Color::Yellow,
        "Read Throughput" | "Write Throughput" => Color::LightGreen,
        _ => Color::White,
    }
}

fn create_chart_x_labels(timestamps: &Vec<std::time::SystemTime>, width: u16) -> Vec<Line> {
    use chrono::{DateTime, Utc};
    
    // Adaptive number of labels based on chart width
    let num_labels = ((width / 12).max(3).min(8)) as usize;
    let num_x_labels = num_labels.min(timestamps.len());
    
    if timestamps.len() <= 1 {
        vec![Line::from(Span::styled(
            {
                let dt: DateTime<Utc> = timestamps[0].into();
                let local_time: chrono::DateTime<chrono::Local> = dt.into();
                format!("{}", local_time.format("%H:%M"))
            },
            Style::default().fg(Color::DarkGray)
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
                    Style::default().fg(Color::DarkGray)
                ))
            })
            .collect()
    }
}

fn create_chart_y_labels(y_bounds: [f64; 2], metric_name: &str, height: u16) -> Vec<Line> {
    // Adaptive number of Y labels based on chart height
    let num_labels = ((height / 3).max(3).min(8)) as usize;
    let y_range = y_bounds[1] - y_bounds[0];
    
    (0..num_labels)
        .map(|i| {
            let ratio = i as f64 / (num_labels - 1) as f64;
            let value = y_bounds[0] + ratio * y_range;
            let formatted = format_chart_y_value(value, metric_name);
            Line::from(Span::styled(
                formatted,
                Style::default().fg(Color::DarkGray)
            ))
        })
        .collect()
}

fn format_chart_y_value(value: f64, metric_name: &str) -> String {
    if metric_name.contains("Memory") || metric_name.contains("Storage") {
        let gb_value = value / (1024.0 * 1024.0 * 1024.0);
        if gb_value >= 1.0 {
            format!("{:.1}G", gb_value)
        } else {
            let mb_value = value / (1024.0 * 1024.0);
            format!("{:.0}M", mb_value)
        }
    } else if metric_name.contains("Throughput") || metric_name.contains("Network") {
        let mb_value = value / (1024.0 * 1024.0);
        if mb_value >= 1.0 {
            format!("{:.1}M", mb_value)
        } else {
            let kb_value = value / 1024.0;
            format!("{:.0}K", kb_value)
        }
    } else if value.abs() >= 1000000.0 {
        format!("{:.1}M", value / 1000000.0)
    } else if value.abs() >= 1000.0 {
        format!("{:.1}K", value / 1000.0)
    } else if value.abs() >= 1.0 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
    }
}

fn get_chart_time_range(timestamps: &Vec<std::time::SystemTime>) -> String {
    if timestamps.len() < 2 {
        return "Real-time".to_string();
    }
    
    use chrono::{DateTime, Utc};
    let start: DateTime<Utc> = timestamps[0].into();
    let end: DateTime<Utc> = timestamps[timestamps.len() - 1].into();
    let duration = end.signed_duration_since(start);
    
    if duration.num_hours() >= 24 {
        format!("{}d", duration.num_days())
    } else if duration.num_hours() >= 1 {
        format!("{}h", duration.num_hours())
    } else {
        format!("{}m", duration.num_minutes())
    }
}

