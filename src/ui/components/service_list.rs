use crate::models::App;
use crate::ui::components::{render_service_selection_list, UniversalBox};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_service_list(f: &mut Frame, app: &mut App, theme: &UnifiedTheme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    render_services(f, chunks[0], app, theme);
    render_controls(f, chunks[1], theme);
}

fn render_services(f: &mut Frame, area: Rect, app: &mut App, theme: &UnifiedTheme) {
    // Use pure service selector component for better formatting and functionality
    render_service_selection_list(
        f,
        area,
        &app.available_services,
        &mut app.service_list_state,
        true, // Always focused in service list view
        theme,
    );
}

fn render_controls(f: &mut Frame, area: Rect, theme: &UnifiedTheme) {
    UniversalBox::new(theme.clone())
        .text_styled(
            "Up/Down: Navigate • Enter: Select Service • t: Change Theme • q: Quit",
            Style::default().fg(theme.secondary),
        )
        .borders(Borders::NONE)
        .render(f, area);
}
