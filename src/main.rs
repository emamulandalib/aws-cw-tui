mod app;
mod aws;
mod config;
mod core;
mod event_handler;
mod models;
mod terminal;
mod ui;
mod utils;

use anyhow::Result;
use clap::Command;
use crossterm::event;
use tracing::{debug, error, info, warn};

// Import enhanced logging
use utils::logging::init_logging as init_utils_logging;

// Import core modules for application initialization and management
use config::DebugConfig;

use aws::session::AwsSessionManager;
use event_handler::handle_event;
use models::{App, AppState};
use terminal::TerminalManager;
use ui::render_app;

async fn validate_aws_credentials() -> Result<()> {
    info!("AWS_CREDENTIALS: Starting credential validation");

    // Use the new centralized session manager for credential validation
    let validation_result = AwsSessionManager::validate_credentials().await?;

    // Display status messages
    for message in &validation_result.status_messages {
        println!("{message}");
        debug!("AWS_CREDENTIALS: Status message: {}", message);
    }

    if validation_result.success {
        info!("AWS_CREDENTIALS: Validation successful");
        // Success case - credential info is already included in status messages
        println!();
        Ok(())
    } else {
        error!("AWS_CREDENTIALS: Validation failed");
        // Error case - display error guidance
        for guidance in &validation_result.error_guidance {
            println!("{guidance}");
            error!("AWS_CREDENTIALS: Error guidance: {}", guidance);
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
    info!("Starting main application loop");
    log_state_transition!("Initialization", "ServiceList", "App started successfully");

    // App starts with ServiceList state; instance loading happens via event handler

    loop {
        // Use timed_operation to measure render performance
        crate::timed_operation!("render_frame", {
            terminal.draw(|f| render_app(f, &mut app))?;
        });

        // Check for loading timeout using core state management
        if app.loading {
            app.check_loading_timeout(); // Using core state management
        }

        if let Ok(event) = event::read() {
            debug!(
                event = ?event,
                state = ?app.state,
                "Received event"
            );
            let should_quit = handle_event(&mut app, event).await?;
            if should_quit {
                info!("User requested quit");
                log_state_transition!("Current", "Shutdown", "User quit request");
                break;
            }
        }

        // Auto-refresh logic using core modules
        if app.needs_refresh() {
            // Using core state management
            match app.state {
                AppState::InstanceList => {
                    // Refresh instance list using core service management
                    if let Some(service) = app.selected_service.clone() {
                        debug!(
                            "AUTO_REFRESH: Refreshing instance list for service: {:?}",
                            service
                        );
                        if let Err(e) = app.load_service_instances(&service).await {
                            error!("AUTO_REFRESH: Failed to load service instances: {}", e);
                            app.set_error(format!("Auto-refresh failed: {}", e));
                        // Using core state management
                        } else {
                            debug!("AUTO_REFRESH: Instance list refreshed successfully");
                            app.mark_refreshed(); // Using core state management
                        }
                    } else {
                        warn!("AUTO_REFRESH: Cannot refresh instances - no service selected");
                    }
                }
                AppState::MetricsSummary | AppState::InstanceDetails => {
                    // Refresh metrics data using core metrics management
                    if let Some(instance_id) = app.get_selected_instance_id() {
                        // Using core instance access
                        debug!(
                            "AUTO_REFRESH: Refreshing metrics for instance: {} in state: {:?}",
                            instance_id, app.state
                        );
                        if let Err(e) = app.load_metrics(&instance_id).await {
                            // Using core metrics management
                            error!("AUTO_REFRESH: Failed to refresh metrics: {}", e);
                            app.set_error(format!("Auto-refresh failed: {}", e));
                        // Using core state management
                        } else {
                            debug!("AUTO_REFRESH: Metrics refreshed successfully");
                            app.mark_refreshed(); // Using core state management
                        }
                    } else {
                        warn!("AUTO_REFRESH: Cannot refresh metrics - no instance selected");
                    }
                }
                AppState::ServiceList => {
                    // Service list doesn't need auto-refresh
                    debug!("AUTO_REFRESH: ServiceList state - no auto-refresh needed");
                }
            }
        }
    }

    info!("APP_LIFECYCLE: Application loop ended cleanly");
    Ok(())
}

/// Initialize the logging system using core utilities
fn init_logging() -> DebugConfig {
    // Initialize tracing logger using core utilities
    init_utils_logging();

    // Create debug configuration from environment for backward compatibility
    let debug_config = DebugConfig::from_env();

    // Print debug info if enabled
    debug_config.print_debug_info();

    info!("Enhanced logging initialized with tracing ecosystem");

    debug_config
}

/// Initialize the application using core initialization module
fn init_application() -> App {
    info!("STARTUP: Initializing application using core modules");

    // Create app using core initialization - this automatically sets up:
    // - Proper default states
    // - List state initialization
    // - Error handling
    // - Time range configuration
    let app = App::new(); // Using core initialization module

    info!("STARTUP: Application initialized successfully");
    debug!("STARTUP: Initial state: {:?}", app.state);
    debug!("STARTUP: Available services: {:?}", app.available_services);
    debug!("STARTUP: Focused panel: {:?}", app.focused_panel);

    app
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize enhanced logging system
    let debug_config = init_logging();

    // Log startup information with tracing
    let args: Vec<String> = std::env::args().collect();
    info!(
        args = ?args,
        pid = std::process::id(),
        "AWS CloudWatch TUI starting up"
    );
    info!("DEBUG_CONFIG: Debug mode enabled: {}", debug_config.enabled);
    info!("DEBUG_CONFIG: Log level: {:?}", debug_config.log_level);
    info!(
        "DEBUG_CONFIG: Log file: {}",
        debug_config.log_file_path.display()
    );

    Command::new("awscw")
        .version("0.1.0")
        .about("AWS CloudWatch TUI")
        .get_matches();

    // Validate AWS credentials before starting the terminal UI
    info!("STARTUP: Validating AWS credentials...");
    if let Err(e) = validate_aws_credentials().await {
        error!("STARTUP: AWS credential validation failed: {}", e);
        println!("Cannot start AWS CloudWatch TUI: {e}");
        std::process::exit(1);
    }
    info!("STARTUP: AWS credentials validated successfully");

    println!("Starting AWS CloudWatch TUI...");
    println!("Press 'q' to quit, 'r' to refresh data");

    if debug_config.enabled {
        println!(
            "Debug mode enabled - check {} for detailed logs",
            debug_config.log_file_path.display()
        );
    }
    println!();

    // Create terminal manager
    info!("STARTUP: Initializing terminal manager");
    let terminal = TerminalManager::new()?;

    // Create app using core initialization and run
    info!("STARTUP: Creating application instance using core modules");
    let app = init_application(); // Using core initialization
    let res = run_app(terminal, app).await;

    if let Err(err) = res {
        error!("APP_LIFECYCLE: Application ended with error: {:?}", err);
        println!("{err:?}");
    } else {
        info!("APP_LIFECYCLE: Application ended successfully");
    }

    Ok(())
}
