use super::super::charts::metrics_chart::render_metrics_unified;
use crate::models::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_instance_details(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header - increased height to show the endpoint
            Constraint::Min(0),    // Content (metrics chart will handle its own controls)
        ])
        .split(f.area());

    // Handle both RDS and SQS instances
    match app.selected_service.as_ref().unwrap_or(&crate::models::AwsService::Rds) {
        crate::models::AwsService::Rds => {
            let instance = match app.get_selected_rds_instance() {
                Some(instance) => instance,
                None => {
                    render_error_message(f, chunks[0], "No RDS instance selected");
                    return;
                }
            };
            render_rds_instance_info(f, chunks[0], instance);
        }
        crate::models::AwsService::Sqs => {
            let queue = match app.get_selected_sqs_queue() {
                Some(queue) => queue,
                None => {
                    render_error_message(f, chunks[0], "No SQS queue selected");
                    return;
                }
            };
            render_sqs_instance_info(f, chunks[0], queue);
        }
    }

    if app.metrics_loading {
        render_metrics_loading(f, chunks[1]);
    } else {
        // For the detailed chart view, we want to show 1 metric per screen for maximum chart size
        let chart_metrics_per_screen = 1;

        // Use the ListState-based selection directly (consistent with MetricsSummary)
        let selected_metric_index = app.sparkline_grid_selected_index;

        render_metrics_unified(
            f,
            chunks[1],
            app,
            selected_metric_index,
            chart_metrics_per_screen,
        );
    }
}

fn render_rds_instance_info(
    f: &mut Frame,
    area: ratatui::layout::Rect,
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

fn render_sqs_instance_info(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    queue: &crate::models::SqsQueue,
) {
    // Get queue attributes for display
    let retention_period = queue.attributes
        .get("MessageRetentionPeriod")
        .and_then(|p| p.parse::<u64>().ok())
        .map(|p| format!("{}d", p / 86400))
        .unwrap_or_else(|| "Unknown".to_string());
    
    let visibility_timeout = queue.attributes
        .get("VisibilityTimeout")
        .unwrap_or(&"30".to_string())
        .clone();
    
    let max_receive_count = queue.attributes
        .get("ApproximateNumberOfMessages")
        .unwrap_or(&"0".to_string())
        .clone();

    let visibility_timeout_str = format!("{}s", visibility_timeout);

    let info_text = vec![
        Line::from(vec![
            Span::styled("Type: ", Style::default().fg(Color::White)),
            Span::styled(&queue.queue_type, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Messages: ", Style::default().fg(Color::White)),
            Span::styled(&max_receive_count, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Retention: ", Style::default().fg(Color::White)),
            Span::styled(&retention_period, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("URL: ", Style::default().fg(Color::White)),
            Span::styled(
                &queue.url,
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled("Visibility Timeout: ", Style::default().fg(Color::White)),
            Span::styled(
                &visibility_timeout_str,
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Queue Information")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(info, area);
}

pub fn render_metrics_loading(f: &mut Frame, area: ratatui::layout::Rect) {
    let loading_msg = Paragraph::new("Loading metrics...")
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("CloudWatch Metrics")
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(loading_msg, area);
}

fn render_error_message(f: &mut Frame, area: ratatui::layout::Rect, message: &str) {
    let error_msg = Paragraph::new(message)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error")
                .border_style(Style::default().fg(Color::Red)),
        );
    f.render_widget(error_msg, area);
}
