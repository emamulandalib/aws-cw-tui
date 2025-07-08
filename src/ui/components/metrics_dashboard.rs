use super::aws_chart::AwsMetricsGrid;
use crate::models::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// AWS Console-style metrics dashboard
pub struct MetricsDashboard;

impl MetricsDashboard {
    /// Render the complete metrics dashboard
    pub fn render(f: &mut Frame, app: &App, area: Rect) {
        // Create layout with title and metrics grid
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title/header area
                Constraint::Min(10),   // Metrics grid area
                Constraint::Length(3), // Status/controls area
            ])
            .split(area);

        // Render header
        Self::render_header(f, chunks[0], app);

        // Render metrics grid
        AwsMetricsGrid::render(f, app, chunks[1]);

        // Render status/controls
        Self::render_status(f, chunks[2], app);
    }

    /// Render dashboard header with service info
    fn render_header(f: &mut Frame, area: Rect, app: &App) {
        let service_name = match app.get_selected_instance() {
            Some(crate::models::ServiceInstance::Rds(instance)) => {
                format!("RDS Instance: {}", instance.identifier)
            }
            Some(crate::models::ServiceInstance::Sqs(queue)) => {
                format!("SQS Queue: {}", queue.name)
            }
            None => "No service selected".to_string(),
        };

        let unit_name = match app.time_range.unit {
            crate::aws::time_range::TimeUnit::Minutes => "minutes",
            crate::aws::time_range::TimeUnit::Hours => "hours", 
            crate::aws::time_range::TimeUnit::Days => "days",
            crate::aws::time_range::TimeUnit::Weeks => "weeks",
            crate::aws::time_range::TimeUnit::Months => "months",
        };
        let time_range_str = format!("{} {}", app.time_range.value, unit_name);

        let header_content = format!("{} • Time Range: {}", service_name, time_range_str);

        let header_widget = Paragraph::new(header_content)
            .style(Style::default().fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("AWS CloudWatch Metrics")
                    .border_style(Style::default().fg(Color::Green))
            );

        f.render_widget(header_widget, area);
    }

    /// Render status and controls information
    fn render_status(f: &mut Frame, area: Rect, app: &App) {
        let metrics_count = Self::get_metrics_count(app);
        let last_updated = Self::get_last_updated_info(app);

        let status_content = format!(
            "Metrics: {} • Last Updated: {} • Press 'r' to refresh • 'b' to go back • 'q' to quit",
            metrics_count, last_updated
        );

        let status_widget = Paragraph::new(status_content)
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Status")
                    .border_style(Style::default().fg(Color::White))
            );

        f.render_widget(status_widget, area);
    }

    /// Get metrics count based on current service
    fn get_metrics_count(app: &App) -> usize {
        match app.selected_service.as_ref() {
            Some(crate::models::AwsService::Rds) => app.metrics.get_available_metrics().len(),
            Some(crate::models::AwsService::Sqs) => app.sqs_metrics.get_available_metrics().len(),
            None => 0,
        }
    }

    /// Get last updated information
    fn get_last_updated_info(app: &App) -> String {
        use chrono::{DateTime, Local, Utc};

        let last_timestamp = match app.selected_service.as_ref() {
            Some(crate::models::AwsService::Rds) => {
                app.metrics.timestamps.last().copied()
            }
            Some(crate::models::AwsService::Sqs) => {
                app.sqs_metrics.timestamps.last().copied()
            }
            None => None,
        };

        match last_timestamp {
            Some(timestamp) => {
                let dt: DateTime<Utc> = timestamp.into();
                let local_time: DateTime<Local> = dt.into();
                format!("{}", local_time.format("%H:%M:%S"))
            }
            None => "Never".to_string(),
        }
    }
}

/// Render the metrics dashboard as the main view
pub fn render_metrics_dashboard(f: &mut Frame, app: &App, area: Rect) {
    MetricsDashboard::render(f, app, area);
} 