use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::models::App;
use super::super::charts::metrics_chart::render_metrics;

pub fn render_instance_details(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header - further reduced height for less padding
            Constraint::Min(0),    // Content (metrics chart will handle its own controls)
        ])
        .split(f.area());

    let instance = &app.rds_instances[app.selected_instance.unwrap()];

    render_instance_info(f, chunks[0], instance);

    if app.metrics_loading {
        render_metrics_loading(f, chunks[1]);
    } else {
        render_metrics(f, chunks[1], &app.metrics, app.scroll_offset, app.metrics_per_screen);
    }
}

fn render_instance_info(f: &mut Frame, area: ratatui::layout::Rect, instance: &crate::models::RdsInstance) {
    let na_string = "N/A".to_string();
    let info_text = vec![
        Line::from(vec![
            Span::styled("Engine: ", Style::default().fg(Color::White)),
            Span::styled(&instance.engine, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(&instance.status, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Class: ", Style::default().fg(Color::White)),
            Span::styled(&instance.instance_class, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Endpoint: ", Style::default().fg(Color::White)),
            Span::styled(
                instance.endpoint.as_ref().unwrap_or(&na_string),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Instance Information")
            .border_style(Style::default().fg(Color::Cyan)))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(info, area);
}

fn render_metrics_loading(f: &mut Frame, area: ratatui::layout::Rect) {
    let loading_msg = Paragraph::new("Loading metrics...")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("CloudWatch Metrics")
            .border_style(Style::default().fg(Color::White)));
    f.render_widget(loading_msg, area);
}
