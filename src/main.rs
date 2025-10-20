mod models;
mod aws;
mod app;
mod ui;

use anyhow::Result;
use clap::{Arg, Command};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use tokio;

use models::{App, AppState};
use ui::ui;

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    // Load RDS instances
    app.load_rds_instances().await?;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::RdsList => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Enter => {
                            app.enter_instance_details();
                            if let Some(selected) = app.selected_instance {
                                let instance_id = app.rds_instances[selected].identifier.clone();
                                app.load_metrics(&instance_id).await?;
                            }
                        },
                        KeyCode::Char('r') => {
                            app.loading = true;
                            app.load_rds_instances().await?;
                        }
                        _ => {}
                    }
                },
                AppState::InstanceDetails => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('b') | KeyCode::Esc => {
                            app.back_to_list();
                            app.reset_scroll();
                        }
                        KeyCode::Char('r') => {
                            if let Some(selected) = app.selected_instance {
                                let instance_id = app.rds_instances[selected].identifier.clone();
                                app.load_metrics(&instance_id).await?;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                        KeyCode::Home => app.reset_scroll(),
                        _ => {}
                    }
                }
            }
        }

        // Auto-refresh logic
        if app.needs_refresh() {
            app.load_rds_instances().await?;
        }
    }
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
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Create app and run
        let app = App::new();
        let res = run_app(&mut terminal, app).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }
    } else {
        println!("Use --rds to show RDS instances");
        println!("Example: awscw --rds");
    }

    Ok(())
}
