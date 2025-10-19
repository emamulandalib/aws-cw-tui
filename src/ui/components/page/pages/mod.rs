pub mod instance_details_page;
pub mod instance_list_page;
pub mod metrics_summary_page;
/// Concrete page implementations for all application screens
///
/// This module contains the actual page implementations that use the
/// standardized page system.
pub mod service_list_page;

pub use instance_details_page::InstanceDetailsPage;
pub use instance_list_page::InstanceListPage;
pub use metrics_summary_page::MetricsSummaryPage;
pub use service_list_page::ServiceListPage;
