use crate::ui::themes::ThemeVariant;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Comprehensive application configuration with persistence support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration version for migration support
    pub version: String,

    /// Theme configuration
    pub theme: ThemeConfig,

    /// UI preferences and settings
    pub ui: UiConfig,

    /// AWS-specific configuration
    pub aws: AwsConfig,

    /// Performance and optimization settings
    pub performance: PerformanceConfig,
}

/// Theme-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Currently selected theme variant
    pub current_theme: ThemeVariant,

    /// Custom color overrides (if any)
    pub custom_colors: Option<std::collections::HashMap<String, String>>,
}

/// UI preferences and behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Auto-refresh interval in seconds
    pub auto_refresh_interval: u64,

    /// Whether auto-refresh is enabled
    pub auto_refresh_enabled: bool,

    /// Default time range for metrics (e.g., "3h", "1d", "1w")
    pub default_time_range: String,

    /// Show help hints in the interface
    pub show_help_hints: bool,

    /// Animation speed setting
    pub animation_speed: AnimationSpeed,

    /// Number of metrics to display per page
    pub metrics_per_page: usize,

    /// Maximum number of history points to keep
    pub max_history_points: usize,

    /// Loading timeout in seconds
    pub loading_timeout: u64,
}

/// AWS-specific preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    /// Preferred AWS region (overrides environment)
    pub region: Option<String>,

    /// AWS profile to use (overrides environment)
    pub profile: Option<String>,

    /// Maximum retries for AWS requests
    pub max_retries: u32,

    /// Request timeout in seconds
    pub request_timeout: u64,

    /// Metric data period in seconds
    pub metric_data_period: u64,

    /// Default services to show on startup
    pub default_services: Vec<String>,
}

/// Performance and resource optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum concurrent AWS requests
    pub max_concurrent_requests: usize,

    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,

    /// Enable rendering optimizations
    pub render_optimization: bool,

    /// Memory usage limit in MB (0 = unlimited)
    pub memory_limit_mb: usize,
}

/// Animation speed settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationSpeed {
    Disabled,
    Slow,
    Normal,
    Fast,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            theme: ThemeConfig::default(),
            ui: UiConfig::default(),
            aws: AwsConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            current_theme: ThemeVariant::Default,
            custom_colors: None,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            auto_refresh_interval: 30,
            auto_refresh_enabled: true,
            default_time_range: "3h".to_string(),
            show_help_hints: true,
            animation_speed: AnimationSpeed::Normal,
            metrics_per_page: 4,
            max_history_points: 100,
            loading_timeout: 30,
        }
    }
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self {
            region: None,
            profile: None,
            max_retries: 3,
            request_timeout: 10,
            metric_data_period: 300, // 5 minutes
            default_services: vec!["RDS".to_string(), "SQS".to_string()],
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 10,
            cache_ttl_seconds: 300, // 5 minutes
            render_optimization: true,
            memory_limit_mb: 0, // Unlimited
        }
    }
}

impl Default for AnimationSpeed {
    fn default() -> Self {
        AnimationSpeed::Normal
    }
}

impl AppConfig {
    /// Create a new configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Get refresh interval as Duration
    pub fn refresh_interval(&self) -> Duration {
        Duration::from_secs(self.ui.auto_refresh_interval)
    }

    /// Get loading timeout as Duration
    pub fn loading_timeout(&self) -> Duration {
        Duration::from_secs(self.ui.loading_timeout)
    }

    /// Get request timeout as Duration
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.aws.request_timeout)
    }

    /// Get metric data period as Duration
    pub fn metric_data_period(&self) -> Duration {
        Duration::from_secs(self.aws.metric_data_period)
    }

    /// Get cache TTL as Duration
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.performance.cache_ttl_seconds)
    }
}
