use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Debug configuration for enhanced logging capabilities
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
    pub log_file_path: PathBuf,
    pub log_level: DebugLevel,
    pub log_state_transitions: bool,
    pub log_user_interactions: bool,
    pub log_metric_operations: bool,
    pub log_aws_operations: bool,
    pub log_ui_events: bool,
    pub log_navigation: bool,
    pub include_timestamps: bool,
    pub include_thread_id: bool,
    pub max_log_file_size_mb: u64,
}

/// Debug logging levels
#[derive(Debug, Clone, PartialEq)]
pub enum DebugLevel {
    /// Only critical errors
    Error,
    /// Warnings and errors
    Warn,
    /// General information, warnings, and errors
    Info,
    /// Detailed debugging information
    Debug,
    /// Extremely verbose tracing
    Trace,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            log_file_path: PathBuf::from("/tmp/aws-cw-tui-debug.log"),
            log_level: DebugLevel::Info,
            log_state_transitions: true,
            log_user_interactions: true,
            log_metric_operations: true,
            log_aws_operations: true,
            log_ui_events: true,
            log_navigation: true,
            include_timestamps: true,
            include_thread_id: false,
            max_log_file_size_mb: 50,
        }
    }
}

impl DebugConfig {
    /// Create a new debug configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create debug configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Check if debug mode is enabled
        config.enabled = env::var("AWS_CW_TUI_DEBUG")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        // Set log level from environment
        if let Ok(level_str) = env::var("AWS_CW_TUI_LOG_LEVEL") {
            config.log_level = match level_str.to_lowercase().as_str() {
                "error" => DebugLevel::Error,
                "warn" | "warning" => DebugLevel::Warn,
                "info" => DebugLevel::Info,
                "debug" => DebugLevel::Debug,
                "trace" => DebugLevel::Trace,
                _ => DebugLevel::Info,
            };
        }

        // Set log file path from environment
        if let Ok(log_path) = env::var("AWS_CW_TUI_LOG_FILE") {
            config.log_file_path = PathBuf::from(log_path);
        }

        // Enable detailed logging in debug mode
        if config.enabled {
            config.log_level = DebugLevel::Debug;
            config.log_state_transitions = true;
            config.log_user_interactions = true;
            config.log_metric_operations = true;
            config.log_aws_operations = true;
            config.log_ui_events = true;
            config.log_navigation = true;
            config.include_timestamps = true;
            config.include_thread_id = true;
        }

        config
    }

    /// Get log filter level for env_logger
    pub fn get_log_filter_level(&self) -> log::LevelFilter {
        match self.log_level {
            DebugLevel::Error => log::LevelFilter::Error,
            DebugLevel::Warn => log::LevelFilter::Warn,
            DebugLevel::Info => log::LevelFilter::Info,
            DebugLevel::Debug => log::LevelFilter::Debug,
            DebugLevel::Trace => log::LevelFilter::Trace,
        }
    }

    /// Create the log file and ensure directory exists
    pub fn ensure_log_file(&self) -> std::io::Result<()> {
        if let Some(parent) = self.log_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create or truncate the log file
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.log_file_path)?;

        Ok(())
    }

    /// Get debug-specific log file path
    pub fn get_debug_log_file(&self) -> PathBuf {
        if self.enabled {
            self.log_file_path.clone()
        } else {
            PathBuf::from("/tmp/aws-cw-tui.log")
        }
    }

    /// Create a custom log format function
    pub fn create_log_formatter(
        &self,
    ) -> Box<dyn Fn(&mut dyn Write, &log::Record) -> std::io::Result<()> + Send + Sync> {
        let include_timestamps = self.include_timestamps;
        let include_thread_id = self.include_thread_id;

        Box::new(move |buf, record| {
            let mut output = String::new();

            if include_timestamps {
                output.push_str(&format!(
                    "{} ",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
                ));
            }

            output.push_str(&format!("[{}]", record.level()));

            if include_thread_id {
                output.push_str(&format!(" [thread:{:?}]", std::thread::current().id()));
            }

            output.push_str(&format!(" {} - {}", record.target(), record.args()));

            writeln!(buf, "{}", output)
        })
    }

    /// Print debug mode information
    pub fn print_debug_info(&self) {
        if self.enabled {
            println!("=== AWS CloudWatch TUI Debug Mode Enabled ===");
            println!("Debug Level: {:?}", self.log_level);
            println!("Log File: {}", self.log_file_path.display());
            println!("State Transitions: {}", self.log_state_transitions);
            println!("User Interactions: {}", self.log_user_interactions);
            println!("Metric Operations: {}", self.log_metric_operations);
            println!("AWS Operations: {}", self.log_aws_operations);
            println!("UI Events: {}", self.log_ui_events);
            println!("Navigation: {}", self.log_navigation);
            println!("=============================================");
            println!();
        }
    }
}

// Note: Logging macros moved to utils::logging module
