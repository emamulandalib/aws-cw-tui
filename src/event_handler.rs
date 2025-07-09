use crate::aws::cloudwatch_service::TimeUnit;
use crate::models::{App, AppState};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use log::{debug, info, warn};

pub async fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match app.state {
            AppState::ServiceList => handle_service_list_event(app, key.code).await,
            AppState::InstanceList => handle_rds_list_event(app, key.code).await,
            AppState::MetricsSummary => handle_metrics_summary_event(app, key).await,
            AppState::InstanceDetails => handle_instance_details_event(app, key.code).await,
        }
    } else {
        Ok(false)
    }
}

async fn handle_service_list_event(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Down | KeyCode::Char('j') => {
            app.service_next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.service_previous();
        }
        KeyCode::Enter => {
            let selected_service = app.select_service().cloned();
            if let Some(service) = selected_service {
                info!("User selected service: {:?}, loading instances...", service);
                app.load_service_instances(&service).await?;
            } else {
                warn!("No service selected when Enter pressed");
            }
        }
        _ => {}
    }
    Ok(false)
}

async fn handle_rds_list_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    // If we're in loading state, allow certain keys to work
    if app.loading {
        match key_code {
            KeyCode::Char('q') => return Ok(true), // Always allow quit
            KeyCode::Esc => {
                // Allow going back even during loading
                app.loading = false;
                app.back_to_service_list();
                return Ok(false);
            }
            _ => return Ok(false), // Ignore other keys during loading
        }
    }

    match key_code {
        KeyCode::Char('q') => Ok(true), // Signal to quit
        KeyCode::Esc => {
            app.back_to_service_list(); // Navigate back to service selection
            Ok(false)
        }
        KeyCode::Down => {
            app.next();
            Ok(false)
        }
        KeyCode::Up => {
            app.previous();
            Ok(false)
        }
        KeyCode::Enter => {
            app.enter_metrics_summary();
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?;
            }
            Ok(false)
        }
        KeyCode::Char('d') => {
            // NEW: Test dynamic metrics loading with 'd' key
            info!("User triggered dynamic metrics loading test");
            app.enter_metrics_summary();
            if let Some(instance_id) = app.get_selected_instance_id() {
                info!("Loading dynamic metrics for instance: {}", instance_id);
                app.load_metrics_dynamic(&instance_id).await?;
            }
            Ok(false)
        }
        KeyCode::Char('r') => {
            info!("User triggered manual refresh");
            app.loading = true;
            let selected_service = app.selected_service.clone();
            if let Some(service) = selected_service {
                debug!("Manual refresh for service: {:?}", service);
                app.load_service_instances(&service).await?;
            } else {
                warn!("Manual refresh triggered but no service selected");
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

async fn handle_metrics_summary_event(app: &mut App, key: KeyEvent) -> Result<bool> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => Ok(true), // Signal to quit
        (KeyCode::Down, _) => {
            match app.get_focused_panel() {
                crate::models::FocusedPanel::SparklineGrid => {
                    app.sparkline_grid_scroll_down(); // Navigate down within grid
                }
                _ => {
                    app.scroll_down(); // Default scroll behavior for other panels
                }
            }
            Ok(false)
        }
        (KeyCode::Up, _) => {
            match app.get_focused_panel() {
                crate::models::FocusedPanel::SparklineGrid => {
                    app.sparkline_grid_scroll_up(); // Navigate up within grid
                }
                _ => {
                    app.scroll_up(); // Default scroll behavior for other panels
                }
            }
            Ok(false)
        }
        (KeyCode::Left, _) => {
            match app.get_focused_panel() {
                crate::models::FocusedPanel::Timezone => {
                    app.timezone_scroll_up(); // Left acts like up in timezone selection
                }
                crate::models::FocusedPanel::Period => {
                    app.period_scroll_up(); // Left acts like up in period selection
                }
                crate::models::FocusedPanel::TimeRanges => {
                    app.time_range_scroll_left();
                }
                crate::models::FocusedPanel::SparklineGrid => {
                    app.sparkline_grid_scroll_left(); // Navigate left within grid
                }
            }
            Ok(false)
        }
        (KeyCode::Right, _) => {
            match app.get_focused_panel() {
                crate::models::FocusedPanel::Timezone => {
                    app.timezone_scroll_down(); // Right acts like down in timezone selection
                }
                crate::models::FocusedPanel::Period => {
                    app.period_scroll_down(); // Right acts like down in period selection
                }
                crate::models::FocusedPanel::TimeRanges => {
                    app.time_range_scroll_right();
                }
                crate::models::FocusedPanel::SparklineGrid => {
                    app.sparkline_grid_scroll_right(); // Navigate right within grid
                }
            }
            Ok(false)
        }
        (KeyCode::Enter, _) => {
            match app.get_focused_panel() {
                crate::models::FocusedPanel::Timezone => {
                    // Timezone selection completed - trigger refresh for any selected instance
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        app.load_metrics(&instance_id).await?;
                    }
                }
                crate::models::FocusedPanel::Period => {
                    // Period selection completed - trigger refresh for any selected instance
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        app.load_metrics(&instance_id).await?;
                    }
                }
                crate::models::FocusedPanel::TimeRanges => {
                    // Select the current time range and reload metrics
                    let current_index = app.get_current_time_range_index();
                    app.select_time_range(current_index)?;
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        app.load_metrics(&instance_id).await?;
                    }
                }
                crate::models::FocusedPanel::SparklineGrid => {
                    // Navigate to Instance Details when Enter is pressed on SparklineGrid
                    app.enter_instance_details();
                }
            }
            Ok(false)
        }
        (KeyCode::Char('t'), _) => {
            // Toggle between absolute and relative time range modes
            app.toggle_time_range_mode();
            Ok(false)
        }
        (KeyCode::Tab, _) => {
            // Cycle through Timezone → Period → TimeRanges → SparklineGrid panels
            app.switch_panel();
            Ok(false)
        }
        (KeyCode::Char('1'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Hours, 1)?;
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        (KeyCode::Char('3'), KeyModifiers::CONTROL) => {
            app.update_time_range(3, TimeUnit::Hours, 1)?;
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        (KeyCode::Char('6'), KeyModifiers::CONTROL) => {
            app.update_time_range(6, TimeUnit::Hours, 1)?;
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Days, 1)?;
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Weeks, 7)?;
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        (KeyCode::Char('m'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Months, 30)?;
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        (KeyCode::Char('b'), _) | (KeyCode::Esc, _) => {
            app.back_to_list();
            app.reset_scroll();
            Ok(false)
        }
        (KeyCode::Char('k'), _) => {
            app.scroll_up();
            Ok(false)
        }
        (KeyCode::Char('j'), _) => {
            app.scroll_down();
            Ok(false)
        }

        (KeyCode::Home, _) => {
            app.reset_scroll();
            Ok(false)
        }
        (KeyCode::Char('r'), _) => {
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

async fn handle_instance_details_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char('q') => Ok(true), // Signal to quit
        KeyCode::Char('b') | KeyCode::Esc => {
            app.back_to_metrics_summary();
            // Don't reset scroll - back_to_metrics_summary() already restores the position
            Ok(false)
        }
        KeyCode::Char('r') => {
            if let Some(instance_id) = app.get_selected_instance_id() {
                app.load_metrics(&instance_id).await?;
            }
            Ok(false)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.scroll_up();
            Ok(false)
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.scroll_down();
            Ok(false)
        }
        KeyCode::Home => {
            app.reset_scroll();
            Ok(false)
        }
        _ => Ok(false),
    }
}
