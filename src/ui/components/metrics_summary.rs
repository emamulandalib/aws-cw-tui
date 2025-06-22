use super::{
    display_utils::calculate_time_panel_width, instance_details::render_metrics_loading,
    metric_list_utils::render_enhanced_metric_list, time_range_utils::render_time_range_panel,
};
use crate::models::App;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_metrics_summary(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header - increased height to show the endpoint
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    // Header - Instance Information
    if let Some(selected_idx) = app.selected_instance {
        if let Some(instance) = app.rds_instances.get(selected_idx) {
            render_instance_info(f, chunks[0], app, instance);
        } else {
            render_default_header(f, chunks[0]);
        }
    } else {
        render_default_header(f, chunks[0]);
    }

    // Content area - check for errors first, then loading, then normal content
    if let Some(error_msg) = &app.error_message {
        render_error_message(f, chunks[1], error_msg);
    } else if app.metrics_loading {
        render_metrics_loading(f, chunks[1]);
    } else {
        // Two-panel layout: Time ranges (left), Full-height metric list (right)
        let time_panel_width = calculate_time_panel_width(chunks[1].width);
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(time_panel_width), // Compact Time Panel (responsive width)
                Constraint::Min(0),                   // Right panel for full-height metric list
            ])
            .split(chunks[1]);

        // Compact Time Range Panel
        render_compact_time_ranges(f, app, content_chunks[0]);

        // Full-height Metric List Panel
        render_enhanced_metric_list(f, app, content_chunks[1]);
    }

    // Controls
    render_controls(f, chunks[2]);
}


fn render_instance_info(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    _app: &crate::models::App,
    instance: &crate::models::RdsInstance,
) {
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
                Style::default().fg(Color::Cyan),
            ),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Instance Information")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(info, area);
}

fn render_default_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let header_block = Paragraph::new("Metrics Summary")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("RDS CloudWatch TUI")
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(header_block, area);
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect) {
    let controls = Paragraph::new(
        "↑/↓: Navigate • Tab: Switch Panels (Time/Sparklines) • Enter: Select • r: Refresh • b/Esc: Back • q: Quit")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(controls, area);
}

fn render_compact_time_ranges(f: &mut Frame, app: &mut App, area: Rect) {
    render_time_range_panel(f, app, area);
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