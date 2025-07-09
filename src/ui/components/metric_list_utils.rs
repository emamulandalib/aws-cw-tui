// 2x2 Grid-based metric list view with scroll functionality
use crate::models::App;
use ratatui::{layout::Rect, Frame};
use ratatui::layout::{Layout, Constraint, Direction};

/// Grid layout configuration for 2x2 metrics view
#[derive(Debug, Clone)]
struct MetricGridLayout {
    rows: usize,
    cols: usize,
}

/// Calculate 2x2 grid layout (fixed 2 columns, 2 rows for 4 metrics per screen)
fn calculate_2x2_grid_layout() -> MetricGridLayout {
    MetricGridLayout { rows: 2, cols: 2 }
}

/// Render the enhanced metric list in 2x2 grid format with scroll functionality
pub fn render_enhanced_metric_list(f: &mut Frame, app: &mut App, area: Rect) {
    use super::{
        metric_utils::{format_value, get_available_metrics_with_history_unified, get_metric_colors},
        sparkline_utils::generate_inline_sparkline,
    };
    use ratatui::{
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    };

    // Determine if this panel is focused
    let is_focused = matches!(
        app.get_focused_panel(),
        crate::models::FocusedPanel::SparklineGrid
    );
    let border_color = if is_focused {
        Color::Green
    } else {
        Color::White
    };
    let title = if is_focused {
        "Metrics [FOCUSED]"
    } else {
        "Metrics"
    };

    let available_metrics = app.get_available_metrics();

    if available_metrics.is_empty() {
        let no_data = Paragraph::new("No metrics available")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(border_color)),
            );
        f.render_widget(no_data, area);
        return;
    }

    // Get metrics with current values and history for enhanced display
    let metrics_with_data = get_available_metrics_with_history_unified(app);

    if metrics_with_data.is_empty() {
        let no_data = Paragraph::new("No metric data available")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(border_color)),
            );
        f.render_widget(no_data, area);
        return;
    }

    let total_metrics = available_metrics.len();
    let selected_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
    let safe_selected_index = selected_index.min(total_metrics.saturating_sub(1));
    
    // Calculate which metrics to show in the current 2x2 grid
    let metrics_per_screen = 4; // 2x2 grid
    let current_page = safe_selected_index / metrics_per_screen;
    let start_index = current_page * metrics_per_screen;
    let end_index = (start_index + metrics_per_screen).min(total_metrics);
    
    // Get the metrics to display in current screen
    let visible_metrics = &available_metrics[start_index..end_index];
    
    // Create the main container block
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            "{} (Page {}/{} - {}/{})",
            title,
            current_page + 1,
            (total_metrics + metrics_per_screen - 1) / metrics_per_screen,
            safe_selected_index + 1,
            total_metrics
        ))
        .border_style(Style::default().fg(border_color));

    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

    // Render the 2x2 grid of metrics
    render_2x2_metrics_grid(f, inner_area, app, visible_metrics, start_index, safe_selected_index, &metrics_with_data);
    
    // Add scrollbar if there are more metrics than can fit on screen
    if total_metrics > metrics_per_screen {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight);
        
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_metrics)
            .position(safe_selected_index);
            
        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

/// Render metrics in 2x2 grid layout
fn render_2x2_metrics_grid(
    f: &mut Frame,
    area: Rect,
    app: &App,
    visible_metrics: &[crate::models::MetricType],
    start_index: usize,
    selected_index: usize,
    metrics_with_data: &[(&str, f64, &Vec<f64>, &str)],
) {
    use super::metric_utils::{format_value, get_metric_colors};
    use super::sparkline_utils::generate_inline_sparkline;
    use ratatui::{
        style::{Color, Style, Modifier},
        widgets::{Block, Borders, Paragraph},
        text::{Line, Span},
    };

    let grid_layout = calculate_2x2_grid_layout();
    
    // Calculate row height - divide available space by number of rows
    let min_row_height = 6u16;
    let row_height = (area.height / grid_layout.rows as u16).max(min_row_height);
    
    // Create row constraints
    let row_constraints: Vec<Constraint> = (0..grid_layout.rows)
        .map(|_| Constraint::Length(row_height))
        .collect();

    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    // Render each row
    for (row_idx, row_area) in row_chunks.iter().enumerate() {
        // Create column constraints (2 columns)
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
            let global_metric_idx = start_index + metric_idx;
            
            if let Some(metric_type) = visible_metrics.get(metric_idx) {
                let is_selected = global_metric_idx == selected_index;
                render_single_metric_card(
                    f,
                    *col_area,
                    metric_type,
                    is_selected,
                    metrics_with_data,
                    app,
                );
            }
        }
    }
}

/// Render a single metric card in the grid
fn render_single_metric_card(
    f: &mut Frame,
    area: Rect,
    metric_type: &crate::models::MetricType,
    is_selected: bool,
    metrics_with_data: &[(&str, f64, &Vec<f64>, &str)],
    _app: &App,
) {
    use super::metric_utils::{format_value, get_metric_colors};
    use super::sparkline_utils::generate_inline_sparkline;
    use ratatui::{
        style::{Color, Style, Modifier},
        widgets::{Block, Borders, Paragraph},
        text::{Line, Span},
    };

    let metric_name = metric_type.display_name();
    let empty_history = Vec::new();
    
    // Find corresponding data for this metric
    let metric_data = metrics_with_data
        .iter()
        .find(|(name, _, _, _)| *name == metric_name);

    let (current_value, history, unit) = match metric_data {
        Some((_, value, history, unit)) => (*value, *history, *unit),
        None => (0.0, &empty_history, ""),
    };

    // Generate sparkline for the available width
    let sparkline_width = area.width.saturating_sub(4) as usize; // Account for borders
    let sparkline = generate_inline_sparkline(history, sparkline_width);

    // Format the value with proper styling
    let formatted_value = format_value(current_value, unit);
    let (value_color, sparkline_color) = get_metric_colors(metric_name, current_value);

    // Create the metric card content
    let border_color = if is_selected { Color::Yellow } else { Color::White };
    let bg_color = if is_selected { Color::DarkGray } else { Color::Reset };
    
    let content = vec![
        Line::from(vec![
            Span::styled(
                format!("{}", metric_name),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
                    .bg(bg_color),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                sparkline,
                Style::default()
                    .fg(sparkline_color)
                    .bg(bg_color),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                formatted_value,
                Style::default()
                    .fg(value_color)
                    .add_modifier(Modifier::BOLD)
                    .bg(bg_color),
            ),
        ]),
    ];

    let metric_widget = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
        )
        .style(Style::default().bg(bg_color));

    f.render_widget(metric_widget, area);
}
