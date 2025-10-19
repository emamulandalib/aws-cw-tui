pub mod aws_chart;
pub mod aws_metrics_grid;
pub mod display_utils;
pub mod error_feedback;
pub mod help_system;
pub mod instance;
pub mod instance_details;
pub mod list_styling;
pub mod metric;
pub mod metric_definitions;
pub mod metric_utils;
pub mod metrics_summary;
pub mod page;
pub mod rds_list;
pub mod service;
pub mod service_list;
pub mod sparkline_utils;
pub mod style_guide;
pub mod time_range_utils;
pub mod universal_box;
pub mod visual_utils;

pub use instance_details::render_instance_details;
pub use metrics_summary::render_metrics_summary;
pub use rds_list::render_rds_list;
pub use service_list::render_service_list;

// Export service-specific components for pure function usage
pub use instance::rds_instance::{render_rds_instance_details, render_rds_instance_list_item};
pub use instance::sqs_queue::{
    render_sqs_queue_details, render_sqs_queue_list_item, render_sqs_queue_list_item_with_metrics,
};
pub use service::service_selector::render_service_selection_list;

// Export universal box component
pub use universal_box::{BoxContent, TextParagraph, UniversalBox};
