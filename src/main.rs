mod app;
mod aws;
mod config;
mod event_handler;
mod models;
mod terminal;
mod ui;

use anyhow::Result;
use clap::Command;
use log::{info, error, debug, warn};
use crossterm::event;

use aws::session::AwsSessionManager;
use event_handler::handle_event;
use models::{App, AppState};
use terminal::TerminalManager;
use ui::render_app;
async fn validate_aws_credentials() -> Result<()> {
    // Use the new centralized session manager for credential validation
    let validation_result = AwsSessionManager::validate_credentials().await?;

    // Display status messages
    for message in &validation_result.status_messages {
        println!("{message}");
    }

    if validation_result.success {
        // Success case - credential info is already included in status messages
        println!();
        Ok(())
    } else {
        // Error case - display error guidance
        for guidance in &validation_result.error_guidance {
            println!("{guidance}");
        }
        println!();

        // Return error if validation failed
        Err(anyhow::anyhow!(
            "AWS credential validation failed: {}",
            validation_result
                .error_message
                .unwrap_or_else(|| "Unknown error".to_string())
        ))
    }
}

async fn run_app(mut terminal: TerminalManager, mut app: App) -> Result<()> {
    // App starts with ServiceList state; instance loading happens via event handler

    loop {
        terminal.draw(|f| render_app(f, &mut app))?;

        // Check for loading timeout
        if app.loading {
            app.check_loading_timeout();
        }

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
            if let Some(service) = app.selected_service.clone() {
                debug!("Auto-refresh triggered for service: {:?}, state: {:?}", service, app.state);
                if let Err(e) = app.load_service_instances(&service).await {
                    error!("Failed to load service instances during auto-refresh: {}", e);
                } else {
                    debug!("Auto-refresh completed successfully for service: {:?}", service);
                }
            } else {
                warn!("Auto-refresh triggered but no service selected");
            }
        }
    }

    Ok(())
}

fn init_logging() {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    // Ensure /tmp directory exists and create log file
    let log_file = "/tmp/aws-cw-tui.log";
    
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
                .expect("Failed to create log file")
        )))
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to file
    init_logging();
    
    info!("AWS CloudWatch TUI starting up");
    
    Command::new("awscw")
        .version("0.1.0")
        .about("AWS CloudWatch TUI")
        .get_matches();

    // Validate AWS credentials before starting the terminal UI
    info!("Validating AWS credentials...");
    if let Err(e) = validate_aws_credentials().await {
        error!("AWS credential validation failed: {}", e);
        println!("Cannot start AWS CloudWatch TUI: {e}");
        std::process::exit(1);
    }
    info!("AWS credentials validated successfully");

    println!("Starting AWS CloudWatch TUI...");
    println!("Press 'q' to quit, 'r' to refresh data");
    println!();

    // Create terminal manager
    let terminal = TerminalManager::new()?;

    // Create app and run - starts with service selection
    let app = App::new();
    let res = run_app(terminal, app).await;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
