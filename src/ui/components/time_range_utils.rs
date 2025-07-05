use super::display_utils::get_selected_time_range_display;
use crate::models::{App, TimeRangeMode, FocusedPanel};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Create AWS Console-style time range list items with categories
pub fn create_aws_console_time_range_items(app: &App) -> Vec<ListItem<'_>> {
    let time_ranges = crate::models::App::get_time_range_options();
    let mut items = Vec::new();
    
    // Add category headers and items
    let categories = [
        ("Minutes", 0..6),
        ("Hours", 6..12),
        ("Days", 12..18),
        ("Weeks", 18..22),
        ("Months", 22..26),
    ];
    
    for (category, range) in categories {
        // Add category header
        items.push(ListItem::new(Line::from(Span::styled(
            category.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        ))));
        
        // Add time range options for this category
        for i in range {
            if let Some((label, _value, _unit, _period)) = time_ranges.get(i) {
                let is_selected = i == app.get_current_time_range_index();
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                
                // Format time range numbers like AWS Console
                let formatted_label = format_time_range_label(label);
                
                // Simple text without icons - just color difference  
                let display_text = format!("    {}", formatted_label);
                
                items.push(ListItem::new(Line::from(Span::styled(display_text, style))));
            }
        }
    }
    
    items
}

/// Format time range labels to match AWS Console style
fn format_time_range_label(label: &str) -> String {
    // Extract number and unit from label
    let parts: Vec<&str> = label.split(' ').collect();
    if parts.len() >= 2 {
        let number = parts[0];
        let unit = parts[1];
        
        // Format like AWS Console (e.g., "1", "3", "5" for numbers)
        match unit {
            "minute" | "minutes" => number.to_string(),
            "hour" | "hours" => number.to_string(),
            "day" | "days" => number.to_string(),
            "week" | "weeks" => number.to_string(),
            "month" | "months" => number.to_string(),
            _ => label.to_string(),
        }
    } else {
        label.to_string()
    }
}

/// Get the title for the time range panel with absolute/relative toggle
pub fn get_time_range_title(app: &App, _is_focused: bool) -> String {
    let mode_text = match app.get_time_range_mode() {
        TimeRangeMode::Absolute => "Absolute",
        TimeRangeMode::Relative => "Relative",
    };
    
    let time_ranges = crate::models::App::get_time_range_options();
    let current_time_range_index = app.get_current_time_range_index();
    let selected_time_period = time_ranges
        .get(current_time_range_index)
        .map(|(label, _, _, _)| *label)
        .unwrap_or("Unknown");

    format!(
        "CloudWatch Dashboard - {} | {}",
        mode_text,
        get_selected_time_range_display(selected_time_period)
    )
}

/// Render period selection panel (like AWS Console)
pub fn render_period_selection_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let period_options = crate::models::App::get_period_options();
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
    
    let is_focused = matches!(
        app.get_focused_panel(),
        FocusedPanel::Period
    );
    
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
        )
        .highlight_symbol("");
    
    // Create list state for proper selection display
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(current_selection));
    
    f.render_stateful_widget(list, area, &mut list_state);
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
    
    let title = format!("Time ({})", get_selected_time_range_display(selected_time_period));
    
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
        )
        .highlight_symbol("");
    
    // Create list state for proper selection display
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(current_selection));
    
    f.render_stateful_widget(list, area, &mut list_state);
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

/// Create compact time range list items with abbreviated labels (backward compatibility)
pub fn create_time_range_items(app: &App) -> Vec<ListItem<'_>> {
    create_aws_console_time_range_items(app)
}

/// Create the time range list widget (backward compatibility)
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

/// Render the complete time range panel (updated to use AWS Console style)
pub fn render_time_range_panel(f: &mut Frame, app: &mut App, area: Rect) {
    render_aws_console_time_range_panel(f, app, area);
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
    
    let is_focused = matches!(
        app.get_focused_panel(),
        FocusedPanel::Timezone
    );
    
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
        )
        .highlight_symbol("");
    
    // Create list state for proper selection display
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(current_selection));
    
    f.render_stateful_widget(list, area, &mut list_state);
}
