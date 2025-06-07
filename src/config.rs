use std::time::Duration;

#[allow(dead_code)]
pub struct Config {
    pub auto_refresh_enabled: bool,
    pub refresh_interval: Duration,
    pub metrics_per_screen: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_refresh_enabled: true,
            refresh_interval: Duration::from_secs(60),
            metrics_per_screen: 1,
        }
    }
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
