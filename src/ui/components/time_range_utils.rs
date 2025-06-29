use super::display_utils::get_selected_time_range_display;
use crate::models::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Create compact time range list items with abbreviated labels
pub fn create_time_range_items(app: &App) -> Vec<ListItem> {
    let time_ranges = crate::models::App::get_time_range_options();

    time_ranges
        .iter()
        .enumerate()
        .map(|(i, &(label, _value, _unit, _period))| {
            let is_selected = i == app.get_current_time_range_index();
            let style = if is_selected {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            // Abbreviate labels for compact display
            let compact_label = get_compact_time_label(label);

            // Add selection indicator
            let display_text = if is_selected {
                format!("● {compact_label}")
            } else {
                format!("  {compact_label}")
            };

            ListItem::new(Line::from(Span::styled(display_text, style)))
        })
        .collect()
}

/// Get compact label for time range display
fn get_compact_time_label(label: &str) -> &str {
    match label {
        "Last 1 Hour" => "1h",
        "Last 3 Hours" => "3h",
        "Last 6 Hours" => "6h",
        "Last 12 Hours" => "12h",
        "Last 24 Hours" => "24h",
        "Last 3 Days" => "3d",
        "Last 7 Days" => "7d",
        _ => label, // Fallback to original if no abbreviation
    }
}

/// Get the title for the time range panel
pub fn get_time_range_title(app: &App, is_focused: bool) -> String {
    let time_ranges = crate::models::App::get_time_range_options();
    let current_time_range_index = app.get_current_time_range_index();
    let selected_time_period = time_ranges
        .get(current_time_range_index)
        .map(|(label, _, _, _)| *label)
        .unwrap_or("Unknown");

    if is_focused {
        format!(
            "Time [F] ({})",
            get_selected_time_range_display(selected_time_period)
        )
    } else {
        format!(
            "Time ({})",
            get_selected_time_range_display(selected_time_period)
        )
    }
}

/// Create the time range list widget
pub fn create_time_range_list(items: Vec<ListItem>, title: String, border_color: Color) -> List {
    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("") // Remove highlight symbol to save space
}

/// Render the complete time range panel
pub fn render_time_range_panel(f: &mut Frame, app: &mut App, area: Rect) {
    // Determine if this panel is focused
    let is_focused = matches!(
        app.get_focused_panel(),
        crate::models::FocusedPanel::TimeRanges
    );
    let border_color = if is_focused {
        Color::Green
    } else {
        Color::White
    };

    // Create time range items
    let items = create_time_range_items(app);

    // Get the title
    let title = get_time_range_title(app, is_focused);

    // Create the list widget
    let time_range_list = create_time_range_list(items, title, border_color);

    // Create a temporary list state for this render
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.get_current_time_range_index()));

    // Render the stateful list widget
    f.render_stateful_widget(time_range_list, area, &mut list_state);
}
