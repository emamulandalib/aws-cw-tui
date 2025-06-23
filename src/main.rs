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
async fn validate_aws_credentials() -> Result<()> {
    println!("Checking AWS credentials...");

    // Get current profile info
    let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
    //
    // Try to load AWS config first to get the actual region that will be used
    println!("Loading AWS configuration...");
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;

    // Get the actual region that the AWS SDK will use
    let region = config.region().map(|r| r.as_ref()).unwrap_or("unknown");

    println!("Using AWS Profile: {}", profile);
    println!("Using AWS Region: {}", region);

    // Test credential access with a simple STS call
    println!("Validating credentials...");
    let sts_client = aws_sdk_sts::Client::new(&config);

    match sts_client.get_caller_identity().send().await {
        Ok(identity) => {
            let account_id = identity.account().unwrap_or("Unknown");
            let user_id = identity.user_id().unwrap_or("Unknown");
            println!("AWS credentials validated successfully!");
            println!("   Account ID: {}", account_id);
            println!("   User/Role: {}", user_id);
            println!();
            Ok(())
        }
        Err(e) => {
            println!("AWS credential validation failed!");
            println!();

            let error_msg = e.to_string();
            if error_msg.contains("credential") || error_msg.contains("no providers in chain") {
                println!("Credential issue detected. Please try:");
                println!("   1. Set your AWS profile: export AWS_PROFILE=your-profile-name");
                println!("   2. Or run: aws configure");
                println!("   3. Or set environment variables:");
                println!("      export AWS_ACCESS_KEY_ID=your-access-key");
                println!("      export AWS_SECRET_ACCESS_KEY=your-secret-key");
                println!("   4. Ensure your profile exists in ~/.aws/credentials");
                println!();
                println!(
                    "Current profile '{}' might not exist or be configured correctly.",
                    profile
                );
            } else {
                println!("Error details: {}", error_msg);
            }

            Err(anyhow::anyhow!("AWS credential validation failed"))
        }
    }
}

async fn run_app(mut terminal: TerminalManager, mut app: App) -> Result<()> {
    // Initial load for RDS instances since we start directly with InstanceList
    if app.state == AppState::InstanceList && app.loading {
        if let Some(service) = &app.selected_service {
            match service {
                crate::models::AwsService::Rds => {
                    app.load_rds_instances().await?;
                }
            }
        }
    }

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
            if let Some(service) = &app.selected_service {
                match service {
                    crate::models::AwsService::Rds => {
                        app.load_rds_instances().await?;
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

    // Validate AWS credentials before starting the terminal UI
    if let Err(e) = validate_aws_credentials().await {
        println!("Cannot start AWS CloudWatch TUI: {}", e);
        std::process::exit(1);
    }

    println!("Starting AWS CloudWatch TUI...");
    println!("Press 'q' to quit, 'r' to refresh data");
    println!();

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
