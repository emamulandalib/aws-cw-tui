use crate::models::App;
use crate::ui::components::{render_rds_instance_list_item, render_sqs_queue_list_item};
use crate::ui::components::list_styling::{
    themes::instance_list_colors,
    utilities::{create_border_style, create_highlight_style},
};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

pub fn render_rds_list(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header - reduced height
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    render_header(f, chunks[0], app);

    // Check for errors first
    if let Some(error_msg) = &app.error_message {
        render_error_message(f, chunks[1], error_msg);
    } else if app.loading {
        render_loading_message(f, chunks[1], app);
    } else if app.get_current_instances().is_empty() {
        render_no_instances_message(f, chunks[1], app);
    } else {
        render_instances_list(f, chunks[1], app);
    }

    render_controls(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let theme = UnifiedTheme::default();
    let (main_title, border_title) = if let Some(service) = &app.selected_service {
        (
            format!("AWS CloudWatch TUI - {} Instances", service.short_name()),
            format!("{} Instances", service.short_name()),
        )
    } else {
        (
            "AWS CloudWatch TUI - Instances".to_string(),
            "Instances".to_string(),
        )
    };

    let header = Paragraph::new(main_title)
        .style(Style::default().fg(theme.primary))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(border_title)
                .border_style(Style::default().fg(theme.accent)),
        );
    f.render_widget(header, area);
}

fn render_loading_message(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let theme = UnifiedTheme::default();
    let service_name = app
        .selected_service
        .as_ref()
        .map(|s| s.short_name())
        .unwrap_or("Service");
    let title = format!("{} Instances", service_name);

    let loading_text = vec![
        "Loading instances...",
        "",
        "This may take a few moments.",
    ];

    let loading_msg = Paragraph::new(loading_text.join("\n"))
        .style(Style::default().fg(theme.info))
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(theme.border)),
        );
    f.render_widget(loading_msg, area);
}

fn render_no_instances_message(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let theme = UnifiedTheme::default();
    let service_name = app
        .selected_service
        .as_ref()
        .map(|s| s.short_name())
        .unwrap_or("Service");
    let title = format!("{} Instances", service_name);

    let no_instances = Paragraph::new(format!(
        "No {} instances found in this account/region",
        service_name
    ))
    .style(Style::default().fg(theme.muted))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(theme.border)),
    );
    f.render_widget(no_instances, area);
}

fn render_error_message(f: &mut Frame, area: ratatui::layout::Rect, error_msg: &str) {
    let theme = UnifiedTheme::default();
    let error_paragraph = Paragraph::new(error_msg)
        .style(Style::default().fg(theme.error))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .border_style(Style::default().fg(theme.error)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(error_paragraph, area);
}
fn render_instances_list(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let colors = instance_list_colors();
    let is_focused = true; // Assuming focused in this context
    
    // Get dynamic title based on selected service
    let title = if let Some(service) = &app.selected_service {
        format!("{} Instances", service.short_name())
    } else {
        "Instances".to_string()
    };

    // Clone the instances to avoid borrowing issues
    let current_instances = app.get_current_instances().clone();

    // Create items from instances using pure service-specific components
    let selected_index = app.list_state.selected().unwrap_or(0);
    let items: Vec<ListItem> = current_instances
        .iter()
        .enumerate()
        .map(|(index, service_instance)| match service_instance {
            crate::models::ServiceInstance::Rds(instance) => {
                render_rds_instance_list_item(instance, index == selected_index)
            }
            crate::models::ServiceInstance::Sqs(queue) => {
                render_sqs_queue_list_item(queue, index == selected_index)
            }
        })
        .collect();

    let items_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(create_border_style(is_focused, &colors)),
        )
        .style(Style::default().fg(colors.primary))
        .highlight_style(create_highlight_style(&colors));

    f.render_stateful_widget(items_list, area, &mut app.list_state);

    // Add scrollbar if there are more instances than can fit on screen
    if current_instances.len() > (area.height.saturating_sub(2)) as usize {
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(current_instances.len())
            .position(app.list_state.selected().unwrap_or(0));

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

// Removed duplicate create_rds_list_item function - now using pure service-specific component

// Removed duplicate create_sqs_list_item function - now using pure service-specific component

// Removed duplicate get_queue_type_style function - styling handled by pure service-specific component

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect) {
    let theme = UnifiedTheme::default();
    let controls = Paragraph::new(
        "Up/Down: Navigate • Enter: View Details • Esc: Back to Services • r: Refresh • q: Quit",
    )
    .style(Style::default().fg(theme.secondary));
    f.render_widget(controls, area);
}

// Removed duplicate get_status_style function - styling handled by pure service-specific component
