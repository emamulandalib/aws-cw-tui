use super::{
    display_utils::calculate_time_panel_width,
    instance_details::render_metrics_loading,
    time_range_utils::{
        render_aws_console_time_range_panel, render_period_selection_panel,
        render_timezone_selection_panel,
    },
};
use crate::models::{App, TimeRangeMode};
use crate::ui::components::{render_rds_instance_details, render_sqs_queue_details};
use crate::log_ui_render;
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_metrics_summary(f: &mut Frame, app: &mut App) {
    log_ui_render!("metrics_summary", f.area(), format!("focused_panel: {:?}", app.focused_panel));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header - increased height to show the endpoint
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    // Header - Instance Information (unified for both RDS and SQS) using pure service-specific components
    if let Some(selected_instance) = app.get_selected_instance() {
        match selected_instance {
            crate::models::ServiceInstance::Rds(instance) => {
                render_rds_instance_details(f, chunks[0], instance, true);
            }
            crate::models::ServiceInstance::Sqs(queue) => {
                render_sqs_queue_details(f, chunks[0], queue, true);
            }
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
        // Two-panel layout: Time Range Panel (left), Metrics (right) - like original
        let time_panel_width = calculate_time_panel_width(chunks[1].width);
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(time_panel_width), // Compact Time Panel (responsive width)
                Constraint::Min(0),                   // Metrics panel takes remaining space
            ])
            .split(chunks[1]);

        // Split the time panel into timezone, period and time range sections
        let time_panel_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Timezone selection panel
                Constraint::Length(10), // Period selection panel
                Constraint::Min(0),     // Time range panel
            ])
            .split(content_chunks[0]);

        // Render timezone selection panel
        render_timezone_selection_panel(f, app, time_panel_chunks[0]);

        // Render period selection panel
        render_period_selection_panel(f, app, time_panel_chunks[1]);

        // Render time range panel
        render_aws_console_time_range_panel(f, app, time_panel_chunks[2]);

        // Metrics Panel - AWS Console Style Grid (direct rendering)
        super::aws_chart::AwsMetricsGrid::render(f, app, content_chunks[1]);
    }

    // Controls
    render_controls(f, chunks[2], app);
}

// Removed duplicate render_rds_instance_info function - now using pure service-specific component

// Removed duplicate render_sqs_instance_info function - now using pure service-specific component

fn render_default_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let header_block = Paragraph::new("Metrics Summary")
        .style(Style::default().white())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("AWS CloudWatch TUI")
                .border_style(Style::default().cyan()),
        );
    f.render_widget(header_block, area);
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mode_text = match app.get_time_range_mode() {
        TimeRangeMode::Absolute => "Absolute",
        TimeRangeMode::Relative => "Relative",
    };

    let controls = Paragraph::new(format!(
        "Up/Down: Navigate • Tab: Switch Panels • t: Toggle Mode ({}) • Enter: Select • r: Refresh • b/Esc: Back • q: Quit", 
        mode_text
    ))
        .style(Style::default().gray());
    f.render_widget(controls, area);
}

fn render_error_message(f: &mut Frame, area: ratatui::layout::Rect, error_msg: &str) {
    let error_paragraph = Paragraph::new(error_msg)
        .style(Style::default().red())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .border_style(Style::default().red()),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(error_paragraph, area);
}
