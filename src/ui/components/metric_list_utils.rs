// All imports are local to the render_enhanced_metric_list function
use crate::models::App;
use ratatui::{layout::Rect, Frame};
use ratatui::layout::{Layout, Constraint};

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
        widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
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
    // Use ListState's selected value as the source of truth for selection
    let selected_index = app.sparkline_grid_list_state.selected().unwrap_or(0);

    // Each metric is now only 1 line (not 3 lines like before)
    let actual_metrics_per_screen = items_per_screen;

    // Use ratatui's built-in layout methods with better proportions
    let layout = Layout::horizontal([
        Constraint::Min(25),         // Minimum 25 chars for metric name, can grow
        Constraint::Fill(1),         // Fill remaining space for sparkline
        Constraint::Length(12),      // Fixed width for value
    ])
    .split(area);

    let name_width = layout[0].width as usize;
    let sparkline_width = layout[1].width as usize;
    let _value_width = layout[2].width as usize;

    // Create enhanced metric blocks for ALL metrics (not just visible ones)
    // This is the ratatui way - let ListState handle viewport management
    let empty_history = Vec::new();
    let mut items: Vec<ListItem> = Vec::new();

    for (original_index, metric_type) in available_metrics.iter().enumerate() {
        let is_selected = original_index == selected_index;

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

        // Add the metric line as a list item (single line per metric) 
        // Note: For multi-line items, we only add the first line to maintain proper ListState alignment
        if let Some(first_line) = content_lines.first() {
            items.push(ListItem::new(first_line.clone()));
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
        .highlight_style(Style::default().bg(ratatui::style::Color::DarkGray))
        .highlight_symbol("▶ ");

    // Use the app's actual ListState - this is the key to proper ratatui scrolling!
    f.render_stateful_widget(list, area, &mut app.sparkline_grid_list_state);
    
    // Add scrollbar if there are more metrics than can fit on screen
    if total_items > (area.height.saturating_sub(2)) as usize {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");
        
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_items)
            .position(selected_index);
            
        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}
