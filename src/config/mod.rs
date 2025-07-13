// Configuration modules
pub mod app_config;
pub mod aws_config;
pub mod debug_config;
pub mod ui_config;

// Re-export only the used configuration types
pub use debug_config::DebugConfig;
