use crate::models::RdsInstance;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, ListItem, Paragraph},
    Frame,
};

/// Render RDS instance details
pub fn render_rds_instance_details(
    f: &mut Frame,
    area: Rect,
    instance: &RdsInstance,
    is_focused: bool,
) {
    let border_color = if is_focused {
        Color::Yellow
    } else {
        Color::White
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title("RDS Instance Details")
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    // Instance ID
    let id_widget = Paragraph::new(format!("ID: {}", instance.identifier))
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left);
    f.render_widget(id_widget, chunks[0]);

    // Engine
    let engine_widget = Paragraph::new(format!("Engine: {}", instance.engine))
        .style(Style::default().fg(Color::Blue))
        .alignment(Alignment::Left);
    f.render_widget(engine_widget, chunks[1]);

    // Status
    let status_color = match instance.status.as_str() {
        "available" => Color::Green,
        "stopped" => Color::Red,
        "starting" | "stopping" => Color::Yellow,
        _ => Color::Gray,
    };
    let status_widget = Paragraph::new(format!("Status: {}", instance.status))
        .style(Style::default().fg(status_color))
        .alignment(Alignment::Left);
    f.render_widget(status_widget, chunks[2]);

    // Instance Class
    let class_widget = Paragraph::new(format!("Class: {}", instance.instance_class))
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left);
    f.render_widget(class_widget, chunks[3]);

    // Endpoint (if available)
    if let Some(endpoint) = &instance.endpoint {
        let endpoint_widget = Paragraph::new(format!("Endpoint: {}", endpoint))
            .style(Style::default().fg(Color::Magenta))
            .alignment(Alignment::Left);
        f.render_widget(endpoint_widget, chunks[4]);
    }
}

/// Render RDS instance list item
pub fn render_rds_instance_list_item(instance: &RdsInstance, is_selected: bool) -> ListItem {
    let status_indicator = match instance.status.as_str() {
        "available" => "●",
        "stopped" => "○",
        "starting" | "stopping" => "◐",
        _ => "?",
    };

    let status_color = match instance.status.as_str() {
        "available" => Color::Green,
        "stopped" => Color::Red,
        "starting" | "stopping" => Color::Yellow,
        _ => Color::Gray,
    };

    let content = format!(
        "{} {} [{}] {}",
        status_indicator, instance.identifier, instance.engine, instance.instance_class
    );

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(status_color)
    };

    ListItem::new(content).style(style)
}
