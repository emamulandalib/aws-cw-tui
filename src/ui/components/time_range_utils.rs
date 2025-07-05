use super::display_utils::get_selected_time_range_display;
use crate::models::{App, TimeRangeMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
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
                
                // Add selection indicator and padding
                let display_text = if is_selected {
                    format!("  ● {}", formatted_label)
                } else {
                    format!("    {}", formatted_label)
                };
                
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
pub fn get_time_range_title(app: &App, is_focused: bool) -> String {
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

    if is_focused {
        format!(
            "CloudWatch Dashboard - {} | {} [F]",
            mode_text,
            get_selected_time_range_display(selected_time_period)
        )
    } else {
        format!(
            "CloudWatch Dashboard - {} | {}",
            mode_text,
            get_selected_time_range_display(selected_time_period)
        )
    }
}

/// Create AWS Console-style time range panel with tabs
pub fn render_aws_console_time_range_panel(f: &mut Frame, app: &mut App, area: Rect) {
    // Split area for tabs and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
        ])
        .split(area);
    
    // Render absolute/relative tabs
    let tab_titles = vec!["Absolute", "Relative"];
    let selected_tab = match app.get_time_range_mode() {
        TimeRangeMode::Absolute => 0,
        TimeRangeMode::Relative => 1,
    };
    
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("CloudWatch Dashboard"))
        .select(selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
    
    f.render_widget(tabs, chunks[0]);
    
    // Render time range content
    render_time_range_content(f, app, chunks[1]);
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

/// Render relative time ranges in AWS Console style
fn render_relative_time_ranges(f: &mut Frame, app: &mut App, area: Rect) {
    // Create the layout with categories
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(16), // Minutes
            Constraint::Percentage(16), // Hours
            Constraint::Percentage(16), // Days
            Constraint::Percentage(16), // Weeks
            Constraint::Percentage(16), // Months
            Constraint::Percentage(20), // Space
        ])
        .split(area);
    
    // Time range categories
    let categories = [
        ("Minutes", vec!["1", "3", "5", "15", "30", "45"], 0..6),
        ("Hours", vec!["1", "2", "3", "6", "8", "12"], 6..12),
        ("Days", vec!["1", "2", "3", "4", "5", "6"], 12..18),
        ("Weeks", vec!["1", "2", "4", "6"], 18..22),
        ("Months", vec!["3", "6", "12", "15"], 22..26),
    ];
    
    let current_selection = app.get_current_time_range_index();
    
    for (idx, (category, options, range)) in categories.iter().enumerate() {
        let mut items = Vec::new();
        
        for (i, option) in options.iter().enumerate() {
            let actual_index = range.start + i;
            let is_selected = actual_index == current_selection;
            
            let style = if is_selected {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            let text = if is_selected {
                format!("● {}", option)
            } else {
                format!("  {}", option)
            };
            
            items.push(ListItem::new(Line::from(Span::styled(text, style))));
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
        
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(category.to_string())
                    .border_style(Style::default().fg(border_color)),
            );
        
        f.render_widget(list, chunks[idx]);
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
    
    let placeholder = Paragraph::new("Absolute time range selection\n(Coming soon...)")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Absolute Time Range")
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::Yellow));
    
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
