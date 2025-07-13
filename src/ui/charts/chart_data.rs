use crate::models::{App, MetricType};
use std::time::SystemTime;

/// AWS Console-style metric chart data
#[derive(Debug, Clone)]
pub struct MetricChartData {
    pub metric_type: MetricType,
    pub current_value: f64,
    pub history: Vec<f64>,
    pub timestamps: Vec<SystemTime>,
}

impl MetricChartData {
    /// Create chart data from app state
    pub fn from_app(app: &App, metric_type: MetricType) -> Option<Self> {
        let service = app.selected_service.as_ref()?;

        match service {
            crate::models::AwsService::Rds => {
                let metrics = &app.metrics;
                let current_value = get_rds_current_value(metrics, &metric_type)?;
                let history = get_rds_history(metrics, &metric_type)?;

                Some(MetricChartData {
                    metric_type,
                    current_value,
                    history: history.clone(),
                    timestamps: metrics.timestamps.clone(),
                })
            }
            crate::models::AwsService::Sqs => {
                let metrics = &app.sqs_metrics;
                let current_value = get_sqs_current_value(metrics, &metric_type)?;
                let history = get_sqs_history(metrics, &metric_type)?;

                Some(MetricChartData {
                    metric_type,
                    current_value,
                    history: history.clone(),
                    timestamps: metrics.timestamps.clone(),
                })
            }
        }
    }
}

// Helper functions to extract current values and history from different metric types
pub fn get_rds_current_value(
    metrics: &crate::models::MetricData,
    metric_type: &MetricType,
) -> Option<f64> {
    match metric_type {
        MetricType::CpuUtilization => Some(metrics.cpu_utilization),
        MetricType::DatabaseConnections => Some(metrics.database_connections),
        MetricType::FreeStorageSpace => Some(metrics.free_storage_space),
        MetricType::ReadLatency => Some(metrics.read_latency),
        MetricType::WriteLatency => Some(metrics.write_latency),
        MetricType::ReadIops => Some(metrics.read_iops),
        MetricType::WriteIops => Some(metrics.write_iops),
        MetricType::FreeableMemory => Some(metrics.freeable_memory),
        MetricType::ReadThroughput => Some(metrics.read_throughput),
        MetricType::WriteThroughput => Some(metrics.write_throughput),
        MetricType::NetworkReceiveThroughput => Some(metrics.network_receive_throughput),
        MetricType::NetworkTransmitThroughput => Some(metrics.network_transmit_throughput),
        MetricType::SwapUsage => Some(metrics.swap_usage),
        MetricType::BurstBalance => Some(metrics.burst_balance),
        MetricType::CpuCreditUsage => Some(metrics.cpu_credit_usage),
        MetricType::CpuCreditBalance => Some(metrics.cpu_credit_balance),
        MetricType::QueueDepth => Some(metrics.queue_depth),
        MetricType::BinLogDiskUsage => Some(metrics.bin_log_disk_usage),
        MetricType::ReplicaLag => Some(metrics.replica_lag),
        MetricType::TransactionLogsGeneration => Some(metrics.transaction_logs_generation),
        MetricType::TransactionLogsDiskUsage => Some(metrics.transaction_logs_disk_usage),
        MetricType::MaximumUsedTransactionIds => Some(metrics.maximum_used_transaction_ids),
        MetricType::OldestReplicationSlotLag => Some(metrics.oldest_replication_slot_lag),
        MetricType::ReplicationSlotDiskUsage => Some(metrics.replication_slot_disk_usage),
        MetricType::FailedSqlServerAgentJobsCount => {
            Some(metrics.failed_sql_server_agent_jobs_count)
        }
        MetricType::CheckpointLag => Some(metrics.checkpoint_lag),
        MetricType::ConnectionAttempts => Some(metrics.connection_attempts),
        _ => None,
    }
}

pub fn get_rds_history<'a>(
    metrics: &'a crate::models::MetricData,
    metric_type: &MetricType,
) -> Option<&'a Vec<f64>> {
    match metric_type {
        MetricType::CpuUtilization => Some(&metrics.cpu_history),
        MetricType::DatabaseConnections => Some(&metrics.connections_history),
        MetricType::FreeStorageSpace => Some(&metrics.free_storage_space_history),
        MetricType::ReadLatency => Some(&metrics.read_latency_history),
        MetricType::WriteLatency => Some(&metrics.write_latency_history),
        MetricType::ReadIops => Some(&metrics.read_iops_history),
        MetricType::WriteIops => Some(&metrics.write_iops_history),
        MetricType::FreeableMemory => Some(&metrics.freeable_memory_history),
        MetricType::ReadThroughput => Some(&metrics.read_throughput_history),
        MetricType::WriteThroughput => Some(&metrics.write_throughput_history),
        MetricType::NetworkReceiveThroughput => Some(&metrics.network_receive_history),
        MetricType::NetworkTransmitThroughput => Some(&metrics.network_transmit_history),
        MetricType::SwapUsage => Some(&metrics.swap_usage_history),
        MetricType::BurstBalance => Some(&metrics.burst_balance_history),
        MetricType::CpuCreditUsage => Some(&metrics.cpu_credit_usage_history),
        MetricType::CpuCreditBalance => Some(&metrics.cpu_credit_balance_history),
        MetricType::QueueDepth => Some(&metrics.queue_depth_history),
        MetricType::BinLogDiskUsage => Some(&metrics.bin_log_disk_usage_history),
        MetricType::ReplicaLag => Some(&metrics.replica_lag_history),
        MetricType::TransactionLogsGeneration => Some(&metrics.transaction_logs_generation_history),
        MetricType::TransactionLogsDiskUsage => Some(&metrics.transaction_logs_disk_usage_history),
        MetricType::MaximumUsedTransactionIds => {
            Some(&metrics.maximum_used_transaction_ids_history)
        }
        MetricType::OldestReplicationSlotLag => Some(&metrics.oldest_replication_slot_lag_history),
        MetricType::ReplicationSlotDiskUsage => Some(&metrics.replication_slot_disk_usage_history),
        MetricType::FailedSqlServerAgentJobsCount => {
            Some(&metrics.failed_sql_server_agent_jobs_count_history)
        }
        MetricType::CheckpointLag => Some(&metrics.checkpoint_lag_history),
        MetricType::ConnectionAttempts => Some(&metrics.connection_attempts_history),
        _ => None,
    }
}

pub fn get_sqs_current_value(
    metrics: &crate::models::SqsMetricData,
    metric_type: &MetricType,
) -> Option<f64> {
    match metric_type {
        MetricType::ApproximateNumberOfMessages => Some(metrics.approximate_number_of_messages),
        MetricType::ApproximateNumberOfMessagesVisible => {
            Some(metrics.approximate_number_of_messages_visible)
        }
        MetricType::ApproximateNumberOfMessagesNotVisible => {
            Some(metrics.approximate_number_of_messages_not_visible)
        }
        MetricType::ApproximateAgeOfOldestMessage => {
            Some(metrics.approximate_age_of_oldest_message)
        }
        MetricType::ApproximateNumberOfMessagesDelayed => {
            Some(metrics.approximate_number_of_messages_delayed)
        }
        MetricType::NumberOfMessagesSent => Some(metrics.number_of_messages_sent),
        MetricType::NumberOfMessagesReceived => Some(metrics.number_of_messages_received),
        MetricType::NumberOfMessagesDeleted => Some(metrics.number_of_messages_deleted),
        MetricType::NumberOfEmptyReceives => Some(metrics.number_of_empty_receives),
        MetricType::SentMessageSize => Some(metrics.sent_message_size),
        _ => None,
    }
}

pub fn get_sqs_history<'a>(
    metrics: &'a crate::models::SqsMetricData,
    metric_type: &MetricType,
) -> Option<&'a Vec<f64>> {
    match metric_type {
        MetricType::ApproximateNumberOfMessages => Some(&metrics.queue_depth_history),
        MetricType::ApproximateNumberOfMessagesVisible => Some(&metrics.messages_visible_history),
        MetricType::ApproximateNumberOfMessagesNotVisible => {
            Some(&metrics.messages_not_visible_history)
        }
        MetricType::ApproximateAgeOfOldestMessage => Some(&metrics.oldest_message_age_history),
        MetricType::ApproximateNumberOfMessagesDelayed => Some(&metrics.messages_delayed_history),
        MetricType::NumberOfMessagesSent => Some(&metrics.messages_sent_history),
        MetricType::NumberOfMessagesReceived => Some(&metrics.messages_received_history),
        MetricType::NumberOfMessagesDeleted => Some(&metrics.messages_deleted_history),
        MetricType::NumberOfEmptyReceives => Some(&metrics.empty_receives_history),
        MetricType::SentMessageSize => Some(&metrics.sent_message_size_history),
        _ => None,
    }
}
