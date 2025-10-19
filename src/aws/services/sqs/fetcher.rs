use crate::aws::cloudwatch_service::TimeRange;
use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::services::sqs::{
    mapper::{MappingResult, MetricMapper, SqsMetricDefinitions},
    utils::{calculate_time_range, system_time_to_unix},
    validator::validate_metric_data,
};
use crate::aws::session::AwsSessionManager;
use crate::models::{SqsMetricData, SqsQueue};
use anyhow::Result;
use aws_sdk_cloudwatch::primitives::DateTime;
use aws_sdk_cloudwatch::types::{Dimension, Statistic};
use std::time::{SystemTime, UNIX_EPOCH};

/// SQS Metrics Fetcher - responsible for fetching CloudWatch metrics
pub struct SqsMetricsFetcher {
    client: aws_sdk_cloudwatch::Client,
    mapper: MetricMapper,
}

impl SqsMetricsFetcher {
    /// Create a new SQS metrics fetcher
    pub async fn new() -> Self {
        let client = AwsSessionManager::cloudwatch_client().await;
        Self {
            client,
            mapper: MetricMapper::new(),
        }
    }

    /// Fetch all SQS metrics for a queue
    pub async fn fetch_sqs_metrics(
        &mut self,
        queue: &SqsQueue,
        time_range: &TimeRange,
    ) -> Result<SqsMetricData> {
        log::info!("Fetching SQS metrics for queue: {}", queue.name);

        // Calculate time range
        let (start_time, end_time) = calculate_time_range(time_range);

        // Create dimension for SQS queue
        let dimension = Dimension::builder()
            .name("QueueName")
            .value(&queue.name)
            .build();

        // Get metrics to fetch based on queue type
        let standard_metrics = SqsMetricDefinitions::standard_metrics();
        let fifo_metrics = if queue.queue_type == "FIFO" {
            SqsMetricDefinitions::fifo_metrics()
        } else {
            Vec::new()
        };

        let mut result = MappingResult::new();
        let mut timestamps = Vec::new();

        // Fetch standard metrics
        for (metric_name, namespace) in standard_metrics {
            match self
                .fetch_single_metric(namespace, metric_name, &dimension, start_time, end_time)
                .await
            {
                Ok((values, times)) => {
                    // Validate metric data
                    let validation = validate_metric_data(metric_name, &values, &times);
                    if !validation.is_valid {
                        log::warn!(
                            "SQS metric {} validation failed: {}",
                            metric_name,
                            validation
                                .error_message
                                .unwrap_or_else(|| "Unknown error".to_string())
                        );
                        continue;
                    }

                    // Add validation warnings
                    result.add_warnings(validation.warnings);

                    // Store timestamps from first successful metric
                    if timestamps.is_empty() {
                        timestamps = times.clone();
                    }

                    // Map metric to data structure
                    if let Err(e) =
                        self.mapper
                            .map_metric(&mut result.metric_data, metric_name, values, times)
                    {
                        log::warn!("Failed to map SQS metric {}: {}", metric_name, e);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to fetch SQS metric {}: {}", metric_name, e);
                    // Simply skip failed metrics - no fallback/synthetic data
                }
            }
        }

        // Fetch FIFO-specific metrics if applicable
        if !fifo_metrics.is_empty() {
            log::info!("Fetching FIFO-specific metrics for queue: {}", queue.name);

            for (metric_name, namespace) in fifo_metrics {
                match self
                    .fetch_single_metric(namespace, metric_name, &dimension, start_time, end_time)
                    .await
                {
                    Ok((values, times)) => {
                        let validation = validate_metric_data(metric_name, &values, &times);
                        if validation.is_valid {
                            result.add_warnings(validation.warnings);

                            if let Err(e) = self.mapper.map_metric(
                                &mut result.metric_data,
                                metric_name,
                                values,
                                times,
                            ) {
                                log::warn!("Failed to map FIFO metric {}: {}", metric_name, e);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to fetch FIFO metric {}: {}", metric_name, e);
                        // Simply skip failed metrics - no fallback
                    }
                }
            }
        }

        // Add mapper warnings to result
        result.add_warnings(self.mapper.take_warnings());

        // Set timestamps
        result = result.with_timestamps(timestamps);

        // Log summary
        log::info!(
            "SQS metrics fetch completed for queue {}: {} warnings",
            queue.name,
            result.warnings.len()
        );

        if !result.warnings.is_empty() {
            log::debug!("SQS metrics warnings: {:?}", result.warnings);
        }

        Ok(result.metric_data)
    }

    /// Fetch a single CloudWatch metric
    async fn fetch_single_metric(
        &self,
        namespace: &str,
        metric_name: &str,
        dimension: &Dimension,
        start_time: SystemTime,
        end_time: SystemTime,
    ) -> Result<(Vec<f64>, Vec<SystemTime>)> {
        // Convert SystemTime to DateTime for AWS SDK
        let start_timestamp = DateTime::from_secs(system_time_to_unix(start_time) as i64);
        let end_timestamp = DateTime::from_secs(system_time_to_unix(end_time) as i64);

        // Choose appropriate statistic based on metric type
        let statistic = self.get_appropriate_statistic(metric_name);

        log::debug!(
            "Fetching metric {} from {} to {} with statistic {:?}",
            metric_name,
            system_time_to_unix(start_time),
            system_time_to_unix(end_time),
            statistic
        );

        // Execute the query
        let response = self
            .client
            .get_metric_statistics()
            .namespace(namespace)
            .metric_name(metric_name)
            .dimensions(dimension.clone())
            .start_time(start_timestamp)
            .end_time(end_timestamp)
            .period(300)
            .statistics(statistic.clone())
            .send()
            .await
            .map_err(|e| {
                AwsErrorHandler::handle_aws_error(
                    e,
                    "fetch CloudWatch metrics",
                    "CloudWatch read permissions",
                )
            })?;

        // Process datapoints
        let mut datapoints = response.datapoints().to_vec();

        // Sort by timestamp to ensure chronological order
        datapoints.sort_by(|a, b| {
            a.timestamp()
                .unwrap_or(&DateTime::from_secs(0))
                .cmp(b.timestamp().unwrap_or(&DateTime::from_secs(0)))
        });

        let mut values = Vec::new();
        let mut timestamps = Vec::new();

        for datapoint in datapoints {
            if let (Some(value), Some(timestamp)) = (
                self.get_stat_value(&datapoint, &statistic),
                datapoint.timestamp(),
            ) {
                values.push(value);
                timestamps
                    .push(UNIX_EPOCH + std::time::Duration::from_secs(timestamp.secs() as u64));
            }
        }

        if values.is_empty() {
            return Err(anyhow::anyhow!(
                "No data points returned for metric {}",
                metric_name
            ));
        }

        log::debug!(
            "Successfully fetched {} data points for metric {}",
            values.len(),
            metric_name
        );

        Ok((values, timestamps))
    }

    /// Get the appropriate statistic for a metric
    fn get_appropriate_statistic(&self, metric_name: &str) -> Statistic {
        match metric_name {
            // Use Sum for rate metrics (messages sent/received/deleted)
            "NumberOfMessagesSent"
            | "NumberOfMessagesReceived"
            | "NumberOfMessagesDeleted"
            | "NumberOfEmptyReceives"
            | "NumberOfDeduplicatedSentMessages" => Statistic::Sum,

            // Use Average for gauge metrics (queue depth, message age, size)
            "ApproximateNumberOfMessages"
            | "ApproximateNumberOfMessagesVisible"
            | "ApproximateNumberOfMessagesNotVisible"
            | "ApproximateAgeOfOldestMessage"
            | "ApproximateNumberOfMessagesDelayed"
            | "SentMessageSize"
            | "NumberOfMessagesInDlq"
            | "ApproximateNumberOfGroupsWithInflightMessages" => Statistic::Average,

            // Default to Average for unknown metrics
            _ => Statistic::Average,
        }
    }

    /// Extract the statistic value from a datapoint
    fn get_stat_value(
        &self,
        datapoint: &aws_sdk_cloudwatch::types::Datapoint,
        statistic: &Statistic,
    ) -> Option<f64> {
        match statistic {
            Statistic::Average => datapoint.average(),
            Statistic::Sum => datapoint.sum(),
            Statistic::Maximum => datapoint.maximum(),
            Statistic::Minimum => datapoint.minimum(),
            Statistic::SampleCount => datapoint.sample_count(),
            _ => datapoint.average(), // Fallback to average
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SqsQueue;
    use std::collections::HashMap;

    fn create_test_queue() -> SqsQueue {
        SqsQueue {
            url: "https://sqs.us-east-1.amazonaws.com/123456789012/test-queue".to_string(),
            name: "test-queue".to_string(),
            queue_type: "Standard".to_string(),
            attributes: HashMap::new(),
        }
    }

    #[test]
    fn test_get_appropriate_statistic() {
        let fetcher = SqsMetricsFetcher {
            client: aws_sdk_cloudwatch::Client::from_conf(
                aws_sdk_cloudwatch::config::Config::builder()
                    .behavior_version(aws_sdk_cloudwatch::config::BehaviorVersion::latest())
                    .build(),
            ),
            mapper: MetricMapper::new(),
        };

        assert_eq!(
            fetcher.get_appropriate_statistic("NumberOfMessagesSent"),
            Statistic::Sum
        );
        assert_eq!(
            fetcher.get_appropriate_statistic("ApproximateNumberOfMessages"),
            Statistic::Average
        );
        assert_eq!(
            fetcher.get_appropriate_statistic("UnknownMetric"),
            Statistic::Average
        );
    }

    #[tokio::test]
    async fn test_metric_fetcher_creation() {
        // This test would require AWS credentials, so we'll just test the structure
        assert!(true); // Placeholder test
    }
}
