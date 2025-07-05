use crate::aws::cloudwatch_service::TimeRange;
use ratatui::widgets::ListState;
use std::time::{Instant, SystemTime};


#[derive(Debug, Clone)]
pub struct RdsInstance {
    pub identifier: String,
    pub engine: String,
    pub status: String,
    pub instance_class: String,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SqsQueue {
    pub url: String,
    pub name: String,
    pub queue_type: String, // "Standard" or "FIFO"
    pub attributes: std::collections::HashMap<String, String>,
}
impl AwsInstance for RdsInstance {
    fn id(&self) -> &str {
        &self.identifier
    }

    fn name(&self) -> Option<&str> {
        Some(&self.identifier)
    }

    fn status(&self) -> &str {
        &self.status
    }

    fn service_type(&self) -> AwsService {
        AwsService::Rds
    }
}
impl AwsInstance for SqsQueue {
    fn id(&self) -> &str {
        &self.url
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn status(&self) -> &str {
        &self.queue_type
    }

    fn service_type(&self) -> AwsService {
        AwsService::Sqs
    }
}
#[derive(Debug)]
pub struct MetricData {
    // Core Performance Metrics
    pub cpu_utilization: f64,
    pub database_connections: f64,
    pub free_storage_space: f64,
    pub read_iops: f64,
    pub write_iops: f64,
    pub read_latency: f64,
    pub write_latency: f64,

    // Extended RDS Metrics
    pub read_throughput: f64,  // Bytes/second
    pub write_throughput: f64, // Bytes/second
    pub network_receive_throughput: f64,
    pub network_transmit_throughput: f64,
    pub swap_usage: f64,      // Bytes
    pub freeable_memory: f64, // Bytes
    pub queue_depth: f64,     // Number of outstanding I/O requests

    // Additional Core RDS Metrics (27 total metrics)
    pub burst_balance: f64,      // Percent - GP2 burst bucket credits
    pub cpu_credit_usage: f64,   // Credits - for T2/T3/T4g instances
    pub cpu_credit_balance: f64, // Credits - for T2/T3/T4g instances
    pub bin_log_disk_usage: f64, // Bytes - MySQL/MariaDB binary logs
    pub replica_lag: f64,        // Seconds - read replica lag
    pub maximum_used_transaction_ids: f64, // Count - PostgreSQL transaction IDs
    pub oldest_replication_slot_lag: f64, // Bytes - PostgreSQL replication slot lag
    pub replication_slot_disk_usage: f64, // Bytes - PostgreSQL replication slots
    pub transaction_logs_disk_usage: f64, // Bytes - PostgreSQL transaction logs
    pub transaction_logs_generation: f64, // Bytes/second - PostgreSQL log generation
    pub failed_sql_server_agent_jobs_count: f64, // Count/minute - SQL Server agent jobs
    pub checkpoint_lag: f64,     // Seconds - checkpoint lag
    pub connection_attempts: f64, // Count - MySQL connection attempts

    // Historical data for 3 hours (36 data points at 5min intervals)
    pub timestamps: Vec<SystemTime>,
    pub cpu_history: Vec<f64>,
    pub connections_history: Vec<f64>,
    pub read_iops_history: Vec<f64>,
    pub write_iops_history: Vec<f64>,
    pub read_latency_history: Vec<f64>,
    pub write_latency_history: Vec<f64>,
    pub read_throughput_history: Vec<f64>,
    pub write_throughput_history: Vec<f64>,
    pub network_receive_history: Vec<f64>,
    pub network_transmit_history: Vec<f64>,
    pub freeable_memory_history: Vec<f64>,
    pub swap_usage_history: Vec<f64>,
    pub queue_depth_history: Vec<f64>,
    pub free_storage_space_history: Vec<f64>,

    // Additional metric histories
    pub burst_balance_history: Vec<f64>,
    pub cpu_credit_usage_history: Vec<f64>,
    pub cpu_credit_balance_history: Vec<f64>,
    pub bin_log_disk_usage_history: Vec<f64>,
    pub replica_lag_history: Vec<f64>,
    pub maximum_used_transaction_ids_history: Vec<f64>,
    pub oldest_replication_slot_lag_history: Vec<f64>,
    pub replication_slot_disk_usage_history: Vec<f64>,
    pub transaction_logs_disk_usage_history: Vec<f64>,
    pub transaction_logs_generation_history: Vec<f64>,
    pub failed_sql_server_agent_jobs_count_history: Vec<f64>,
    pub checkpoint_lag_history: Vec<f64>,
    pub connection_attempts_history: Vec<f64>,
}

#[derive(Debug)]
pub struct SqsMetricData {
    // Message Flow Metrics
    pub number_of_messages_sent: f64,
    pub number_of_messages_received: f64,
    pub number_of_messages_deleted: f64,
    pub approximate_number_of_messages: f64, // Total queue depth (visible + not visible)
    pub approximate_number_of_messages_visible: f64, // Currently available messages
    pub approximate_number_of_messages_not_visible: f64,

    // Queue Performance Metrics
    pub approximate_age_of_oldest_message: f64,
    pub number_of_empty_receives: f64,
    pub approximate_number_of_messages_delayed: f64, // FIFO only
    pub sent_message_size: f64, // Message size in bytes

    // Dead Letter Queue Metrics
    pub number_of_messages_in_dlq: f64,

    // FIFO-specific Metrics
    pub approximate_number_of_groups_with_inflight_messages: f64, // FIFO only
    pub number_of_deduplicated_sent_messages: f64, // FIFO only

    // Historical data for 3 hours (36 data points at 5min intervals)
    pub timestamps: Vec<SystemTime>,
    pub messages_sent_history: Vec<f64>,
    pub messages_received_history: Vec<f64>,
    pub messages_deleted_history: Vec<f64>,
    pub queue_depth_history: Vec<f64>, // approximate_number_of_messages
    pub messages_visible_history: Vec<f64>, // approximate_number_of_messages_visible
    pub messages_not_visible_history: Vec<f64>,
    pub oldest_message_age_history: Vec<f64>,
    pub empty_receives_history: Vec<f64>,
    pub messages_delayed_history: Vec<f64>,
    pub sent_message_size_history: Vec<f64>,
    pub dlq_messages_history: Vec<f64>,
    pub groups_with_inflight_messages_history: Vec<f64>, // FIFO only
    pub deduplicated_sent_messages_history: Vec<f64>, // FIFO only
}

impl Default for SqsMetricData {
    fn default() -> Self {
        Self {
            // Current metrics
            number_of_messages_sent: 0.0,
            number_of_messages_received: 0.0,
            number_of_messages_deleted: 0.0,
            approximate_number_of_messages: 0.0,
            approximate_number_of_messages_visible: 0.0,
            approximate_number_of_messages_not_visible: 0.0,
            approximate_age_of_oldest_message: 0.0,
            number_of_empty_receives: 0.0,
            approximate_number_of_messages_delayed: 0.0,
            sent_message_size: 0.0,
            number_of_messages_in_dlq: 0.0,
            
            // FIFO-specific metrics
            approximate_number_of_groups_with_inflight_messages: 0.0,
            number_of_deduplicated_sent_messages: 0.0,

            // Historical data
            timestamps: Vec::new(),
            messages_sent_history: Vec::new(),
            messages_received_history: Vec::new(),
            messages_deleted_history: Vec::new(),
            queue_depth_history: Vec::new(),
            messages_visible_history: Vec::new(),
            messages_not_visible_history: Vec::new(),
            oldest_message_age_history: Vec::new(),
            empty_receives_history: Vec::new(),
            messages_delayed_history: Vec::new(),
            sent_message_size_history: Vec::new(),
            dlq_messages_history: Vec::new(),
            groups_with_inflight_messages_history: Vec::new(),
            deduplicated_sent_messages_history: Vec::new(),
        }
    }
}

impl SqsMetricData {
    pub fn count_available_metrics(&self) -> usize {
        let mut count = 0;

        let histories = [
            &self.messages_sent_history,
            &self.messages_received_history,
            &self.messages_deleted_history,
            &self.queue_depth_history,
            &self.messages_visible_history,
            &self.messages_not_visible_history,
            &self.oldest_message_age_history,
            &self.empty_receives_history,
            &self.messages_delayed_history,
            &self.sent_message_size_history,
            &self.dlq_messages_history,
            &self.groups_with_inflight_messages_history,
            &self.deduplicated_sent_messages_history,
        ];

        for history in histories.iter() {
            if !history.is_empty() {
                count += 1;
            }
        }

        count
    }

    pub fn get_available_metrics(&self) -> Vec<MetricType> {
        let mut available_metrics = Vec::new();

        if !self.messages_sent_history.is_empty() {
            available_metrics.push(MetricType::NumberOfMessagesSent);
        }
        if !self.messages_received_history.is_empty() {
            available_metrics.push(MetricType::NumberOfMessagesReceived);
        }
        if !self.messages_deleted_history.is_empty() {
            available_metrics.push(MetricType::NumberOfMessagesDeleted);
        }
        if !self.queue_depth_history.is_empty() {
            available_metrics.push(MetricType::ApproximateNumberOfMessages);
        }
        if !self.messages_visible_history.is_empty() {
            available_metrics.push(MetricType::ApproximateNumberOfMessagesVisible);
        }
        if !self.messages_not_visible_history.is_empty() {
            available_metrics.push(MetricType::ApproximateNumberOfMessagesNotVisible);
        }
        if !self.oldest_message_age_history.is_empty() {
            available_metrics.push(MetricType::ApproximateAgeOfOldestMessage);
        }
        if !self.empty_receives_history.is_empty() {
            available_metrics.push(MetricType::NumberOfEmptyReceives);
        }
        if !self.messages_delayed_history.is_empty() {
            available_metrics.push(MetricType::ApproximateNumberOfMessagesDelayed);
        }
        if !self.sent_message_size_history.is_empty() {
            available_metrics.push(MetricType::SentMessageSize);
        }
        if !self.dlq_messages_history.is_empty() {
            available_metrics.push(MetricType::NumberOfMessagesInDlq);
        }
        
        // FIFO-specific metrics
        if !self.groups_with_inflight_messages_history.is_empty() {
            available_metrics.push(MetricType::ApproximateNumberOfGroupsWithInflightMessages);
        }
        if !self.deduplicated_sent_messages_history.is_empty() {
            available_metrics.push(MetricType::NumberOfDeduplicatedSentMessages);
        }

        available_metrics
    }

    pub fn get_metric_history(&self, metric_type: &MetricType) -> &Vec<f64> {
        match metric_type {
            MetricType::NumberOfMessagesSent => &self.messages_sent_history,
            MetricType::NumberOfMessagesReceived => &self.messages_received_history,
            MetricType::NumberOfMessagesDeleted => &self.messages_deleted_history,
            MetricType::ApproximateNumberOfMessages => &self.queue_depth_history,
            MetricType::ApproximateNumberOfMessagesVisible => &self.messages_visible_history,
            MetricType::ApproximateNumberOfMessagesNotVisible => &self.messages_not_visible_history,
            MetricType::ApproximateAgeOfOldestMessage => &self.oldest_message_age_history,
            MetricType::NumberOfEmptyReceives => &self.empty_receives_history,
            MetricType::ApproximateNumberOfMessagesDelayed => &self.messages_delayed_history,
            MetricType::SentMessageSize => &self.sent_message_size_history,
            MetricType::NumberOfMessagesInDlq => &self.dlq_messages_history,
            
            // FIFO-specific metrics
            MetricType::ApproximateNumberOfGroupsWithInflightMessages => &self.groups_with_inflight_messages_history,
            MetricType::NumberOfDeduplicatedSentMessages => &self.deduplicated_sent_messages_history,
            
            // RDS metrics - return empty for SQS MetricData
            _ => {
                // Use a static empty vector to avoid temporary value issues
                static EMPTY_VEC: Vec<f64> = Vec::new();
                &EMPTY_VEC
            }
        }
    }
}

impl Default for MetricData {
    fn default() -> Self {
        Self {
            timestamps: Vec::new(),
            cpu_utilization: 0.0,
            database_connections: 0.0,
            free_storage_space: 0.0,
            read_iops: 0.0,
            write_iops: 0.0,
            read_latency: 0.0,
            write_latency: 0.0,
            read_throughput: 0.0,
            write_throughput: 0.0,
            network_receive_throughput: 0.0,
            network_transmit_throughput: 0.0,
            swap_usage: 0.0,
            freeable_memory: 0.0,
            queue_depth: 0.0,

            // Additional Core RDS Metrics
            burst_balance: 0.0,
            cpu_credit_usage: 0.0,
            cpu_credit_balance: 0.0,
            bin_log_disk_usage: 0.0,
            replica_lag: 0.0,
            maximum_used_transaction_ids: 0.0,
            oldest_replication_slot_lag: 0.0,
            replication_slot_disk_usage: 0.0,
            transaction_logs_disk_usage: 0.0,
            transaction_logs_generation: 0.0,
            failed_sql_server_agent_jobs_count: 0.0,
            checkpoint_lag: 0.0,
            connection_attempts: 0.0,

            cpu_history: Vec::new(),
            connections_history: Vec::new(),
            read_iops_history: Vec::new(),
            write_iops_history: Vec::new(),
            read_latency_history: Vec::new(),
            write_latency_history: Vec::new(),
            read_throughput_history: Vec::new(),
            write_throughput_history: Vec::new(),
            network_receive_history: Vec::new(),
            network_transmit_history: Vec::new(),
            freeable_memory_history: Vec::new(),
            swap_usage_history: Vec::new(),
            queue_depth_history: Vec::new(),
            free_storage_space_history: Vec::new(),

            // Additional metric histories
            burst_balance_history: Vec::new(),
            cpu_credit_usage_history: Vec::new(),
            cpu_credit_balance_history: Vec::new(),
            bin_log_disk_usage_history: Vec::new(),
            replica_lag_history: Vec::new(),
            maximum_used_transaction_ids_history: Vec::new(),
            oldest_replication_slot_lag_history: Vec::new(),
            replication_slot_disk_usage_history: Vec::new(),
            transaction_logs_disk_usage_history: Vec::new(),
            transaction_logs_generation_history: Vec::new(),
            failed_sql_server_agent_jobs_count_history: Vec::new(),
            checkpoint_lag_history: Vec::new(),
            connection_attempts_history: Vec::new(),
        }
    }
}

impl MetricData {
    pub fn count_available_metrics(&self) -> usize {
        let mut count = 0;

        // Core metrics (14) - always counted if they have data
        let core_metric_names = [
            "cpu_history",
            "connections_history",
            "read_iops_history",
            "write_iops_history",
            "read_latency_history",
            "write_latency_history",
            "free_storage_space_history",
            "read_throughput_history",
            "write_throughput_history",
            "network_receive_history",
            "network_transmit_history",
            "freeable_memory_history",
            "swap_usage_history",
            "queue_depth_history",
        ];

        let core_histories = [
            &self.cpu_history,
            &self.connections_history,
            &self.read_iops_history,
            &self.write_iops_history,
            &self.read_latency_history,
            &self.write_latency_history,
            &self.free_storage_space_history,
            &self.read_throughput_history,
            &self.write_throughput_history,
            &self.network_receive_history,
            &self.network_transmit_history,
            &self.freeable_memory_history,
            &self.swap_usage_history,
            &self.queue_depth_history,
        ];

        for (_, history) in core_metric_names.iter().zip(core_histories.iter()) {
            if !history.is_empty() {
                count += 1;
            }
        }

        // Advanced metrics (13) - only counted if they have data
        let advanced_metric_names = [
            "burst_balance_history",
            "cpu_credit_usage_history",
            "cpu_credit_balance_history",
            "bin_log_disk_usage_history",
            "replica_lag_history",
            "maximum_used_transaction_ids_history",
            "oldest_replication_slot_lag_history",
            "replication_slot_disk_usage_history",
            "transaction_logs_disk_usage_history",
            "transaction_logs_generation_history",
            "failed_sql_server_agent_jobs_count_history",
            "checkpoint_lag_history",
            "connection_attempts_history",
        ];

        let advanced_histories = [
            &self.burst_balance_history,
            &self.cpu_credit_usage_history,
            &self.cpu_credit_balance_history,
            &self.bin_log_disk_usage_history,
            &self.replica_lag_history,
            &self.maximum_used_transaction_ids_history,
            &self.oldest_replication_slot_lag_history,
            &self.replication_slot_disk_usage_history,
            &self.transaction_logs_disk_usage_history,
            &self.transaction_logs_generation_history,
            &self.failed_sql_server_agent_jobs_count_history,
            &self.checkpoint_lag_history,
            &self.connection_attempts_history,
        ];

        for (_, history) in advanced_metric_names.iter().zip(advanced_histories.iter()) {
            if !history.is_empty() {
                count += 1;
            }
        }

        count
    }

    pub fn get_available_metrics(&self) -> Vec<MetricType> {
        let mut available_metrics = Vec::new();

        // Core metrics
        if !self.cpu_history.is_empty() {
            available_metrics.push(MetricType::CpuUtilization);
        }
        if !self.connections_history.is_empty() {
            available_metrics.push(MetricType::DatabaseConnections);
        }
        if !self.free_storage_space_history.is_empty() {
            available_metrics.push(MetricType::FreeStorageSpace);
        }
        if !self.read_iops_history.is_empty() {
            available_metrics.push(MetricType::ReadIops);
        }
        if !self.write_iops_history.is_empty() {
            available_metrics.push(MetricType::WriteIops);
        }
        if !self.read_latency_history.is_empty() {
            available_metrics.push(MetricType::ReadLatency);
        }
        if !self.write_latency_history.is_empty() {
            available_metrics.push(MetricType::WriteLatency);
        }
        if !self.read_throughput_history.is_empty() {
            available_metrics.push(MetricType::ReadThroughput);
        }
        if !self.write_throughput_history.is_empty() {
            available_metrics.push(MetricType::WriteThroughput);
        }
        if !self.network_receive_history.is_empty() {
            available_metrics.push(MetricType::NetworkReceiveThroughput);
        }
        if !self.network_transmit_history.is_empty() {
            available_metrics.push(MetricType::NetworkTransmitThroughput);
        }
        if !self.freeable_memory_history.is_empty() {
            available_metrics.push(MetricType::FreeableMemory);
        }
        if !self.swap_usage_history.is_empty() {
            available_metrics.push(MetricType::SwapUsage);
        }
        if !self.queue_depth_history.is_empty() {
            available_metrics.push(MetricType::QueueDepth);
        }

        // Advanced metrics
        if !self.burst_balance_history.is_empty() {
            available_metrics.push(MetricType::BurstBalance);
        }
        if !self.cpu_credit_usage_history.is_empty() {
            available_metrics.push(MetricType::CpuCreditUsage);
        }
        if !self.cpu_credit_balance_history.is_empty() {
            available_metrics.push(MetricType::CpuCreditBalance);
        }
        if !self.bin_log_disk_usage_history.is_empty() {
            available_metrics.push(MetricType::BinLogDiskUsage);
        }
        if !self.replica_lag_history.is_empty() {
            available_metrics.push(MetricType::ReplicaLag);
        }
        if !self.maximum_used_transaction_ids_history.is_empty() {
            available_metrics.push(MetricType::MaximumUsedTransactionIds);
        }
        if !self.oldest_replication_slot_lag_history.is_empty() {
            available_metrics.push(MetricType::OldestReplicationSlotLag);
        }
        if !self.replication_slot_disk_usage_history.is_empty() {
            available_metrics.push(MetricType::ReplicationSlotDiskUsage);
        }
        if !self.transaction_logs_disk_usage_history.is_empty() {
            available_metrics.push(MetricType::TransactionLogsDiskUsage);
        }
        if !self.transaction_logs_generation_history.is_empty() {
            available_metrics.push(MetricType::TransactionLogsGeneration);
        }
        if !self.failed_sql_server_agent_jobs_count_history.is_empty() {
            available_metrics.push(MetricType::FailedSqlServerAgentJobsCount);
        }
        if !self.checkpoint_lag_history.is_empty() {
            available_metrics.push(MetricType::CheckpointLag);
        }
        if !self.connection_attempts_history.is_empty() {
            available_metrics.push(MetricType::ConnectionAttempts);
        }

        available_metrics
    }

    pub fn get_metric_history(&self, metric_type: &MetricType) -> &Vec<f64> {
        match metric_type {
            MetricType::CpuUtilization => &self.cpu_history,
            MetricType::DatabaseConnections => &self.connections_history,
            MetricType::FreeStorageSpace => &self.free_storage_space_history,
            MetricType::ReadIops => &self.read_iops_history,
            MetricType::WriteIops => &self.write_iops_history,
            MetricType::ReadLatency => &self.read_latency_history,
            MetricType::WriteLatency => &self.write_latency_history,
            MetricType::ReadThroughput => &self.read_throughput_history,
            MetricType::WriteThroughput => &self.write_throughput_history,
            MetricType::NetworkReceiveThroughput => &self.network_receive_history,
            MetricType::NetworkTransmitThroughput => &self.network_transmit_history,
            MetricType::FreeableMemory => &self.freeable_memory_history,
            MetricType::SwapUsage => &self.swap_usage_history,
            MetricType::QueueDepth => &self.queue_depth_history,
            MetricType::BurstBalance => &self.burst_balance_history,
            MetricType::CpuCreditUsage => &self.cpu_credit_usage_history,
            MetricType::CpuCreditBalance => &self.cpu_credit_balance_history,
            MetricType::BinLogDiskUsage => &self.bin_log_disk_usage_history,
            MetricType::ReplicaLag => &self.replica_lag_history,
            MetricType::MaximumUsedTransactionIds => &self.maximum_used_transaction_ids_history,
            MetricType::OldestReplicationSlotLag => &self.oldest_replication_slot_lag_history,
            MetricType::ReplicationSlotDiskUsage => &self.replication_slot_disk_usage_history,
            MetricType::TransactionLogsDiskUsage => &self.transaction_logs_disk_usage_history,
            MetricType::TransactionLogsGeneration => &self.transaction_logs_generation_history,
            MetricType::FailedSqlServerAgentJobsCount => {
                &self.failed_sql_server_agent_jobs_count_history
            }
            MetricType::CheckpointLag => &self.checkpoint_lag_history,
            MetricType::ConnectionAttempts => &self.connection_attempts_history,
            // SQS metrics - return empty for RDS MetricData
            MetricType::NumberOfMessagesSent |
            MetricType::NumberOfMessagesReceived |
            MetricType::NumberOfMessagesDeleted |
            MetricType::ApproximateNumberOfMessages |
            MetricType::ApproximateNumberOfMessagesVisible |
            MetricType::ApproximateNumberOfMessagesNotVisible |
            MetricType::ApproximateAgeOfOldestMessage |
            MetricType::NumberOfEmptyReceives |
            MetricType::ApproximateNumberOfMessagesDelayed |
            MetricType::SentMessageSize |
            MetricType::NumberOfMessagesInDlq |
            MetricType::ApproximateNumberOfGroupsWithInflightMessages |
            MetricType::NumberOfDeduplicatedSentMessages => {
                static EMPTY_VEC: Vec<f64> = Vec::new();
                &EMPTY_VEC
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AwsService {
    Rds,
    Sqs,
}

impl AwsService {
    pub fn display_name(&self) -> &'static str {
        match self {
            AwsService::Rds => "RDS (Relational Database Service)",
            AwsService::Sqs => "SQS (Simple Queue Service)",
        }
    }

    #[allow(dead_code)]
    pub fn short_name(&self) -> &'static str {
        match self {
            AwsService::Rds => "RDS",
            AwsService::Sqs => "SQS",
        }
    }
}

// Generic instance container to hold different service instances
// Currently supports RDS and SQS, easily extensible for future services
#[derive(Debug, Clone)]
pub enum ServiceInstance {
    Rds(RdsInstance),
    Sqs(SqsQueue),
    // Future services will be added here when needed
    // Ec2(Ec2Instance),
}

impl ServiceInstance {
    pub fn as_aws_instance(&self) -> &dyn AwsInstance {
        match self {
            ServiceInstance::Rds(instance) => instance,
            ServiceInstance::Sqs(queue) => queue,
        }
    }
}

// Generic instance trait that different AWS services can implement
#[allow(dead_code)]
pub trait AwsInstance {
    fn id(&self) -> &str;
    fn name(&self) -> Option<&str>;
    fn status(&self) -> &str;
    fn service_type(&self) -> AwsService;
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    ServiceList,     // NEW: Show list of available AWS services
    InstanceList,    // RENAMED: Show instances for selected service (was RdsList)
    MetricsSummary,  // Show metrics summary for selected instance
    InstanceDetails, // Show detailed metrics for selected instance
}

#[derive(Debug, PartialEq, Clone)]
pub enum FocusedPanel {
    Timezone,
    Period,
    TimeRanges,
    SparklineGrid,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    // RDS Metrics
    CpuUtilization,
    DatabaseConnections,
    FreeStorageSpace,
    ReadIops,
    WriteIops,
    ReadLatency,
    WriteLatency,
    ReadThroughput,
    WriteThroughput,
    NetworkReceiveThroughput,
    NetworkTransmitThroughput,
    SwapUsage,
    FreeableMemory,
    QueueDepth,
    BurstBalance,
    CpuCreditUsage,
    CpuCreditBalance,
    BinLogDiskUsage,
    ReplicaLag,
    MaximumUsedTransactionIds,
    OldestReplicationSlotLag,
    ReplicationSlotDiskUsage,
    TransactionLogsDiskUsage,
    TransactionLogsGeneration,
    FailedSqlServerAgentJobsCount,
    CheckpointLag,
    ConnectionAttempts,

    // SQS Metrics (11 total)
    NumberOfMessagesSent,
    NumberOfMessagesReceived,
    NumberOfMessagesDeleted,
    ApproximateNumberOfMessages,               // Total queue depth (visible + not visible)
    ApproximateNumberOfMessagesVisible,        // Currently available messages
    ApproximateNumberOfMessagesNotVisible,
    ApproximateAgeOfOldestMessage,
    NumberOfEmptyReceives,
    ApproximateNumberOfMessagesDelayed,
    NumberOfMessagesInDlq,
    SentMessageSize,                          // Message size in bytes

    // FIFO-specific SQS Metrics
    ApproximateNumberOfGroupsWithInflightMessages,  // FIFO only
    NumberOfDeduplicatedSentMessages,               // FIFO only
}

impl MetricType {
    pub fn display_name(&self) -> &'static str {
        match self {
            // RDS Metrics
            MetricType::CpuUtilization => "CPU Utilization",
            MetricType::DatabaseConnections => "Database Connections",
            MetricType::FreeStorageSpace => "Free Storage Space",
            MetricType::ReadIops => "Read IOPS",
            MetricType::WriteIops => "Write IOPS",
            MetricType::ReadLatency => "Read Latency",
            MetricType::WriteLatency => "Write Latency",
            MetricType::ReadThroughput => "Read Throughput",
            MetricType::WriteThroughput => "Write Throughput",
            MetricType::NetworkReceiveThroughput => "Network Receive Throughput",
            MetricType::NetworkTransmitThroughput => "Network Transmit Throughput",
            MetricType::SwapUsage => "Swap Usage",
            MetricType::FreeableMemory => "Freeable Memory",
            MetricType::QueueDepth => "Queue Depth",
            MetricType::BurstBalance => "Burst Balance",
            MetricType::CpuCreditUsage => "CPU Credit Usage",
            MetricType::CpuCreditBalance => "CPU Credit Balance",
            MetricType::BinLogDiskUsage => "Binary Log Disk Usage",
            MetricType::ReplicaLag => "Replica Lag",
            MetricType::MaximumUsedTransactionIds => "Maximum Used Transaction IDs",
            MetricType::OldestReplicationSlotLag => "Oldest Replication Slot Lag",
            MetricType::ReplicationSlotDiskUsage => "Replication Slot Disk Usage",
            MetricType::TransactionLogsDiskUsage => "Transaction Logs Disk Usage",
            MetricType::TransactionLogsGeneration => "Transaction Logs Generation",
            MetricType::FailedSqlServerAgentJobsCount => "Failed SQL Server Agent Jobs",
            MetricType::CheckpointLag => "Checkpoint Lag",
            MetricType::ConnectionAttempts => "Connection Attempts",

            // SQS Metrics (11 total)
            MetricType::NumberOfMessagesSent => "Messages Sent",
            MetricType::NumberOfMessagesReceived => "Messages Received",
            MetricType::NumberOfMessagesDeleted => "Messages Deleted",
            MetricType::ApproximateNumberOfMessages => "Total Queue Depth",
            MetricType::ApproximateNumberOfMessagesVisible => "Messages Visible",
            MetricType::ApproximateNumberOfMessagesNotVisible => "Messages Not Visible",
            MetricType::ApproximateAgeOfOldestMessage => "Oldest Message Age",
            MetricType::NumberOfEmptyReceives => "Empty Receives",
            MetricType::ApproximateNumberOfMessagesDelayed => "Messages Delayed",
            MetricType::SentMessageSize => "Message Size",
            MetricType::NumberOfMessagesInDlq => "DLQ Messages",
            
            // FIFO-specific SQS Metrics
            MetricType::ApproximateNumberOfGroupsWithInflightMessages => "Groups with In-flight Messages",
            MetricType::NumberOfDeduplicatedSentMessages => "Deduplicated Messages",
        }
    }
}

pub struct App {
    // Service selection state (focused on RDS for now)
    pub available_services: Vec<AwsService>,
    pub service_list_state: ListState,
    pub selected_service: Option<AwsService>,

    // Instance list state (generic for all services, but RDS-focused)
    pub instances: Vec<ServiceInstance>,
    pub rds_instances: Vec<RdsInstance>, // Keep for backward compatibility during transition
    pub sqs_queues: Vec<SqsQueue>, // SQS queues for the selected service
    pub list_state: ListState,
    pub loading: bool,
    pub state: AppState,
    pub selected_instance: Option<usize>,
    pub metrics: MetricData,
    pub sqs_metrics: SqsMetricData,
    pub metrics_loading: bool,
    pub last_refresh: Option<Instant>,
    pub auto_refresh_enabled: bool,
    pub scroll_offset: usize,
    pub metrics_per_screen: usize,
    pub metrics_summary_scroll: usize, // Track metrics summary scroll position separately
    pub time_range_scroll: usize,      // Track time range selection scroll position
    pub focused_panel: FocusedPanel,   // Track which panel has focus (metrics or time ranges)
    pub saved_focused_panel: FocusedPanel, // Save focused panel state when transitioning to details
    pub time_range: TimeRange,

    // Sparkline grid state
    pub selected_metric: Option<MetricType>, // Currently selected metric in sparkline grid
    pub sparkline_grid_scroll: usize,        // Track scroll position in sparkline grid
    pub sparkline_grid_selected_index: usize, // Track currently selected metric index in grid
    pub saved_sparkline_grid_selected_index: usize, // Save selected metric index when transitioning to details

    // Error handling
    pub error_message: Option<String>, // Store user-friendly error messages

    // Loading timeout management
    pub loading_start_time: Option<Instant>, // Track when loading started
    
    // Time range display mode
    pub time_range_mode: TimeRangeMode, // Toggle between absolute and relative
    
    // Period selection
    pub period_scroll: usize, // Track period selection scroll position
    
    // Timezone selection
    pub timezone: Timezone, // Current timezone selection
    pub timezone_scroll: usize, // Track timezone selection scroll position
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeRangeMode {
    Absolute,
    Relative,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Timezone {
    Utc,
    Local,
}

impl Timezone {
    pub fn display_name(&self) -> &'static str {
        match self {
            Timezone::Utc => "UTC timezone",
            Timezone::Local => "Local timezone",
        }
    }
    
    pub fn get_timezone_options() -> Vec<Timezone> {
        vec![Timezone::Local, Timezone::Utc]
    }
}
