use super::display_utils::get_selected_time_range_display;
use crate::models::{App, FocusedPanel, TimeRangeMode};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

/// Render period selection panel (like AWS Console)
pub fn render_period_selection_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let period_options = app.get_period_options();
    let current_selection = app.get_current_period_index();

    let mut items = Vec::new();

    // Create simple list items like time range design
    for (i, (label, _seconds)) in period_options.iter().enumerate() {
        let is_selected = i == current_selection;
        let style = if is_selected {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Simple text without icons - just color difference
        let display_text = format!("  {}", label);

        items.push(ListItem::new(Line::from(Span::styled(display_text, style))));
    }

    let is_focused = matches!(app.get_focused_panel(), FocusedPanel::Period);

    let border_color = if is_focused {
        Color::Green
    } else {
        Color::White
    };

    // Get current period for title
    let selected_period = period_options
        .get(current_selection)
        .map(|(label, _)| *label)
        .unwrap_or("Unknown");

    let title = format!("Period ({})", selected_period);

    let list = List::new(items)
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
        );

    // Use the app's built-in ListState for proper selection display and scrolling
    f.render_stateful_widget(list, area, &mut app.period_list_state);

    // Add scrollbar if there are more period options than can fit on screen
    if period_options.len() > (area.height.saturating_sub(2)) as usize {
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(period_options.len())
            .position(current_selection);

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

/// Create time range panel with compact absolute/relative toggle
pub fn render_aws_console_time_range_panel(f: &mut Frame, app: &mut App, area: Rect) {
    // Render time range content directly (removed mode indicator)
    render_time_range_content(f, app, area);
}

/// Render the time range content based on current mode
fn render_time_range_content(f: &mut Frame, app: &mut App, area: Rect) {
    match app.get_time_range_mode() {
        TimeRangeMode::Relative => {
            render_relative_time_ranges(f, app, area);
        }
        TimeRangeMode::Absolute => {
            render_absolute_time_picker(f, app, area);
        }
    }
}

/// Render relative time ranges in simple vertical list style (like original)
fn render_relative_time_ranges(f: &mut Frame, app: &mut App, area: Rect) {
    let time_ranges = crate::models::App::get_time_range_options();
    let current_selection = app.get_current_time_range_index();

    let mut items = Vec::new();

    // Create simple list items like the original design
    for (i, (label, _value, _unit, _period)) in time_ranges.iter().enumerate() {
        let is_selected = i == current_selection;
        let style = if is_selected {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Simple text without icons - just color difference
        let display_text = format!("  {}", label);

        items.push(ListItem::new(Line::from(Span::styled(display_text, style))));
    }

    let is_focused = matches!(
        app.get_focused_panel(),
        crate::models::FocusedPanel::TimeRanges
    );

    let border_color = if is_focused {
        Color::Green
    } else {
        Color::White
    };

    // Get current time range for title
    let selected_time_period = time_ranges
        .get(current_selection)
        .map(|(label, _, _, _)| *label)
        .unwrap_or("Unknown");

    let title = format!(
        "Time ({})",
        get_selected_time_range_display(selected_time_period)
    );

    let list = List::new(items)
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
        );

    // Use the app's built-in ListState for proper selection display and scrolling
    f.render_stateful_widget(list, area, &mut app.time_range_list_state);

    // Add scrollbar if there are more time ranges than can fit on screen
    if time_ranges.len() > (area.height.saturating_sub(2)) as usize {
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(time_ranges.len())
            .position(current_selection);

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

/// Render absolute time picker (placeholder for now)
fn render_absolute_time_picker(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = matches!(
        app.get_focused_panel(),
        crate::models::FocusedPanel::TimeRanges
    );

    let border_color = if is_focused {
        Color::Green
    } else {
        Color::White
    };

    let title = "Absolute Time Range";

    let placeholder = Paragraph::new(
        "Absolute time range selection\n\nFeatures coming soon:\n• Custom date/time picker\n• Start/end time selection\n• UTC/Local timezone support\n\nPress 't' to switch to Relative mode"
    )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(placeholder, area);
}

/// Render timezone selection panel
pub fn render_timezone_selection_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let timezone_options = App::get_timezone_options();
    let current_selection = app.get_current_timezone_index();

    let mut items = Vec::new();

    // Create simple list items like period design
    for (i, timezone) in timezone_options.iter().enumerate() {
        let is_selected = i == current_selection;
        let style = if is_selected {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Simple text without icons - just color difference
        let final_text = format!("  {}", timezone.display_name());

        items.push(ListItem::new(Line::from(Span::styled(final_text, style))));
    }

    let is_focused = matches!(app.get_focused_panel(), FocusedPanel::Timezone);

    let border_color = if is_focused {
        Color::Green
    } else {
        Color::White
    };

    // Get current timezone for title
    let selected_timezone = app.get_current_timezone();

    let title = format!("Timezone ({})", selected_timezone.display_name());

    let list = List::new(items)
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
        );

    // Use the app's built-in ListState for proper selection display and scrolling
    f.render_stateful_widget(list, area, &mut app.timezone_list_state);

    // Add scrollbar if there are more timezone options than can fit on screen
    if timezone_options.len() > (area.height.saturating_sub(2)) as usize {
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(timezone_options.len())
            .position(current_selection);

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}
