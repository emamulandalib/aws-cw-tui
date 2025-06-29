use crate::models::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_service_list(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header - reduced height
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_services(f, chunks[1], app);
    render_controls(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("AWS CloudWatch TUI - Service Selection")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(header, area);
}

fn render_services(f: &mut Frame, area: Rect, app: &mut App) {
    let services: Vec<ListItem> = app
        .available_services
        .iter()
        .map(|service| {
            let content = vec![Line::from(vec![Span::styled(
                service.display_name(),
                Style::default().fg(Color::Green),
            )])];
            ListItem::new(content)
        })
        .collect();

    let services_list = List::new(services)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Available Services")
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");

    f.render_stateful_widget(services_list, area, &mut app.service_list_state);
}

fn render_controls(f: &mut Frame, area: Rect) {
    let controls = Paragraph::new("↑/↓: Navigate • Enter: Select Service • q: Quit")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(controls, area);
}
