use crate::models::MetricData;
use anyhow::Result;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use std::time::SystemTime;

// Import our new modules
use super::metric_fetcher::fetch_comprehensive_metric;
use super::metric_types::{AdvancedMetrics, CoreMetrics, MetricFetchParams};
use super::session::AwsSessionManager;
use super::time_range::calculate_period_seconds;

// Re-export for backward compatibility
pub use super::time_range::{TimeRange, TimeUnit};

pub async fn load_metrics(instance_id: &str, time_range: TimeRange) -> Result<MetricData> {
    // Use shared AWS session manager for CloudWatch client
    let client = AwsSessionManager::cloudwatch_client().await;

    let end_time = SystemTime::now();
    let start_time = end_time - time_range.duration();

    let instance_id_owned = instance_id.to_string();

    // Calculate period based on time range duration and period_days
    let period_seconds = calculate_period_seconds(&time_range);

    // Fetch core metrics concurrently with error handling
    let core_metrics = fetch_core_metrics(
        &client,
        &instance_id_owned,
        start_time,
        end_time,
        period_seconds,
    )
    .await;

    let advanced_metrics = fetch_advanced_metrics(
        &client,
        &instance_id_owned,
        start_time,
        end_time,
        period_seconds,
    )
    .await;

    Ok(build_metric_data_inline(core_metrics, advanced_metrics))
}

/// Load metrics with instance characteristics for intelligent filtering
pub async fn load_metrics_with_instance(
    instance: &crate::models::RdsInstance,
    time_range: TimeRange,
) -> Result<MetricData> {
    // Use shared AWS session manager for CloudWatch client
    let client = AwsSessionManager::cloudwatch_client().await;

    let end_time = SystemTime::now();
    let start_time = end_time - time_range.duration();

    let instance_id_owned = instance.identifier.clone();

    // Calculate period based on time range duration and period_days
    let period_seconds = calculate_period_seconds(&time_range);

    // Create instance characteristics for intelligent filtering
    let characteristics = super::metric_types::RdsInstanceCharacteristics::from_instance(instance);
    let relevant_metrics = characteristics.get_relevant_metrics();

    // Only fetch metrics that are relevant for this instance
    let core_metrics = fetch_relevant_core_metrics(
        &client,
        &instance_id_owned,
        start_time,
        end_time,
        period_seconds,
        &relevant_metrics,
    )
    .await;

    let advanced_metrics = fetch_relevant_advanced_metrics(
        &client,
        &instance_id_owned,
        start_time,
        end_time,
        period_seconds,
        &relevant_metrics,
    )
    .await;

    Ok(build_metric_data_with_characteristics(
        core_metrics,
        advanced_metrics,
        Some(characteristics),
    ))
}

async fn fetch_core_metrics(
    client: &CloudWatchClient,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    period_seconds: i32,
) -> CoreMetrics {
    let (
        (cpu, cpu_hist, cpu_timestamps),
        (connections, conn_hist, _),
        (free_storage, free_storage_hist, _),
        (read_iops, read_iops_hist, _),
        (write_iops, write_iops_hist, _),
        (read_latency, read_lat_hist, _),
        (write_latency, write_lat_hist, _),
        (read_throughput, read_throughput_hist, _),
        (write_throughput, write_throughput_hist, _),
        (net_receive, net_receive_hist, _),
        (net_transmit, net_transmit_hist, _),
        (swap_usage, swap_hist, _),
        (freeable_memory, memory_hist, _),
        (queue_depth, queue_depth_hist, _),
    ) = tokio::join!(
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "CPUUtilization".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Percent".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "DatabaseConnections".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "FreeStorageSpace".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "ReadIOPS".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "WriteIOPS".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "ReadLatency".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Seconds".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "WriteLatency".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Seconds".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "ReadThroughput".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "WriteThroughput".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "NetworkReceiveThroughput".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "NetworkTransmitThroughput".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "SwapUsage".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "FreeableMemory".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "DiskQueueDepth".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
    );

    CoreMetrics {
        cpu_utilization: cpu,
        cpu_history: cpu_hist,
        timestamps: cpu_timestamps,
        database_connections: connections,
        connections_history: conn_hist,
        free_storage_space: free_storage,
        free_storage_space_history: free_storage_hist,
        read_iops,
        read_iops_history: read_iops_hist,
        write_iops,
        write_iops_history: write_iops_hist,
        read_latency,
        read_latency_history: read_lat_hist,
        write_latency,
        write_latency_history: write_lat_hist,
        read_throughput,
        read_throughput_history: read_throughput_hist,
        write_throughput,
        write_throughput_history: write_throughput_hist,
        network_receive_throughput: net_receive,
        network_receive_history: net_receive_hist,
        network_transmit_throughput: net_transmit,
        network_transmit_history: net_transmit_hist,
        swap_usage,
        swap_usage_history: swap_hist,
        freeable_memory,
        freeable_memory_history: memory_hist,
        queue_depth,
        queue_depth_history: queue_depth_hist,
    }
}

async fn fetch_advanced_metrics(
    client: &CloudWatchClient,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    period_seconds: i32,
) -> AdvancedMetrics {
    let (
        (burst_balance, burst_balance_hist, _),
        (cpu_credit_usage, cpu_credit_usage_hist, _),
        (cpu_credit_balance, cpu_credit_balance_hist, _),
        // Missing CPU surplus credit metrics results
        (cpu_surplus_credit_balance, cpu_surplus_credit_balance_hist, _),
        (cpu_surplus_credits_charged, cpu_surplus_credits_charged_hist, _),
        // Missing EBS performance metrics results
        (ebs_byte_balance, ebs_byte_balance_hist, _),
        (ebs_io_balance, ebs_io_balance_hist, _),
        (bin_log_disk_usage, bin_log_disk_usage_hist, _),
        (replica_lag, replica_lag_hist, _),
        (max_transaction_ids, max_transaction_ids_hist, _),
        (oldest_replication_slot_lag, oldest_replication_slot_lag_hist, _),
        // Missing logical replication slot lag metric result
        (oldest_logical_replication_slot_lag, oldest_logical_replication_slot_lag_hist, _),
        (replication_slot_disk_usage, replication_slot_disk_usage_hist, _),
        (transaction_logs_disk_usage, transaction_logs_disk_usage_hist, _),
        (transaction_logs_generation, transaction_logs_generation_hist, _),
        (failed_sql_server_agent_jobs, failed_sql_server_agent_jobs_hist, _),
        (checkpoint_lag, checkpoint_lag_hist, _),
        (connection_attempts, connection_attempts_hist, _),
    ) = tokio::join!(
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "BurstBalance".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Percent".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "CPUCreditUsage".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: None,
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "CPUCreditBalance".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: None,
            },
            start_time,
            end_time,
            period_seconds
        ),
        // Missing CPU surplus credit metrics
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "CPUSurplusCreditBalance".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: None,
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "CPUSurplusCreditsCharged".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: None,
            },
            start_time,
            end_time,
            period_seconds
        ),
        // Missing EBS performance metrics
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "EBSByteBalance%".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Percent".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "EBSIOBalance%".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Percent".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "BinLogDiskUsage".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "ReplicaLag".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Seconds".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "MaximumUsedTransactionIDs".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "OldestReplicationSlotLag".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        // Missing logical replication slot lag metric
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "OldestLogicalReplicationSlotLag".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "ReplicationSlotDiskUsage".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "TransactionLogsDiskUsage".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "TransactionLogsGeneration".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Bytes/Second".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "FailedSQLServerAgentJobsCount".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "CheckpointLag".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Seconds".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
        fetch_comprehensive_metric(
            client,
            MetricFetchParams {
                metric_name: "ConnectionAttempts".to_string(),
                namespace: "AWS/RDS".to_string(),
                instance_id: instance_id.to_string(),
                unit: Some("Count".to_string()),
            },
            start_time,
            end_time,
            period_seconds
        ),
    );

    AdvancedMetrics {
        burst_balance,
        burst_balance_history: burst_balance_hist,
        cpu_credit_usage,
        cpu_credit_usage_history: cpu_credit_usage_hist,
        cpu_credit_balance,
        cpu_credit_balance_history: cpu_credit_balance_hist,
        // Missing CPU surplus credit metrics
        cpu_surplus_credit_balance,
        cpu_surplus_credit_balance_history: cpu_surplus_credit_balance_hist,
        cpu_surplus_credits_charged,
        cpu_surplus_credits_charged_history: cpu_surplus_credits_charged_hist,
        // Missing EBS performance metrics
        ebs_byte_balance,
        ebs_byte_balance_history: ebs_byte_balance_hist,
        ebs_io_balance,
        ebs_io_balance_history: ebs_io_balance_hist,
        bin_log_disk_usage,
        bin_log_disk_usage_history: bin_log_disk_usage_hist,
        replica_lag,
        replica_lag_history: replica_lag_hist,
        maximum_used_transaction_ids: max_transaction_ids,
        maximum_used_transaction_ids_history: max_transaction_ids_hist,
        oldest_replication_slot_lag,
        oldest_replication_slot_lag_history: oldest_replication_slot_lag_hist,
        // Missing logical replication slot lag metric
        oldest_logical_replication_slot_lag,
        oldest_logical_replication_slot_lag_history: oldest_logical_replication_slot_lag_hist,
        replication_slot_disk_usage,
        replication_slot_disk_usage_history: replication_slot_disk_usage_hist,
        transaction_logs_disk_usage,
        transaction_logs_disk_usage_history: transaction_logs_disk_usage_hist,
        transaction_logs_generation,
        transaction_logs_generation_history: transaction_logs_generation_hist,
        failed_sql_server_agent_jobs_count: failed_sql_server_agent_jobs,
        failed_sql_server_agent_jobs_count_history: failed_sql_server_agent_jobs_hist,
        checkpoint_lag,
        checkpoint_lag_history: checkpoint_lag_hist,
        connection_attempts,
        connection_attempts_history: connection_attempts_hist,
    }
}

async fn fetch_relevant_core_metrics(
    client: &CloudWatchClient,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    period_seconds: i32,
    _relevant_metrics: &[crate::models::MetricType],
) -> CoreMetrics {
    // For now, fetch all core metrics (dynamic filtering can be added later)
    fetch_core_metrics(client, instance_id, start_time, end_time, period_seconds).await
}

async fn fetch_relevant_advanced_metrics(
    client: &CloudWatchClient,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    period_seconds: i32,
    _relevant_metrics: &[crate::models::MetricType],
) -> AdvancedMetrics {
    // For now, fetch all advanced metrics (dynamic filtering can be added later)
    fetch_advanced_metrics(client, instance_id, start_time, end_time, period_seconds).await
}

/// Build MetricData from core and advanced metrics
fn build_metric_data_inline(core: CoreMetrics, advanced: AdvancedMetrics) -> crate::models::MetricData {
    crate::models::MetricData {
        timestamps: core.timestamps,
        
        // Core metrics
        cpu_utilization: core.cpu_utilization,
        database_connections: core.database_connections,
        free_storage_space: core.free_storage_space,
        read_iops: core.read_iops,
        write_iops: core.write_iops,
        read_latency: core.read_latency,
        write_latency: core.write_latency,
        read_throughput: core.read_throughput,
        write_throughput: core.write_throughput,
        network_receive_throughput: core.network_receive_throughput,
        network_transmit_throughput: core.network_transmit_throughput,
        swap_usage: core.swap_usage,
        freeable_memory: core.freeable_memory,
        queue_depth: core.queue_depth,
        
        // Advanced metrics
        burst_balance: advanced.burst_balance,
        cpu_credit_usage: advanced.cpu_credit_usage,
        cpu_credit_balance: advanced.cpu_credit_balance,
        cpu_surplus_credit_balance: advanced.cpu_surplus_credit_balance,
        cpu_surplus_credits_charged: advanced.cpu_surplus_credits_charged,
        ebs_byte_balance: advanced.ebs_byte_balance,
        ebs_io_balance: advanced.ebs_io_balance,
        bin_log_disk_usage: advanced.bin_log_disk_usage,
        replica_lag: advanced.replica_lag,
        maximum_used_transaction_ids: advanced.maximum_used_transaction_ids,
        oldest_replication_slot_lag: advanced.oldest_replication_slot_lag,
        oldest_logical_replication_slot_lag: advanced.oldest_logical_replication_slot_lag,
        replication_slot_disk_usage: advanced.replication_slot_disk_usage,
        transaction_logs_disk_usage: advanced.transaction_logs_disk_usage,
        transaction_logs_generation: advanced.transaction_logs_generation,
        failed_sql_server_agent_jobs_count: advanced.failed_sql_server_agent_jobs_count,
        checkpoint_lag: advanced.checkpoint_lag,
        connection_attempts: advanced.connection_attempts,
        
        // History vectors
        cpu_history: core.cpu_history,
        connections_history: core.connections_history,
        read_iops_history: core.read_iops_history,
        write_iops_history: core.write_iops_history,
        read_latency_history: core.read_latency_history,
        write_latency_history: core.write_latency_history,
        read_throughput_history: core.read_throughput_history,
        write_throughput_history: core.write_throughput_history,
        network_receive_history: core.network_receive_history,
        network_transmit_history: core.network_transmit_history,
        freeable_memory_history: core.freeable_memory_history,
        swap_usage_history: core.swap_usage_history,
        queue_depth_history: core.queue_depth_history,
        free_storage_space_history: core.free_storage_space_history,
        
        // Advanced history vectors
        burst_balance_history: advanced.burst_balance_history,
        cpu_credit_usage_history: advanced.cpu_credit_usage_history,
        cpu_credit_balance_history: advanced.cpu_credit_balance_history,
        cpu_surplus_credit_balance_history: advanced.cpu_surplus_credit_balance_history,
        cpu_surplus_credits_charged_history: advanced.cpu_surplus_credits_charged_history,
        ebs_byte_balance_history: advanced.ebs_byte_balance_history,
        ebs_io_balance_history: advanced.ebs_io_balance_history,
        oldest_logical_replication_slot_lag_history: advanced.oldest_logical_replication_slot_lag_history,
        bin_log_disk_usage_history: advanced.bin_log_disk_usage_history,
        replica_lag_history: advanced.replica_lag_history,
        maximum_used_transaction_ids_history: advanced.maximum_used_transaction_ids_history,
        oldest_replication_slot_lag_history: advanced.oldest_replication_slot_lag_history,
        replication_slot_disk_usage_history: advanced.replication_slot_disk_usage_history,
        transaction_logs_disk_usage_history: advanced.transaction_logs_disk_usage_history,
        transaction_logs_generation_history: advanced.transaction_logs_generation_history,
        failed_sql_server_agent_jobs_count_history: advanced.failed_sql_server_agent_jobs_count_history,
        checkpoint_lag_history: advanced.checkpoint_lag_history,
        connection_attempts_history: advanced.connection_attempts_history,
    }
}

/// Build MetricData with characteristics  
fn build_metric_data_with_characteristics(
    core: CoreMetrics, 
    advanced: AdvancedMetrics,
    _characteristics: Option<crate::aws::metric_types::RdsInstanceCharacteristics>
) -> crate::models::MetricData {
    // For now, just ignore characteristics and use regular builder
    build_metric_data_inline(core, advanced)
}
