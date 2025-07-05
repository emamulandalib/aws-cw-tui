// All imports are local to the render_enhanced_metric_list function
use crate::models::App;
use ratatui::{layout::Rect, Frame};

/// Calculate the layout parameters for the metric list
///
/// Create metric list items for display
///
/// Create and configure the metric list state
///
/// Render empty state for metric list
///
/// Create the final metric list widget
///
/// Render the enhanced metric list with full functionality
pub fn render_enhanced_metric_list(f: &mut Frame, app: &mut App, area: Rect) {
    use super::{
        metric_utils::{format_value, get_available_metrics_with_history_unified, get_metric_colors},
        sparkline_utils::generate_inline_sparkline,
        visual_utils::{create_metric_block, MetricBlockParams},
    };
    use ratatui::{
        style::{Color, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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

    // Calculate items that can fit on screen for scrolling
    let items_per_screen = (area.height.saturating_sub(2)) as usize; // Account for borders
    let total_items = available_metrics.len();
    let selected_index = app.get_sparkline_grid_selected_index();

    // Each metric is now only 1 line (not 3 lines like before)
    let actual_metrics_per_screen = items_per_screen;

    // Use the app's scroll offset directly
    let scroll_offset = app.scroll_offset;

    // Calculate responsive widths to fill the terminal width
    // Don't subtract borders here since visual_utils will handle the final layout
    let total_width = area.width as usize; // Use full area width for calculation
    let border_and_padding = 6; // Account for borders (2) and padding (4) in visual_utils
    let available_width = total_width.saturating_sub(border_and_padding);
    
    let value_width = 12; // Fixed width for values
    let separators_width = 4; // Space for separators between columns
    let name_width = (available_width * 30 / 100).clamp(18, 30); // 30% of available width for names
    let sparkline_width = available_width
        .saturating_sub(name_width + value_width + separators_width)
        .max(15); // Rest for sparkline, minimum 15 chars

    // Create enhanced metric blocks with distinct visual separation and spacing
    let empty_history = Vec::new();
    let mut items: Vec<ListItem> = Vec::new();
    let mut metric_positions: Vec<usize> = Vec::new(); // Track which positions contain actual metrics

    for (original_index, metric_type) in available_metrics
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(actual_metrics_per_screen)
    {
        let is_selected = original_index == selected_index;

        // Track the position of this metric in the items list (single line per metric)
        let metric_position = items.len(); // Current position in the items list
        metric_positions.push(metric_position);

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

        // Create compact single-line metric item 
        let content_lines = create_metric_block(MetricBlockParams {
            metric_name: metric_name.to_string(),
            sparkline,
            formatted_value,
            is_selected,
            value_color,
            sparkline_color,
            name_width,
            sparkline_width,
        });

        // Add the metric line as a list item (now single line per metric)
        for line in content_lines {
            items.push(ListItem::new(line));
        }
    }

    // Create list state for navigation and scrolling
    let mut list_state = ratatui::widgets::ListState::default();
    let has_items = !items.is_empty();
    if has_items
        && selected_index >= scroll_offset
        && selected_index < scroll_offset + actual_metrics_per_screen
    {
        // Find the position of the selected metric in our items list
        let relative_index = selected_index - scroll_offset;
        if let Some(&position) = metric_positions.get(relative_index) {
            list_state.select(Some(position));
        }
    }

    // Create the list widget with enhanced styling
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "{} ({}/{})",
                    title,
                    selected_index.saturating_add(1).min(total_items),
                    total_items
                ))
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(Style::default()) // Remove highlight since we handle it manually
        .highlight_symbol(""); // Remove default highlight symbol

    // Render the list with scrolling support
    f.render_stateful_widget(list, area, &mut list_state);

    // Scroll indicator removed per user request
}
