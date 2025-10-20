use super::{
    display_utils::calculate_time_panel_width,
    instance_details::render_metrics_loading,
    time_range_utils::{
        render_aws_console_time_range_panel, render_period_selection_panel,
        render_timezone_selection_panel,
    },
};
use crate::log_ui_render;
use crate::models::app_state::App;
use crate::models::TimeRangeMode;
use crate::ui::components::universal_box::UniversalBox;
use crate::ui::components::{render_rds_instance_details, render_sqs_queue_details};
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_metrics_summary(f: &mut Frame, app: &mut App, theme: &UnifiedTheme) {
    log_ui_render!(
        "metrics_summary",
        f.area(),
        format!("focused_panel: {:?}", app.focused_panel)
    );
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Instance header area
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Controls at bottom
        ])
        .split(f.area());

    // Header - Instance Information (unified for both RDS and SQS) using pure service-specific components
    if let Some(selected_instance) = app.get_selected_instance() {
        match selected_instance {
            crate::models::ServiceInstance::Rds(instance) => {
                render_rds_instance_details(f, chunks[0], instance, true, theme);
            }
            crate::models::ServiceInstance::Sqs(queue) => {
                render_sqs_queue_details(f, chunks[0], &queue.name, true, theme);
            }
        }
    }

    // Content area - check for errors first, then loading, then normal content
    if let Some(error_msg) = &app.error_message {
        render_error_message(f, chunks[1], error_msg, theme);
    } else if app.metrics_loading {
        render_metrics_loading(f, chunks[1], theme);
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
        render_aws_console_time_range_panel(f, app, time_panel_chunks[0], theme);

        // Render period selection panel (now in middle)
        render_period_selection_panel(f, app, time_panel_chunks[1], theme);

        // Render timezone selection panel (now at bottom)
        render_timezone_selection_panel(f, app, time_panel_chunks[2], theme);

        // Metrics Panel - AWS Console Style Grid (direct rendering)
        super::aws_chart::AwsMetricsGrid::render(f, app, content_chunks[1], theme);
    }

    // Controls
    render_controls(f, chunks[2], app, theme);
}

// Removed duplicate render_rds_instance_info function - now using pure service-specific component

// Removed duplicate render_sqs_instance_info function - now using pure service-specific component

fn render_controls(f: &mut Frame, area: ratatui::layout::Rect, app: &App, theme: &UnifiedTheme) {
    let mode_text = match app.get_time_range_mode() {
        TimeRangeMode::Absolute => "Absolute",
        TimeRangeMode::Relative => "Relative",
    };

    let controls = Paragraph::new(format!(
        "Up/Down: Navigate • Tab: Switch Panels • t: Change Theme ({}) • Enter: Select • r: Refresh • b/Esc: Back • q: Quit", 
        mode_text
    ))
        .style(Style::default().fg(theme.secondary));
    f.render_widget(controls, area);
}

/// Render error message with helpful suggestions
fn render_error_message(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    error_msg: &str,
    theme: &UnifiedTheme,
) {
    UniversalBox::error_box("Error", error_msg, theme.clone()).render(f, area);
}
