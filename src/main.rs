mod models;
mod aws;
mod app;
mod ui;
mod event_handler;
mod terminal;
mod config;

#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::{Arg, Command};
use crossterm::event;
use tokio;

use models::App;
use ui::render_app;
use event_handler::handle_event;
use terminal::TerminalManager;

async fn run_app(mut terminal: TerminalManager, mut app: App) -> Result<()> {
    // Load RDS instances
    app.load_rds_instances().await?;

    loop {
        terminal.draw(|f| render_app(f, &mut app))?;

        if let Ok(event) = event::read() {
            let should_quit = handle_event(&mut app, event).await?;
            if should_quit {
                break;
            }
        }

        // Auto-refresh logic
        if app.needs_refresh() {
            app.load_rds_instances().await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("awscw")
        .version("0.1.0")
        .about("AWS CloudWatch TUI")
        .arg(
            Arg::new("rds")
                .long("rds")
                .help("Show RDS instances")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("rds") {
        // Create terminal manager
        let terminal = TerminalManager::new()?;

        // Create app and run
        let app = App::new();
        let res = run_app(terminal, app).await;

        if let Err(err) = res {
            println!("{:?}", err);
        }
    } else {
        println!("Use --rds to show RDS instances");
        println!("Example: awscw --rds");
    }

    Ok(())
}
