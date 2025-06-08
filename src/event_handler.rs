use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};
use crate::models::{App, AppState};
use crate::aws::cloudwatch_service::TimeUnit;

pub async fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match app.state {
            AppState::RdsList => handle_rds_list_event(app, key.code).await,
            AppState::MetricsSummary => handle_metrics_summary_event(app, key).await,
            AppState::InstanceDetails => handle_instance_details_event(app, key.code).await,
        }
    } else {
        Ok(false)
    }
}

async fn handle_rds_list_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char('q') => Ok(true), // Signal to quit
        KeyCode::Down => {
            app.next();
            Ok(false)
        },
        KeyCode::Up => {
            app.previous();
            Ok(false)
        },
        KeyCode::Enter => {
            app.enter_metrics_summary();
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?;
            }
            Ok(false)
        },
        KeyCode::Char('r') => {
            app.loading = true;
            app.load_rds_instances().await?;
            Ok(false)
        }
        _ => Ok(false)
    }
}

async fn handle_metrics_summary_event(app: &mut App, key: KeyEvent) -> Result<bool> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => Ok(true), // Signal to quit
        (KeyCode::Down, _) => {
            app.scroll_down();
            Ok(false)
        },
        (KeyCode::Up, _) => {
            app.scroll_up();
            Ok(false)
        },
        (KeyCode::Enter, _) => {
            // If TimeRanges panel is focused, select the current time range
            if matches!(app.get_focused_panel(), crate::models::FocusedPanel::TimeRanges) {
                let current_index = app.get_current_time_range_index();
                app.select_time_range(current_index)?;
                // Reload metrics with new time range
                if let Some(selected) = app.selected_instance {
                    let instance_id = app.rds_instances[selected].identifier.clone();
                    app.load_metrics(&instance_id).await?;
                }
            } else {
                // If Metrics panel is focused, enter instance details
                app.enter_instance_details();
            }
            Ok(false)
        },
        (KeyCode::Tab, _) => {
            app.switch_panel();
            Ok(false)
        },
        (KeyCode::Char('1'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Hours, 1)?;
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        (KeyCode::Char('3'), KeyModifiers::CONTROL) => {
            app.update_time_range(3, TimeUnit::Hours, 1)?;
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        (KeyCode::Char('6'), KeyModifiers::CONTROL) => {
            app.update_time_range(6, TimeUnit::Hours, 1)?;
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Days, 1)?;
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Weeks, 7)?;
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        (KeyCode::Char('m'), KeyModifiers::CONTROL) => {
            app.update_time_range(1, TimeUnit::Months, 30)?;
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        (KeyCode::Char('b'), _) | (KeyCode::Esc, _) => {
            app.back_to_list();
            app.reset_scroll();
            Ok(false)
        },
        (KeyCode::Char('k'), _) => {
            app.scroll_up();
            Ok(false)
        },
        (KeyCode::Char('j'), _) => {
            app.scroll_down();
            Ok(false)
        },
        (KeyCode::Left, _) | (KeyCode::Right, _) => {
            app.switch_panel();
            Ok(false)
        },
        (KeyCode::Home, _) => {
            app.reset_scroll();
            Ok(false)
        },
        (KeyCode::Char('r'), _) => {
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?
            }
            Ok(false)
        },
        _ => Ok(false)
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
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?;
            }
            Ok(false)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.scroll_up();
            Ok(false)
        },
        KeyCode::Down | KeyCode::Char('j') => {
            app.scroll_down();
            Ok(false)
        },
        KeyCode::Home => {
            app.reset_scroll();
            Ok(false)
        },
        _ => Ok(false)
    }
}
