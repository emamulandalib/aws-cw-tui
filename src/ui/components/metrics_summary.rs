use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

    // Split content area into two columns: Available Metrics and Time Ranges
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Available Metrics (70%)
            Constraint::Percentage(30), // Time Ranges (30%)
        ])
        .split(chunks[1]);

    // Content area - Available Metrics
    render_metrics_with_sparklines(f, app, content_chunks[0]);

    // Content area - Time Ranges
    render_time_ranges(f, app, content_chunks[1]);

    // Controls
    render_controls(f, chunks[2]);
}

fn render_metrics_with_sparklines(f: &mut Frame, app: &mut App, area: Rect) {
    let metrics = get_available_metrics_with_history(&app.metrics);
    
    // Calculate the inner content width of the List widget
    // List widget with borders consumes 2 chars (left + right border)
    // The highlight symbol is handled internally by the List widget
    let inner_width = area.width.saturating_sub(2); // Only subtract border width
    let sparkline_width = (inner_width as usize).max(10);
    
    let items: Vec<ListItem> = metrics
        .iter()
        .map(|(name, current_value, history, unit)| {
            // Generate sparkline with maximum available width
            let sparkline = generate_sparkline(history, sparkline_width);
            let (value_color, trend_color) = get_metric_colors(*name, *current_value);
            
            // Format current value with appropriate units
            let formatted_value = format_value(*current_value, unit);
            
            let content = vec![
                Line::from(vec![
                    Span::styled(*name, Style::default().fg(Color::Cyan)),
                    Span::styled(": ", Style::default().fg(Color::White)),
                    Span::styled(formatted_value, Style::default().fg(value_color)),
                ]),
                Line::from(Span::styled(sparkline, Style::default().fg(trend_color))),
                Line::from(""), // Empty line for padding between items
            ];
            
            ListItem::new(content)
        })
        .collect();

    // Determine if this panel is focused
    let is_focused = matches!(app.get_focused_panel(), crate::models::FocusedPanel::Metrics);
    let border_color = if is_focused { Color::Green } else { Color::White };
    let title = if is_focused { "Available Metrics [FOCUSED]" } else { "Available Metrics" };

    let metrics_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(border_color)))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    // Create a list state for this render that maintains scroll position
    let mut list_state = ratatui::widgets::ListState::default();
    let current_scroll = app.get_current_scroll_position();
    list_state.select(Some(current_scroll));

    // Render the stateful list widget
    f.render_stateful_widget(metrics_list, area, &mut list_state);
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

fn generate_sparkline(data: &[f64], width: usize) -> String {
    if data.is_empty() || width == 0 {
        return "─".repeat(width);
    }
    
    // Take the last `width` data points
    let start_idx = if data.len() > width { data.len() - width } else { 0 };
    let slice = &data[start_idx..];
    
    if slice.len() < 2 {
        return "─".repeat(width);
    }
    
    let min_val = slice.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = slice.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    if (max_val - min_val).abs() < f64::EPSILON {
        return "─".repeat(width);
    }
    
    let mut result = String::new();
    for &value in slice {
        let normalized = (value - min_val) / (max_val - min_val);
        let level = (normalized * 7.0) as usize;
        let char = match level {
            0 => '▁',
            1 => '▂',
            2 => '▃',
            3 => '▄',
            4 => '▅',
            5 => '▆',
            6 => '▇',
            _ => '█',
        };
        result.push(char);
    }
    
    // Pad to desired width if needed
    while result.len() < width {
        result.push('▁');
    }
    
    result
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

fn render_time_ranges(f: &mut Frame, app: &mut App, area: Rect) {
    let time_ranges = crate::models::App::get_time_range_options();
    
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
            
            ListItem::new(Line::from(Span::styled(label, style)))
        })
        .collect();

    // Determine if this panel is focused
    let is_focused = matches!(app.get_focused_panel(), crate::models::FocusedPanel::TimeRanges);
    let border_color = if is_focused { Color::Green } else { Color::White };
    let title = if is_focused { "Time Ranges [FOCUSED]" } else { "Time Ranges" };

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
        .highlight_symbol("► ");

    // Create a temporary list state for this render
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.get_current_time_range_index()));

    // Render the stateful list widget
    f.render_stateful_widget(time_range_list, area, &mut list_state);
}
