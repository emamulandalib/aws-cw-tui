use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::models::{App, RdsInstance};

pub fn render_rds_list(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    render_header(f, chunks[0]);

    if app.loading {
        render_loading_message(f, chunks[1]);
        return;
    }

    if app.rds_instances.is_empty() {
        render_no_instances_message(f, chunks[1]);
        return;
    }

    render_instances_list(f, chunks[1], app);
}

fn render_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new("AWS CloudWatch TUI - RDS Instances")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, area);
}

fn render_loading_message(f: &mut Frame, area: ratatui::layout::Rect) {
    let loading_msg = Paragraph::new("Loading RDS instances...")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(loading_msg, area);
}

fn render_no_instances_message(f: &mut Frame, area: ratatui::layout::Rect) {
    let no_instances = Paragraph::new("No RDS instances found in this account/region")
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL).title("RDS Instances"));
    f.render_widget(no_instances, area);
}

fn render_instances_list(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .rds_instances
        .iter()
        .map(|instance| create_instance_list_item(instance))
        .collect();

    let items = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("RDS Instances (↑/↓ to navigate, Enter to view details, q to quit)"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(items, area, &mut app.list_state);
}

fn create_instance_list_item(instance: &RdsInstance) -> ListItem {
    let lines = vec![Line::from(vec![
        Span::styled(
            format!("{}", instance.identifier),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            &instance.engine,
            Style::default().fg(Color::Green),
        ),
        Span::raw(" | "),
        Span::styled(
            &instance.status,
            get_status_style(&instance.status),
        ),
        Span::raw(" | "),
        Span::styled(
            &instance.instance_class,
            Style::default().fg(Color::Cyan),
        ),
    ])];
    ListItem::new(lines)
}

fn get_status_style(status: &str) -> Style {
    match status {
        "available" => Style::default().fg(Color::Green),
        "stopped" => Style::default().fg(Color::Red),
        "starting" | "stopping" => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::Gray),
    }
}
