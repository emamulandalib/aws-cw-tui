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
use chrono::{Local, Duration};
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

    // Define all metric pairs with their data
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
            ("Free Storage", format!("{:.1} GB", metrics.free_storage_space / 1024.0 / 1024.0 / 1024.0), &metrics.free_storage_space_history, Color::White, 1000.0)
        ),
    ];

    render_scrollable_metrics(f, main_chunks[0], &metrics.timestamps, &metric_pairs, scroll_offset, metrics_per_screen);

    // Simplified instructions
    let instructions = Paragraph::new("↑/↓ scroll • r refresh • b back • q quit")
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
    let min_height_per_pair = 12; // Increased height for taller charts with better Braille resolution
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
            Constraint::Min(8),     // Larger chart area for better high-resolution display
        ])
        .split(area);

    // Enhanced metric header with value integrated
    let title_widget = Paragraph::new(format!("{}: {}", name, value))
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(title_widget, widget_chunks[0]);

    // Render high-resolution chart if we have data
    if !history.is_empty() && widget_chunks[1].height >= 5 {
        render_high_resolution_chart(f, widget_chunks[1], timestamps, history, color, max_val, name);
    } else {
        // Show status message with debug info for connections
        let status_msg = if history.is_empty() { 
            if name == "DB Connections" {
                format!("Loading data... ({})", history.len())
            } else {
                "Loading data...".to_string()
            }
        } else { 
            "Area too small for chart".to_string() 
        };
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
    _max_val: f64,
    metric_name: &str
) {
    // Create time-based X-axis that matches actual data periods
    let (time_bounds, data_points) = if !history.is_empty() && !timestamps.is_empty() {
        use chrono::{DateTime, Utc};
        
        // Convert timestamps to minutes since start for proper time scaling
        let start_time: DateTime<Utc> = timestamps[0].into();
        
        // Create data points with all available data for maximum density
        let mut points: Vec<(f64, f64)> = timestamps
            .iter()
            .zip(history.iter())
            .map(|(timestamp, &value)| {
                let dt: DateTime<Utc> = (*timestamp).into();
                let minutes_since_start = (dt - start_time).num_minutes() as f64;
                (minutes_since_start, value)
            })
            .collect();
        
        // Add interpolated points for better density if we have sparse data
        if points.len() >= 2 && points.len() < 20 {
            let mut interpolated_points = Vec::new();
            for i in 0..points.len() - 1 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[i + 1];
                interpolated_points.push((x1, y1));
                
                // Add intermediate points if the gap is large
                let time_gap = x2 - x1;
                if time_gap > 10.0 { // More than 10 minutes gap
                    let num_intermediate = ((time_gap / 5.0) as usize).min(3); // Max 3 intermediate points
                    for j in 1..=num_intermediate {
                        let ratio = j as f64 / (num_intermediate + 1) as f64;
                        let x_interp = x1 + ratio * (x2 - x1);
                        let y_interp = y1 + ratio * (y2 - y1); // Linear interpolation
                        interpolated_points.push((x_interp, y_interp));
                    }
                }
            }
            interpolated_points.push(points[points.len() - 1]);
            points = interpolated_points;
        }
        
        // Calculate time bounds based on actual data range
        let max_minutes = if points.len() > 1 {
            points.last().unwrap().0
        } else {
            180.0 // 3 hours fallback
        };
        
        // Ensure we have a reasonable time range even for single data points
        let time_range = if max_minutes <= 0.0 { 180.0 } else { max_minutes };
        
        ((0.0, time_range), points)
    } else {
        // Fallback with sample data points for better visualization when no real data
        let sample_points = (0..=36).map(|i| {
            let x = (i as f64) * 5.0; // Every 5 minutes over 3 hours
            let y = 0.0;
            (x, y)
        }).collect();
        ((0.0, 180.0), sample_points)
    };
    
    // Dynamic Y-axis bounds based on actual data values with intelligent scaling
    let (chart_min, chart_max) = if !history.is_empty() {
        let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
        
        if min_val.is_finite() && max_val.is_finite() {
            if min_val == max_val {
                // Handle constant values by creating a reasonable range around the value
                let center = min_val;
                let margin = if center.abs() > 1.0 { center.abs() * 0.1 } else { 1.0 };
                (center - margin, center + margin)
            } else {
                let range = max_val - min_val;
                let padding = range * 0.1; // 10% padding for better visualization
                
                // Ensure we don't go below zero for metrics that shouldn't be negative
                let chart_min = if min_val >= 0.0 { 
                    (min_val - padding).max(0.0) 
                } else { 
                    min_val - padding 
                };
                let chart_max = max_val + padding;
                
                (chart_min, chart_max)
            }
        } else {
            (0.0, 1.0)
        }
    } else {
        (0.0, 1.0)
    };

    // Ensure valid bounds
    let final_max = if chart_max <= chart_min { chart_min + 1.0 } else { chart_max };

    // Create high-resolution dataset with enhanced markers for better density and visibility  
    let dataset = Dataset::default()
        .name("")  // No legend needed for single metric
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)  // Use Block marker for dense, visible data points
        .style(Style::default().fg(color))
        .data(&data_points);

    // Create enhanced chart with dynamic bounds and labels - no duplicate title
    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray))
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([time_bounds.0, time_bounds.1])
                .labels({
                    // Time labels that match actual data periods
                    if true {
                        use chrono::{DateTime, Utc};
                        
                        let start_time: DateTime<Utc> = timestamps[0].into();
                        let end_time: DateTime<Utc> = timestamps[timestamps.len() - 1].into();
                        let total_minutes = (end_time - start_time).num_minutes() as f64;
                        
                        let num_labels = 5;
                        let mut labels = Vec::new();
                        
                        for i in 0..num_labels {
                            let minutes_offset = (i as f64 / (num_labels - 1) as f64) * total_minutes;
                            let label_time = start_time + chrono::Duration::minutes(minutes_offset as i64);
                            let local_time: DateTime<Local> = label_time.into();
                            
                            labels.push(Line::from(Span::styled(
                                format!("{}", local_time.format("%H:%M")), 
                                Style::default().fg(Color::DarkGray)
                            )));
                        }
                        labels
                    } else {
                        // Fallback with proper time periods
                        let now = Local::now();
                        let start_time = now - Duration::hours(3);
                        
                        vec![
                            Line::from(Span::styled(format!("{}", start_time.format("%H:%M")), Style::default().fg(Color::DarkGray))),
                            Line::from(Span::styled(format!("{}", (start_time + Duration::minutes(45)).format("%H:%M")), Style::default().fg(Color::DarkGray))),
                            Line::from(Span::styled(format!("{}", (start_time + Duration::minutes(90)).format("%H:%M")), Style::default().fg(Color::DarkGray))),
                            Line::from(Span::styled(format!("{}", (start_time + Duration::minutes(135)).format("%H:%M")), Style::default().fg(Color::DarkGray))),
                            Line::from(Span::styled(format!("{}", now.format("%H:%M")), Style::default().fg(Color::DarkGray)))
                        ]
                    }
                })
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([chart_min, final_max])
                .labels({
                    // More Y-axis labels that align with data points
                    let format_value = |v: f64| -> String {
                        // Special handling for memory metrics (convert bytes to GB) - compact
                        if metric_name.contains("Memory") || metric_name.contains("Storage") {
                            let gb_value = v / 1024.0 / 1024.0 / 1024.0;
                            if gb_value >= 10.0 {
                                format!("{:.0}G", gb_value)  // No decimal for large values
                            } else if gb_value >= 1.0 {
                                format!("{:.1}G", gb_value)
                            } else {
                                let mb_value = v / 1024.0 / 1024.0;
                                format!("{:.0}M", mb_value)
                            }
                        } else if metric_name.contains("Swap") {
                            let mb_value = v / 1024.0 / 1024.0;
                            if mb_value >= 1000.0 {
                                format!("{:.0}G", mb_value / 1024.0)
                            } else {
                                format!("{:.0}M", mb_value)
                            }
                        } else if metric_name.contains("Throughput") || metric_name.contains("Network") {
                            let mb_value = v / 1024.0 / 1024.0;
                            if mb_value >= 100.0 {
                                format!("{:.0}M", mb_value)
                            } else {
                                format!("{:.1}M", mb_value)
                            }
                        } else if v.abs() >= 1_000_000.0 {
                            format!("{:.0}M", v / 1_000_000.0)  // No decimal for millions
                        } else if v.abs() >= 10_000.0 {
                            format!("{:.0}K", v / 1_000.0)  // No decimal for 10K+
                        } else if v.abs() >= 1_000.0 {
                            format!("{:.1}K", v / 1_000.0)
                        } else if v.abs() >= 100.0 {
                            format!("{:.0}", v)  // No decimal for 100+
                        } else if v.abs() >= 1.0 {
                            format!("{:.1}", v)
                        } else if v.abs() >= 0.1 {
                            format!("{:.2}", v)
                        } else {
                            format!("{:.3}", v)
                        }
                    };
                    
                    // Enhanced Y-axis with 5 labels that correspond to data ranges
                    let range = final_max - chart_min;
                    let num_y_labels = 5;
                    let mut y_labels = Vec::new();
                    
                    for i in 0..num_y_labels {
                        let value = chart_min + (i as f64 / (num_y_labels - 1) as f64) * range;
                        y_labels.push(Line::from(Span::styled(
                            format_value(value), 
                            Style::default().fg(Color::DarkGray)
                        )));
                    }
                    
                    y_labels
                })
        );

    f.render_widget(chart, area);
}


