use crate::config::app_config::AppConfig;
use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Configuration manager with file-based persistence and environment variable support
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
    config_path: PathBuf,
    watchers: Vec<Box<dyn ConfigWatcher + Send + Sync>>,
}

/// Trait for configuration change watchers
pub trait ConfigWatcher {
    fn on_config_changed(&self, config: &AppConfig);
}

/// Configuration sources for loading configuration
pub trait ConfigSource {
    fn load_config(&self) -> Result<AppConfig>;
    fn save_config(&self, config: &AppConfig) -> Result<()>;
}

/// File-based configuration source
pub struct FileConfigSource {
    path: PathBuf,
}

/// Environment variable configuration source
pub struct EnvironmentConfigSource;

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },

    #[error("Invalid configuration format: {message}")]
    InvalidFormat { message: String },

    #[error("Configuration validation failed: {message}")]
    ValidationFailed { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_create_config(&config_path)?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
            watchers: Vec::new(),
        })
    }

    /// Create configuration manager with custom path
    pub fn with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_path = path.as_ref().to_path_buf();
        let config = Self::load_or_create_config(&config_path)?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
            watchers: Vec::new(),
        })
    }

    /// Get the default configuration file path
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .context("Could not determine configuration directory")?;

        let app_config_dir = config_dir.join("awscw");

        // Create config directory if it doesn't exist
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)
                .context("Failed to create configuration directory")?;
        }

        Ok(app_config_dir.join("config.json"))
    }

    /// Load configuration from file or create default if not exists
    fn load_or_create_config(path: &Path) -> Result<AppConfig> {
        if path.exists() {
            Self::load_config_from_file(path)
        } else {
            let mut config = AppConfig::default();
            Self::apply_environment_overrides(&mut config);
            Self::save_config_to_file(&config, path)?;
            Ok(config)
        }
    }

    /// Load configuration from file with validation and fallback
    fn load_config_from_file(path: &Path) -> Result<AppConfig> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        // Try to parse the configuration
        match serde_json::from_str::<AppConfig>(&content) {
            Ok(config) => {
                // Validate configuration
                Self::validate_config(&config)?;

                Ok(config)
            }
            Err(e) => {
                // If parsing fails, create backup and use defaults
                let backup_path = path.with_extension("json.backup");
                if let Err(backup_err) = fs::copy(path, &backup_path) {
                    eprintln!(
                        "Warning: Failed to create backup of corrupted config: {}",
                        backup_err
                    );
                }

                eprintln!(
                    "Warning: Configuration file is corrupted ({}), using defaults",
                    e
                );
                let config = AppConfig::default();
                Self::save_config_to_file(&config, path)?;
                Ok(config)
            }
        }
    }

    /// Save configuration to file
    fn save_config_to_file(config: &AppConfig, path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content =
            serde_json::to_string_pretty(config).context("Failed to serialize configuration")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Apply environment variable overrides to configuration
    fn apply_environment_overrides(config: &mut AppConfig) {
        // AWS configuration overrides
        if let Ok(region) = std::env::var("AWS_DEFAULT_REGION") {
            config.aws.region = Some(region);
        }

        if let Ok(profile) = std::env::var("AWS_PROFILE") {
            config.aws.profile = Some(profile);
        }

        // UI configuration overrides
        if let Ok(theme) = std::env::var("AWSCW_THEME") {
            if let Ok(theme_variant) = Self::parse_theme_variant(&theme) {
                config.theme.current_theme = theme_variant;
            }
        }

        if let Ok(refresh_interval) = std::env::var("AWSCW_REFRESH_INTERVAL") {
            if let Ok(interval) = refresh_interval.parse::<u64>() {
                config.ui.auto_refresh_interval = interval;
            }
        }

        if let Ok(auto_refresh) = std::env::var("AWSCW_AUTO_REFRESH") {
            config.ui.auto_refresh_enabled = auto_refresh.to_lowercase() == "true";
        }

        // Performance configuration overrides
        if let Ok(max_requests) = std::env::var("AWSCW_MAX_CONCURRENT_REQUESTS") {
            if let Ok(max) = max_requests.parse::<usize>() {
                config.performance.max_concurrent_requests = max;
            }
        }
    }

    /// Parse theme variant from string
    fn parse_theme_variant(theme_str: &str) -> Result<crate::ui::themes::ThemeVariant> {
        use crate::ui::themes::ThemeVariant;

        match theme_str.to_lowercase().as_str() {
            "default" => Ok(ThemeVariant::Default),
            "warm_sunset" | "warmsunset" => Ok(ThemeVariant::WarmSunset),
            "blue_gold" | "bluegold" => Ok(ThemeVariant::BlueGold),
            "high_contrast" | "highcontrast" => Ok(ThemeVariant::HighContrast),
            "monochrome" => Ok(ThemeVariant::Monochrome),
            "terminal_cyan" | "terminalcyan" => Ok(ThemeVariant::TerminalCyan),
            _ => Err(anyhow::anyhow!("Unknown theme variant: {}", theme_str)),
        }
    }

    /// Validate configuration values
    fn validate_config(config: &AppConfig) -> Result<()> {
        // Validate refresh interval
        if config.ui.auto_refresh_interval == 0 {
            return Err(ConfigError::ValidationFailed {
                message: "Auto refresh interval must be greater than 0".to_string(),
            }
            .into());
        }

        // Validate timeout values
        if config.ui.loading_timeout == 0 {
            return Err(ConfigError::ValidationFailed {
                message: "Loading timeout must be greater than 0".to_string(),
            }
            .into());
        }

        if config.aws.request_timeout == 0 {
            return Err(ConfigError::ValidationFailed {
                message: "Request timeout must be greater than 0".to_string(),
            }
            .into());
        }

        // Validate metrics per page
        if config.ui.metrics_per_page == 0 {
            return Err(ConfigError::ValidationFailed {
                message: "Metrics per page must be greater than 0".to_string(),
            }
            .into());
        }

        // Validate performance settings
        if config.performance.max_concurrent_requests == 0 {
            return Err(ConfigError::ValidationFailed {
                message: "Max concurrent requests must be greater than 0".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Get current configuration (read-only)
    pub fn get_config(&self) -> AppConfig {
        self.config.read().unwrap().clone()
    }

    /// Update configuration and save to file
    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        {
            let mut config = self.config.write().unwrap();
            updater(&mut config);

            // Validate updated configuration
            Self::validate_config(&config)?;

            // Save to file
            Self::save_config_to_file(&config, &self.config_path)?;
        }

        // Notify watchers
        let config = self.get_config();
        for watcher in &self.watchers {
            watcher.on_config_changed(&config);
        }

        Ok(())
    }

    /// Set a specific configuration value
    pub fn set_theme(&self, theme: crate::ui::themes::ThemeVariant) -> Result<()> {
        self.update_config(|config| {
            config.theme.current_theme = theme;
        })
    }

    /// Set auto-refresh interval
    pub fn set_auto_refresh_interval(&self, interval_seconds: u64) -> Result<()> {
        self.update_config(|config| {
            config.ui.auto_refresh_interval = interval_seconds;
        })
    }

    /// Set auto-refresh enabled state
    pub fn set_auto_refresh_enabled(&self, enabled: bool) -> Result<()> {
        self.update_config(|config| {
            config.ui.auto_refresh_enabled = enabled;
        })
    }

    /// Set default time range
    pub fn set_default_time_range(&self, time_range: String) -> Result<()> {
        self.update_config(|config| {
            config.ui.default_time_range = time_range;
        })
    }

    /// Set AWS region preference
    pub fn set_aws_region(&self, region: Option<String>) -> Result<()> {
        self.update_config(|config| {
            config.aws.region = region;
        })
    }

    /// Set AWS profile preference
    pub fn set_aws_profile(&self, profile: Option<String>) -> Result<()> {
        self.update_config(|config| {
            config.aws.profile = profile;
        })
    }

    /// Reset configuration to defaults
    pub fn reset_to_defaults(&self) -> Result<()> {
        self.update_config(|config| {
            *config = AppConfig::default();
        })
    }

    /// Add a configuration watcher
    pub fn add_watcher(&mut self, watcher: Box<dyn ConfigWatcher + Send + Sync>) {
        self.watchers.push(watcher);
    }

    /// Reload configuration from file
    pub fn reload(&self) -> Result<()> {
        let new_config = Self::load_config_from_file(&self.config_path)?;

        {
            let mut config = self.config.write().unwrap();
            *config = new_config;
        }

        // Notify watchers
        let config = self.get_config();
        for watcher in &self.watchers {
            watcher.on_config_changed(&config);
        }

        Ok(())
    }

    /// Get configuration file path
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}

impl FileConfigSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl ConfigSource for FileConfigSource {
    fn load_config(&self) -> Result<AppConfig> {
        ConfigManager::load_config_from_file(&self.path)
    }

    fn save_config(&self, config: &AppConfig) -> Result<()> {
        ConfigManager::save_config_to_file(config, &self.path)
    }
}

impl ConfigSource for EnvironmentConfigSource {
    fn load_config(&self) -> Result<AppConfig> {
        let mut config = AppConfig::default();
        ConfigManager::apply_environment_overrides(&mut config);
        Ok(config)
    }

    fn save_config(&self, _config: &AppConfig) -> Result<()> {
        // Environment variables are read-only, so this is a no-op
        Ok(())
    }
}

// Include test module
#[cfg(test)]
#[path = "test_config_manager.rs"]
mod test_config_manager;
