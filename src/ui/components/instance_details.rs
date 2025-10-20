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
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(f.area());

    let instance = &app.rds_instances[app.selected_instance.unwrap()];

    render_instance_info(f, chunks[0], instance);

    if app.metrics_loading {
        render_metrics_loading(f, chunks[1]);
        return;
    }

    render_metrics(f, chunks[1], &app.metrics, app.scroll_offset, app.metrics_per_screen);
}

fn render_instance_info(f: &mut Frame, area: ratatui::layout::Rect, instance: &crate::models::RdsInstance) {
    let na_string = "N/A".to_string();
    let info_text = vec![
        Line::from(vec![
            Span::styled("Engine: ", Style::default().fg(Color::White)),
            Span::styled(&instance.engine, Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(&instance.status, get_status_style(&instance.status)),
            Span::raw("  "),
            Span::styled("Class: ", Style::default().fg(Color::White)),
            Span::styled(&instance.instance_class, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Endpoint: ", Style::default().fg(Color::White)),
            Span::styled(
                instance.endpoint.as_ref().unwrap_or(&na_string),
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Instance Information"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(info, area);
}

fn render_metrics_loading(f: &mut Frame, area: ratatui::layout::Rect) {
    let loading_msg = Paragraph::new("Loading metrics...")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("CloudWatch Metrics"));
    f.render_widget(loading_msg, area);
}

fn get_status_style(status: &str) -> Style {
    match status {
        "available" => Style::default().fg(Color::Green),
        "stopped" => Style::default().fg(Color::Red),
        _ => Style::default().fg(Color::Yellow),
    }
}
