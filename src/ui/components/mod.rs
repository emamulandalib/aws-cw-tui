pub mod instance_details;
pub mod metrics_summary;
pub mod rds_list;

pub mod aws_chart;
pub mod aws_metrics_grid;
pub mod display_utils;
pub mod list_styling;
pub mod metric_definitions;
pub mod metric_utils;
pub mod service_list;
pub mod style_guide;
pub mod time_range_utils;
pub mod universal_box;


// New modular components
pub mod instance;
pub mod metric;
pub mod service;

pub use instance_details::render_instance_details;
// Removed unused import
pub use metrics_summary::render_metrics_summary;
pub use rds_list::render_rds_list;
pub use service_list::render_service_list;

// Export service-specific components for pure function usage
pub use instance::rds_instance::{render_rds_instance_details, render_rds_instance_list_item};
pub use instance::sqs_queue::{render_sqs_queue_details, render_sqs_queue_list_item};
pub use service::service_selector::render_service_selection_list;

// Export universal box component
pub use universal_box::{UniversalBox, BoxContent, TextParagraph};
