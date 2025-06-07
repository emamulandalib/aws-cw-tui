use ratatui::Frame;
use crate::models::{App, AppState};
use super::components::{render_rds_list, render_instance_details, render_metrics_summary};

pub fn render_app(f: &mut Frame, app: &mut App) {
    match app.state {
        AppState::RdsList => render_rds_list(f, app),
        AppState::MetricsSummary => render_metrics_summary(f, app),
        AppState::InstanceDetails => render_instance_details(f, app),
    }
}
