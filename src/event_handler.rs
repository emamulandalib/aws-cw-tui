use crate::models::{App, AppState};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use tracing::{debug, info, warn};

// Include enhanced logging macros
use crate::{log_focus_change, log_key_press, log_metric_operation, log_state_transition, log_user_interaction};

pub async fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    if let Event::Key(key) = event {
        // Enhanced key press logging with tracing
        log_key_press!(key.code, key.modifiers, app.state, app.focused_panel);
        
        debug!(
            key = ?key,
            state = ?app.state,
            panel = ?app.focused_panel,
            "Processing key event"
        );

        match app.state {
            AppState::ServiceList => handle_service_list_event(app, key.code).await,
            AppState::InstanceList => handle_rds_list_event(app, key.code).await,
            AppState::MetricsSummary => handle_metrics_summary_event(app, key).await,
            AppState::InstanceDetails => handle_instance_details_event(app, key.code).await,
        }
    } else {
        debug!(event = ?event, "Non-key event received");
        Ok(false)
    }
}

async fn handle_service_list_event(app: &mut App, key: KeyCode) -> Result<bool> {
    debug!("SERVICE_LIST_EVENT: Handling key: {:?}", key);

    match key {
        KeyCode::Char('q') => {
            log_user_interaction!("Quit requested", "ServiceList screen");
            return Ok(true);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            log_user_interaction!("Navigate down", "ServiceList");
            let old_selection = app.service_list_state.selected();
            app.service_next();
            let new_selection = app.service_list_state.selected();
            debug!(
                "SERVICE_NAVIGATION: Selection changed from {:?} to {:?}",
                old_selection, new_selection
            );
        }
        KeyCode::Up | KeyCode::Char('k') => {
            log_user_interaction!("Navigate up", "ServiceList");
            let old_selection = app.service_list_state.selected();
            app.service_previous();
            let new_selection = app.service_list_state.selected();
            debug!(
                "SERVICE_NAVIGATION: Selection changed from {:?} to {:?}",
                old_selection, new_selection
            );
        }
        KeyCode::Enter => {
            log_user_interaction!("Service selection", "Enter pressed on ServiceList");
            let selected_service = app.select_service().cloned();
            if let Some(service) = selected_service {
                info!(
                    "SERVICE_SELECTION: User selected service: {:?}, loading instances...",
                    service
                );
                log_state_transition!(
                    "ServiceList",
                    "InstanceList",
                    format!("Selected service: {:?}", service)
                );
                app.load_service_instances(&service).await?;
            } else {
                warn!("SERVICE_SELECTION: No service selected when Enter pressed");
            }
        }
        _ => {
            debug!("SERVICE_LIST_EVENT: Unhandled key: {:?}", key);
        }
    }
    Ok(false)
}

async fn handle_rds_list_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    debug!("INSTANCE_LIST_EVENT: Handling key: {:?}", key_code);

    // If we're in loading state, allow certain keys to work
    if app.loading {
        debug!("INSTANCE_LIST_EVENT: App is loading, limited key handling");
        match key_code {
            KeyCode::Char('q') => {
                log_user_interaction!("Quit during loading", "InstanceList");
                return Ok(true);
            }
            KeyCode::Esc => {
                log_user_interaction!("Back to ServiceList during loading", "InstanceList");
                app.loading = false;
                log_state_transition!("InstanceList", "ServiceList", "Escape during loading");
                app.back_to_service_list();
                return Ok(false);
            }
            _ => {
                debug!(
                    "INSTANCE_LIST_EVENT: Ignoring key {:?} during loading",
                    key_code
                );
                return Ok(false);
            }
        }
    }

    match key_code {
        KeyCode::Char('q') => {
            log_user_interaction!("Quit requested", "InstanceList");
            Ok(true)
        }
        KeyCode::Esc => {
            log_user_interaction!("Back to ServiceList", "InstanceList");
            log_state_transition!("InstanceList", "ServiceList", "User pressed Escape");
            app.back_to_service_list();
            Ok(false)
        }
        KeyCode::Down => {
            log_user_interaction!("Navigate down", "InstanceList");
            let old_selection = app.list_state.selected();
            app.next();
            let new_selection = app.list_state.selected();
            debug!(
                "INSTANCE_NAVIGATION: Selection changed from {:?} to {:?}",
                old_selection, new_selection
            );
            if let Some(instance_id) = app.get_selected_instance_id() {
                debug!(
                    "INSTANCE_NAVIGATION: Now selected instance: {}",
                    instance_id
                );
            }
            Ok(false)
        }
        KeyCode::Up => {
            log_user_interaction!("Navigate up", "InstanceList");
            let old_selection = app.list_state.selected();
            app.previous();
            let new_selection = app.list_state.selected();
            debug!(
                "INSTANCE_NAVIGATION: Selection changed from {:?} to {:?}",
                old_selection, new_selection
            );
            if let Some(instance_id) = app.get_selected_instance_id() {
                debug!(
                    "INSTANCE_NAVIGATION: Now selected instance: {}",
                    instance_id
                );
            }
            Ok(false)
        }
        KeyCode::Enter => {
            log_user_interaction!("Instance selection", "Enter pressed on InstanceList");
            if let Some(instance_id) = app.get_selected_instance_id() {
                info!("INSTANCE_SELECTION: Selected instance: {}", instance_id);
                log_state_transition!(
                    "InstanceList",
                    "MetricsSummary",
                    format!("Selected instance: {}", instance_id)
                );
                app.enter_metrics_summary();
                log_metric_operation!("Load metrics", &instance_id, "User selected instance");
                app.load_metrics(&instance_id).await?;
            } else {
                warn!("INSTANCE_SELECTION: No instance selected when Enter pressed");
            }
            Ok(false)
        }
        KeyCode::Char('d') => {
            log_user_interaction!("Dynamic metrics test", "d key pressed");
            info!("DYNAMIC_METRICS: User triggered dynamic metrics loading test");
            if let Some(instance_id) = app.get_selected_instance_id() {
                info!(
                    "DYNAMIC_METRICS: Loading dynamic metrics for instance: {}",
                    instance_id
                );
                log_state_transition!(
                    "InstanceList",
                    "MetricsSummary",
                    format!("Dynamic metrics test for: {}", instance_id)
                );
                app.enter_metrics_summary();
                log_metric_operation!("Load dynamic metrics", &instance_id, "User test command");
                app.load_metrics_dynamic(&instance_id).await?;
            } else {
                warn!("DYNAMIC_METRICS: No instance selected for dynamic metrics test");
            }
            Ok(false)
        }
        KeyCode::Char('r') => {
            log_user_interaction!("Manual refresh", "r key pressed on InstanceList");
            info!("MANUAL_REFRESH: User triggered manual refresh");
            app.loading = true;
            let selected_service = app.selected_service.clone();
            if let Some(service) = selected_service {
                debug!(
                    "MANUAL_REFRESH: Refreshing instances for service: {:?}",
                    service
                );
                app.load_service_instances(&service).await?;
            } else {
                warn!("MANUAL_REFRESH: No service selected for refresh");
            }
            Ok(false)
        }
        _ => {
            debug!("INSTANCE_LIST_EVENT: Unhandled key: {:?}", key_code);
            Ok(false)
        }
    }
}

async fn handle_metrics_summary_event(app: &mut App, key: KeyEvent) -> Result<bool> {
    debug!(
        "METRICS_SUMMARY_EVENT: Handling key: {:?}, focused panel: {:?}",
        key,
        app.get_focused_panel()
    );

    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => {
            log_user_interaction!("Quit requested", "MetricsSummary");
            Ok(true)
        }
        (KeyCode::Down, _) => {
            log_user_interaction!(
                "Navigate down",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            match app.get_focused_panel() {
                crate::models::FocusedPanel::SparklineGrid => {
                    let old_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    app.sparkline_grid_scroll_down();
                    let new_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    if old_index != new_index {
                        debug!(
                            "SPARKLINE_NAVIGATION: Grid navigation down {} -> {}",
                            old_index, new_index
                        );
                        if let Some(ref metrics) = app.dynamic_metrics {
                            if let Some(metric_name) =
                                metrics.get_available_metric_names().get(new_index)
                            {
                                log_metric_operation!(
                                    "Navigate to metric",
                                    metric_name,
                                    "Grid down navigation"
                                );
                            }
                        }
                    }
                }
                _ => {
                    app.scroll_down();
                }
            }
            Ok(false)
        }
        (KeyCode::Up, _) => {
            log_user_interaction!(
                "Navigate up",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            match app.get_focused_panel() {
                crate::models::FocusedPanel::SparklineGrid => {
                    let old_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    app.sparkline_grid_scroll_up();
                    let new_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    if old_index != new_index {
                        debug!(
                            "SPARKLINE_NAVIGATION: Grid navigation up {} -> {}",
                            old_index, new_index
                        );
                        if let Some(ref metrics) = app.dynamic_metrics {
                            if let Some(metric_name) =
                                metrics.get_available_metric_names().get(new_index)
                            {
                                log_metric_operation!(
                                    "Navigate to metric",
                                    metric_name,
                                    "Grid up navigation"
                                );
                            }
                        }
                    }
                }
                _ => {
                    app.scroll_up();
                }
            }
            Ok(false)
        }
        (KeyCode::Left, _) => {
            log_user_interaction!(
                "Navigate left",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            match app.get_focused_panel() {
                crate::models::FocusedPanel::SparklineGrid => {
                    let old_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    app.sparkline_grid_scroll_left();
                    let new_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    if old_index != new_index {
                        debug!(
                            "SPARKLINE_NAVIGATION: Grid navigation left {} -> {}",
                            old_index, new_index
                        );
                        if let Some(ref metrics) = app.dynamic_metrics {
                            if let Some(metric_name) =
                                metrics.get_available_metric_names().get(new_index)
                            {
                                log_metric_operation!(
                                    "Navigate to metric",
                                    metric_name,
                                    "Grid left navigation"
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(false)
        }
        (KeyCode::Right, _) => {
            log_user_interaction!(
                "Navigate right",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            match app.get_focused_panel() {
                crate::models::FocusedPanel::SparklineGrid => {
                    let old_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    app.sparkline_grid_scroll_right();
                    let new_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                    if old_index != new_index {
                        debug!(
                            "SPARKLINE_NAVIGATION: Grid navigation right {} -> {}",
                            old_index, new_index
                        );
                        if let Some(ref metrics) = app.dynamic_metrics {
                            if let Some(metric_name) =
                                metrics.get_available_metric_names().get(new_index)
                            {
                                log_metric_operation!(
                                    "Navigate to metric",
                                    metric_name,
                                    "Grid right navigation"
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(false)
        }
        (KeyCode::Enter, _) => {
            log_user_interaction!(
                "Enter pressed",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            match app.get_focused_panel() {
                crate::models::FocusedPanel::Timezone => {
                    log_user_interaction!("Timezone selection", "MetricsSummary");
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        log_metric_operation!("Reload metrics", &instance_id, "Timezone changed");
                        app.load_metrics(&instance_id).await?;
                    }
                }
                crate::models::FocusedPanel::Period => {
                    log_user_interaction!("Period selection", "MetricsSummary");
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        log_metric_operation!("Reload metrics", &instance_id, "Period changed");
                        app.load_metrics(&instance_id).await?;
                    }
                }
                crate::models::FocusedPanel::TimeRanges => {
                    log_user_interaction!("Time range selection", "MetricsSummary");
                    let current_index = app.get_current_time_range_index();
                    app.select_time_range(current_index)?;
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        log_metric_operation!("Reload metrics", &instance_id, "Time range changed");
                        app.load_metrics(&instance_id).await?;
                    }
                }
                crate::models::FocusedPanel::SparklineGrid => {
                    log_user_interaction!(
                        "Navigate to InstanceDetails",
                        "MetricsSummary SparklineGrid"
                    );
                    if let Some(ref metrics) = app.dynamic_metrics {
                        let current_selected_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
                        if let Some(metric_name) = metrics
                            .get_available_metric_names()
                            .get(current_selected_index)
                        {
                            log_metric_operation!(
                                "View detailed metric",
                                metric_name,
                                "Navigate to InstanceDetails"
                            );
                            info!(
                                "METRIC_DETAIL: Navigating to detailed view for metric: {}",
                                metric_name
                            );
                        }
                    }
                    log_state_transition!(
                        "MetricsSummary",
                        "InstanceDetails",
                        "User selected metric for detailed view"
                    );
                    app.enter_instance_details();
                }
            }
            Ok(false)
        }
        (KeyCode::Char('t'), _) => {
            log_user_interaction!("Toggle time range mode", "MetricsSummary");
            app.toggle_time_range_mode();
            debug!("TIME_RANGE: Mode toggled to: {:?}", app.time_range_mode);
            Ok(false)
        }
        (KeyCode::Tab, _) => {
            let old_panel = app.get_focused_panel().clone();
            log_user_interaction!(
                "Tab - switch panel",
                format!("MetricsSummary from {:?}", old_panel)
            );
            app.switch_panel();
            let new_panel = app.get_focused_panel().clone();
            log_focus_change!(old_panel, new_panel);
            debug!(
                "PANEL_SWITCH: {} -> {}",
                format!("{:?}", old_panel),
                format!("{:?}", new_panel)
            );
            Ok(false)
        }
        (KeyCode::Esc, _) => {
            log_user_interaction!("Back to InstanceList", "MetricsSummary");
            log_state_transition!("MetricsSummary", "InstanceList", "User pressed Escape");
            app.back_to_list();
            Ok(false)
        }
        (KeyCode::Char('k'), _) => {
            log_user_interaction!(
                "Navigate up (vim key)",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            app.scroll_up();
            Ok(false)
        }
        (KeyCode::Char('j'), _) => {
            log_user_interaction!(
                "Navigate down (vim key)",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            app.scroll_down();
            Ok(false)
        }
        (KeyCode::Home, _) => {
            log_user_interaction!("Reset scroll", "MetricsSummary");
            app.reset_scroll();
            Ok(false)
        }
        (KeyCode::Char('r'), _) => {
            log_user_interaction!("Manual refresh", "MetricsSummary");
            if let Some(instance_id) = app.get_selected_instance_id() {
                log_metric_operation!(
                    "Manual reload metrics",
                    &instance_id,
                    "User refresh request"
                );
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        _ => {
            debug!("METRICS_SUMMARY_EVENT: Unhandled key: {:?}", key);
            Ok(false)
        }
    }
}

async fn handle_instance_details_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    debug!("INSTANCE_DETAILS_EVENT: Handling key: {:?}", key_code);

    match key_code {
        KeyCode::Char('q') => {
            log_user_interaction!("Quit requested", "InstanceDetails");
            Ok(true)
        }
        KeyCode::Char('b') | KeyCode::Esc => {
            log_user_interaction!("Back to MetricsSummary", "InstanceDetails");
            log_state_transition!("InstanceDetails", "MetricsSummary", "User navigated back");
            app.back_to_metrics_summary();
            Ok(false)
        }
        KeyCode::Char('r') => {
            log_user_interaction!("Manual refresh", "InstanceDetails");
            if let Some(instance_id) = app.get_selected_instance_id() {
                log_metric_operation!(
                    "Manual reload metrics",
                    &instance_id,
                    "User refresh from InstanceDetails"
                );
                app.load_metrics(&instance_id).await?;
            }
            Ok(false)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            log_user_interaction!("Scroll up", "InstanceDetails");
            let old_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
            app.scroll_up();
            let new_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
            if old_index != new_index {
                debug!(
                    "INSTANCE_DETAILS_SCROLL: Scrolled up {} -> {}",
                    old_index, new_index
                );
                if let Some(ref metrics) = app.dynamic_metrics {
                    if let Some(metric_name) = metrics.get_available_metric_names().get(new_index) {
                        log_metric_operation!(
                            "Scroll to metric",
                            metric_name,
                            "InstanceDetails scroll up"
                        );
                    }
                }
            }
            Ok(false)
        }
        KeyCode::Down | KeyCode::Char('j') => {
            log_user_interaction!("Scroll down", "InstanceDetails");
            let old_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
            app.scroll_down();
            let new_index = app.sparkline_grid_list_state.selected().unwrap_or(0);
            if old_index != new_index {
                debug!(
                    "INSTANCE_DETAILS_SCROLL: Scrolled down {} -> {}",
                    old_index, new_index
                );
                if let Some(ref metrics) = app.dynamic_metrics {
                    if let Some(metric_name) = metrics.get_available_metric_names().get(new_index) {
                        log_metric_operation!(
                            "Scroll to metric",
                            metric_name,
                            "InstanceDetails scroll down"
                        );
                    }
                }
            }
            Ok(false)
        }
        KeyCode::Home => {
            log_user_interaction!("Reset scroll", "InstanceDetails");
            app.reset_scroll();
            Ok(false)
        }
        _ => {
            debug!("INSTANCE_DETAILS_EVENT: Unhandled key: {:?}", key_code);
            Ok(false)
        }
    }
}
