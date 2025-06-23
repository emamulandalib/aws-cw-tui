//! Dynamic metric builder that supports multiple AWS services
//!
//! This module provides builders that can construct MetricData from both legacy
//! metric types and new dynamic service metrics from the provider system.

use super::metric_types::{AdvancedMetrics, CoreMetrics};
use crate::models::{MetricData, AwsService};
use crate::aws::metrics::types::ServiceMetrics;
use crate::aws::metrics::providers::MetricProvider;
use crate::aws::metrics::factory::MetricServiceFactory;
use anyhow::{Result, anyhow};
use std::any::Any;

/// Type alias for the result of metric transformation operations
type MetricResult<T> = Result<T>;

/// Type alias for boxed transformation data from providers
type TransformedData = Box<dyn Any>;


/// Dynamic metric builder that constructs MetricData from various AWS service sources
/// 
/// This builder provides a unified interface for converting service-specific metrics
/// into the legacy MetricData format used throughout the application. It leverages
/// the provider pattern to support multiple AWS services while maintaining backward
/// compatibility with existing RDS-focused code.
/// 
/// # Examples
/// 
/// ```rust
/// let builder = DynamicMetricBuilder::new();
/// let metric_data = builder.build_from_service_metrics(service_metrics)?;
/// ```
pub struct DynamicMetricBuilder {
    factory: MetricServiceFactory,
}


impl DynamicMetricBuilder {
    /// Create a new dynamic metric builder with the default factory
    pub fn new() -> Self {
        Self {
            factory: MetricServiceFactory::new(),
        }
    }
    
    /// Create a builder with a custom factory
    pub fn with_factory(factory: MetricServiceFactory) -> Self {
        Self { factory }
    }
    
    /// Build MetricData from ServiceMetrics using the appropriate service provider
    pub fn build_from_service_metrics(&self, service_metrics: ServiceMetrics) -> Result<MetricData> {
        let service_type = service_metrics.service_type.clone();
        self.transform_service_metrics_to_legacy(service_metrics, &service_type)
    }


    
    /// Build MetricData dynamically based on service type and raw metrics
    /// 
    /// This method provides a unified interface for transforming service metrics
    /// from any AWS service into the legacy MetricData format. It uses the 
    /// provider system to handle service-specific transformations.
    pub fn build_dynamic(&self, 
        service_type: AwsService, 
        service_metrics: ServiceMetrics
    ) -> Result<MetricData> {
        self.transform_service_metrics_to_legacy(service_metrics, &service_type)
    }
    
    /// Internal helper method that handles the common transformation logic
    /// 
    /// This method encapsulates the provider lookup, transformation, and 
    /// downcasting logic that is shared between different public methods.
    /// 
    /// # Arguments
    /// * `service_metrics` - Raw metrics data from CloudWatch
    /// * `service_type` - The AWS service type for provider lookup
    /// 
    /// # Returns
    /// * `Result<MetricData>` - Transformed legacy format or error
    fn transform_service_metrics_to_legacy(
        &self, 
        service_metrics: ServiceMetrics, 
        service_type: &AwsService
    ) -> MetricResult<MetricData> {
        let provider = self.get_service_provider(service_type)?;
        let transformed_data = self.apply_provider_transformation(provider, service_metrics);
        self.downcast_to_metric_data(transformed_data, service_type)
    }
    
    /// Get the appropriate provider for the given service type
    fn get_service_provider(&self, service_type: &AwsService) -> MetricResult<&dyn MetricProvider> {
        self.factory.get_provider(service_type)
            .map_err(|e| anyhow!(
                "No provider registered for service {:?}. {}", 
                service_type, 
                e
            ))
    }
    
    /// Apply the provider's transformation to convert ServiceMetrics to Any
    fn apply_provider_transformation(
        &self, 
        provider: &dyn MetricProvider, 
        service_metrics: ServiceMetrics
    ) -> TransformedData {
        provider.transform_raw_data(service_metrics)
    }
    
    /// Downcast the transformed data to MetricData with helpful error messages
    fn downcast_to_metric_data(
        &self, 
        transformed_data: TransformedData, 
        service_type: &AwsService
    ) -> MetricResult<MetricData> {
        transformed_data
            .downcast::<MetricData>()
            .map(|boxed| *boxed)
            .map_err(|_| anyhow!(
                "Provider for {:?} returned incompatible data type. Expected MetricData but got different type. \
                 This indicates a provider implementation error.", 
                service_type
            ))
    }
    
    /// Get the underlying factory for provider management
    pub fn factory(&self) -> &MetricServiceFactory {
        &self.factory
    }
    
    /// Get a mutable reference to the factory for registering new providers
    pub fn factory_mut(&mut self) -> &mut MetricServiceFactory {
        &mut self.factory
    }
}

impl Default for DynamicMetricBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy function that builds MetricData from CoreMetrics and AdvancedMetrics
///
/// This function is preserved for backward compatibility with existing code.
/// New code should prefer using DynamicMetricBuilder for more flexibility.
pub fn build_metric_data(core: CoreMetrics, advanced: AdvancedMetrics) -> MetricData {
    MetricData {
        timestamps: core.timestamps,
        cpu_utilization: core.cpu_utilization,
        cpu_history: core.cpu_history,
        database_connections: core.database_connections,
        connections_history: core.connections_history,
        free_storage_space: core.free_storage_space,
        free_storage_space_history: core.free_storage_space_history,
        read_iops: core.read_iops,
        read_iops_history: core.read_iops_history,
        write_iops: core.write_iops,
        write_iops_history: core.write_iops_history,
        read_latency: core.read_latency,
        read_latency_history: core.read_latency_history,
        write_latency: core.write_latency,
        write_latency_history: core.write_latency_history,
        read_throughput: core.read_throughput,
        read_throughput_history: core.read_throughput_history,
        write_throughput: core.write_throughput,
        write_throughput_history: core.write_throughput_history,
        network_receive_throughput: core.network_receive_throughput,
        network_receive_history: core.network_receive_history,
        network_transmit_throughput: core.network_transmit_throughput,
        network_transmit_history: core.network_transmit_history,
        swap_usage: core.swap_usage,
        swap_usage_history: core.swap_usage_history,
        freeable_memory: core.freeable_memory,
        freeable_memory_history: core.freeable_memory_history,
        queue_depth: core.queue_depth,
        queue_depth_history: core.queue_depth_history,

        // Advanced metrics
        burst_balance: advanced.burst_balance,
        burst_balance_history: advanced.burst_balance_history,
        cpu_credit_usage: advanced.cpu_credit_usage,
        cpu_credit_usage_history: advanced.cpu_credit_usage_history,
        cpu_credit_balance: advanced.cpu_credit_balance,
        cpu_credit_balance_history: advanced.cpu_credit_balance_history,
        bin_log_disk_usage: advanced.bin_log_disk_usage,
        bin_log_disk_usage_history: advanced.bin_log_disk_usage_history,
        replica_lag: advanced.replica_lag,
        replica_lag_history: advanced.replica_lag_history,
        maximum_used_transaction_ids: advanced.maximum_used_transaction_ids,
        maximum_used_transaction_ids_history: advanced.maximum_used_transaction_ids_history,
        oldest_replication_slot_lag: advanced.oldest_replication_slot_lag,
        oldest_replication_slot_lag_history: advanced.oldest_replication_slot_lag_history,
        replication_slot_disk_usage: advanced.replication_slot_disk_usage,
        replication_slot_disk_usage_history: advanced.replication_slot_disk_usage_history,
        transaction_logs_disk_usage: advanced.transaction_logs_disk_usage,
        transaction_logs_disk_usage_history: advanced.transaction_logs_disk_usage_history,
        transaction_logs_generation: advanced.transaction_logs_generation,
        transaction_logs_generation_history: advanced.transaction_logs_generation_history,
        failed_sql_server_agent_jobs_count: advanced.failed_sql_server_agent_jobs_count,
        failed_sql_server_agent_jobs_count_history: advanced
            .failed_sql_server_agent_jobs_count_history,
        checkpoint_lag: advanced.checkpoint_lag,
        checkpoint_lag_history: advanced.checkpoint_lag_history,
        connection_attempts: advanced.connection_attempts,
        connection_attempts_history: advanced.connection_attempts_history,
    }
}

/// Convenience function to build MetricData from ServiceMetrics using default factory
/// 
/// This provides a simple, stateless interface for converting new-style ServiceMetrics 
/// to legacy MetricData format. For repeated operations, prefer creating a 
/// DynamicMetricBuilder instance to avoid factory recreation overhead.
/// 
/// # Arguments
/// * `service_metrics` - Raw metrics data from any AWS service
/// 
/// # Returns
/// * `MetricResult<MetricData>` - Transformed legacy format or error
/// 
/// # Examples
/// ```rust
/// let metric_data = build_from_service_metrics(rds_metrics)?;
/// ```
pub fn build_from_service_metrics(service_metrics: ServiceMetrics) -> MetricResult<MetricData> {
    let builder = DynamicMetricBuilder::new();
    builder.build_from_service_metrics(service_metrics)
}

/// Create a MetricData with sensible defaults for a given AWS service
/// 
/// This function provides fallback MetricData when no metrics are available
/// from CloudWatch but the application still needs to display something.
/// All metrics will be initialized to zero values.
/// 
/// # Arguments
/// * `service_type` - The AWS service type to create defaults for
/// 
/// # Returns
/// * `MetricData` - Empty metric data structure with zero values
/// 
/// # Examples
/// ```rust
/// let empty_rds_data = build_empty_metric_data_for_service(AwsService::Rds);
/// assert_eq!(empty_rds_data.cpu_utilization, 0.0);
/// ```
pub fn build_empty_metric_data_for_service(service_type: AwsService) -> MetricData {
    match service_type {
        AwsService::Rds => create_default_rds_metrics(),
    }
}

/// Create default RDS-specific metric data
fn create_default_rds_metrics() -> MetricData {
    MetricData::default()
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::metrics::types::{ServiceMetrics, MetricValue};
    use std::collections::HashMap;
    use std::time::SystemTime;

    #[test]
    fn test_dynamic_builder_creation() {
        let builder = DynamicMetricBuilder::new();
        assert!(builder.factory().has_provider(&AwsService::Rds));
    }

    #[test]
    fn test_build_from_service_metrics_rds() {
        let builder = DynamicMetricBuilder::new();
        
        // Create test ServiceMetrics
        let mut service_metrics = ServiceMetrics::new(AwsService::Rds);
        service_metrics.add_metric("CPUUtilization".to_string(), MetricValue::new(50.0, vec![45.0, 48.0, 50.0]));
        service_metrics.add_metric("DatabaseConnections".to_string(), MetricValue::new(10.0, vec![8.0, 9.0, 10.0]));
        service_metrics.timestamps = vec![SystemTime::now(); 3];
        
        let result = builder.build_from_service_metrics(service_metrics);
        assert!(result.is_ok());
        
        let metric_data = result.unwrap();
        assert_eq!(metric_data.cpu_utilization, 50.0);
        assert_eq!(metric_data.database_connections, 10.0);
    }

    #[test]
    fn test_empty_metric_data_for_service() {
        let rds_empty = build_empty_metric_data_for_service(AwsService::Rds);
        assert_eq!(rds_empty.cpu_utilization, 0.0);
        assert!(rds_empty.timestamps.is_empty());
    }

    #[test]
    fn test_legacy_build_function_compatibility() {
        // Test that the legacy function still works
        let core = CoreMetrics {
            cpu_utilization: 75.0,
            cpu_history: vec![70.0, 72.0, 75.0],
            timestamps: vec![SystemTime::now(); 3],
            database_connections: 15.0,
            connections_history: vec![12.0, 14.0, 15.0],
            free_storage_space: 1000000000.0,
            free_storage_space_history: vec![1100000000.0, 1050000000.0, 1000000000.0],
            read_iops: 100.0,
            read_iops_history: vec![95.0, 98.0, 100.0],
            write_iops: 50.0,
            write_iops_history: vec![45.0, 48.0, 50.0],
            read_latency: 0.01,
            read_latency_history: vec![0.009, 0.0095, 0.01],
            write_latency: 0.015,
            write_latency_history: vec![0.012, 0.014, 0.015],
            read_throughput: 1000000.0,
            read_throughput_history: vec![900000.0, 950000.0, 1000000.0],
            write_throughput: 500000.0,
            write_throughput_history: vec![450000.0, 475000.0, 500000.0],
            network_receive_throughput: 100000.0,
            network_receive_history: vec![95000.0, 97000.0, 100000.0],
            network_transmit_throughput: 80000.0,
            network_transmit_history: vec![75000.0, 78000.0, 80000.0],
            swap_usage: 1000000.0,
            swap_usage_history: vec![900000.0, 950000.0, 1000000.0],
            freeable_memory: 2000000000.0,
            freeable_memory_history: vec![1900000000.0, 1950000000.0, 2000000000.0],
            queue_depth: 5.0,
            queue_depth_history: vec![4.0, 4.5, 5.0],
        };

        let advanced = AdvancedMetrics {
            burst_balance: 90.0,
            burst_balance_history: vec![85.0, 88.0, 90.0],
            cpu_credit_usage: 10.0,
            cpu_credit_usage_history: vec![8.0, 9.0, 10.0],
            cpu_credit_balance: 100.0,
            cpu_credit_balance_history: vec![95.0, 98.0, 100.0],
            bin_log_disk_usage: 50000000.0,
            bin_log_disk_usage_history: vec![45000000.0, 48000000.0, 50000000.0],
            replica_lag: 0.5,
            replica_lag_history: vec![0.4, 0.45, 0.5],
            maximum_used_transaction_ids: 1000.0,
            maximum_used_transaction_ids_history: vec![900.0, 950.0, 1000.0],
            oldest_replication_slot_lag: 100000.0,
            oldest_replication_slot_lag_history: vec![90000.0, 95000.0, 100000.0],
            replication_slot_disk_usage: 200000000.0,
            replication_slot_disk_usage_history: vec![180000000.0, 190000000.0, 200000000.0],
            transaction_logs_disk_usage: 300000000.0,
            transaction_logs_disk_usage_history: vec![270000000.0, 285000000.0, 300000000.0],
            transaction_logs_generation: 1000000.0,
            transaction_logs_generation_history: vec![900000.0, 950000.0, 1000000.0],
            failed_sql_server_agent_jobs_count: 0.0,
            failed_sql_server_agent_jobs_count_history: vec![0.0, 0.0, 0.0],
            checkpoint_lag: 0.1,
            checkpoint_lag_history: vec![0.08, 0.09, 0.1],
            connection_attempts: 50.0,
            connection_attempts_history: vec![45.0, 48.0, 50.0],
        };

        let metric_data = build_metric_data(core, advanced);
        assert_eq!(metric_data.cpu_utilization, 75.0);
        assert_eq!(metric_data.database_connections, 15.0);
        assert_eq!(metric_data.burst_balance, 90.0);
    }
}
