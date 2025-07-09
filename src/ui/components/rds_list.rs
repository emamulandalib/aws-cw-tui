use crate::models::{App, RdsInstance, SqsQueue};
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
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
        .style(Style::default().white())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(border_title)
                .border_style(Style::default().cyan()),
        );
    f.render_widget(header, area);
}

fn render_loading_message(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let service_name = app
        .selected_service
        .as_ref()
        .map(|s| s.short_name())
        .unwrap_or("Service");

    let loading_text = [
        format!("Loading {} instances...", service_name),
        "".to_string(),
        "Press 'q' to quit or 'Esc' to go back".to_string(),
        "Loading will timeout after 30 seconds".to_string(),
    ];

    let loading_msg = Paragraph::new(loading_text.join(
        "
",
    ))
    .style(Style::default().yellow())
    .alignment(ratatui::layout::Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Status")
            .border_style(Style::default().white()),
    );
    f.render_widget(loading_msg, area);
}

fn render_no_instances_message(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
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
    .style(Style::default().red())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().white()),
    );
    f.render_widget(no_instances, area);
}

fn render_error_message(f: &mut Frame, area: ratatui::layout::Rect, error_msg: &str) {
    let error_paragraph = Paragraph::new(error_msg)
        .style(Style::default().red())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .border_style(Style::default().red()),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(error_paragraph, area);
}
fn render_instances_list(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    // Get dynamic title based on selected service
    let title = if let Some(service) = &app.selected_service {
        format!("{} Instances", service.short_name())
    } else {
        "Instances".to_string()
    };

    // Clone the instances to avoid borrowing issues
    let current_instances = app.get_current_instances().clone();

    // Create items from instances
    let items: Vec<ListItem> = current_instances
        .iter()
        .map(|service_instance| match service_instance {
            crate::models::ServiceInstance::Rds(instance) => create_rds_list_item(instance),
            crate::models::ServiceInstance::Sqs(queue) => create_sqs_list_item(queue),
        })
        .collect();

    let items_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().white()),
        )
        .highlight_style(Style::default().on_dark_gray().bold());

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

fn create_rds_list_item(instance: &RdsInstance) -> ListItem<'_> {
    let lines = vec![Line::from(vec![
        Span::styled(
            instance.identifier.to_string(),
            Style::default().white().bold(),
        ),
        Span::raw(" | "),
        Span::styled(&instance.engine, Style::default().green()),
        Span::raw(" | "),
        Span::styled(&instance.status, get_status_style(&instance.status)),
        Span::raw(" | "),
        Span::styled(&instance.instance_class, Style::default().cyan()),
    ])];
    ListItem::new(lines)
}

fn create_sqs_list_item(queue: &SqsQueue) -> ListItem<'_> {
    // Get queue depth from attributes if available
    let queue_depth = queue
        .attributes
        .get("ApproximateNumberOfMessages")
        .unwrap_or(&"0".to_string())
        .clone();

    // Get message retention period for display
    let retention_period = queue
        .attributes
        .get("MessageRetentionPeriod")
        .and_then(|p| p.parse::<u64>().ok())
        .map(|p| format!("{}d", p / 86400))
        .unwrap_or_else(|| "Unknown".to_string());

    let lines = vec![Line::from(vec![
        Span::styled(queue.name.to_string(), Style::default().white().bold()),
        Span::raw(" | "),
        Span::styled(&queue.queue_type, get_queue_type_style(&queue.queue_type)),
        Span::raw(" | "),
        Span::styled(
            format!("Messages: {}", queue_depth),
            Style::default().cyan(),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Retention: {}", retention_period),
            Style::default().yellow(),
        ),
    ])];
    ListItem::new(lines)
}

fn get_queue_type_style(queue_type: &str) -> Style {
    match queue_type {
        "FIFO" => Style::default().magenta(),
        "Standard" => Style::default().green(),
        _ => Style::default().gray(),
    }
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect) {
    let controls = Paragraph::new(
        "Up/Down: Navigate • Enter: View Details • Esc: Back to Services • r: Refresh • q: Quit",
    )
    .style(Style::default().gray());
    f.render_widget(controls, area);
}

fn get_status_style(status: &str) -> Style {
    match status {
        "available" => Style::default().green(),
        "stopped" => Style::default().red(),
        "starting" | "stopping" => Style::default().yellow(),
        _ => Style::default().gray(),
    }
}
