use crate::models::{App, RdsInstance};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

    render_header(f, chunks[0]);

    // Check for errors first
    if let Some(error_msg) = &app.error_message {
        render_error_message(f, chunks[1], error_msg);
    } else if app.loading {
        render_loading_message(f, chunks[1]);
    } else if app.get_current_instances().is_empty() {
        render_no_instances_message(f, chunks[1]);
    } else {
        render_instances_list(f, chunks[1], app);
    }

    render_controls(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new("AWS CloudWatch TUI - RDS Instances")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("RDS Instances")
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(header, area);
}

fn render_loading_message(f: &mut Frame, area: ratatui::layout::Rect) {
    let loading_msg = Paragraph::new("Loading RDS instances...")
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Status")
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(loading_msg, area);
}

fn render_no_instances_message(f: &mut Frame, area: ratatui::layout::Rect) {
    let no_instances = Paragraph::new("No RDS instances found in this account/region")
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("RDS Instances")
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(no_instances, area);
}

fn render_error_message(f: &mut Frame, area: ratatui::layout::Rect, error_msg: &str) {
    let error_paragraph = Paragraph::new(error_msg)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .border_style(Style::default().fg(Color::Red)),
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
        .map(|service_instance| {
            match service_instance {
                crate::models::ServiceInstance::Rds(instance) => {
                    create_instance_list_item(instance)
                }
                // Future service instances will be handled here
            }
        })
        .collect();

    let items_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");

    f.render_stateful_widget(items_list, area, &mut app.list_state);
}

fn create_instance_list_item(instance: &RdsInstance) -> ListItem {
    let lines = vec![Line::from(vec![
        Span::styled(
            instance.identifier.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(&instance.engine, Style::default().fg(Color::Green)),
        Span::raw(" | "),
        Span::styled(&instance.status, get_status_style(&instance.status)),
        Span::raw(" | "),
        Span::styled(&instance.instance_class, Style::default().fg(Color::Cyan)),
    ])];
    ListItem::new(lines)
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect) {
    let controls = Paragraph::new(
        "↑/↓: Navigate • Enter: View Details • Esc: Back to Services • r: Refresh • q: Quit",
    )
    .style(Style::default().fg(Color::Gray));
    f.render_widget(controls, area);
}

fn get_status_style(status: &str) -> Style {
    match status {
        "available" => Style::default().fg(Color::Green),
        "stopped" => Style::default().fg(Color::Red),
        "starting" | "stopping" => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::Gray),
    }
}
