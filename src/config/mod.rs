// Configuration modules
pub mod app_config;
pub mod aws_config;
pub mod config_manager;
pub mod debug_config;
pub mod ui_config;

// Re-export main configuration types
pub use app_config::{
    AnimationSpeed, AppConfig, AwsConfig, PerformanceConfig, ThemeConfig, UiConfig,
};
pub use config_manager::{ConfigError, ConfigManager, ConfigSource, ConfigWatcher};
pub use debug_config::DebugConfig;
