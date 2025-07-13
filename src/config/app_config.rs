use std::time::Duration;

/// Application configuration settings
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub refresh_interval: Duration,
    pub auto_refresh_enabled: bool,
    pub loading_timeout: Duration,
    pub metrics_per_page: usize,
    pub max_history_points: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_secs(30),
            auto_refresh_enabled: true,
            loading_timeout: Duration::from_secs(30),
            metrics_per_page: 4, // 2x2 grid
            max_history_points: 100,
        }
    }
}

impl AppConfig {
    /// Create a new configuration with custom settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set refresh interval
    pub fn with_refresh_interval(mut self, interval: Duration) -> Self {
        self.refresh_interval = interval;
        self
    }

    /// Enable or disable auto-refresh
    pub fn with_auto_refresh(mut self, enabled: bool) -> Self {
        self.auto_refresh_enabled = enabled;
        self
    }

    /// Set loading timeout
    pub fn with_loading_timeout(mut self, timeout: Duration) -> Self {
        self.loading_timeout = timeout;
        self
    }
}
