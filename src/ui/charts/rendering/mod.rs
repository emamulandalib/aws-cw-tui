//! Chart Rendering module with focused responsibilities
//!
//! This module provides a clean, maintainable structure for chart rendering:
//! - `metric_charts`: Rendering for standard metric charts with health indicators
//! - `dynamic_charts`: Rendering for dynamic metrics discovered from CloudWatch
//! - `chart_titles`: Title rendering with value formatting and health colors
//! - `time_series`: Time series chart rendering with proper scaling and labels
//! - `simple_charts`: Simple metric display for small areas or no data scenarios

pub mod chart_titles;
pub mod dynamic_charts;
pub mod metric_charts;
pub mod simple_charts;
pub mod time_series;

// Re-export main rendering functions for easy access
pub use dynamic_charts::render_dynamic_metric_chart;
pub use metric_charts::render_metric_chart;
