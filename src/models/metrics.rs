use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::models::aws_services::AwsService;
use std::time::SystemTime;

/// Metric type enumeration for different AWS services
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
    CpuSurplusCreditBalance,
    CpuSurplusCreditsCharged,
    EbsByteBalance,
    EbsIoBalance,
    BinLogDiskUsage,
    ReplicaLag,
    MaximumUsedTransactionIds,
    OldestReplicationSlotLag,
    OldestLogicalReplicationSlotLag,
    ReplicationSlotDiskUsage,
    TransactionLogsDiskUsage,
    TransactionLogsGeneration,
    FailedSqlServerAgentJobsCount,
    CheckpointLag,
    ConnectionAttempts,

    // SQS Metrics
    NumberOfMessagesSent,
    NumberOfMessagesReceived,
    NumberOfMessagesDeleted,
    ApproximateNumberOfMessages,
    ApproximateNumberOfMessagesVisible,
    ApproximateNumberOfMessagesNotVisible,
    ApproximateAgeOfOldestMessage,
    NumberOfEmptyReceives,
    ApproximateNumberOfMessagesDelayed,
    SentMessageSize,
    NumberOfMessagesInDlq,
    ApproximateNumberOfGroupsWithInflightMessages,
    NumberOfDeduplicatedSentMessages,

    // Generic metric types for testing
    Percentage,
    Count,
    Bytes,
    Seconds,
}

impl MetricType {
    pub fn display_name(&self) -> &'static str {
        match self {
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
            MetricType::CpuSurplusCreditBalance => "CPU Surplus Credit Balance",
            MetricType::CpuSurplusCreditsCharged => "CPU Surplus Credits Charged",
            MetricType::EbsByteBalance => "EBS Byte Balance",
            MetricType::EbsIoBalance => "EBS IO Balance",
            MetricType::BinLogDiskUsage => "BinLog Disk Usage",
            MetricType::ReplicaLag => "Replica Lag",
            MetricType::MaximumUsedTransactionIds => "Maximum Used Transaction IDs",
            MetricType::OldestReplicationSlotLag => "Oldest Replication Slot Lag",
            MetricType::OldestLogicalReplicationSlotLag => "Oldest Logical Replication Slot Lag",
            MetricType::ReplicationSlotDiskUsage => "Replication Slot Disk Usage",
            MetricType::TransactionLogsDiskUsage => "Transaction Logs Disk Usage",
            MetricType::TransactionLogsGeneration => "Transaction Logs Generation",
            MetricType::FailedSqlServerAgentJobsCount => "Failed SQL Server Agent Jobs",
            MetricType::CheckpointLag => "Checkpoint Lag",
            MetricType::ConnectionAttempts => "Connection Attempts",
            MetricType::NumberOfMessagesSent => "Number Of Messages Sent",
            MetricType::NumberOfMessagesReceived => "Number Of Messages Received",
            MetricType::NumberOfMessagesDeleted => "Number Of Messages Deleted",
            MetricType::ApproximateNumberOfMessages => "Approximate Number Of Messages",
            MetricType::ApproximateNumberOfMessagesVisible => "Approximate Number Visible",
            MetricType::ApproximateNumberOfMessagesNotVisible => "Approximate Number Not Visible",
            MetricType::ApproximateAgeOfOldestMessage => "Approximate Age Of Oldest Message",
            MetricType::NumberOfEmptyReceives => "Number Of Empty Receives",
            MetricType::ApproximateNumberOfMessagesDelayed => "Approximate Number Delayed",
            MetricType::SentMessageSize => "Sent Message Size",
            MetricType::NumberOfMessagesInDlq => "Number Of Messages In DLQ",
            MetricType::ApproximateNumberOfGroupsWithInflightMessages => {
                "Approximate Inflight Groups"
            }
            MetricType::NumberOfDeduplicatedSentMessages => "Number Of Deduplicated Sent Messages",
            MetricType::Percentage => "Percentage",
            MetricType::Count => "Count",
            MetricType::Bytes => "Bytes",
            MetricType::Seconds => "Seconds",
        }
    }
}

/// Dynamic metric collection for any AWS service
/// This replaces the hardcoded MetricData and SqsMetricData structs
#[derive(Debug, Clone)]
pub struct DynamicMetrics {
    pub metrics: Vec<DynamicMetricData>,
    pub last_updated: SystemTime,
    service_type: AwsService,
    instance_id: String,
}

impl DynamicMetrics {
    pub fn new(service_type: AwsService, instance_id: String) -> Self {
        Self {
            metrics: Vec::new(),
            last_updated: SystemTime::now(),
            service_type,
            instance_id,
        }
    }

    pub fn update_metrics(&mut self, metrics: Vec<DynamicMetricData>) {
        self.metrics = metrics;
        self.last_updated = SystemTime::now();
    }

    pub fn get_available_metric_names(&self) -> Vec<String> {
        // Sort by display name to match the UI display order
        let mut sorted_metrics = self.metrics.clone();
        sorted_metrics.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        sorted_metrics
            .iter()
            .map(|m| m.display_name.clone())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    /// Get the service type for this metrics collection
    pub fn service_type(&self) -> &AwsService {
        &self.service_type
    }

    /// Get the instance ID for this metrics collection
    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    /// Get all metric names
    pub fn metric_names(&self) -> Vec<&str> {
        self.metrics
            .iter()
            .map(|m| m.display_name.as_str())
            .collect()
    }

    /// Get a specific metric by name
    pub fn get_metric(&self, name: &str) -> Option<&DynamicMetricData> {
        self.metrics.iter().find(|m| m.display_name == name)
    }

    /// Add a metric for testing purposes
    pub fn add_metric(
        &mut self,
        name: String,
        _metric_type: MetricType,
        data_points: Vec<(f64, f64)>,
    ) {
        let history: Vec<f64> = data_points.iter().map(|(_, value)| *value).collect();
        let timestamps: Vec<SystemTime> = data_points
            .iter()
            .map(|(timestamp, _)| {
                SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(*timestamp as u64)
            })
            .collect();

        let current_value = history.last().copied().unwrap_or(0.0);

        let metric_data = DynamicMetricData {
            display_name: name.clone(),
            metric_name: name,
            current_value,
            history,
            timestamps,
            unit: Some("None".to_string()),
        };
        self.metrics.push(metric_data);
    }
}

/// Legacy RDS metric data structure
/// TEMPORARY: For backward compatibility during UI transition
#[derive(Debug)]
pub struct MetricData {
    pub timestamps: Vec<SystemTime>,
    // Core metrics
    pub cpu_utilization: f64,
    pub database_connections: f64,
    pub free_storage_space: f64,
    pub read_iops: f64,
    pub write_iops: f64,
    pub read_latency: f64,
    pub write_latency: f64,
    pub read_throughput: f64,
    pub write_throughput: f64,
    pub network_receive_throughput: f64,
    pub network_transmit_throughput: f64,
    pub swap_usage: f64,
    pub freeable_memory: f64,
    pub queue_depth: f64,
    pub burst_balance: f64,
    pub cpu_credit_usage: f64,
    pub cpu_credit_balance: f64,
    pub cpu_surplus_credit_balance: f64,
    pub cpu_surplus_credits_charged: f64,
    pub ebs_byte_balance: f64,
    pub ebs_io_balance: f64,
    pub bin_log_disk_usage: f64,
    pub replica_lag: f64,
    pub maximum_used_transaction_ids: f64,
    pub oldest_replication_slot_lag: f64,
    pub oldest_logical_replication_slot_lag: f64,
    pub replication_slot_disk_usage: f64,
    pub transaction_logs_disk_usage: f64,
    pub transaction_logs_generation: f64,
    pub failed_sql_server_agent_jobs_count: f64,
    pub checkpoint_lag: f64,
    pub connection_attempts: f64,

    // History vectors
    pub cpu_history: Vec<f64>,
    pub connections_history: Vec<f64>,
    pub free_storage_space_history: Vec<f64>,
    pub read_iops_history: Vec<f64>,
    pub write_iops_history: Vec<f64>,
    pub read_latency_history: Vec<f64>,
    pub write_latency_history: Vec<f64>,
    pub read_throughput_history: Vec<f64>,
    pub write_throughput_history: Vec<f64>,
    pub network_receive_history: Vec<f64>,
    pub network_transmit_history: Vec<f64>,
    pub swap_usage_history: Vec<f64>,
    pub freeable_memory_history: Vec<f64>,
    pub queue_depth_history: Vec<f64>,
    pub burst_balance_history: Vec<f64>,
    pub cpu_credit_usage_history: Vec<f64>,
    pub cpu_credit_balance_history: Vec<f64>,
    pub cpu_surplus_credit_balance_history: Vec<f64>,
    pub cpu_surplus_credits_charged_history: Vec<f64>,
    pub ebs_byte_balance_history: Vec<f64>,
    pub ebs_io_balance_history: Vec<f64>,
    pub bin_log_disk_usage_history: Vec<f64>,
    pub replica_lag_history: Vec<f64>,
    pub maximum_used_transaction_ids_history: Vec<f64>,
    pub oldest_replication_slot_lag_history: Vec<f64>,
    pub oldest_logical_replication_slot_lag_history: Vec<f64>,
    pub replication_slot_disk_usage_history: Vec<f64>,
    pub transaction_logs_disk_usage_history: Vec<f64>,
    pub transaction_logs_generation_history: Vec<f64>,
    pub failed_sql_server_agent_jobs_count_history: Vec<f64>,
    pub checkpoint_lag_history: Vec<f64>,
    pub connection_attempts_history: Vec<f64>,
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
            burst_balance: 0.0,
            cpu_credit_usage: 0.0,
            cpu_credit_balance: 0.0,
            cpu_surplus_credit_balance: 0.0,
            cpu_surplus_credits_charged: 0.0,
            ebs_byte_balance: 0.0,
            ebs_io_balance: 0.0,
            bin_log_disk_usage: 0.0,
            replica_lag: 0.0,
            maximum_used_transaction_ids: 0.0,
            oldest_replication_slot_lag: 0.0,
            oldest_logical_replication_slot_lag: 0.0,
            replication_slot_disk_usage: 0.0,
            transaction_logs_disk_usage: 0.0,
            transaction_logs_generation: 0.0,
            failed_sql_server_agent_jobs_count: 0.0,
            checkpoint_lag: 0.0,
            connection_attempts: 0.0,
            cpu_history: Vec::new(),
            connections_history: Vec::new(),
            free_storage_space_history: Vec::new(),
            read_iops_history: Vec::new(),
            write_iops_history: Vec::new(),
            read_latency_history: Vec::new(),
            write_latency_history: Vec::new(),
            read_throughput_history: Vec::new(),
            write_throughput_history: Vec::new(),
            network_receive_history: Vec::new(),
            network_transmit_history: Vec::new(),
            swap_usage_history: Vec::new(),
            freeable_memory_history: Vec::new(),
            queue_depth_history: Vec::new(),
            burst_balance_history: Vec::new(),
            cpu_credit_usage_history: Vec::new(),
            cpu_credit_balance_history: Vec::new(),
            cpu_surplus_credit_balance_history: Vec::new(),
            cpu_surplus_credits_charged_history: Vec::new(),
            ebs_byte_balance_history: Vec::new(),
            ebs_io_balance_history: Vec::new(),
            bin_log_disk_usage_history: Vec::new(),
            replica_lag_history: Vec::new(),
            maximum_used_transaction_ids_history: Vec::new(),
            oldest_replication_slot_lag_history: Vec::new(),
            oldest_logical_replication_slot_lag_history: Vec::new(),
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
    pub fn get_available_metrics_with_data(&self) -> Vec<MetricType> {
        let mut available = Vec::new();

        let mut push_if_has_data = |history: &Vec<f64>, metric: MetricType| {
            if !history.is_empty() {
                available.push(metric);
            }
        };

        push_if_has_data(&self.cpu_history, MetricType::CpuUtilization);
        push_if_has_data(&self.connections_history, MetricType::DatabaseConnections);
        push_if_has_data(
            &self.free_storage_space_history,
            MetricType::FreeStorageSpace,
        );
        push_if_has_data(&self.read_iops_history, MetricType::ReadIops);
        push_if_has_data(&self.write_iops_history, MetricType::WriteIops);
        push_if_has_data(&self.read_latency_history, MetricType::ReadLatency);
        push_if_has_data(&self.write_latency_history, MetricType::WriteLatency);
        push_if_has_data(&self.read_throughput_history, MetricType::ReadThroughput);
        push_if_has_data(&self.write_throughput_history, MetricType::WriteThroughput);
        push_if_has_data(
            &self.network_receive_history,
            MetricType::NetworkReceiveThroughput,
        );
        push_if_has_data(
            &self.network_transmit_history,
            MetricType::NetworkTransmitThroughput,
        );
        push_if_has_data(&self.swap_usage_history, MetricType::SwapUsage);
        push_if_has_data(&self.freeable_memory_history, MetricType::FreeableMemory);
        push_if_has_data(&self.queue_depth_history, MetricType::QueueDepth);
        push_if_has_data(&self.burst_balance_history, MetricType::BurstBalance);
        push_if_has_data(&self.cpu_credit_usage_history, MetricType::CpuCreditUsage);
        push_if_has_data(
            &self.cpu_credit_balance_history,
            MetricType::CpuCreditBalance,
        );
        push_if_has_data(
            &self.cpu_surplus_credit_balance_history,
            MetricType::CpuSurplusCreditBalance,
        );
        push_if_has_data(
            &self.cpu_surplus_credits_charged_history,
            MetricType::CpuSurplusCreditsCharged,
        );
        push_if_has_data(&self.ebs_byte_balance_history, MetricType::EbsByteBalance);
        push_if_has_data(&self.ebs_io_balance_history, MetricType::EbsIoBalance);
        push_if_has_data(
            &self.bin_log_disk_usage_history,
            MetricType::BinLogDiskUsage,
        );
        push_if_has_data(&self.replica_lag_history, MetricType::ReplicaLag);
        push_if_has_data(
            &self.maximum_used_transaction_ids_history,
            MetricType::MaximumUsedTransactionIds,
        );
        push_if_has_data(
            &self.oldest_replication_slot_lag_history,
            MetricType::OldestReplicationSlotLag,
        );
        push_if_has_data(
            &self.oldest_logical_replication_slot_lag_history,
            MetricType::OldestLogicalReplicationSlotLag,
        );
        push_if_has_data(
            &self.replication_slot_disk_usage_history,
            MetricType::ReplicationSlotDiskUsage,
        );
        push_if_has_data(
            &self.transaction_logs_disk_usage_history,
            MetricType::TransactionLogsDiskUsage,
        );
        push_if_has_data(
            &self.transaction_logs_generation_history,
            MetricType::TransactionLogsGeneration,
        );
        push_if_has_data(
            &self.failed_sql_server_agent_jobs_count_history,
            MetricType::FailedSqlServerAgentJobsCount,
        );
        push_if_has_data(&self.checkpoint_lag_history, MetricType::CheckpointLag);
        push_if_has_data(
            &self.connection_attempts_history,
            MetricType::ConnectionAttempts,
        );

        available
    }

    pub fn get_metric_value(&self, metric_type: &MetricType) -> f64 {
        match metric_type {
            MetricType::CpuUtilization => self.cpu_utilization,
            MetricType::DatabaseConnections => self.database_connections,
            MetricType::FreeStorageSpace => self.free_storage_space,
            MetricType::ReadIops => self.read_iops,
            MetricType::WriteIops => self.write_iops,
            MetricType::ReadLatency => self.read_latency,
            MetricType::WriteLatency => self.write_latency,
            MetricType::ReadThroughput => self.read_throughput,
            MetricType::WriteThroughput => self.write_throughput,
            MetricType::NetworkReceiveThroughput => self.network_receive_throughput,
            MetricType::NetworkTransmitThroughput => self.network_transmit_throughput,
            MetricType::SwapUsage => self.swap_usage,
            MetricType::FreeableMemory => self.freeable_memory,
            MetricType::QueueDepth => self.queue_depth,
            MetricType::BurstBalance => self.burst_balance,
            MetricType::CpuCreditUsage => self.cpu_credit_usage,
            MetricType::CpuCreditBalance => self.cpu_credit_balance,
            MetricType::CpuSurplusCreditBalance => self.cpu_surplus_credit_balance,
            MetricType::CpuSurplusCreditsCharged => self.cpu_surplus_credits_charged,
            MetricType::EbsByteBalance => self.ebs_byte_balance,
            MetricType::EbsIoBalance => self.ebs_io_balance,
            MetricType::BinLogDiskUsage => self.bin_log_disk_usage,
            MetricType::ReplicaLag => self.replica_lag,
            MetricType::MaximumUsedTransactionIds => self.maximum_used_transaction_ids,
            MetricType::OldestReplicationSlotLag => self.oldest_replication_slot_lag,
            MetricType::OldestLogicalReplicationSlotLag => self.oldest_logical_replication_slot_lag,
            MetricType::ReplicationSlotDiskUsage => self.replication_slot_disk_usage,
            MetricType::TransactionLogsDiskUsage => self.transaction_logs_disk_usage,
            MetricType::TransactionLogsGeneration => self.transaction_logs_generation,
            MetricType::FailedSqlServerAgentJobsCount => self.failed_sql_server_agent_jobs_count,
            MetricType::CheckpointLag => self.checkpoint_lag,
            MetricType::ConnectionAttempts => self.connection_attempts,
            _ => 0.0,
        }
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
            MetricType::SwapUsage => &self.swap_usage_history,
            MetricType::FreeableMemory => &self.freeable_memory_history,
            MetricType::QueueDepth => &self.queue_depth_history,
            MetricType::BurstBalance => &self.burst_balance_history,
            MetricType::CpuCreditUsage => &self.cpu_credit_usage_history,
            MetricType::CpuCreditBalance => &self.cpu_credit_balance_history,
            MetricType::CpuSurplusCreditBalance => &self.cpu_surplus_credit_balance_history,
            MetricType::CpuSurplusCreditsCharged => &self.cpu_surplus_credits_charged_history,
            MetricType::EbsByteBalance => &self.ebs_byte_balance_history,
            MetricType::EbsIoBalance => &self.ebs_io_balance_history,
            MetricType::BinLogDiskUsage => &self.bin_log_disk_usage_history,
            MetricType::ReplicaLag => &self.replica_lag_history,
            MetricType::MaximumUsedTransactionIds => &self.maximum_used_transaction_ids_history,
            MetricType::OldestReplicationSlotLag => &self.oldest_replication_slot_lag_history,
            MetricType::OldestLogicalReplicationSlotLag => {
                &self.oldest_logical_replication_slot_lag_history
            }
            MetricType::ReplicationSlotDiskUsage => &self.replication_slot_disk_usage_history,
            MetricType::TransactionLogsDiskUsage => &self.transaction_logs_disk_usage_history,
            MetricType::TransactionLogsGeneration => &self.transaction_logs_generation_history,
            MetricType::FailedSqlServerAgentJobsCount => {
                &self.failed_sql_server_agent_jobs_count_history
            }
            MetricType::CheckpointLag => &self.checkpoint_lag_history,
            MetricType::ConnectionAttempts => &self.connection_attempts_history,
            _ => &self.cpu_history,
        }
    }
}

/// Legacy SQS metric data structure
/// TEMPORARY: For backward compatibility during UI transition
#[derive(Debug)]
pub struct SqsMetricData {
    pub number_of_messages_sent: f64,
    pub number_of_messages_received: f64,
    pub number_of_messages_deleted: f64,
    pub approximate_number_of_messages: f64,
    pub approximate_number_of_messages_visible: f64,
    pub approximate_number_of_messages_not_visible: f64,
    pub approximate_age_of_oldest_message: f64,
    pub number_of_empty_receives: f64,
    pub approximate_number_of_messages_delayed: f64,
    pub sent_message_size: f64,
    pub number_of_messages_in_dlq: f64,
    pub approximate_number_of_groups_with_inflight_messages: f64,
    pub number_of_deduplicated_sent_messages: f64,
    pub timestamps: Vec<SystemTime>,
    pub messages_sent_history: Vec<f64>,
    pub messages_received_history: Vec<f64>,
    pub messages_deleted_history: Vec<f64>,
    pub queue_depth_history: Vec<f64>,
    pub messages_visible_history: Vec<f64>,
    pub messages_not_visible_history: Vec<f64>,
    pub oldest_message_age_history: Vec<f64>,
    pub empty_receives_history: Vec<f64>,
    pub messages_delayed_history: Vec<f64>,
    pub sent_message_size_history: Vec<f64>,
    pub dlq_messages_history: Vec<f64>,
    pub groups_with_inflight_messages_history: Vec<f64>,
    pub deduplicated_sent_messages_history: Vec<f64>,
}

impl Default for SqsMetricData {
    fn default() -> Self {
        Self {
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
            approximate_number_of_groups_with_inflight_messages: 0.0,
            number_of_deduplicated_sent_messages: 0.0,
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
    /// Return all SQS metrics that have historical data available.
    pub fn get_available_metrics(&self) -> Vec<MetricType> {
        let mut available = Vec::new();

        let mut push_if_has_data = |history: &Vec<f64>, metric: MetricType| {
            if !history.is_empty() {
                available.push(metric);
            }
        };

        push_if_has_data(
            &self.messages_sent_history,
            MetricType::NumberOfMessagesSent,
        );
        push_if_has_data(
            &self.messages_received_history,
            MetricType::NumberOfMessagesReceived,
        );
        push_if_has_data(
            &self.messages_deleted_history,
            MetricType::NumberOfMessagesDeleted,
        );
        push_if_has_data(
            &self.queue_depth_history,
            MetricType::ApproximateNumberOfMessages,
        );
        push_if_has_data(
            &self.messages_visible_history,
            MetricType::ApproximateNumberOfMessagesVisible,
        );
        push_if_has_data(
            &self.messages_not_visible_history,
            MetricType::ApproximateNumberOfMessagesNotVisible,
        );
        push_if_has_data(
            &self.oldest_message_age_history,
            MetricType::ApproximateAgeOfOldestMessage,
        );
        push_if_has_data(
            &self.empty_receives_history,
            MetricType::NumberOfEmptyReceives,
        );
        push_if_has_data(
            &self.messages_delayed_history,
            MetricType::ApproximateNumberOfMessagesDelayed,
        );
        push_if_has_data(&self.sent_message_size_history, MetricType::SentMessageSize);
        push_if_has_data(
            &self.dlq_messages_history,
            MetricType::NumberOfMessagesInDlq,
        );
        push_if_has_data(
            &self.groups_with_inflight_messages_history,
            MetricType::ApproximateNumberOfGroupsWithInflightMessages,
        );
        push_if_has_data(
            &self.deduplicated_sent_messages_history,
            MetricType::NumberOfDeduplicatedSentMessages,
        );

        available
    }
}
