use crate::models::{App, AppState, FocusedPanel};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tracing::{debug, info, warn};

// Include enhanced logging macros
use crate::{log_key_press, log_user_interaction};

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

        // Handle global help system first (works in all states)
        if (key.code == KeyCode::Char('?') || key.code == KeyCode::Char('h'))
            && key.modifiers == KeyModifiers::NONE
        {
            use crate::ui::components::help_system::HelpSystemExt;
            app.toggle_help();
            log_user_interaction!(
                "Help toggled",
                format!(
                    "State: {:?}, Visible: {}",
                    app.state,
                    app.get_help_system().visible
                )
            );
            info!(
                "HELP_SYSTEM: Help toggled - visible: {}",
                app.get_help_system().visible
            );
            return Ok(false);
        }

        // Handle global theme switching (works in all states)
        if key.code == KeyCode::Char('t') && key.modifiers == KeyModifiers::NONE {
            let old_theme = app.get_current_theme_name();
            app.next_theme();
            let new_theme = app.get_current_theme_name();

            // Save theme preference to configuration
            if let Err(e) = app.save_theme_preference() {
                warn!("Failed to save theme preference: {}", e);
            }

            log_user_interaction!(
                format!("Theme switched from {} to {}", old_theme, new_theme),
                format!("State: {:?}", app.state)
            );
            info!(
                "THEME_SWITCH: User switched theme from {} to {} (saved to config)",
                old_theme, new_theme
            );
            return Ok(false);
        }

        // Update help context when state changes
        use crate::ui::components::help_system::HelpSystemExt;
        app.update_help_context();

        match app.state {
            AppState::ServiceList => handle_service_list_event(app, key.code).await,
            AppState::InstanceList => handle_instance_list_event(app, key.code).await,
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
            Ok(true)
        }
        KeyCode::Down | KeyCode::Char('j') => {
            log_user_interaction!("Navigate down", "ServiceList");
            let old_selection = app.service_list_state.selected();
            app.service_next(); // Using core navigation module
            let new_selection = app.service_list_state.selected();
            debug!(
                "SERVICE_NAVIGATION: Selection changed from {:?} to {:?}",
                old_selection, new_selection
            );
            Ok(false)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            log_user_interaction!("Navigate up", "ServiceList");
            let old_selection = app.service_list_state.selected();
            app.service_previous(); // Using core navigation module
            let new_selection = app.service_list_state.selected();
            debug!(
                "SERVICE_NAVIGATION: Selection changed from {:?} to {:?}",
                old_selection, new_selection
            );
            Ok(false)
        }
        KeyCode::Enter => {
            log_user_interaction!("Service selection", "Enter pressed on ServiceList");
            if let Some(service) = app.select_service().cloned() {
                // Using core service management
                info!(
                    "SERVICE_SELECTION: User selected service: {:?}, loading instances...",
                    service
                );
                app.load_service_instances(&service).await?; // Using core service management
            } else {
                warn!("SERVICE_SELECTION: No service selected when Enter pressed");
            }
            Ok(false)
        }
        _ => {
            debug!("SERVICE_LIST_EVENT: Unhandled key: {:?}", key);
            Ok(false)
        }
    }
}

async fn handle_instance_list_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    debug!("INSTANCE_LIST_EVENT: Handling key: {:?}", key_code);

    // If we're in loading state, allow certain keys to work
    if app.loading {
        debug!("INSTANCE_LIST_EVENT: App is loading, limited key handling");
        match key_code {
            KeyCode::Char('q') => {
                log_user_interaction!("Quit during loading", "InstanceList");
                Ok(true)
            }
            KeyCode::Esc => {
                log_user_interaction!("Back to ServiceList during loading", "InstanceList");
                app.stop_loading(); // Using core state management
                app.back_to_service_list(); // Using core screen navigation
                Ok(false)
            }
            _ => {
                debug!(
                    "INSTANCE_LIST_EVENT: Ignoring key during loading: {:?}",
                    key_code
                );
                Ok(false)
            }
        }
    } else {
        match key_code {
            KeyCode::Char('q') => {
                log_user_interaction!("Quit requested", "InstanceList");
                Ok(true)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                log_user_interaction!("Navigate down", "InstanceList");
                let old_selection = app.list_state.selected();
                app.next(); // Using core navigation module
                let new_selection = app.list_state.selected();
                debug!(
                    "INSTANCE_NAVIGATION: Selection changed from {:?} to {:?}",
                    old_selection, new_selection
                );
                Ok(false)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                log_user_interaction!("Navigate up", "InstanceList");
                let old_selection = app.list_state.selected();
                app.previous(); // Using core navigation module
                let new_selection = app.list_state.selected();
                debug!(
                    "INSTANCE_NAVIGATION: Selection changed from {:?} to {:?}",
                    old_selection, new_selection
                );
                Ok(false)
            }
            KeyCode::Enter => {
                log_user_interaction!("Instance selection", "Enter pressed on InstanceList");
                if let Some(instance_id) = app.get_selected_instance_id() {
                    info!(
                        "INSTANCE_SELECTION: User selected instance: {}, loading metrics...",
                        instance_id
                    );

                    app.enter_metrics_summary(); // Using core screen navigation
                    app.load_metrics(&instance_id).await?; // Using core metrics management
                } else {
                    warn!("INSTANCE_SELECTION: No instance selected when Enter pressed");
                }
                Ok(false)
            }
            KeyCode::Esc => {
                log_user_interaction!("Back to ServiceList", "InstanceList");
                app.back_to_service_list(); // Using core screen navigation
                Ok(false)
            }
            KeyCode::Char('r') => {
                log_user_interaction!("Manual refresh", "InstanceList");
                app.clear_error(); // Using core state management
                app.start_loading(); // Using core state management

                if let Some(service) = app.selected_service.clone() {
                    debug!(
                        "MANUAL_REFRESH: Refreshing instances for service: {:?}",
                        service
                    );
                    app.load_service_instances(&service).await?; // Using core service management
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
        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
            log_user_interaction!(
                "Navigate down",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            app.scroll_down(); // Using core UI state management
            Ok(false)
        }
        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
            log_user_interaction!(
                "Navigate up",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            app.scroll_up(); // Using core UI state management
            Ok(false)
        }
        (KeyCode::Left, _) | (KeyCode::Char('h'), _) => {
            log_user_interaction!(
                "Navigate left",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            app.scroll_left(); // Using core UI state management
            Ok(false)
        }
        (KeyCode::Right, _) | (KeyCode::Char('l'), _) => {
            log_user_interaction!(
                "Navigate right",
                format!("MetricsSummary - {:?} panel", app.get_focused_panel())
            );
            app.scroll_right(); // Using core UI state management
            Ok(false)
        }
        (KeyCode::Tab, _) => {
            log_user_interaction!("Panel switch", "MetricsSummary");
            app.switch_panel(); // Using core UI state management
            Ok(false)
        }
        (KeyCode::Enter, _) => {
            match app.get_focused_panel() {
                FocusedPanel::Timezone | FocusedPanel::Period | FocusedPanel::TimeRanges => {
                    log_user_interaction!("Refresh data with time selection", "MetricsSummary");
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        info!(
                            "TIME_SELECTION_REFRESH: Refreshing metrics for instance: {} with new time settings",
                            instance_id
                        );
                        app.load_metrics(&instance_id).await?; // Refresh with current time settings
                    } else {
                        warn!("TIME_SELECTION_REFRESH: No instance selected for refresh");
                    }
                    Ok(false)
                }
                FocusedPanel::SparklineGrid => {
                    log_user_interaction!("Enter instance details", "MetricsSummary");
                    app.enter_instance_details(); // Using core screen navigation
                    Ok(false)
                }
            }
        }
        (KeyCode::Esc, _) => {
            log_user_interaction!("Back to InstanceList", "MetricsSummary");
            app.back_to_list(); // Using core screen navigation
            Ok(false)
        }
        (KeyCode::Char('r'), _) => {
            log_user_interaction!("Manual refresh", "MetricsSummary");
            if let Some(instance_id) = app.get_selected_instance_id() {
                info!(
                    "MANUAL_REFRESH: Refreshing metrics for instance: {}",
                    instance_id
                );
                app.load_metrics(&instance_id).await?; // Using core metrics management
            } else {
                warn!("MANUAL_REFRESH: No instance selected for refresh");
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
        KeyCode::Down | KeyCode::Char('j') => {
            log_user_interaction!("Navigate down", "InstanceDetails");
            app.scroll_down(); // Using core UI state management
            Ok(false)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            log_user_interaction!("Navigate up", "InstanceDetails");
            app.scroll_up(); // Using core UI state management
            Ok(false)
        }
        KeyCode::Esc => {
            log_user_interaction!("Back to MetricsSummary", "InstanceDetails");
            app.back_to_metrics_summary(); // Using core screen navigation
            Ok(false)
        }
        KeyCode::Char('r') => {
            log_user_interaction!("Manual refresh", "InstanceDetails");
            if let Some(instance_id) = app.get_selected_instance_id() {
                info!(
                    "MANUAL_REFRESH: Refreshing instance details for: {}",
                    instance_id
                );
                app.load_metrics(&instance_id).await?; // Using core metrics management
            } else {
                warn!("MANUAL_REFRESH: No instance selected for refresh");
            }
            Ok(false)
        }
        _ => {
            debug!("INSTANCE_DETAILS_EVENT: Unhandled key: {:?}", key_code);
            Ok(false)
        }
    }
}
