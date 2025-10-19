use super::display_format::DisplayFormat;
use super::health_thresholds::HealthThresholds;
use super::metric_definition::MetricDefinition;
use crate::models::MetricType;
use crate::ui::themes::UnifiedTheme;
use ratatui::style::Color;

/// AWS console-style metric definitions registry
pub struct MetricRegistry;

impl MetricRegistry {
    /// Get metric definition for a given metric type
    pub fn get_definition(metric_type: &MetricType) -> MetricDefinition {
        Self::get_definition_simplified(metric_type)
    }

    /// Get theme-based color for a metric type
    fn get_metric_color(metric_type: &MetricType) -> Color {
        let theme = UnifiedTheme::default();
        match metric_type {
            // CPU and performance metrics - use accent (cyan)
            MetricType::CpuUtilization
            | MetricType::CpuCreditBalance
            | MetricType::CpuCreditUsage => theme.accent,

            // Database connections and network - use info (blue)
            MetricType::DatabaseConnections
            | MetricType::ReadLatency
            | MetricType::WriteLatency => theme.info,

            // Storage and memory - use chart_secondary (light blue)
            MetricType::FreeStorageSpace | MetricType::FreeableMemory => theme.chart_secondary,

            // IOPS and throughput - use chart_accent (steel blue)
            MetricType::ReadIops
            | MetricType::WriteIops
            | MetricType::ReadThroughput
            | MetricType::WriteThroughput => theme.chart_accent,

            // Queue metrics - use accent (cyan)
            MetricType::ApproximateNumberOfMessages
            | MetricType::ApproximateNumberOfMessagesVisible
            | MetricType::ApproximateNumberOfMessagesNotVisible
            | MetricType::ApproximateAgeOfOldestMessage => theme.accent,

            // Message operations - use info (blue)
            MetricType::NumberOfMessagesSent
            | MetricType::NumberOfMessagesReceived
            | MetricType::NumberOfMessagesDeleted
            | MetricType::NumberOfEmptyReceives => theme.info,

            // Size metrics - use chart_secondary (light blue)
            MetricType::SentMessageSize | MetricType::ApproximateNumberOfMessagesNotVisible => {
                theme.chart_secondary
            }

            // Default fallback
            _ => theme.chart_primary,
        }
    }

    /// Get metric definition (simplified without dynamic info)
    pub fn get_definition_simplified(metric_type: &MetricType) -> MetricDefinition {
        match metric_type {
            // === RDS Core Metrics ===
            MetricType::CpuUtilization => MetricDefinition::new(
                "CPU Utilization",
                DisplayFormat::Percentage,
                Some(HealthThresholds::new(80.0, 60.0, false)),
                Self::get_metric_color(metric_type),
                "Percentage of CPU utilization",
            ),

            MetricType::DatabaseConnections => MetricDefinition::new(
                "Database Connections",
                DisplayFormat::Integer,
                Some(HealthThresholds::new(1000.0, 500.0, false)),
                Self::get_metric_color(metric_type),
                "Number of client network connections to the database",
            ),

            MetricType::FreeStorageSpace => MetricDefinition::new(
                "Free Storage Space",
                DisplayFormat::Bytes,
                Some(HealthThresholds::new(
                    1_073_741_824.0,
                    5_368_709_120.0,
                    true,
                )),
                Self::get_metric_color(metric_type),
                "Amount of available storage space",
            ),

            MetricType::ReadLatency => MetricDefinition::new(
                "Read Latency",
                DisplayFormat::Duration,
                Some(HealthThresholds::new(0.1, 0.05, false)),
                Self::get_metric_color(metric_type),
                "Average time taken for read operations",
            ),

            MetricType::WriteLatency => MetricDefinition::new(
                "Write Latency",
                DisplayFormat::Duration,
                Some(HealthThresholds::new(0.1, 0.05, false)),
                Self::get_metric_color(metric_type),
                "Average time taken for write operations",
            ),

            MetricType::ReadIops => MetricDefinition::new(
                "Read IOPS",
                DisplayFormat::Integer,
                None,
                Color::Green,
                "Read input/output operations per second",
            ),

            MetricType::WriteIops => MetricDefinition::new(
                "Write IOPS",
                DisplayFormat::Integer,
                None,
                Self::get_metric_color(metric_type),
                "Write input/output operations per second",
            ),

            MetricType::ReadThroughput => MetricDefinition::new(
                "Read Throughput",
                DisplayFormat::Bytes,
                None,
                Self::get_metric_color(metric_type),
                "Read throughput in bytes per second",
            ),

            MetricType::WriteThroughput => MetricDefinition::new(
                "Write Throughput",
                DisplayFormat::Bytes,
                None,
                Self::get_metric_color(metric_type),
                "Write throughput in bytes per second",
            ),

            MetricType::NetworkReceiveThroughput => MetricDefinition::new(
                "Network Receive Throughput",
                DisplayFormat::Bytes,
                None,
                Color::LightGreen,
                "Network receive throughput in bytes per second",
            ),

            MetricType::NetworkTransmitThroughput => MetricDefinition::new(
                "Network Transmit Throughput",
                DisplayFormat::Bytes,
                None,
                Color::LightBlue,
                "Network transmit throughput in bytes per second",
            ),

            MetricType::SwapUsage => MetricDefinition::new(
                "Swap Usage",
                DisplayFormat::Bytes,
                Some(HealthThresholds::new(1_073_741_824.0, 536_870_912.0, false)),
                Color::Red,
                "Amount of swap space used",
            ),

            MetricType::FreeableMemory => MetricDefinition::new(
                "Freeable Memory",
                DisplayFormat::Bytes,
                Some(HealthThresholds::new(268_435_456.0, 536_870_912.0, true)),
                Color::Blue,
                "Amount of available memory",
            ),

            MetricType::QueueDepth => MetricDefinition::new(
                "Queue Depth",
                DisplayFormat::Integer,
                Some(HealthThresholds::new(64.0, 32.0, false)),
                Color::Yellow,
                "Number of outstanding I/O requests",
            ),

            MetricType::BurstBalance => MetricDefinition::new(
                "Burst Balance",
                DisplayFormat::Percentage,
                Some(HealthThresholds::new(10.0, 20.0, true)),
                Color::Magenta,
                "Percentage of General Purpose SSD burst-bucket I/O credits remaining",
            ),

            MetricType::CpuCreditUsage => MetricDefinition::new(
                "CPU Credit Usage",
                DisplayFormat::Integer,
                None,
                Color::Blue,
                "CPU credits consumed during the period",
            ),

            MetricType::CpuCreditBalance => MetricDefinition::new(
                "CPU Credit Balance",
                DisplayFormat::Integer,
                Some(HealthThresholds::new(10.0, 50.0, true)),
                Color::Green,
                "Number of CPU credits available for bursting",
            ),

            // === Advanced RDS Metrics ===
            MetricType::BinLogDiskUsage => MetricDefinition::new(
                "Binary Log Disk Usage",
                DisplayFormat::Bytes,
                None,
                Color::Yellow,
                "Amount of disk space occupied by binary logs",
            ),

            MetricType::ReplicaLag => MetricDefinition::new(
                "Replica Lag",
                DisplayFormat::Duration,
                Some(HealthThresholds::new(300.0, 60.0, false)),
                Color::Red,
                "Time when the replica DB instance lags behind the source DB instance",
            ),

            MetricType::TransactionLogsGeneration => MetricDefinition::new(
                "Transaction Logs Generation",
                DisplayFormat::Bytes,
                None,
                Color::Cyan,
                "Size of transaction logs generated per second",
            ),

            MetricType::TransactionLogsDiskUsage => MetricDefinition::new(
                "Transaction Logs Disk Usage",
                DisplayFormat::Bytes,
                None,
                Color::Magenta,
                "Disk space used by transaction logs",
            ),

            MetricType::MaximumUsedTransactionIds => MetricDefinition::new(
                "Maximum Used Transaction IDs",
                DisplayFormat::Integer,
                None,
                Color::Yellow,
                "Maximum transaction IDs that have been used",
            ),

            MetricType::OldestReplicationSlotLag => MetricDefinition::new(
                "Oldest Replication Slot Lag",
                DisplayFormat::Duration,
                Some(HealthThresholds::new(3600.0, 1800.0, false)),
                Color::Red,
                "Age of the oldest replication slot lag",
            ),

            MetricType::ReplicationSlotDiskUsage => MetricDefinition::new(
                "Replication Slot Disk Usage",
                DisplayFormat::Bytes,
                None,
                Color::Blue,
                "Disk space used by replication slot files",
            ),

            MetricType::FailedSqlServerAgentJobsCount => MetricDefinition::new(
                "Failed SQL Server Agent Jobs",
                DisplayFormat::Integer,
                Some(HealthThresholds::new(1.0, 0.0, false)),
                Color::Red,
                "Number of failed Microsoft SQL Server Agent jobs",
            ),

            MetricType::CheckpointLag => MetricDefinition::new(
                "Checkpoint Lag",
                DisplayFormat::Duration,
                Some(HealthThresholds::new(600.0, 300.0, false)),
                Color::Yellow,
                "Time since the last checkpoint",
            ),

            MetricType::ConnectionAttempts => MetricDefinition::new(
                "Connection Attempts",
                DisplayFormat::Integer,
                None,
                Color::Green,
                "Number of attempts to connect to the instance",
            ),

            // === SQS Metrics ===
            MetricType::ApproximateNumberOfMessages => MetricDefinition::new(
                "Approximate Number of Messages",
                DisplayFormat::Integer,
                None,
                Color::Blue,
                "Approximate number of messages available for retrieval from the queue",
            ),

            MetricType::ApproximateNumberOfMessagesVisible => MetricDefinition::new(
                "Approximate Number of Messages Visible",
                DisplayFormat::Integer,
                None,
                Color::Green,
                "Approximate number of messages visible in the queue",
            ),

            MetricType::ApproximateNumberOfMessagesNotVisible => MetricDefinition::new(
                "Approximate Number of Messages Not Visible",
                DisplayFormat::Integer,
                None,
                Color::Yellow,
                "Approximate number of messages that are in flight",
            ),

            MetricType::ApproximateAgeOfOldestMessage => MetricDefinition::new(
                "Approximate Age of Oldest Message",
                DisplayFormat::Duration,
                Some(HealthThresholds::new(3600.0, 1800.0, false)),
                Color::Red,
                "Approximate age of the oldest non-deleted message in the queue",
            ),

            MetricType::ApproximateNumberOfMessagesDelayed => MetricDefinition::new(
                "Approximate Number of Messages Delayed",
                DisplayFormat::Integer,
                None,
                Color::Magenta,
                "Approximate number of messages in the queue that are delayed",
            ),

            MetricType::NumberOfMessagesSent => MetricDefinition::new(
                "Number of Messages Sent",
                DisplayFormat::Integer,
                None,
                Color::Green,
                "Number of messages added to a queue",
            ),

            MetricType::NumberOfMessagesReceived => MetricDefinition::new(
                "Number of Messages Received",
                DisplayFormat::Integer,
                None,
                Color::Blue,
                "Number of messages returned by calls to the ReceiveMessage action",
            ),

            MetricType::NumberOfMessagesDeleted => MetricDefinition::new(
                "Number of Messages Deleted",
                DisplayFormat::Integer,
                None,
                Color::Cyan,
                "Number of messages deleted from the queue",
            ),

            MetricType::NumberOfEmptyReceives => MetricDefinition::new(
                "Number of Empty Receives",
                DisplayFormat::Integer,
                None,
                Color::Yellow,
                "Number of ReceiveMessage API calls that did not return a message",
            ),

            MetricType::SentMessageSize => MetricDefinition::new(
                "Sent Message Size",
                DisplayFormat::Bytes,
                None,
                Color::Magenta,
                "Size of messages added to a queue",
            ),

            // Default fallback
            _ => MetricDefinition::new(
                "Unknown Metric",
                DisplayFormat::Decimal(2),
                None,
                Color::Gray,
                "Unknown metric type",
            ),
        }
    }
}
