use super::components::{render_instance_details, render_metrics_summary, render_rds_list};
use crate::models::{App, AppState};
use ratatui::Frame;

pub fn render_app(f: &mut Frame, app: &mut App) {
    match app.state {
        AppState::RdsList => render_rds_list(f, app),
        AppState::MetricsSummary => render_metrics_summary(f, app),
        AppState::InstanceDetails => render_instance_details(f, app),
    }
}
