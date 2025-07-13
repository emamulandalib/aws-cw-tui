use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Enhanced logging configuration using tracing
pub struct TracingLogger {
    pub enabled: bool,
    pub log_file_path: PathBuf,
    pub _guard: Option<WorkerGuard>,
}

impl TracingLogger {
    /// Initialize tracing logger with file output
    pub fn init() -> Self {
        let enabled = std::env::var("AWS_CW_TUI_DEBUG").is_ok();
        let log_file_path = PathBuf::from(
            std::env::var("AWS_CW_TUI_LOG_FILE")
                .unwrap_or_else(|_| "/tmp/aws-cw-tui-debug.log".to_string()),
        );

        // Create log directory if it doesn't exist
        if let Some(parent) = log_file_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let file_appender = tracing_appender::rolling::never(
            log_file_path.parent().unwrap_or_else(|| std::path::Path::new("/tmp")),
            log_file_path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("aws-cw-tui.log")),
        );

        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        // Configure tracing subscriber
        let env_filter = if enabled {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
        } else {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
        };

        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::Layer::new()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(true)
                    .with_timer(fmt::time::ChronoLocal::default())
                    .with_thread_ids(true)
                    .with_thread_names(true)
            );

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        // Initialize color-eyre for better error reporting
        if enabled {
            color_eyre::install().expect("Failed to install color-eyre");
        }

        info!("Enhanced logging initialized with tracing");
        info!("Debug mode: {}", enabled);
        info!("Log file: {}", log_file_path.display());

        Self {
            enabled,
            log_file_path,
            _guard: Some(guard),
        }
    }

    /// Log with structured context
    pub fn log_with_context(&self, level: &str, message: &str, context: &[(&str, &str)]) {
        let span = tracing::info_span!("context", message = message);
        let _enter = span.enter();

        for (key, value) in context {
            tracing::info!(key = key, value = value);
        }

        match level {
            "error" => error!("{}", message),
            "warn" => warn!("{}", message),
            "info" => info!("{}", message),
            "debug" => debug!("{}", message),
            "trace" => trace!("{}", message),
            _ => info!("{}", message),
        }
    }
}

// Enhanced logging macros using tracing
#[macro_export]
macro_rules! log_key_press {
    ($key:expr, $modifiers:expr, $state:expr) => {
        tracing::debug!(
            key = ?$key,
            modifiers = ?$modifiers,
            state = ?$state,
            timestamp = ?std::time::SystemTime::now(),
            "Key pressed"
        );
    };
    ($key:expr, $modifiers:expr, $state:expr, $panel:expr) => {
        tracing::debug!(
            key = ?$key,
            modifiers = ?$modifiers,
            state = ?$state,
            panel = ?$panel,
            timestamp = ?std::time::SystemTime::now(),
            "Key pressed with panel context"
        );
    };
}

#[macro_export]
macro_rules! log_ui_render {
    ($component:expr, $area:expr) => {
        tracing::debug!(
            component = $component,
            area = ?$area,
            "UI component rendering"
        );
    };
    ($component:expr, $area:expr, $context:expr) => {
        tracing::debug!(
            component = $component,
            area = ?$area,
            context = $context,
            "UI component rendering with context"
        );
    };
}

#[macro_export]
macro_rules! log_performance {
    ($operation:expr, $duration:expr) => {
        tracing::info!(
            operation = $operation,
            duration = ?$duration,
            "Performance measurement"
        );
    };
    ($operation:expr, $duration:expr, $context:expr) => {
        tracing::info!(
            operation = $operation,
            duration = ?$duration,
            context = $context,
            "Performance measurement with context"
        );
    };
}

#[macro_export]
macro_rules! log_error_context {
    ($error:expr, $context:expr) => {
        tracing::error!(
            error = %$error,
            context = $context,
            "Error occurred"
        );
    };
    ($error:expr, $context:expr, $details:expr) => {
        tracing::error!(
            error = %$error,
            context = $context,
            details = $details,
            "Error occurred with details"
        );
    };
}

#[macro_export]
macro_rules! log_state_change {
    ($from:expr, $to:expr, $field:expr) => {
        tracing::info!(
            from = ?$from,
            to = ?$to,
            field = $field,
            "State changed"
        );
    };
    ($from:expr, $to:expr, $field:expr, $context:expr) => {
        tracing::info!(
            from = ?$from,
            to = ?$to,
            field = $field,
            context = $context,
            "State changed with context"
        );
    };
}

#[macro_export]
macro_rules! log_aws_operation {
    ($operation:expr, $service:expr, $duration:expr) => {
        tracing::info!(
            operation = $operation,
            service = $service,
            duration = ?$duration,
            "AWS operation completed"
        );
    };
    ($operation:expr, $service:expr, $duration:expr, $context:expr) => {
        tracing::info!(
            operation = $operation,
            service = $service,
            duration = ?$duration,
            context = $context,
            "AWS operation completed with context"
        );
    };
}

#[macro_export]
macro_rules! log_navigation {
    ($from:expr, $to:expr, $method:expr) => {
        tracing::debug!(
            from = $from,
            to = $to,
            method = $method,
            "Navigation event"
        );
    };
}

#[macro_export]
macro_rules! log_focus_change {
    ($from:expr, $to:expr) => {
        tracing::debug!(
            from = ?$from,
            to = ?$to,
            "Focus changed"
        );
    };
}

#[macro_export]
macro_rules! log_metric_operation {
    ($operation:expr, $metric:expr, $context:expr) => {
        tracing::info!(
            operation = $operation,
            metric = $metric,
            context = $context,
            "Metric operation"
        );
    };
    ($operation:expr, $metric:expr) => {
        tracing::info!(
            operation = $operation,
            metric = $metric,
            "Metric operation"
        );
    };
}

#[macro_export]
macro_rules! log_user_interaction {
    ($action:expr, $context:expr) => {
        tracing::debug!(
            action = $action,
            context = $context,
            "User interaction"
        );
    };
    ($action:expr) => {
        tracing::debug!(
            action = $action,
            "User interaction"
        );
    };
}

#[macro_export]
macro_rules! log_state_transition {
    ($from:expr, $to:expr, $context:expr) => {
        tracing::info!(
            from = $from,
            to = $to,
            context = $context,
            "State transition"
        );
    };
    ($from:expr, $to:expr) => {
        tracing::info!(
            from = $from,
            to = $to,
            "State transition"
        );
    };
}

/// Create a span for tracking operation duration
#[macro_export]
macro_rules! timed_operation {
    ($name:expr, $operation:expr) => {{
        let span = tracing::info_span!("timed_operation", name = $name);
        let _enter = span.enter();
        let start = std::time::Instant::now();
        let result = $operation;
        let duration = start.elapsed();
        tracing::info!(
            operation = $name,
            duration = ?duration,
            "Operation completed"
        );
        result
    }};
}

/// Create a span for UI rendering operations
#[macro_export]
macro_rules! ui_span {
    ($component:expr, $operation:expr) => {{
        let span = tracing::debug_span!("ui_render", component = $component);
        let _enter = span.enter();
        $operation
    }};
}

/// Global tracing logger instance
static mut TRACING_LOGGER: Option<TracingLogger> = None;

/// Initialize the global tracing logger
pub fn init_tracing_logger() -> &'static TracingLogger {
    unsafe {
        if TRACING_LOGGER.is_none() {
            TRACING_LOGGER = Some(TracingLogger::init());
        }
        TRACING_LOGGER.as_ref().unwrap()
    }
}

/// Get the global tracing logger
pub fn get_tracing_logger() -> Option<&'static TracingLogger> {
    unsafe { TRACING_LOGGER.as_ref() }
} 