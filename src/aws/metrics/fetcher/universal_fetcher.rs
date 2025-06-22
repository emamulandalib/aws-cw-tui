//! Universal metric fetcher that works with any MetricProvider

use crate::aws::metrics::providers::MetricProvider;
use crate::aws::metrics::types::{ServiceMetrics, MetricValue};
use crate::aws::metric_fetcher::fetch_comprehensive_metric;
use crate::aws::metric_types::MetricFetchParams;
use crate::aws::time_range::{TimeRange, calculate_period_seconds};
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use anyhow::Result;
use std::time::SystemTime;

/// Universal metric fetcher that can work with any service provider
pub struct UniversalMetricFetcher {
    client: CloudWatchClient,
}

impl UniversalMetricFetcher {
    pub fn new(client: CloudWatchClient) -> Self {
        Self { client }
    }
    
    /// Fetch metrics for any service using its provider
    pub async fn fetch_metrics<T: MetricProvider + ?Sized>(
        &self,
        provider: &T,
        instance_id: &str,
        time_range: TimeRange,
    ) -> Result<ServiceMetrics> {
        let end_time = SystemTime::now();
        let start_time = end_time - time_range.duration();
        let period_seconds = calculate_period_seconds(&time_range);
        
        let metrics_config = provider.get_metrics_config();
        let _dimension_mappings = provider.get_dimension_mappings();
        let namespace = provider.get_service_namespace();
        
        // Fetch all metrics concurrently
        let metric_futures: Vec<_> = metrics_config
            .iter()
            .map(|metric_def| {
                let params = MetricFetchParams {
                    metric_name: metric_def.name.clone(),
                    namespace: namespace.to_string(),
                    instance_id: instance_id.to_string(),
                    unit: metric_def.unit.clone(),
                };
                
                fetch_comprehensive_metric(
                    &self.client,
                    params,
                    start_time,
                    end_time,
                    period_seconds,
                )
            })
            .collect();
        
        // Execute all fetches concurrently
        // Execute all fetches sequentially for now (can be optimized to concurrent later)
        let mut results = Vec::new();
        for future in metric_futures {
            results.push(future.await);
        }
        
        // Process results into ServiceMetrics
        let mut service_metrics = ServiceMetrics::new(provider.get_service_type());
        let mut timestamps = Vec::new();
        
        for (i, (current, history, metric_timestamps)) in results.into_iter().enumerate() {
            let metric_name = &metrics_config[i].name;
            
            // Use the first metric's timestamps as the canonical timestamps
            if timestamps.is_empty() && !metric_timestamps.is_empty() {
                timestamps = metric_timestamps;
            }
            
            let metric_value = MetricValue::new(current, history);
            service_metrics.add_metric(metric_name.clone(), metric_value);
        }
        
        service_metrics.timestamps = timestamps;
        
        Ok(service_metrics)
    }
    
    /// Get the underlying CloudWatch client
    pub fn client(&self) -> &CloudWatchClient {
        &self.client
    }
}