// === Core Application Modules ===
pub mod app;
pub mod aws;
pub mod config;
pub mod core;
pub mod event_handler;
pub mod models;
pub mod services;
pub mod terminal;
pub mod ui;
pub mod utils;

#[cfg(test)]
pub mod tests;

// === Core Module Exports ===
// Provide clean interfaces for the core application modules

/// Core application functionality (all modules combined)
pub mod core_app {
    pub use crate::core::app::*;
}

// === Model Exports ===
// Provide clean interfaces for application models

/// Application state models
pub mod app_state {
    pub use crate::models::app_state::*;
}

/// AWS service models
pub mod aws_services {
    pub use crate::models::aws_services::*;
}

/// Metrics data models
pub mod metrics {
    pub use crate::models::metrics::*;
}

// === UI Component Exports ===
// Provide clean interfaces for UI components

/// Metric-related UI components
pub mod metric_components {
    pub use crate::ui::components::metric::*;
}

/// Instance-related UI components
pub mod instance_components {
    pub use crate::ui::components::instance::*;
}

/// Service-related UI components
pub mod service_components {
    pub use crate::ui::components::service::*;
}

/// Chart rendering components
pub mod chart_components {
    pub use crate::ui::charts::rendering::*;
}

// === Utility Exports ===
// Provide clean interfaces for utility functions

/// Formatting utilities
pub mod formatting {
    pub use crate::utils::formatting::*;
}

/// Time utilities
pub mod time {
    // Removed unused time utilities to eliminate compiler warnings
}

/// Validation utilities
pub mod validation {
    pub use crate::utils::validation::*;
}

/// Logging utilities and macros
pub mod logging {
    pub use crate::utils::logging::*;
}

// === Configuration Exports ===
// Provide clean interfaces for configuration

/// Application configuration
pub mod app_config {
    pub use crate::config::app_config::*;
}

/// AWS configuration
pub mod aws_config {
    pub use crate::config::aws_config::*;
}

/// Debug configuration
pub mod debug_config {
    pub use crate::config::debug_config::*;
}

/// UI configuration
pub mod ui_config {
    pub use crate::config::ui_config::*;
}

// === AWS Service Exports ===
// Provide clean interfaces for AWS services

/// CloudWatch service interface
pub mod cloudwatch {
    pub use crate::aws::cloudwatch_service::*;
}

/// RDS service interface
pub mod rds {
    pub use crate::aws::rds_service::*;
}

/// SQS service interface
pub mod sqs {
    pub use crate::aws::sqs_service::*;
}

/// AWS session management
pub mod aws_session {
    pub use crate::aws::session::*;
}

// === Prelude Module ===
// Commonly used types and traits for convenience

/// Prelude module containing commonly used types and traits
pub mod prelude {
    // Core application types
    pub use crate::models::app_state::{App, AppState, FocusedPanel, TimeRangeMode, Timezone};
    pub use crate::models::aws_services::{AwsService, RdsInstance, ServiceInstance, SqsQueue};

    // Core functionality
    pub use crate::core::app::*;

    // Utility functions
    pub use crate::formatting::*;
    pub use crate::logging::*;
    pub use crate::validation::*;

    // Terminal and UI
    pub use crate::event_handler::handle_event;
    pub use crate::terminal::TerminalManager;
    pub use crate::ui::render_app;

    // Configuration
    pub use crate::config::DebugConfig;

    // Common external types
    pub use anyhow::Result;
    pub use crossterm::event::{Event, KeyCode, KeyEvent};
    pub use ratatui::widgets::ListState;
    pub use std::time::{Duration, Instant, SystemTime};
    pub use tracing::{debug, error, info, warn};
}

// === Legacy Exports ===
// Re-export logging macros for backward compatibility
pub use utils::logging::*;
