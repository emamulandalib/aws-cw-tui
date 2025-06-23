//! RDS-specific metric provider implementation

use super::MetricProvider;
use crate::aws::metric_builder::build_metric_data;
use crate::aws::metrics::types::{MetricCategory, MetricDefinition, ServiceMetrics, StatisticType};
use crate::models::AwsService;
use std::any::Any;
use std::collections::HashMap;

/// RDS metric provider that implements the MetricProvider trait
pub struct RdsMetricProvider;

impl MetricProvider for RdsMetricProvider {
    fn get_service_namespace(&self) -> &'static str {
        "AWS/RDS"
    }

    fn get_metrics_config(&self) -> Vec<MetricDefinition> {
        vec![
            // Core Performance Metrics
            MetricDefinition {
                name: "CPUUtilization".to_string(),
                unit: Some("Percent".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Core,
            },
            MetricDefinition {
                name: "DatabaseConnections".to_string(),
                unit: Some("Count".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Core,
            },
            MetricDefinition {
                name: "FreeStorageSpace".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Storage,
            },
            MetricDefinition {
                name: "ReadIOPS".to_string(),
                unit: Some("Count/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "WriteIOPS".to_string(),
                unit: Some("Count/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "ReadLatency".to_string(),
                unit: Some("Seconds".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "WriteLatency".to_string(),
                unit: Some("Seconds".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "ReadThroughput".to_string(),
                unit: Some("Bytes/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "WriteThroughput".to_string(),
                unit: Some("Bytes/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "NetworkReceiveThroughput".to_string(),
                unit: Some("Bytes/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Network,
            },
            MetricDefinition {
                name: "NetworkTransmitThroughput".to_string(),
                unit: Some("Bytes/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Network,
            },
            MetricDefinition {
                name: "SwapUsage".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "FreeableMemory".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            MetricDefinition {
                name: "DiskQueueDepth".to_string(),
                unit: Some("Count".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Performance,
            },
            // Advanced Metrics
            MetricDefinition {
                name: "BurstBalance".to_string(),
                unit: Some("Percent".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "CPUCreditUsage".to_string(),
                unit: None,
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "CPUCreditBalance".to_string(),
                unit: None,
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "BinLogDiskUsage".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "ReplicaLag".to_string(),
                unit: Some("Seconds".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "MaximumUsedTransactionIDs".to_string(),
                unit: Some("Count".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "OldestReplicationSlotLag".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "ReplicationSlotDiskUsage".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "TransactionLogsDiskUsage".to_string(),
                unit: Some("Bytes".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "TransactionLogsGeneration".to_string(),
                unit: Some("Bytes/Second".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "FailedSQLServerAgentJobsCount".to_string(),
                unit: Some("Count".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "CheckpointLag".to_string(),
                unit: Some("Seconds".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
            MetricDefinition {
                name: "ConnectionAttempts".to_string(),
                unit: Some("Count".to_string()),
                statistic: StatisticType::Average,
                category: MetricCategory::Advanced,
            },
        ]
    }

    fn get_dimension_mappings(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(
            "instance_id".to_string(),
            "DBInstanceIdentifier".to_string(),
        );
        map
    }

    fn transform_raw_data(&self, data: ServiceMetrics) -> Box<dyn Any> {
        // Transform ServiceMetrics back to legacy MetricData format for backward compatibility
        use crate::aws::metric_types::{AdvancedMetrics, CoreMetrics};

        // Extract metric values or use defaults
        let get_metric = |name: &str| -> (f64, Vec<f64>) {
            data.raw_metrics
                .get(name)
                .map(|mv| (mv.current, mv.history.clone()))
                .unwrap_or((0.0, Vec::new()))
        };

        // Build CoreMetrics
        let core_metrics = CoreMetrics {
            cpu_utilization: get_metric("CPUUtilization").0,
            cpu_history: get_metric("CPUUtilization").1,
            timestamps: data.timestamps.clone(),
            database_connections: get_metric("DatabaseConnections").0,
            connections_history: get_metric("DatabaseConnections").1,
            free_storage_space: get_metric("FreeStorageSpace").0,
            free_storage_space_history: get_metric("FreeStorageSpace").1,
            read_iops: get_metric("ReadIOPS").0,
            read_iops_history: get_metric("ReadIOPS").1,
            write_iops: get_metric("WriteIOPS").0,
            write_iops_history: get_metric("WriteIOPS").1,
            read_latency: get_metric("ReadLatency").0,
            read_latency_history: get_metric("ReadLatency").1,
            write_latency: get_metric("WriteLatency").0,
            write_latency_history: get_metric("WriteLatency").1,
            read_throughput: get_metric("ReadThroughput").0,
            read_throughput_history: get_metric("ReadThroughput").1,
            write_throughput: get_metric("WriteThroughput").0,
            write_throughput_history: get_metric("WriteThroughput").1,
            network_receive_throughput: get_metric("NetworkReceiveThroughput").0,
            network_receive_history: get_metric("NetworkReceiveThroughput").1,
            network_transmit_throughput: get_metric("NetworkTransmitThroughput").0,
            network_transmit_history: get_metric("NetworkTransmitThroughput").1,
            swap_usage: get_metric("SwapUsage").0,
            swap_usage_history: get_metric("SwapUsage").1,
            freeable_memory: get_metric("FreeableMemory").0,
            freeable_memory_history: get_metric("FreeableMemory").1,
            queue_depth: get_metric("DiskQueueDepth").0,
            queue_depth_history: get_metric("DiskQueueDepth").1,
        };

        // Build AdvancedMetrics
        let advanced_metrics = AdvancedMetrics {
            burst_balance: get_metric("BurstBalance").0,
            burst_balance_history: get_metric("BurstBalance").1,
            cpu_credit_usage: get_metric("CPUCreditUsage").0,
            cpu_credit_usage_history: get_metric("CPUCreditUsage").1,
            cpu_credit_balance: get_metric("CPUCreditBalance").0,
            cpu_credit_balance_history: get_metric("CPUCreditBalance").1,
            bin_log_disk_usage: get_metric("BinLogDiskUsage").0,
            bin_log_disk_usage_history: get_metric("BinLogDiskUsage").1,
            replica_lag: get_metric("ReplicaLag").0,
            replica_lag_history: get_metric("ReplicaLag").1,
            maximum_used_transaction_ids: get_metric("MaximumUsedTransactionIDs").0,
            maximum_used_transaction_ids_history: get_metric("MaximumUsedTransactionIDs").1,
            oldest_replication_slot_lag: get_metric("OldestReplicationSlotLag").0,
            oldest_replication_slot_lag_history: get_metric("OldestReplicationSlotLag").1,
            replication_slot_disk_usage: get_metric("ReplicationSlotDiskUsage").0,
            replication_slot_disk_usage_history: get_metric("ReplicationSlotDiskUsage").1,
            transaction_logs_disk_usage: get_metric("TransactionLogsDiskUsage").0,
            transaction_logs_disk_usage_history: get_metric("TransactionLogsDiskUsage").1,
            transaction_logs_generation: get_metric("TransactionLogsGeneration").0,
            transaction_logs_generation_history: get_metric("TransactionLogsGeneration").1,
            failed_sql_server_agent_jobs_count: get_metric("FailedSQLServerAgentJobsCount").0,
            failed_sql_server_agent_jobs_count_history: get_metric("FailedSQLServerAgentJobsCount")
                .1,
            checkpoint_lag: get_metric("CheckpointLag").0,
            checkpoint_lag_history: get_metric("CheckpointLag").1,
            connection_attempts: get_metric("ConnectionAttempts").0,
            connection_attempts_history: get_metric("ConnectionAttempts").1,
        };

        // Use the existing build_metric_data function
        let legacy_data = build_metric_data(core_metrics, advanced_metrics);
        Box::new(legacy_data)
    }

    fn get_service_type(&self) -> AwsService {
        AwsService::Rds
    }
}

impl RdsMetricProvider {
    pub fn new() -> Self {
        Self
    }
}
