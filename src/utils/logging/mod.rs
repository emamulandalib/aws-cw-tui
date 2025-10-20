// Logging utilities and macros

use std::time::Duration;

/// Initialize logging system
pub fn init_logging() {
    // Placeholder for logging initialization
}

/// Get the current debug configuration
pub fn get_debug_config() -> crate::config::DebugConfig {
    // This would normally get the config from the app state
    // For now, return default config
    crate::config::DebugConfig::default()
}

/// Macro for logging AWS operations
#[macro_export]
macro_rules! log_aws_operation {
    ($operation:expr, $service:expr, $duration:expr, $details:expr) => {
        if $crate::utils::logging::get_debug_config().log_aws_operations {
            tracing::info!(
                "AWS_OPERATION: {} | Service: {} | Duration: {:.2}ms | {}",
                $operation,
                $service,
                $duration.as_millis(),
                $details
            );
        }
    };
}

/// Macro for logging timed operations
#[macro_export]
macro_rules! timed_operation {
    ($operation:expr, $code:expr) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        tracing::debug!(
            "TIMED_OPERATION: {} completed in {:.2}ms",
            $operation,
            duration.as_millis()
        );
        result
    }};
}

/// Macro for logging focus changes
#[macro_export]
macro_rules! log_focus_change {
    ($from:expr, $to:expr) => {
        if $crate::utils::logging::get_debug_config().log_navigation {
            tracing::debug!("FOCUS_CHANGE: {} -> {}", $from, $to);
        }
    };
}

/// Macro for logging navigation events
#[macro_export]
macro_rules! log_navigation {
    ($from:expr, $to:expr, $action:expr) => {
        if $crate::utils::logging::get_debug_config().log_navigation {
            tracing::debug!("NAVIGATION: {} -> {} | {}", $from, $to, $action);
        }
    };
}

/// Macro for logging state transitions
#[macro_export]
macro_rules! log_state_transition {
    ($from:expr, $to:expr, $description:expr) => {
        if $crate::utils::logging::get_debug_config().log_state_transitions {
            tracing::debug!("STATE_TRANSITION: {} -> {} | {}", $from, $to, $description);
        }
    };
}

/// Macro for logging metric operations
#[macro_export]
macro_rules! log_metric_operation {
    ($operation:expr, $metric:expr, $details:expr) => {
        if $crate::utils::logging::get_debug_config().log_metric_operations {
            tracing::info!(
                "METRIC_OPERATION: {} | Metric: {} | {}",
                $operation,
                $metric,
                $details
            );
        }
    };
}

/// Macro for logging key presses
#[macro_export]
macro_rules! log_key_press {
    ($key_code:expr, $modifiers:expr, $state:expr, $focused_panel:expr) => {
        if $crate::utils::logging::get_debug_config().log_user_interactions {
            tracing::debug!(
                "KEY_PRESS: {:?} + {:?} | State: {:?} | Panel: {}",
                $key_code,
                $modifiers,
                $state,
                $focused_panel
            );
        }
    };
}

/// Macro for logging user interactions
#[macro_export]
macro_rules! log_user_interaction {
    ($action:expr, $context:expr) => {
        if $crate::utils::logging::get_debug_config().log_user_interactions {
            tracing::debug!("USER_INTERACTION: {} | {}", $action, $context);
        }
    };
}

/// Macro for logging UI rendering events
#[macro_export]
macro_rules! log_ui_render {
    ($component:expr, $area:expr, $details:expr) => {
        if $crate::utils::logging::get_debug_config().log_ui_events {
            tracing::trace!("UI_RENDER: {} | {} | {}", $component, $area, $details);
        }
    };
}

/// Macro for creating UI spans
#[macro_export]
macro_rules! ui_span {
    ($name:expr, $expr:expr) => {{
        let _span = tracing::trace_span!($name);
        let _enter = _span.enter();
        $expr
    }};
}
