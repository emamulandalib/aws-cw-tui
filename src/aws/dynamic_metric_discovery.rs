use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use anyhow::Result;
use aws_sdk_cloudwatch::types::{Dimension, DimensionFilter};
// use std::collections::HashMap; // Removed unused import
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub metric_name: String,
    pub namespace: String,
    pub unit: Option<String>,
    pub statistic: String, // "Average", "Sum", "Maximum", etc.
}

#[derive(Debug, Clone)]
pub struct DynamicMetricData {
    pub metric_name: String,
    pub display_name: String,
    pub current_value: f64,
    pub history: Vec<f64>,
    pub timestamps: Vec<SystemTime>,
    pub unit: Option<String>,
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
            AwsErrorHandler::handle_aws_error(
                e,
                "list CloudWatch SQS metrics",
                "CloudWatch list permissions",
            )
        })?;

    let mut metrics = Vec::new();

    if let Some(metric_list) = response.metrics {
        for metric in metric_list {
            if let Some(metric_name) = metric.metric_name {
                // Note: Unit information is not available in list_metrics response
                // We'll determine unit based on metric name  
                let unit = determine_sqs_metric_unit(&metric_name);
                
                // Determine best statistic for SQS metrics
                let statistic = determine_sqs_statistic(&metric_name);

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

/// Fetch data for discovered metrics
pub async fn fetch_discovered_metrics(
    metrics: Vec<MetricDefinition>,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    period_seconds: i32,
) -> Result<Vec<DynamicMetricData>> {
    let client = AwsSessionManager::cloudwatch_client().await;
    let mut metric_data = Vec::new();

    for metric_def in metrics {
        let dimension_name = if metric_def.namespace == "AWS/RDS" {
            "DBInstanceIdentifier"
        } else if metric_def.namespace == "AWS/SQS" {
            "QueueName"
        } else {
            "InstanceId" // Default fallback
        };

        let dimension = Dimension::builder()
            .name(dimension_name)
            .value(instance_id)
            .build();

        let mut request = client
            .get_metric_statistics()
            .namespace(&metric_def.namespace)
            .metric_name(&metric_def.metric_name)
            .dimensions(dimension)
            .start_time(aws_sdk_cloudwatch::primitives::DateTime::from(start_time))
            .end_time(aws_sdk_cloudwatch::primitives::DateTime::from(end_time))
            .period(period_seconds)
            .statistics(
                aws_sdk_cloudwatch::types::Statistic::from(metric_def.statistic.as_str())
            );

        // Set unit if available
        if let Some(ref unit_str) = metric_def.unit {
            if let Some(unit) = parse_cloudwatch_unit(unit_str) {
                request = request.unit(unit);
            }
        }

        match request.send().await {
            Ok(response) => {
                if let Some(mut datapoints) = response.datapoints {
                    datapoints.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

                    let current_value = datapoints
                        .last()
                        .and_then(|dp| get_statistic_value(dp, &metric_def.statistic))
                        .unwrap_or(0.0);

                    let recent_datapoints: Vec<_> = datapoints.iter().rev().take(36).rev().collect();

                    let history: Vec<f64> = recent_datapoints
                        .iter()
                        .filter_map(|dp| get_statistic_value(dp, &metric_def.statistic))
                        .collect();

                    let timestamps: Vec<SystemTime> = recent_datapoints
                        .iter()
                        .map(|dp| {
                            dp.timestamp
                                .map(|ts| {
                                    SystemTime::UNIX_EPOCH
                                        + std::time::Duration::from_secs(ts.secs() as u64)
                                })
                                .unwrap_or_else(SystemTime::now)
                        })
                        .collect();

                    // Only include metrics that have data and valid values
                    if !history.is_empty() && !timestamps.is_empty() {
                        // Validate data consistency
                        if history.len() != timestamps.len() {
                            log::warn!("Skipping metric {} due to data length mismatch: history={}, timestamps={}", 
                                metric_def.metric_name, history.len(), timestamps.len());
                            continue;
                        }
                        
                        // Check for finite values
                        let valid_count = history.iter().filter(|&&v| v.is_finite()).count();
                        if valid_count == 0 {
                            log::warn!("Skipping metric {} due to no valid data points", metric_def.metric_name);
                            continue;
                        }
                        
                        // Warn if we have invalid values but continue with valid ones
                        if valid_count < history.len() {
                            log::warn!("Metric {} has {} invalid values out of {}", 
                                metric_def.metric_name, history.len() - valid_count, history.len());
                        }

                        // Validate timestamps are chronologically ordered
                        let mut timestamps_valid = true;
                        for i in 1..timestamps.len() {
                            if timestamps[i] < timestamps[i-1] {
                                log::warn!("Metric {} has unordered timestamps at index {}", metric_def.metric_name, i);
                                timestamps_valid = false;
                                break;
                            }
                        }

                        if !timestamps_valid {
                            log::warn!("Skipping metric {} due to timestamp ordering issues", metric_def.metric_name);
                            continue;
                        }

                        // Ensure current value is finite
                        if !current_value.is_finite() {
                            log::warn!("Metric {} has invalid current value: {}, using last valid value from history", 
                                metric_def.metric_name, current_value);
                            
                            // Find the last valid value from history
                            let corrected_current_value = history.iter()
                                .rev()
                                .find(|&&v| v.is_finite())
                                .copied()
                                .unwrap_or(0.0);
                            
                            metric_data.push(DynamicMetricData {
                                metric_name: metric_def.metric_name.clone(),
                                display_name: metric_def.metric_name.clone(),
                                current_value: corrected_current_value,
                                history,
                                timestamps,
                                unit: metric_def.unit.clone(),
                            });
                        } else {
                            metric_data.push(DynamicMetricData {
                                metric_name: metric_def.metric_name.clone(),
                                display_name: metric_def.metric_name.clone(), // Use AWS SDK metric name directly
                                current_value,
                                history,
                                timestamps,
                                unit: metric_def.unit.clone(),
                            });
                        }
                    } else {
                        log::debug!("Skipping metric {} due to empty data", metric_def.metric_name);
                    }
                }
            }
            Err(_) => {
                // Skip metrics that fail to fetch (might not be available for this engine/instance)
                continue;
            }
        }
    }

    Ok(metric_data)
}

/// Determine the unit for RDS metrics based on metric name
fn determine_metric_unit(metric_name: &str) -> Option<String> {
    match metric_name {
        "CPUUtilization" | "BurstBalance" => Some("Percent".to_string()),
        "DatabaseConnections" | "ReadIOPS" | "WriteIOPS" | "DiskQueueDepth" 
        | "MaximumUsedTransactionIDs" | "FailedSQLServerAgentJobsCount" => Some("Count".to_string()),
        "FreeStorageSpace" | "FreeableMemory" | "SwapUsage" | "BinLogDiskUsage"
        | "ReplicationSlotDiskUsage" | "TransactionLogsDiskUsage" 
        | "OldestReplicationSlotLag" | "OldestLogicalReplicationSlotLag" => Some("Bytes".to_string()),
        "ReadThroughput" | "WriteThroughput" | "NetworkReceiveThroughput" 
        | "NetworkTransmitThroughput" | "TransactionLogsGeneration" => Some("Bytes/Second".to_string()),
        "ReadLatency" | "WriteLatency" | "ReplicaLag" | "CheckpointLag" => Some("Seconds".to_string()),
        "ConnectionAttempts" => Some("Count/Second".to_string()),
        _ => None, // Unknown metrics don't have unit specified
    }
}

/// Determine the unit for SQS metrics based on metric name
fn determine_sqs_metric_unit(metric_name: &str) -> Option<String> {
    match metric_name {
        "NumberOfMessagesSent" | "NumberOfMessagesReceived" | "NumberOfMessagesDeleted"
        | "ApproximateNumberOfMessages" | "ApproximateNumberOfMessagesVisible"
        | "ApproximateNumberOfMessagesNotVisible" | "ApproximateNumberOfMessagesDelayed"
        | "NumberOfEmptyReceives" | "ApproximateNumberOfGroupsWithInflightMessages"
        | "NumberOfDeduplicatedSentMessages" | "NumberOfMessagesInDlq" => Some("Count".to_string()),
        "ApproximateAgeOfOldestMessage" => Some("Seconds".to_string()),
        "SentMessageSize" => Some("Bytes".to_string()),
        _ => None,
    }
}

/// Determine the best statistic for RDS metrics
fn determine_best_statistic(metric_name: &str) -> String {
    match metric_name {
        // Use Average for utilization and performance metrics
        "CPUUtilization" | "ReadLatency" | "WriteLatency" | "FreeableMemory" | "SwapUsage" => {
            "Average".to_string()
        }
        // Use Sum for count-based metrics over time periods
        "DatabaseConnections" => "Average".to_string(), // Concurrent connections
        "ReadIOPS" | "WriteIOPS" => "Average".to_string(),
        // Use Maximum for storage and queue metrics (worst case)
        "FreeStorageSpace" | "DiskQueueDepth" => "Average".to_string(),
        // Default to Average
        _ => "Average".to_string(),
    }
}

/// Determine the best statistic for SQS metrics
fn determine_sqs_statistic(metric_name: &str) -> String {
    match metric_name {
        // Count-based metrics use Sum for cumulative data
        "NumberOfMessagesSent" | "NumberOfMessagesReceived" | "NumberOfMessagesDeleted" 
        | "NumberOfEmptyReceives" | "NumberOfDeduplicatedSentMessages" => "Sum".to_string(),
        
        // Size metrics use Average
        "SentMessageSize" => "Average".to_string(),
        
        // Approximate metrics use Maximum for point-in-time values
        "ApproximateNumberOfMessages" | "ApproximateNumberOfMessagesVisible" 
        | "ApproximateNumberOfMessagesNotVisible" | "ApproximateNumberOfMessagesDelayed"
        | "ApproximateNumberOfGroupsWithInflightMessages" => "Maximum".to_string(),
        
        // Age metrics use Maximum (worst case scenario)
        "ApproximateAgeOfOldestMessage" => "Maximum".to_string(),
        
        // Default to Average
        _ => "Average".to_string(),
    }
}

/// Parse CloudWatch unit string to StandardUnit enum
fn parse_cloudwatch_unit(unit_str: &str) -> Option<aws_sdk_cloudwatch::types::StandardUnit> {
    match unit_str {
        "Percent" => Some(aws_sdk_cloudwatch::types::StandardUnit::Percent),
        "Count" => Some(aws_sdk_cloudwatch::types::StandardUnit::Count),
        "Count/Second" => Some(aws_sdk_cloudwatch::types::StandardUnit::CountSecond),
        "Bytes" => Some(aws_sdk_cloudwatch::types::StandardUnit::Bytes),
        "Bytes/Second" => Some(aws_sdk_cloudwatch::types::StandardUnit::BytesSecond),
        "Seconds" => Some(aws_sdk_cloudwatch::types::StandardUnit::Seconds),
        _ => None,
    }
}

/// Extract statistic value from datapoint based on statistic type
fn get_statistic_value(
    datapoint: &aws_sdk_cloudwatch::types::Datapoint,
    statistic: &str,
) -> Option<f64> {
    match statistic {
        "Average" => datapoint.average,
        "Sum" => datapoint.sum,
        "Maximum" => datapoint.maximum,
        "Minimum" => datapoint.minimum,
        "SampleCount" => datapoint.sample_count,
        _ => datapoint.average.or(datapoint.sum).or(datapoint.maximum),
    }
}

/// Format metric name for display (convert PascalCase to readable format)
fn format_metric_display_name(metric_name: &str) -> String {
    // Convert PascalCase to readable format
    let mut result = String::new();
    let mut chars = metric_name.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() && !result.is_empty() {
            result.push(' ');
        }
        result.push(ch);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_metric_display_name() {
        assert_eq!(format_metric_display_name("CPUUtilization"), "C P U Utilization");
        assert_eq!(format_metric_display_name("DatabaseConnections"), "Database Connections");
        assert_eq!(format_metric_display_name("ReadIOPS"), "Read I O P S");
    }

    #[test]
    fn test_determine_best_statistic() {
        assert_eq!(determine_best_statistic("CPUUtilization"), "Average");
        assert_eq!(determine_best_statistic("DatabaseConnections"), "Average");
        assert_eq!(determine_best_statistic("UnknownMetric"), "Average");
    }

    #[test]
    fn test_determine_sqs_statistic() {
        assert_eq!(determine_sqs_statistic("NumberOfMessagesSent"), "Sum");
        assert_eq!(determine_sqs_statistic("ApproximateNumberOfMessages"), "Maximum");
        assert_eq!(determine_sqs_statistic("SentMessageSize"), "Average");
    }
} 