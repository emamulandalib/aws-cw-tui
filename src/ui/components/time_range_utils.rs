use super::display_utils::get_selected_time_range_display;
use crate::models::{App, FocusedPanel, TimeRangeMode};
use crate::ui::components::list_styling::{
    border_factory::create_theme_border_style,
    themes::time_range_colors_with_theme,
    utilities::{create_highlight_style, create_simple_list_item},
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, List, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

/// Render period selection panel with enhanced styling
pub fn render_period_selection_panel(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    theme: &UnifiedTheme,
) {
    let period_options = app.get_period_options();
    let current_selection = app.get_current_period_index();
    let colors = time_range_colors_with_theme(theme);

    let mut items = Vec::new();

    // Create list items with consistent formatting
    for (i, (label, _seconds)) in period_options.iter().enumerate() {
        let is_selected = i == current_selection;

        let item = create_simple_list_item(
            label,
            Some(&format!("{}s", _seconds)),
            is_selected,
            matches!(app.get_focused_panel(), FocusedPanel::Period),
            &colors,
        );

        items.push(item);
    }

    let is_focused = matches!(app.get_focused_panel(), FocusedPanel::Period);

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
                .border_style(create_theme_border_style(theme, is_focused)),
        )
        .highlight_style(create_highlight_style(&colors));

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
pub fn render_aws_console_time_range_panel(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    theme: &UnifiedTheme,
) {
    // Render time range content directly (removed mode indicator)
    render_time_range_content(f, app, area, theme);
}

/// Render the time range content based on current mode
fn render_time_range_content(f: &mut Frame, app: &mut App, area: Rect, theme: &UnifiedTheme) {
    match app.get_time_range_mode() {
        TimeRangeMode::Relative => {
            render_relative_time_ranges(f, app, area, theme);
        }
        TimeRangeMode::Absolute => {
            render_absolute_time_picker(f, app, area, theme);
        }
    }
}

/// Render relative time ranges with enhanced styling
fn render_relative_time_ranges(f: &mut Frame, app: &mut App, area: Rect, theme: &UnifiedTheme) {
    let time_ranges = crate::models::App::get_time_range_options();
    let current_selection = app.get_current_time_range_index();
    let colors = time_range_colors_with_theme(theme);

    let mut items = Vec::new();

    // Create list items with consistent formatting
    for (i, (_label, value, unit, _period)) in time_ranges.iter().enumerate() {
        let is_selected = i == current_selection;

        let item = create_simple_list_item(
            &format!("{} {:?}", value, unit),
            None,
            is_selected,
            matches!(
                app.get_focused_panel(),
                crate::models::FocusedPanel::TimeRanges
            ),
            &colors,
        );

        items.push(item);
    }

    let is_focused = matches!(
        app.get_focused_panel(),
        crate::models::FocusedPanel::TimeRanges
    );

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
                .border_style(create_theme_border_style(theme, is_focused)),
        )
        .highlight_style(create_highlight_style(&colors));

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
fn render_absolute_time_picker(f: &mut Frame, app: &mut App, area: Rect, theme: &UnifiedTheme) {
    let is_focused = matches!(
        app.get_focused_panel(),
        crate::models::FocusedPanel::TimeRanges
    );

    let title = "Absolute Time Range";

    let placeholder = Paragraph::new(
        "Absolute time range selection\n\nFeatures coming soon:\n• Custom date/time picker\n• Start/end time selection\n• UTC/Local timezone support\n\nPress 't' to switch to Relative mode"
    )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(create_theme_border_style(theme, is_focused)),
        )
        .style(Style::default().fg(theme.secondary))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(placeholder, area);
}

/// Render timezone selection panel
pub fn render_timezone_selection_panel(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    theme: &UnifiedTheme,
) {
    let timezone_options = App::get_timezone_options();
    let current_selection = app.get_current_timezone_index();
    let colors = time_range_colors_with_theme(theme);

    let mut items = Vec::new();

    // Create list items with consistent formatting
    for (i, tz) in timezone_options.iter().enumerate() {
        let is_selected = i == current_selection;

        let item = create_simple_list_item(
            &tz.display_name(),
            None,
            is_selected,
            matches!(app.get_focused_panel(), FocusedPanel::Timezone),
            &colors,
        );

        items.push(item);
    }

    let is_focused = matches!(app.get_focused_panel(), FocusedPanel::Timezone);

    // Get current timezone for title
    let selected_timezone = app.get_current_timezone();

    let title = format!("Timezone ({})", selected_timezone.display_name());

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(create_theme_border_style(theme, is_focused)),
        )
        .highlight_style(create_highlight_style(&colors));

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
