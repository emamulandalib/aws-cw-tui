use crate::models::App;
use crate::ui::components::render_service_selection_list;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
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
        .style(Style::default().white())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().cyan()),
        );
    f.render_widget(header, area);
}

fn render_services(f: &mut Frame, area: Rect, app: &mut App) {
    // Use pure service selector component for better formatting and functionality
    render_service_selection_list(
        f,
        area,
        &app.available_services,
        &mut app.service_list_state,
        true, // Always focused in service list view
    );
}

fn render_controls(f: &mut Frame, area: Rect) {
    let controls = Paragraph::new("Up/Down: Navigate • Enter: Select Service • q: Quit")
        .style(Style::default().gray());
    f.render_widget(controls, area);
}
