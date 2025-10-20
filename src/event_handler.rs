use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use crate::models::{App, AppState};

pub async fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match app.state {
            AppState::RdsList => handle_rds_list_event(app, key.code).await,
            AppState::MetricsSummary => handle_metrics_summary_event(app, key.code).await,
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

async fn handle_metrics_summary_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char('q') => Ok(true), // Signal to quit
        KeyCode::Char('b') | KeyCode::Esc => {
            app.back_to_list();
            app.reset_scroll();
            Ok(false)
        },
        KeyCode::Enter => {
            app.enter_instance_details();
            Ok(false)
        },
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
        KeyCode::Char('r') => {
            if let Some(selected) = app.selected_instance {
                let instance_id = app.rds_instances[selected].identifier.clone();
                app.load_metrics(&instance_id).await?;
            }
            Ok(false)
        }
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
