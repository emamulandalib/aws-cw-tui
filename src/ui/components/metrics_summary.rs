use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::models::{App, MetricData, MetricType};

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

    // Two-panel layout: Time ranges (left), Full-height metric list (right)
    let time_panel_width = calculate_time_panel_width(chunks[1].width);
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(time_panel_width), // Compact Time Panel (responsive width)
            Constraint::Min(0),                   // Right panel for full-height metric list
        ])
        .split(chunks[1]);

    // Compact Time Range Panel
    render_compact_time_ranges(f, app, content_chunks[0]);

    // Full-height Metric List Panel
    render_enhanced_metric_list(f, app, content_chunks[1]);

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
    
    // Use the same logic as MetricData.get_available_metrics() for consistency
    let available_metric_types = metrics.get_available_metrics();
    
    for metric_type in available_metric_types {
        let metric_name = metric_type.display_name();
        let history = metrics.get_metric_history(&metric_type);
        let unit = get_metric_unit(&metric_type);
        
        // Get current value based on metric type
        let current_value = match metric_type {
            MetricType::CpuUtilization => metrics.cpu_utilization,
            MetricType::DatabaseConnections => metrics.database_connections,
            MetricType::FreeStorageSpace => metrics.free_storage_space,
            MetricType::ReadIops => metrics.read_iops,
            MetricType::WriteIops => metrics.write_iops,
            MetricType::ReadLatency => metrics.read_latency,
            MetricType::WriteLatency => metrics.write_latency,
            MetricType::ReadThroughput => metrics.read_throughput,
            MetricType::WriteThroughput => metrics.write_throughput,
            MetricType::NetworkReceiveThroughput => metrics.network_receive_throughput,
            MetricType::NetworkTransmitThroughput => metrics.network_transmit_throughput,
            MetricType::FreeableMemory => metrics.freeable_memory,
            MetricType::SwapUsage => metrics.swap_usage,
            MetricType::QueueDepth => metrics.queue_depth,
            MetricType::BurstBalance => metrics.burst_balance,
            MetricType::CpuCreditUsage => metrics.cpu_credit_usage,
            MetricType::CpuCreditBalance => metrics.cpu_credit_balance,
            MetricType::BinLogDiskUsage => metrics.bin_log_disk_usage,
            MetricType::ReplicaLag => metrics.replica_lag,
            MetricType::MaximumUsedTransactionIds => metrics.maximum_used_transaction_ids,
            MetricType::OldestReplicationSlotLag => metrics.oldest_replication_slot_lag,
            MetricType::ReplicationSlotDiskUsage => metrics.replication_slot_disk_usage,
            MetricType::TransactionLogsDiskUsage => metrics.transaction_logs_disk_usage,
            MetricType::TransactionLogsGeneration => metrics.transaction_logs_generation,
            MetricType::FailedSqlServerAgentJobsCount => metrics.failed_sql_server_agent_jobs_count,
            MetricType::CheckpointLag => metrics.checkpoint_lag,
            MetricType::ConnectionAttempts => metrics.connection_attempts,
        };
        
        available.push((metric_name, current_value, history, unit));
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
        "↑/↓: Navigate • Tab: Switch Panels (Time/Sparklines) • Enter: Select • r: Refresh • b/Esc: Back • q: Quit")
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
    
    // Get the current selected time period for consistent display
    let time_ranges = crate::models::App::get_time_range_options();
    let current_time_range_index = app.get_current_time_range_index();
    let selected_time_period = time_ranges.get(current_time_range_index)
        .map(|(label, _, _, _)| *label)
        .unwrap_or("Unknown");
    
    let title = if is_focused {
        format!("Time [F] ({})", get_selected_time_range_display(selected_time_period))
    } else {
        format!("Time ({})", get_selected_time_range_display(selected_time_period))
    };

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

fn get_metric_unit(metric_type: &MetricType) -> &'static str {
    match metric_type {
        MetricType::CpuUtilization | MetricType::BurstBalance => "Percent",
        MetricType::DatabaseConnections | MetricType::ReadIops | MetricType::WriteIops |
        MetricType::QueueDepth | MetricType::ConnectionAttempts |
        MetricType::MaximumUsedTransactionIds | MetricType::FailedSqlServerAgentJobsCount => "Count",
        MetricType::ReadLatency | MetricType::WriteLatency | MetricType::ReplicaLag |
        MetricType::CheckpointLag => "Seconds",
        MetricType::FreeStorageSpace | MetricType::FreeableMemory | MetricType::SwapUsage |
        MetricType::BinLogDiskUsage | MetricType::ReplicationSlotDiskUsage |
        MetricType::TransactionLogsDiskUsage | MetricType::OldestReplicationSlotLag => "Bytes",
        MetricType::ReadThroughput | MetricType::WriteThroughput |
        MetricType::NetworkReceiveThroughput | MetricType::NetworkTransmitThroughput |
        MetricType::TransactionLogsGeneration => "Bytes/Second",
        MetricType::CpuCreditUsage | MetricType::CpuCreditBalance => "Credits",
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

fn render_scroll_indicator(f: &mut Frame, area: Rect, current_index: usize, total_items: usize, items_per_screen: usize) {
    if total_items <= items_per_screen {
        return; // No need for scroll indicator
    }

    let indicator_x = area.x + area.width.saturating_sub(1);
    let indicator_height = area.height.saturating_sub(2);
    
    // Calculate scroll position as a ratio
    let scroll_ratio = current_index as f64 / (total_items.saturating_sub(1)) as f64;
    let indicator_y = area.y + 1 + (scroll_ratio * (indicator_height.saturating_sub(1)) as f64) as u16;
    
    // Render scroll indicator
    let indicator_area = Rect {
        x: indicator_x,
        y: indicator_y,
        width: 1,
        height: 1,
    };
    
    let indicator = Paragraph::new("▐")
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(indicator, indicator_area);
}

/// Creates a distinct visual block for each metric item with proper spacing and styling
fn create_metric_block(
    metric_name: &str,
    sparkline: &str,
    formatted_value: &str,
    is_selected: bool,
    value_color: Color,
    sparkline_color: Color,
    name_width: usize,
    sparkline_width: usize,
) -> Line<'static> {
    if is_selected {
        // Selected metric block with enhanced styling
        Line::from(vec![
            Span::styled("▌", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("▶ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:<width$}", truncate_string(metric_name, name_width), width = name_width),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            ),
            Span::styled(" │ ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{:<width$}", sparkline, width = sparkline_width),
                Style::default().fg(sparkline_color).add_modifier(Modifier::BOLD)
            ),
            Span::styled(" │ ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{:>12}", formatted_value),
                Style::default().fg(value_color).add_modifier(Modifier::BOLD)
            ),
            Span::styled(" ▐", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ])
    } else {
        // Regular metric block with subtle styling
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("{:<width$}", truncate_string(metric_name, name_width), width = name_width),
                Style::default().fg(Color::Cyan)
            ),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:<width$}", sparkline, width = sparkline_width),
                Style::default().fg(sparkline_color)
            ),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:>12}", formatted_value),
                Style::default().fg(value_color)
            ),
            Span::styled(" │", Style::default().fg(Color::DarkGray)),
        ])
    }
}

/// Generates an elegant inline sparkline using Braille characters for compact visualization
/// Adapted from the sparkline_chart.rs implementation for single-line use
fn generate_inline_sparkline(history: &Vec<f64>, width: usize) -> String {
    if history.is_empty() || width == 0 {
        return "⠀".repeat(width.max(8));
    }

    if history.len() == 1 {
        // Single data point - show as centered indicator
        let half_width = width / 2;
        let mut sparkline = "⠀".repeat(half_width);
        sparkline.push('⣿');
        sparkline.push_str(&"⠀".repeat(width.saturating_sub(half_width + 1)));
        return sparkline;
    }

    // Calculate Y bounds for normalization
    let (y_min, y_max) = calculate_sparkline_y_bounds(history);
    let y_range = y_max - y_min;
    
    if y_range == 0.0 {
        // All values are the same - show as flat sparkline with indicators
        return generate_flat_sparkline(width);
    }

    // Sample data to fit the available width
    let sampled_data = sample_sparkline_data(history, width);
    
    // Normalize data to 0-1 range for Braille character selection
    let normalized_data: Vec<f64> = sampled_data
        .iter()
        .map(|&value| ((value - y_min) / y_range).clamp(0.0, 1.0))
        .collect();

    // Generate inline sparkline using Braille characters
    generate_braille_inline_sparkline(&normalized_data, width)
}

/// Calculate Y bounds for sparkline normalization
fn calculate_sparkline_y_bounds(history: &Vec<f64>) -> (f64, f64) {
    let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = history.iter().cloned().fold(-f64::INFINITY, f64::max);
    
    if min_val.is_finite() && max_val.is_finite() {
        if min_val == max_val {
            (min_val, max_val)
        } else {
            let range = max_val - min_val;
            let padding = range * 0.05; // Smaller padding for compact display
            (min_val - padding, max_val + padding)
        }
    } else {
        (0.0, 1.0)
    }
}

/// Sample data points to fit the target width for sparklines
fn sample_sparkline_data(history: &Vec<f64>, target_width: usize) -> Vec<f64> {
    if history.len() <= target_width {
        return history.clone();
    }

    let mut sampled = Vec::with_capacity(target_width);
    for i in 0..target_width {
        let idx = if target_width == 1 {
            0
        } else {
            (i * (history.len() - 1)) / (target_width - 1)
        };
        sampled.push(history[idx]);
    }
    sampled
}

/// Generate a flat sparkline for constant values
fn generate_flat_sparkline(width: usize) -> String {
    if width <= 2 {
        return "⣿".repeat(width);
    }
    
    let mut sparkline = String::with_capacity(width);
    sparkline.push('⣿');
    sparkline.push_str(&"⣤".repeat(width.saturating_sub(2)));
    sparkline.push('⣿');
    sparkline
}

/// Generate inline sparkline using Braille characters for elegant visualization
fn generate_braille_inline_sparkline(normalized_data: &Vec<f64>, width: usize) -> String {
    if normalized_data.is_empty() {
        return "⠀".repeat(width);
    }

    let mut sparkline = String::with_capacity(width);
    
    for i in 0..width {
        let char_to_use = if i < normalized_data.len() {
            let current_value = normalized_data[i];
            
            // Map normalized value to Braille character representing vertical position
            match (current_value * 8.0) as usize {
                0 => '⣀', // Bottom level
                1 => '⣄', // Low level
                2 => '⣆', // Low-medium level
                3 => '⣇', // Medium-low level
                4 => '⣧', // Medium level
                5 => '⣷', // Medium-high level
                6 => '⣿', // High level
                7..=8 => '⣿', // Maximum level
                _ => '⣤', // Default medium
            }
        } else {
            '⠀' // Empty space
        };
        
        sparkline.push(char_to_use);
    }
    
    sparkline
}

fn render_enhanced_metric_list(f: &mut Frame, app: &mut App, area: Rect) {
    // Determine if this panel is focused
    let is_focused = matches!(app.get_focused_panel(), crate::models::FocusedPanel::SparklineGrid);
    let border_color = if is_focused { Color::Green } else { Color::White };
    let title = if is_focused { "Metrics [FOCUSED]" } else { "Metrics" };

    let available_metrics = app.get_available_metrics();
    
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

    // Get metrics with current values and history for enhanced display
    let metrics_with_data = get_available_metrics_with_history(&app.metrics);

    if metrics_with_data.is_empty() {
        let no_data = Paragraph::new("No metric data available")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(border_color)));
        f.render_widget(no_data, area);
        return;
    }

    // Calculate items that can fit on screen for scrolling
    let items_per_screen = (area.height.saturating_sub(2)) as usize; // Account for borders
    let total_items = available_metrics.len();
    let selected_index = app.get_sparkline_grid_selected_index();
    
    // Update app's metrics_per_screen for the navigation functions to use
    // Since we have spacing between items, we need to account for that
    let actual_metrics_per_screen = (items_per_screen + 1) / 2; // Each metric takes 2 lines (content + spacing)
    app.metrics_per_screen = actual_metrics_per_screen;
    
    // Use the app's scroll offset directly
    let scroll_offset = app.scroll_offset;

    // Calculate responsive widths to fill the terminal width
    let total_width = area.width.saturating_sub(4) as usize; // Account for borders
    let value_width = 12; // Fixed width for values
    let separators_width = 8; // Space for separators and padding
    let name_width = (total_width * 30 / 100).max(18).min(30); // 30% of width for names
    let sparkline_width = total_width.saturating_sub(name_width + value_width + separators_width).max(20); // Rest for sparkline

    // Create enhanced metric blocks with distinct visual separation and spacing
    let empty_history = Vec::new();
    let mut items: Vec<ListItem> = Vec::new();
    let mut metric_positions: Vec<usize> = Vec::new(); // Track which positions contain actual metrics
    
    for (item_index, (original_index, metric_type)) in available_metrics
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(actual_metrics_per_screen)
        .enumerate()
    {
        let is_selected = original_index == selected_index;
        
        // Track the position of this metric in the items list
        metric_positions.push(items.len());
        
        // Find corresponding data for this metric
        let metric_name = metric_type.display_name();
        let metric_data = metrics_with_data
            .iter()
            .find(|(name, _, _, _)| *name == metric_name);
        
        let (current_value, history, unit) = match metric_data {
            Some((_, value, history, unit)) => (*value, *history, *unit),
            None => (0.0, &empty_history, ""),
        };

        // Generate elegant inline sparkline
        let sparkline = generate_inline_sparkline(history, sparkline_width);
        
        // Format the value with proper styling
        let formatted_value = format_value(current_value, unit);
        let (value_color, sparkline_color) = get_metric_colors(metric_name, current_value);
        
        // Create distinct visual block for each metric
        let content = create_metric_block(
            metric_name,
            &sparkline,
            &formatted_value,
            is_selected,
            value_color,
            sparkline_color,
            name_width,
            sparkline_width,
        );
        
        items.push(ListItem::new(content));
        
        // Add spacing between metrics for better readability (except for the last item)
        // Only add spacing if this is not the last metric in the entire list AND
        // not the last item we're displaying in this view
        let is_last_metric_overall = (scroll_offset + item_index + 1) >= total_items;
        let is_last_item_in_view = item_index >= actual_metrics_per_screen.saturating_sub(1);
        
        if !is_last_metric_overall && !is_last_item_in_view {
            items.push(ListItem::new(Line::from("")));
        }
    }

    // Create list state for navigation and scrolling
    let mut list_state = ratatui::widgets::ListState::default();
    let has_items = !items.is_empty();
    if has_items && selected_index >= scroll_offset && selected_index < scroll_offset + actual_metrics_per_screen {
        // Find the position of the selected metric in our items list
        let relative_index = selected_index - scroll_offset;
        if let Some(&position) = metric_positions.get(relative_index) {
            list_state.select(Some(position));
        }
    }

    // Create the list widget with enhanced styling
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("{} ({}/{})", title, selected_index.saturating_add(1).min(total_items), total_items))
            .border_style(Style::default().fg(border_color)))
        .highlight_style(Style::default()) // Remove highlight since we handle it manually
        .highlight_symbol(""); // Remove default highlight symbol

    // Render the list with scrolling support
    f.render_stateful_widget(list, area, &mut list_state);

    // Render scroll indicator if needed
    if total_items > actual_metrics_per_screen {
        render_scroll_indicator(f, area, selected_index, total_items, actual_metrics_per_screen);
    }
}

fn get_selected_time_range_display(selected_time_period: &str) -> String {
    match selected_time_period {
        "5 minutes" => "5m".to_string(),
        "1 hour" => "1h".to_string(),
        "3 hours" => "3h".to_string(),
        "6 hours" => "6h".to_string(),
        "12 hours" => "12h".to_string(),
        "1 day" => "1d".to_string(),
        "3 days" => "3d".to_string(),
        "1 week" => "1w".to_string(),
        "2 weeks" => "2w".to_string(),
        "1 month" => "1m".to_string(),
        _ => selected_time_period.to_string(), // Fallback to original if no match
    }
}
