use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::{debug, info};

pub struct TerminalManager {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl TerminalManager {
    pub fn new() -> Result<Self> {
        info!("Initializing terminal manager");

        enable_raw_mode()?;
        debug!("Raw mode enabled");

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        debug!("Alternate screen and mouse capture enabled");

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let size = terminal.size()?;
        info!(
            width = size.width,
            height = size.height,
            "Terminal initialized successfully"
        );

        Ok(Self { terminal })
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        info!("Restoring terminal to normal state");

        disable_raw_mode()?;
        debug!("Raw mode disabled");

        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        debug!("Left alternate screen and disabled mouse capture");

        self.terminal.show_cursor()?;
        debug!("Cursor shown");

        info!("Terminal restored successfully");
        Ok(())
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        debug!("TerminalManager dropping, restoring terminal");
        let _ = self.restore();
    }
}
