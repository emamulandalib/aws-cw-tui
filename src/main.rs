mod app;
mod aws;
mod config;
mod event_handler;
mod models;
mod terminal;
mod ui;

use anyhow::Result;
use clap::Command;
use crossterm::event;

use event_handler::handle_event;
use models::{App, AppState};
use terminal::TerminalManager;
use ui::render_app;

async fn run_app(mut terminal: TerminalManager, mut app: App) -> Result<()> {
    // Start with service selection - no need to load RDS instances immediately
    // They will be loaded when a service is selected

    loop {
        terminal.draw(|f| render_app(f, &mut app))?;

        if let Ok(event) = event::read() {
            let should_quit = handle_event(&mut app, event).await?;
            if should_quit {
                break;
            }
        }

        // Auto-refresh logic - only refresh if we're in a state that needs data
        if app.needs_refresh()
            && matches!(
                app.state,
                AppState::InstanceList | AppState::MetricsSummary | AppState::InstanceDetails
            )
        {
            if let Some(service) = &app.selected_service {
                match service {
                    crate::models::AwsService::Rds => {
                        app.load_rds_instances().await?;
                    }
                    crate::models::AwsService::Sqs => {
                        // TODO: Refresh SQS queues when SQS support is implemented
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _matches = Command::new("awscw")
        .version("0.1.0")
        .about("AWS CloudWatch TUI")
        .get_matches();

    // Create terminal manager
    let terminal = TerminalManager::new()?;

    // Create app and run - starts with service selection
    let app = App::new();
    let res = run_app(terminal, app).await;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
