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
use crate::ui::themes::UnifiedTheme;
use crate::log_ui_render;
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
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
                Constraint::Min(0),     // Time range panel (moved to top)
                Constraint::Length(10), // Period selection panel
                Constraint::Length(6),  // Timezone selection panel (moved to bottom)
            ])
            .split(content_chunks[0]);

        // Render time range panel (now at top)
        render_aws_console_time_range_panel(f, app, time_panel_chunks[0]);

        // Render period selection panel (now in middle)
        render_period_selection_panel(f, app, time_panel_chunks[1]);

        // Render timezone selection panel (now at bottom)
        render_timezone_selection_panel(f, app, time_panel_chunks[2]);

        // Metrics Panel - AWS Console Style Grid (direct rendering)
        super::aws_chart::AwsMetricsGrid::render(f, app, content_chunks[1]);
    }

    // Controls
    render_controls(f, chunks[2], app);
}

// Removed duplicate render_rds_instance_info function - now using pure service-specific component

// Removed duplicate render_sqs_instance_info function - now using pure service-specific component

fn render_default_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let theme = UnifiedTheme::default();
    let header_block = Paragraph::new("Metrics Summary")
        .style(Style::default().fg(theme.primary))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("AWS CloudWatch TUI")
                .border_style(Style::default().fg(theme.accent)),
        );
    f.render_widget(header_block, area);
}

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let theme = UnifiedTheme::default();
    let mode_text = match app.get_time_range_mode() {
        TimeRangeMode::Absolute => "Absolute",
        TimeRangeMode::Relative => "Relative",
    };

    let controls = Paragraph::new(format!(
        "Up/Down: Navigate • Tab: Switch Panels • t: Toggle Mode ({}) • Enter: Select • r: Refresh • b/Esc: Back • q: Quit", 
        mode_text
    ))
        .style(Style::default().fg(theme.secondary));
    f.render_widget(controls, area);
}

/// Render error message with helpful suggestions
fn render_error_message(f: &mut Frame, area: Rect, error_msg: &str) {
    let mut text = vec![
        Line::from(vec![
            Span::styled("Error: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(error_msg),
        ])
    ];
    
    // Add helpful suggestions based on the error message
    if error_msg.contains("CloudWatch") || error_msg.contains("metrics") {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled("Possible causes:", Style::default().fg(Color::Yellow))));
        text.push(Line::from("• No activity in the selected time range"));
        text.push(Line::from("• CloudWatch data not yet available"));
        text.push(Line::from("• Time range exceeds data retention period"));
        text.push(Line::from("• Insufficient AWS permissions"));
        text.push(Line::from(""));
        text.push(Line::from(Span::styled("Suggestions:", Style::default().fg(Color::Green))));
        text.push(Line::from("• Try a shorter time range (1-24 hours)"));
        text.push(Line::from("• Check if the resource has recent activity"));
        text.push(Line::from("• Press 'r' to refresh metrics"));
        text.push(Line::from("• Verify AWS credentials and permissions"));
    }
    
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title("Metrics Error")
        )
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}
