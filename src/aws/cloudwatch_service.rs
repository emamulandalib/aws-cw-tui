use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use std::time::{SystemTime, Duration};
use crate::models::MetricData;

#[derive(Debug, Clone, Copy)]
pub enum TimeUnit {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
}

#[derive(Debug, Clone, Copy)]
pub struct TimeRange {
    pub value: u32,
    pub unit: TimeUnit,
    pub period_days: u32,
}

impl TimeRange {
    pub fn new(value: u32, unit: TimeUnit, period_days: u32) -> Result<Self> {
        // Validate input values
        match unit {
            TimeUnit::Minutes if value < 1 => {
                return Err(anyhow::anyhow!("Minutes must be at least 1"));
            }
            TimeUnit::Months if value > 15 => {
                return Err(anyhow::anyhow!("Months must not exceed 15"));
            }
            _ => {}
        }

        if period_days < 1 || period_days > 30 {
            return Err(anyhow::anyhow!("Period must be between 1 and 30 days"));
        }

        Ok(Self {
            value,
            unit,
            period_days,
        })
    }

    pub fn to_duration(&self) -> Duration {
        let seconds = match self.unit {
            TimeUnit::Minutes => self.value as u64 * 60,
            TimeUnit::Hours => self.value as u64 * 3600,
            TimeUnit::Days => self.value as u64 * 86400,
            TimeUnit::Weeks => self.value as u64 * 604800,
            TimeUnit::Months => self.value as u64 * 2592000, // Approximate: 30 days per month
        };
        Duration::from_secs(seconds)
    }
}

pub async fn load_metrics(instance_id: &str, time_range: TimeRange) -> Result<MetricData> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = CloudWatchClient::new(&config);

    let end_time = SystemTime::now();
    let start_time = end_time - time_range.to_duration();

    let instance_id_owned = instance_id.to_string();
    
    // Calculate period based on time range duration and period_days
    let period_seconds = calculate_period_seconds(&time_range);
    
    // Fetch core metrics concurrently
    let core_metrics = fetch_core_metrics(&client, &instance_id_owned, start_time, end_time, period_seconds).await;
    let advanced_metrics = fetch_advanced_metrics(&client, &instance_id_owned, start_time, end_time, period_seconds).await;

    Ok(build_metric_data(core_metrics, advanced_metrics))
}

fn calculate_period_seconds(time_range: &TimeRange) -> i32 {
    // Calculate appropriate period based on time range duration and period_days
    let duration_seconds = time_range.to_duration().as_secs();
    
    // Use period_days to influence the granularity
    // Shorter period_days means finer granularity, longer means coarser
    let base_period = match duration_seconds {
        0..=3600 => 60,        // 1 minute for <= 1 hour
        3601..=21600 => 300,   // 5 minutes for <= 6 hours
        21601..=86400 => 900,  // 15 minutes for <= 1 day
        86401..=604800 => 3600, // 1 hour for <= 1 week
        _ => {
            // For longer periods, use period_days to calculate appropriate granularity
            let target_points = 100; // Target ~100 data points
            let calculated_period = (duration_seconds / target_points).max(3600) as i32;
            
            // Ensure period aligns with CloudWatch supported periods
            match calculated_period {
                0..=300 => 300,
                301..=900 => 900,
                901..=3600 => 3600,
                _ => ((calculated_period / 3600) * 3600).min(86400), // Round to hours, max 1 day
            }
        }
    };
    
    // Adjust period based on period_days setting
    // Lower period_days = finer granularity, higher period_days = coarser granularity
    let period_multiplier = match time_range.period_days {
        1..=3 => 1,      // Fine granularity for short periods
        4..=7 => 2,      // Medium granularity
        8..=14 => 3,     // Coarser granularity for medium periods
        _ => 4,          // Coarsest granularity for long periods
    };
    
    let adjusted_period = base_period * period_multiplier;
    
    // Ensure the period doesn't exceed CloudWatch limits (max 1 day = 86400 seconds)
    adjusted_period.min(86400)
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
        fetch_comprehensive_metric(client, "CPUUtilization", "AWS/RDS", instance_id, start_time, end_time, Some("Percent"), period_seconds),
        fetch_comprehensive_metric(client, "DatabaseConnections", "AWS/RDS", instance_id, start_time, end_time, Some("Count"), period_seconds),
        fetch_comprehensive_metric(client, "FreeStorageSpace", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "ReadIOPS", "AWS/RDS", instance_id, start_time, end_time, Some("Count/Second"), period_seconds),
        fetch_comprehensive_metric(client, "WriteIOPS", "AWS/RDS", instance_id, start_time, end_time, Some("Count/Second"), period_seconds),
        fetch_comprehensive_metric(client, "ReadLatency", "AWS/RDS", instance_id, start_time, end_time, Some("Seconds"), period_seconds),
        fetch_comprehensive_metric(client, "WriteLatency", "AWS/RDS", instance_id, start_time, end_time, Some("Seconds"), period_seconds),
        fetch_comprehensive_metric(client, "ReadThroughput", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes/Second"), period_seconds),
        fetch_comprehensive_metric(client, "WriteThroughput", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes/Second"), period_seconds),
        fetch_comprehensive_metric(client, "NetworkReceiveThroughput", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes/Second"), period_seconds),
        fetch_comprehensive_metric(client, "NetworkTransmitThroughput", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes/Second"), period_seconds),
        fetch_comprehensive_metric(client, "SwapUsage", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "FreeableMemory", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "DiskQueueDepth", "AWS/RDS", instance_id, start_time, end_time, Some("Count"), period_seconds),
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
        (bin_log_disk_usage, bin_log_disk_usage_hist, _),
        (replica_lag, replica_lag_hist, _),
        (max_transaction_ids, max_transaction_ids_hist, _),
        (oldest_replication_slot_lag, oldest_replication_slot_lag_hist, _),
        (replication_slot_disk_usage, replication_slot_disk_usage_hist, _),
        (transaction_logs_disk_usage, transaction_logs_disk_usage_hist, _),
        (transaction_logs_generation, transaction_logs_generation_hist, _),
        (failed_sql_server_agent_jobs, failed_sql_server_agent_jobs_hist, _),
        (checkpoint_lag, checkpoint_lag_hist, _),
        (connection_attempts, connection_attempts_hist, _),
    ) = tokio::join!(
        fetch_comprehensive_metric(client, "BurstBalance", "AWS/RDS", instance_id, start_time, end_time, Some("Percent"), period_seconds),
        fetch_comprehensive_metric(client, "CPUCreditUsage", "AWS/RDS", instance_id, start_time, end_time, None, period_seconds),
        fetch_comprehensive_metric(client, "CPUCreditBalance", "AWS/RDS", instance_id, start_time, end_time, None, period_seconds),
        fetch_comprehensive_metric(client, "BinLogDiskUsage", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "ReplicaLag", "AWS/RDS", instance_id, start_time, end_time, Some("Seconds"), period_seconds),
        fetch_comprehensive_metric(client, "MaximumUsedTransactionIDs", "AWS/RDS", instance_id, start_time, end_time, Some("Count"), period_seconds),
        fetch_comprehensive_metric(client, "OldestReplicationSlotLag", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "ReplicationSlotDiskUsage", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "TransactionLogsDiskUsage", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes"), period_seconds),
        fetch_comprehensive_metric(client, "TransactionLogsGeneration", "AWS/RDS", instance_id, start_time, end_time, Some("Bytes/Second"), period_seconds),
        fetch_comprehensive_metric(client, "FailedSQLServerAgentJobsCount", "AWS/RDS", instance_id, start_time, end_time, Some("Count"), period_seconds),
        fetch_comprehensive_metric(client, "CheckpointLag", "AWS/RDS", instance_id, start_time, end_time, Some("Seconds"), period_seconds),
        fetch_comprehensive_metric(client, "ConnectionAttempts", "AWS/RDS", instance_id, start_time, end_time, Some("Count"), period_seconds),
    );

    AdvancedMetrics {
        burst_balance,
        burst_balance_history: burst_balance_hist,
        cpu_credit_usage,
        cpu_credit_usage_history: cpu_credit_usage_hist,
        cpu_credit_balance,
        cpu_credit_balance_history: cpu_credit_balance_hist,
        bin_log_disk_usage,
        bin_log_disk_usage_history: bin_log_disk_usage_hist,
        replica_lag,
        replica_lag_history: replica_lag_hist,
        maximum_used_transaction_ids: max_transaction_ids,
        maximum_used_transaction_ids_history: max_transaction_ids_hist,
        oldest_replication_slot_lag,
        oldest_replication_slot_lag_history: oldest_replication_slot_lag_hist,
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

async fn fetch_comprehensive_metric(
    client: &CloudWatchClient,
    metric_name: &str,
    namespace: &str,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    unit: Option<&str>,
    period_seconds: i32,
) -> (f64, Vec<f64>, Vec<SystemTime>) {
    let mut request = client
        .get_metric_statistics()
        .namespace(namespace)
        .metric_name(metric_name)
        .dimensions(
            aws_sdk_cloudwatch::types::Dimension::builder()
                .name("DBInstanceIdentifier")
                .value(instance_id)
                .build(),
        )
        .start_time(aws_sdk_cloudwatch::primitives::DateTime::from(start_time))
        .end_time(aws_sdk_cloudwatch::primitives::DateTime::from(end_time))
        .period(period_seconds)
        .statistics(aws_sdk_cloudwatch::types::Statistic::Average);

    if let Some(u) = unit {
        match u {
            "Percent" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Percent),
            "Count" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Count),
            "Count/Second" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::CountSecond),
            "Bytes" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Bytes),
            "Bytes/Second" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::BytesSecond),
            "Seconds" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds),
            _ => {}
        }
    }

    let resp = request.send().await;

    match resp {
        Ok(data) => {
            if let Some(mut datapoints) = data.datapoints {
                datapoints.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
                
                let latest_value = datapoints.last()
                    .and_then(|dp| dp.average)
                    .unwrap_or(0.0);
                
                let recent_datapoints: Vec<_> = datapoints
                    .iter()
                    .rev()
                    .take(36)
                    .rev()
                    .collect();
                
                let history: Vec<f64> = recent_datapoints
                    .iter()
                    .filter_map(|dp| dp.average)
                    .collect();
                
                let timestamps: Vec<SystemTime> = recent_datapoints
                    .iter()
                    .map(|dp| {
                        dp.timestamp
                            .map(|ts| SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(ts.secs() as u64))
                            .unwrap_or_else(SystemTime::now)
                    })
                    .collect();
                
                (latest_value, history, timestamps)
            } else {
                (0.0, Vec::new(), Vec::new())
            }
        }
        Err(_) => {
            (0.0, Vec::new(), Vec::new())
        }
    }
}

// Helper structs to organize metric data
struct CoreMetrics {
    cpu_utilization: f64,
    cpu_history: Vec<f64>,
    timestamps: Vec<SystemTime>,
    database_connections: f64,
    connections_history: Vec<f64>,
    free_storage_space: f64,
    free_storage_space_history: Vec<f64>,
    read_iops: f64,
    read_iops_history: Vec<f64>,
    write_iops: f64,
    write_iops_history: Vec<f64>,
    read_latency: f64,
    read_latency_history: Vec<f64>,
    write_latency: f64,
    write_latency_history: Vec<f64>,
    read_throughput: f64,
    read_throughput_history: Vec<f64>,
    write_throughput: f64,
    write_throughput_history: Vec<f64>,
    network_receive_throughput: f64,
    network_receive_history: Vec<f64>,
    network_transmit_throughput: f64,
    network_transmit_history: Vec<f64>,
    swap_usage: f64,
    swap_usage_history: Vec<f64>,
    freeable_memory: f64,
    freeable_memory_history: Vec<f64>,
    queue_depth: f64,
    queue_depth_history: Vec<f64>,
}

struct AdvancedMetrics {
    burst_balance: f64,
    burst_balance_history: Vec<f64>,
    cpu_credit_usage: f64,
    cpu_credit_usage_history: Vec<f64>,
    cpu_credit_balance: f64,
    cpu_credit_balance_history: Vec<f64>,
    bin_log_disk_usage: f64,
    bin_log_disk_usage_history: Vec<f64>,
    replica_lag: f64,
    replica_lag_history: Vec<f64>,
    maximum_used_transaction_ids: f64,
    maximum_used_transaction_ids_history: Vec<f64>,
    oldest_replication_slot_lag: f64,
    oldest_replication_slot_lag_history: Vec<f64>,
    replication_slot_disk_usage: f64,
    replication_slot_disk_usage_history: Vec<f64>,
    transaction_logs_disk_usage: f64,
    transaction_logs_disk_usage_history: Vec<f64>,
    transaction_logs_generation: f64,
    transaction_logs_generation_history: Vec<f64>,
    failed_sql_server_agent_jobs_count: f64,
    failed_sql_server_agent_jobs_count_history: Vec<f64>,
    checkpoint_lag: f64,
    checkpoint_lag_history: Vec<f64>,
    connection_attempts: f64,
    connection_attempts_history: Vec<f64>,
}

fn build_metric_data(core: CoreMetrics, advanced: AdvancedMetrics) -> MetricData {
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
        failed_sql_server_agent_jobs_count_history: advanced.failed_sql_server_agent_jobs_count_history,
        checkpoint_lag: advanced.checkpoint_lag,
        checkpoint_lag_history: advanced.checkpoint_lag_history,
        connection_attempts: advanced.connection_attempts,
        connection_attempts_history: advanced.connection_attempts_history,
    }
}
