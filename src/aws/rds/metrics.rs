use crate::aws::{cloudwatch_service, time_range::TimeRange};
use crate::models::MetricData;
use anyhow::Result;
use std::collections::HashMap;

/// RDS metrics management - centralized metrics operations for RDS
pub struct RdsMetricsManager;

impl RdsMetricsManager {
    /// Load metrics for a specific RDS instance
    pub async fn load_metrics(
        instance_id: &str,
        _metric_names: &[String], // For now, not used but kept for future enhancement
    ) -> Result<HashMap<String, MetricData>> {
        // Use a default time range for now
        let time_range = TimeRange::new(3, crate::aws::time_range::TimeUnit::Hours, 1).unwrap();

        // Use existing cloudwatch_service but with RDS-specific context
        let metric_data = cloudwatch_service::load_metrics(instance_id, time_range).await?;

        // Convert single MetricData to HashMap for consistency
        let mut metrics_map = HashMap::new();
        metrics_map.insert("primary".to_string(), metric_data);

        Ok(metrics_map)
    }

    /// Get CloudWatch namespace for RDS
    pub fn namespace() -> &'static str {
        "AWS/RDS"
    }

    /// Get dimension key for RDS instances
    pub fn dimension_key() -> &'static str {
        "DBInstanceIdentifier"
    }
}
