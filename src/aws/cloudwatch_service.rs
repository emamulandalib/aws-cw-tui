use crate::models::{AwsService, DynamicMetrics};
use crate::{log_aws_operation, timed_operation};
use anyhow::Result;
use std::time::SystemTime;
use tracing::{info, debug};

// Import our dynamic modules
use super::dynamic_metric_discovery::{
    discover_rds_metrics, discover_sqs_metrics, fetch_discovered_metrics,
};
use super::time_range::calculate_period_seconds;
use crate::aws::services::sqs::mapper::SqsMetricDefinitions;
use crate::aws::metrics::statistics::determine_sqs_statistic;
use crate::aws::metrics::discovery::MetricDefinition;

// Re-export for backward compatibility
pub use super::time_range::TimeRange;

/// Load metrics dynamically for any AWS service using CloudWatch list_metrics API
/// This replaces the hardcoded metric approach
pub async fn load_dynamic_metrics(
    service: &AwsService,
    instance_id: &str,
    time_range: TimeRange,
    period_seconds: Option<i32>, // Optional manual period override
) -> Result<DynamicMetrics> {
    let start_time = SystemTime::now();
    let end_time = SystemTime::now();
    let query_start_time = end_time - time_range.duration();
    let period_seconds = period_seconds.unwrap_or_else(|| calculate_period_seconds(&time_range));

    info!(
        service = ?service,
        instance_id = instance_id,
        time_range = ?time_range,
        "Loading dynamic metrics"
    );

    // Step 1: Discover available metrics for the service
    let mut discovered_metrics = timed_operation!(
        "discover_metrics",
        match service {
            AwsService::Rds => {
                debug!("Discovering RDS metrics for instance: {}", instance_id);
                discover_rds_metrics(instance_id).await?
            }
            AwsService::Sqs => {
                debug!("Discovering SQS metrics for queue: {}", instance_id);
                discover_sqs_metrics(instance_id).await?
            }
        }
    );

    if discovered_metrics.is_empty() && matches!(service, AwsService::Sqs) {
        info!("No metrics discovered for SQS, falling back to standard metrics");
        let standard = SqsMetricDefinitions::standard_metrics();
        let fifo = SqsMetricDefinitions::fifo_metrics();
        let all_metrics = standard.into_iter().chain(fifo.into_iter());
        for (metric_name, namespace) in all_metrics {
            let unit = SqsMetricDefinitions::get_unit(metric_name).to_string();
            let statistic = determine_sqs_statistic(metric_name).to_string();
            discovered_metrics.push(MetricDefinition {
                metric_name: metric_name.to_string(),
                namespace: namespace.to_string(),
                unit: Some(unit),
                statistic,
            });
        }
    }

    info!(
        discovered_count = discovered_metrics.len(),
        service = ?service,
        "Discovered metrics"
    );

    // Step 2: Fetch data for discovered metrics
    let metric_data = timed_operation!(
        "fetch_metric_data",
        fetch_discovered_metrics(
            discovered_metrics,
            instance_id,
            query_start_time,
            end_time,
            period_seconds,
        )
        .await?
    );

    info!(
        fetched_count = metric_data.len(),
        "Successfully fetched metric data"
    );

    // Step 3: Create DynamicMetrics container
    let mut dynamic_metrics = DynamicMetrics::new(service.clone(), instance_id.to_string());
    dynamic_metrics.update_metrics(metric_data);

    // Log the operation completion
    if let Ok(duration) = SystemTime::now().duration_since(start_time) {
        log_aws_operation!(
            "load_dynamic_metrics",
            format!("{:?}", service),
            duration,
            format!("Instance: {}, Metrics: {}", instance_id, dynamic_metrics.len())
        );
    }

    Ok(dynamic_metrics)
}
