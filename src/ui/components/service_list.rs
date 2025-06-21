use crate::models::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_service_list(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Service list
            Constraint::Length(3), // Controls
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_services(f, chunks[1], app);
    render_controls(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("AWS CloudWatch TUI - Service Selection")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("AWS Services"));
    f.render_widget(header, area);
}

fn render_services(f: &mut Frame, area: Rect, app: &mut App) {
    let services: Vec<ListItem> = app
        .available_services
        .iter()
        .map(|service| {
            let content = vec![Line::from(vec![
                Span::styled(
                    service.short_name(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - "),
                Span::raw(service.display_name()),
            ])];
            ListItem::new(content)
        })
        .collect();

    let services_list = List::new(services)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Available Services"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(services_list, area, &mut app.service_list_state);
}

fn render_controls(f: &mut Frame, area: Rect) {
    let controls = Paragraph::new("↑↓: Navigate | Enter: Select Service | q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls, area);
}
