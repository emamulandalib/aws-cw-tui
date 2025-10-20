use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::metrics::statistics::determine_best_statistic;
use crate::aws::metrics::units::determine_metric_unit;
use crate::aws::session::AwsSessionManager;
use anyhow::Result;
use aws_sdk_cloudwatch::types::DimensionFilter;

#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub metric_name: String,
    pub namespace: String,
    pub unit: Option<String>,
    pub statistic: String, // "Average", "Sum", "Maximum", etc.
}

/// Discover available metrics for RDS instance
pub async fn discover_rds_metrics(instance_id: &str) -> Result<Vec<MetricDefinition>> {
    let client = AwsSessionManager::cloudwatch_client().await;

    let dimension_filter = DimensionFilter::builder()
        .name("DBInstanceIdentifier")
        .value(instance_id)
        .build();

    let response = client
        .list_metrics()
        .namespace("AWS/RDS")
        .dimensions(dimension_filter)
        .send()
        .await
        .map_err(|e| {
            AwsErrorHandler::handle_aws_error(
                e,
                "list CloudWatch metrics",
                "CloudWatch list permissions",
            )
        })?;

    let mut metrics = Vec::new();

    if let Some(metric_list) = response.metrics {
        for metric in metric_list {
            if let Some(metric_name) = metric.metric_name {
                // Note: Unit information is not available in list_metrics response
                // We'll determine unit based on metric name
                let unit = determine_metric_unit(&metric_name);

                // Determine best statistic for this metric type
                let statistic = determine_best_statistic(&metric_name);

                metrics.push(MetricDefinition {
                    metric_name: metric_name.clone(),
                    namespace: "AWS/RDS".to_string(),
                    unit,
                    statistic,
                });
            }
        }
    }

    Ok(metrics)
}

/// Discover available metrics for SQS queue
pub async fn discover_sqs_metrics(queue_name: &str) -> Result<Vec<MetricDefinition>> {
    let client = AwsSessionManager::cloudwatch_client().await;

    let dimension_filter = DimensionFilter::builder()
        .name("QueueName")
        .value(queue_name)
        .build();

    let response = client
        .list_metrics()
        .namespace("AWS/SQS")
        .dimensions(dimension_filter)
        .send()
        .await
        .map_err(|e| {
            log::error!("CloudWatch list_metrics API call failed: {}", e);
            AwsErrorHandler::handle_aws_error(
                e,
                "list CloudWatch SQS metrics",
                "CloudWatch list permissions",
            )
        })?;

    let mut metrics = Vec::new();

    if let Some(metric_list) = response.metrics {
        for (_i, metric) in metric_list.iter().enumerate() {
            if let Some(metric_name) = &metric.metric_name {
                // Note: Unit information is not available in list_metrics response
                // We'll determine unit based on metric name
                let unit = crate::aws::metrics::units::determine_sqs_metric_unit(metric_name);

                // Determine best statistic for SQS metrics
                let statistic =
                    crate::aws::metrics::statistics::determine_sqs_statistic(metric_name);

                metrics.push(MetricDefinition {
                    metric_name: metric_name.clone(),
                    namespace: "AWS/SQS".to_string(),
                    unit,
                    statistic,
                });
            }
        }
    }

    Ok(metrics)
}
