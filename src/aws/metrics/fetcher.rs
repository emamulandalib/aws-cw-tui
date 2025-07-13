use crate::aws::metrics::discovery::MetricDefinition;
use crate::aws::metrics::formatter::format_metric_display_name;
use crate::aws::metrics::statistics::get_statistic_value;
use crate::aws::metrics::units::parse_cloudwatch_unit;
use crate::aws::session::AwsSessionManager;
use anyhow::Result;
use aws_sdk_cloudwatch::types::Dimension;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct DynamicMetricData {
    pub metric_name: String,
    pub display_name: String,
    pub current_value: f64,
    pub history: Vec<f64>,
    pub timestamps: Vec<SystemTime>,
    pub unit: Option<String>,
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
            .statistics(aws_sdk_cloudwatch::types::Statistic::from(
                metric_def.statistic.as_str(),
            ));

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

                    let recent_datapoints: Vec<_> =
                        datapoints.iter().rev().take(36).rev().collect();

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

                        // Skip metrics with invalid values (NaN/Infinite only)
                        if history.iter().any(|&v| v.is_nan() || v.is_infinite()) {
                            log::warn!(
                                "Skipping metric {} due to invalid values",
                                metric_def.metric_name
                            );
                            continue;
                        }

                        metric_data.push(DynamicMetricData {
                            metric_name: metric_def.metric_name.clone(),
                            display_name: format_metric_display_name(&metric_def.metric_name),
                            current_value,
                            history,
                            timestamps,
                            unit: metric_def.unit.clone(),
                        });
                    } else {
                        log::debug!("Skipping metric {} - no valid data", metric_def.metric_name);
                    }
                } else {
                    log::debug!(
                        "No datapoints received for metric: {}",
                        metric_def.metric_name
                    );
                }
            }
            Err(e) => {
                log::warn!("Failed to fetch metric {}: {}", metric_def.metric_name, e);
                // Continue with other metrics instead of failing completely
            }
        }
    }

    log::info!(
        "Successfully processed {} metrics with data",
        metric_data.len()
    );
    Ok(metric_data)
}

impl DynamicMetricData {
    /// Create a new DynamicMetricData instance
    pub fn new(
        metric_name: String,
        display_name: String,
        current_value: f64,
        history: Vec<f64>,
        timestamps: Vec<SystemTime>,
        unit: Option<String>,
    ) -> Self {
        Self {
            metric_name,
            display_name,
            current_value,
            history,
            timestamps,
            unit,
        }
    }

    /// Check if the metric has valid data
    pub fn has_valid_data(&self) -> bool {
        !self.history.is_empty()
            && !self.timestamps.is_empty()
            && self.history.len() == self.timestamps.len()
            && !self.history.iter().any(|&v| v.is_nan() || v.is_infinite())
    }

    /// Get the metric's unit as a string
    pub fn unit_string(&self) -> &str {
        self.unit.as_deref().unwrap_or("")
    }

    /// Get the latest timestamp
    pub fn latest_timestamp(&self) -> Option<SystemTime> {
        self.timestamps.last().copied()
    }

    /// Get the number of data points
    pub fn data_point_count(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_dynamic_metric_data_creation() {
        let history = vec![10.0, 20.0, 30.0];
        let timestamps = vec![
            UNIX_EPOCH + Duration::from_secs(100),
            UNIX_EPOCH + Duration::from_secs(200),
            UNIX_EPOCH + Duration::from_secs(300),
        ];

        let metric = DynamicMetricData::new(
            "TestMetric".to_string(),
            "Test Metric".to_string(),
            30.0,
            history.clone(),
            timestamps.clone(),
            Some("Percent".to_string()),
        );

        assert_eq!(metric.metric_name, "TestMetric");
        assert_eq!(metric.display_name, "Test Metric");
        assert_eq!(metric.current_value, 30.0);
        assert_eq!(metric.history, history);
        assert_eq!(metric.timestamps, timestamps);
        assert_eq!(metric.unit_string(), "Percent");
        assert!(metric.has_valid_data());
        assert_eq!(metric.data_point_count(), 3);
    }

    #[test]
    fn test_invalid_data_detection() {
        // Test empty data
        let empty_metric = DynamicMetricData::new(
            "Empty".to_string(),
            "Empty".to_string(),
            0.0,
            vec![],
            vec![],
            None,
        );
        assert!(!empty_metric.has_valid_data());

        // Test all zero values
        let zero_metric = DynamicMetricData::new(
            "Zero".to_string(),
            "Zero".to_string(),
            0.0,
            vec![0.0, 0.0, 0.0],
            vec![UNIX_EPOCH; 3],
            None,
        );
        assert!(!zero_metric.has_valid_data());

        // Test NaN values
        let nan_metric = DynamicMetricData::new(
            "NaN".to_string(),
            "NaN".to_string(),
            0.0,
            vec![1.0, f64::NAN, 3.0],
            vec![UNIX_EPOCH; 3],
            None,
        );
        assert!(!nan_metric.has_valid_data());
    }
}
