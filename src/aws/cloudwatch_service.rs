use crate::models::{AwsService, DynamicMetrics};
use anyhow::Result;
use std::time::SystemTime;

// Import our dynamic modules
use super::dynamic_metric_discovery::{discover_rds_metrics, discover_sqs_metrics, fetch_discovered_metrics};
use super::time_range::calculate_period_seconds;

// Re-export for backward compatibility
pub use super::time_range::{TimeRange, TimeUnit};

/// Load metrics dynamically for any AWS service using CloudWatch list_metrics API
/// This replaces the hardcoded metric approach
pub async fn load_dynamic_metrics(
    service: &AwsService,
    instance_id: &str,
    time_range: TimeRange,
) -> Result<DynamicMetrics> {
    let end_time = SystemTime::now();
    let start_time = end_time - time_range.duration();
    let period_seconds = calculate_period_seconds(&time_range);

    // Step 1: Discover available metrics for the service
    let discovered_metrics = match service {
        AwsService::Rds => {
            log::info!("Discovering RDS metrics for instance: {}", instance_id);
            discover_rds_metrics(instance_id).await?
        }
        AwsService::Sqs => {
            log::info!("Discovering SQS metrics for queue: {}", instance_id);
            discover_sqs_metrics(instance_id).await?
        }
    };

    log::info!("Discovered {} metrics for {:?} service", discovered_metrics.len(), service);

    // Step 2: Fetch data for discovered metrics
    let metric_data = fetch_discovered_metrics(
        discovered_metrics,
        instance_id,
        start_time,
        end_time,
        period_seconds,
    ).await?;

    log::info!("Successfully fetched data for {} metrics", metric_data.len());

    // Step 3: Create DynamicMetrics container
    let mut dynamic_metrics = DynamicMetrics::new(service.clone(), instance_id.to_string());
    dynamic_metrics.update_metrics(metric_data);

    Ok(dynamic_metrics)
}
