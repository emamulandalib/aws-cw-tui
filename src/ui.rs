use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, 
        Paragraph, Chart, Axis, Dataset
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
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(f.area());

    let instance = &app.rds_instances[app.selected_instance.unwrap()];

    // Header
    let header = Paragraph::new(format!("RDS Instance Details - {}", instance.identifier))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

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
    f.render_widget(info, chunks[1]);

    if app.metrics_loading {
        let loading_msg = Paragraph::new("Loading metrics...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("CloudWatch Metrics"));
        f.render_widget(loading_msg, chunks[2]);
        return;
    }

    // Metrics
    render_metrics(f, chunks[2], &app.metrics, app.scroll_offset, app.metrics_per_screen);
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

    // Define all metric pairs with their data
    let empty_history = Vec::new(); // Shared empty vector for metrics without history
    let metric_pairs = vec![
        // Core Performance
        (
            ("CPU Utilization", format!("{:.1}%", metrics.cpu_utilization), &metrics.cpu_history, Color::Red, 100.0),
            ("DB Connections", format!("{:.0}", metrics.database_connections), &metrics.connections_history, Color::Blue, 200.0)
        ),
        // Storage Performance 
        (
            ("Read IOPS", format!("{:.0}", metrics.read_iops), &metrics.read_iops_history, Color::Green, 1000.0),
            ("Write IOPS", format!("{:.0}", metrics.write_iops), &metrics.write_iops_history, Color::Yellow, 1000.0)
        ),
        // Latency Metrics
        (
            ("Read Latency", format!("{:.2} ms", metrics.read_latency * 1000.0), &metrics.read_latency_history, Color::Red, 0.1),
            ("Write Latency", format!("{:.2} ms", metrics.write_latency * 1000.0), &metrics.write_latency_history, Color::Magenta, 0.1)
        ),
        // Throughput Metrics
        (
            ("Read Throughput", format!("{:.1} MB/s", metrics.read_throughput / 1024.0 / 1024.0), &metrics.read_throughput_history, Color::Cyan, 100_000_000.0),
            ("Write Throughput", format!("{:.1} MB/s", metrics.write_throughput / 1024.0 / 1024.0), &metrics.write_throughput_history, Color::LightYellow, 100_000_000.0)
        ),
        // Network Metrics
        (
            ("Network RX", format!("{:.1} MB/s", metrics.network_receive_throughput / 1024.0 / 1024.0), &metrics.network_receive_history, Color::LightBlue, 100_000_000.0),
            ("Network TX", format!("{:.1} MB/s", metrics.network_transmit_throughput / 1024.0 / 1024.0), &metrics.network_transmit_history, Color::LightGreen, 100_000_000.0)
        ),
        // Memory Metrics
        (
            ("Freeable Memory", format!("{:.1} GB", metrics.freeable_memory / 1024.0 / 1024.0 / 1024.0), &metrics.freeable_memory_history, Color::LightMagenta, 10_000_000_000.0),
            ("Swap Usage", format!("{:.1} MB", metrics.swap_usage / 1024.0 / 1024.0), &metrics.swap_usage_history, Color::Gray, 1_000_000_000.0)
        ),
        // I/O Queue Metrics
        (
            ("Queue Depth", format!("{:.2}", metrics.queue_depth), &metrics.queue_depth_history, Color::DarkGray, 100.0),
            ("Free Storage", format!("{:.1} GB", metrics.free_storage_space / 1024.0 / 1024.0 / 1024.0), &empty_history, Color::White, 1000.0) // Use shared empty vector
        ),
    ];

    render_scrollable_metrics(f, main_chunks[0], &metrics.timestamps, &metric_pairs, scroll_offset, metrics_per_screen);

    // Enhanced instructions
    let instructions = Paragraph::new("Navigation: ↑/↓ or k/j to scroll • r to refresh • b to go back • q to quit • Home to reset scroll | High-Resolution CloudWatch RDS Monitoring")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(instructions, main_chunks[1]);
}

// New function to render scrollable metrics with larger charts
fn render_scrollable_metrics(
    f: &mut Frame, 
    area: ratatui::layout::Rect,
    timestamps: &Vec<SystemTime>,
    metric_pairs: &[((&str, String, &Vec<f64>, Color, f64), (&str, String, &Vec<f64>, Color, f64))],
    scroll_offset: usize,
    metrics_per_screen: usize
) {
    // Calculate how many metric pairs we can show - each pair needs significant height for good charts
    let min_height_per_pair = 8; // Minimum height for readable charts with Braille
    let available_height = area.height as usize;
    let max_pairs_visible = (available_height / min_height_per_pair).max(1).min(metrics_per_screen);
    
    // Apply scroll offset to get visible pairs
    let start_idx = scroll_offset;
    let end_idx = (start_idx + max_pairs_visible).min(metric_pairs.len());
    let visible_pairs: Vec<_> = metric_pairs[start_idx..end_idx].iter().collect();
    
    if visible_pairs.is_empty() {
        let no_data = Paragraph::new("No metrics to display")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(no_data, area);
        return;
    }
    
    // Create constraints for each visible pair
    let constraints: Vec<Constraint> = (0..visible_pairs.len())
        .map(|_| Constraint::Percentage((100 / visible_pairs.len()) as u16))
        .collect();
    
    let pair_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    
    // Render each metric pair with enhanced charts
    for (i, &(left_metric, right_metric)) in visible_pairs.iter().enumerate() {
        render_enhanced_metric_pair(f, pair_chunks[i], timestamps, left_metric.clone(), right_metric.clone());
    }
}

// Enhanced metric pair rendering with larger, high-resolution charts
fn render_enhanced_metric_pair(
    f: &mut Frame, 
    area: ratatui::layout::Rect,
    timestamps: &Vec<SystemTime>,
    left_metric: (&str, String, &Vec<f64>, Color, f64),
    right_metric: (&str, String, &Vec<f64>, Color, f64)
) {
    // Split into left and right sections
    let row_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    render_large_metric_chart(f, row_chunks[0], timestamps, left_metric);
    render_large_metric_chart(f, row_chunks[1], timestamps, right_metric);
}

// New large metric chart widget with enhanced Braille plotting
fn render_large_metric_chart(
    f: &mut Frame, 
    area: ratatui::layout::Rect, 
    timestamps: &Vec<SystemTime>,
    metric: (&str, String, &Vec<f64>, Color, f64)
) {
    let (name, value, history, color, max_val) = metric;
    
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
            Constraint::Min(5),     // Large chart area for high resolution
        ])
        .split(area);

    // Enhanced metric title with current value
    let title_widget = Paragraph::new(format!("{}: {}", name, value))
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title_widget, widget_chunks[0]);

    // Render high-resolution chart if we have data
    if !history.is_empty() && widget_chunks[1].height >= 5 {
        render_high_resolution_chart(f, widget_chunks[1], timestamps, history, color, max_val, name);
    } else {
        // Show status message
        let status_msg = if history.is_empty() { "Loading data..." } else { "Area too small for chart" };
        let status_widget = Paragraph::new(status_msg)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("3hr"));
        f.render_widget(status_widget, widget_chunks[1]);
    }
}

// High-resolution chart rendering with enhanced Braille markers
fn render_high_resolution_chart(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    timestamps: &Vec<SystemTime>,
    history: &Vec<f64>,
    color: Color,
    max_val: f64,
    metric_name: &str
) {
    // Convert timestamps to relative time in hours (negative values for past)
    let data_points: Vec<(f64, f64)> = if timestamps.len() == history.len() {
        // Use actual timestamps for precise time-based plotting
        timestamps
            .iter()
            .zip(history.iter())
            .map(|(timestamp, &value)| {
                let hours_ago = timestamp.elapsed()
                    .map(|d| -(d.as_secs() as f64 / 3600.0))
                    .unwrap_or(-3.0);
                (hours_ago, value)
            })
            .collect()
    } else {
        // Fallback to evenly distributed time points
        history
            .iter()
            .enumerate()
            .map(|(i, &value)| {
                let hours_ago = if history.len() > 1 {
                    -3.0 + (i as f64 * 3.0 / (history.len() as f64 - 1.0))
                } else {
                    0.0
                };
                (hours_ago, value)
            })
            .collect()
    };

    // Calculate intelligent chart bounds
    let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val_from_history = history.iter().cloned().fold(0.0, f64::max);
    
    // Use adaptive bounds for better visualization
    let chart_min = if min_val.is_finite() { 
        (min_val * 0.95).min(0.0) // Add 5% padding below minimum
    } else { 
        0.0 
    };
    
    let chart_max = if max_val > 0.0 { 
        max_val * 1.1 // Add 10% padding above maximum
    } else { 
        if max_val_from_history > chart_min { 
            max_val_from_history * 1.1 
        } else { 
            chart_min + 1.0 
        }
    };

    // Ensure we have valid bounds
    let final_max = if chart_max <= chart_min { chart_min + 1.0 } else { chart_max };

    // Create high-resolution dataset with Braille markers
    let dataset = Dataset::default()
        .name("")  // No legend needed for single metric
        .marker(symbols::Marker::Braille)  // High-resolution Braille plotting
        .style(Style::default().fg(color))
        .data(&data_points);

    // Create enhanced chart with CloudWatch-style presentation
    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("3hr • {}", metric_name))
                .title_style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::Gray))
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([-3.0, 0.0])  // 3 hours ago to now
                .labels(vec![
                    Line::from("-3h"),
                    Line::from("-2h"),
                    Line::from("-1h"), 
                    Line::from("now")
                ])
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([chart_min, final_max])
                .labels(vec![
                    Line::from(format!("{:.2}", chart_min)),
                    Line::from(format!("{:.2}", (chart_min + final_max) / 2.0)),
                    Line::from(format!("{:.2}", final_max))
                ])
        );

    f.render_widget(chart, area);
}


