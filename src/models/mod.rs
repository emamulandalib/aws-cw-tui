// Models module - split into focused submodules for better organization
// This maintains backward compatibility while improving code structure

pub mod app_state;
pub mod aws_services;
pub mod metrics;

// Re-export all public types for backward compatibility
pub use app_state::{App, AppState, FocusedPanel, TimeRangeMode, Timezone};
pub use aws_services::{AwsService, RdsInstance, ServiceInstance, SqsQueue};
pub use metrics::{DynamicMetrics, MetricData, MetricType, SqsMetricData};
