use crate::models::App;
use std::time::{Duration, Instant};

impl App {
    /// Check if the application needs a data refresh
    pub fn needs_refresh(&self) -> bool {
        if !self.auto_refresh_enabled {
            return false;
        }
        match self.last_refresh {
            None => true,
            Some(last) => last.elapsed() > Duration::from_secs(30),
        }
    }

    /// Mark the current time as when data was last refreshed
    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Some(Instant::now());
    }

    /// Clear any error messages
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Check if loading operations have timed out and handle accordingly
    pub fn check_loading_timeout(&mut self) -> bool {
        if let Some(start_time) = self.loading_start_time {
            if start_time.elapsed() > Duration::from_secs(30) {
                self.loading = false;
                self.loading_start_time = None;
                self.error_message = Some(
                    "Loading timeout - operation took too long. Press 'r' to retry.".to_string(),
                );
                return true;
            }
        }
        false
    }

    /// Set error message with consistent formatting
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error_message = Some(error.into());
        self.loading = false;
        self.loading_start_time = None;
    }

    /// Start loading operation with timeout tracking
    pub fn start_loading(&mut self) {
        self.loading = true;
        self.loading_start_time = Some(Instant::now());
        self.clear_error();
    }

    /// Stop loading operation
    pub fn stop_loading(&mut self) {
        self.loading = false;
        self.loading_start_time = None;
    }
}
