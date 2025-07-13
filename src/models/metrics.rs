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
}

/// Dynamic metric collection for any AWS service
/// This replaces the hardcoded MetricData and SqsMetricData structs
#[derive(Debug, Clone)]
pub struct DynamicMetrics {
    pub metrics: Vec<DynamicMetricData>,
    pub last_updated: SystemTime,
}

impl DynamicMetrics {
    pub fn new(_service_type: AwsService, _instance_id: String) -> Self {
        Self {
            metrics: Vec::new(),
            last_updated: SystemTime::now(),
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
        sorted_metrics.iter().map(|m| m.display_name.clone()).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.metrics.len()
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
        // Return empty for now - UI will use dynamic metrics
        Vec::new()
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
    pub fn get_available_metrics(&self) -> Vec<MetricType> {
        // If we have timestamps (meaning we successfully loaded data), return available metrics
        if !self.timestamps.is_empty() {
            let mut available = Vec::new();

            // Check which metrics have actual data
            if !self.messages_sent_history.is_empty() {
                available.push(MetricType::NumberOfMessagesSent);
            }
            if !self.messages_received_history.is_empty() {
                available.push(MetricType::NumberOfMessagesReceived);
            }
            if !self.messages_deleted_history.is_empty() {
                available.push(MetricType::NumberOfMessagesDeleted);
            }
            if !self.queue_depth_history.is_empty() {
                available.push(MetricType::ApproximateNumberOfMessages);
            }
            if !self.messages_visible_history.is_empty() {
                available.push(MetricType::ApproximateNumberOfMessagesVisible);
            }
            if !self.messages_not_visible_history.is_empty() {
                available.push(MetricType::ApproximateNumberOfMessagesNotVisible);
            }
            if !self.oldest_message_age_history.is_empty() {
                available.push(MetricType::ApproximateAgeOfOldestMessage);
            }
            if !self.empty_receives_history.is_empty() {
                available.push(MetricType::NumberOfEmptyReceives);
            }

            available
        } else {
            // Return empty for now - UI will use dynamic metrics
            Vec::new()
        }
    }
}
