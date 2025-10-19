use super::components::{
    render_instance_details, render_metrics_summary, render_rds_list, render_service_list,
};
use crate::models::{App, AppState};
use crate::{log_ui_render, ui_span};
use ratatui::Frame;

pub fn render_app(f: &mut Frame, app: &mut App) {
    ui_span!("main_render", {
        log_ui_render!("main_app", f.area(), format!("state: {:?}", app.state));

        // Get the current theme from app state
        let theme = app.get_current_theme();

        match app.state {
            AppState::ServiceList => {
                ui_span!("service_list", render_service_list(f, app, &theme));
            }
            AppState::InstanceList => {
                ui_span!("instance_list", render_rds_list(f, app, &theme));
            }
            AppState::MetricsSummary => {
                ui_span!("metrics_summary", render_metrics_summary(f, app, &theme));
            }
            AppState::InstanceDetails => {
                ui_span!("instance_details", render_instance_details(f, app, &theme));
            }
        }
    });
}
