use crate::models::MetricType;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct HealthThresholds {
    pub critical: f64,
    pub warning: f64,
    pub reverse_logic: bool, // true for metrics where lower values are bad (like free memory)
}

#[derive(Debug, Clone)]
pub enum MetricUnit {
    Percent,
    Count,
    Seconds,
    Bytes,
    BytesPerSecond,
    Credits,
}

#[derive(Debug, Clone)]
pub enum DisplayFormat {
    Integer,
    Decimal(u8), // number of decimal places
    Bytes,
    Duration,
    Percentage,
}

#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub name: &'static str,
    pub unit: MetricUnit,
    pub display_format: DisplayFormat,
    pub thresholds: Option<HealthThresholds>,
    pub color: Color,
    pub description: &'static str,
}

impl MetricDefinition {
    /// Get health color based on current value and thresholds
    pub fn get_health_color(&self, value: f64) -> Color {
        match &self.thresholds {
            Some(thresholds) => {
                if thresholds.reverse_logic {
                    // For metrics like free memory where lower is worse
                    if value < thresholds.critical {
                        Color::Red
                    } else if value < thresholds.warning {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                } else {
                    // For metrics like CPU where higher is worse
                    if value > thresholds.critical {
                        Color::Red
                    } else if value > thresholds.warning {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                }
            }
            None => self.color, // Use default color if no thresholds
        }
    }

    /// Format value according to display format
    pub fn format_value(&self, value: f64) -> String {
        match &self.display_format {
            DisplayFormat::Integer => format!("{:.0}", value),
            DisplayFormat::Decimal(places) => format!("{:.1$}", value, *places as usize),
            DisplayFormat::Bytes => format_bytes(value),
            DisplayFormat::Duration => format_duration(value),
            DisplayFormat::Percentage => format!("{:.1}%", value),
        }
    }

    /// Get unit suffix for display
    pub fn unit_suffix(&self) -> &'static str {
        match &self.unit {
            MetricUnit::Percent => "%",
            MetricUnit::Count => "",
            MetricUnit::Seconds => "s",
            MetricUnit::Bytes => "B",
            MetricUnit::BytesPerSecond => "B/s",
            MetricUnit::Credits => " Credits",
        }
    }
}

/// AWS console-style metric definitions registry
pub struct MetricRegistry;

impl MetricRegistry {
    /// Get metric definition for a given metric type
    pub fn get_definition(metric_type: &MetricType) -> MetricDefinition {
        match metric_type {
            // === RDS Core Metrics ===
            MetricType::CpuUtilization => MetricDefinition {
                name: "CPU Utilization",
                unit: MetricUnit::Percent,
                display_format: DisplayFormat::Percentage,
                thresholds: Some(HealthThresholds {
                    critical: 80.0,
                    warning: 60.0,
                    reverse_logic: false,
                }),
                color: Color::Blue,
                description: "Percentage of CPU utilization",
            },

            MetricType::FreeStorageSpace => MetricDefinition {
                name: "Free Storage Space",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 1_073_741_824.0, // 1 GB
                    warning: 5_368_709_120.0,  // 5 GB
                    reverse_logic: true, // lower is worse
                }),
                color: Color::Cyan,
                description: "Amount of available storage space",
            },

            MetricType::ReadLatency => MetricDefinition {
                name: "Read Latency",
                unit: MetricUnit::Seconds,
                display_format: DisplayFormat::Duration,
                thresholds: Some(HealthThresholds {
                    critical: 0.1, // 100ms
                    warning: 0.05,  // 50ms
                    reverse_logic: false,
                }),
                color: Color::Red,
                description: "Average time taken for read operations",
            },

            MetricType::WriteLatency => MetricDefinition {
                name: "Write Latency",
                unit: MetricUnit::Seconds,
                display_format: DisplayFormat::Duration,
                thresholds: Some(HealthThresholds {
                    critical: 0.1, // 100ms
                    warning: 0.05,  // 50ms
                    reverse_logic: false,
                }),
                color: Color::Magenta,
                description: "Average time taken for write operations",
            },

            MetricType::ReadIops => MetricDefinition {
                name: "Read IOPS",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None, // No universal thresholds for IOPS
                color: Color::Green,
                description: "Read input/output operations per second",
            },

            MetricType::WriteIops => MetricDefinition {
                name: "Write IOPS",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None,
                color: Color::Yellow,
                description: "Write input/output operations per second",
            },

            MetricType::ReadThroughput => MetricDefinition {
                name: "Read Throughput",
                unit: MetricUnit::BytesPerSecond,
                display_format: DisplayFormat::Bytes,
                thresholds: None,
                color: Color::Cyan,
                description: "Read throughput in bytes per second",
            },

            MetricType::WriteThroughput => MetricDefinition {
                name: "Write Throughput",
                unit: MetricUnit::BytesPerSecond,
                display_format: DisplayFormat::Bytes,
                thresholds: None,
                color: Color::Magenta,
                description: "Write throughput in bytes per second",
            },

            MetricType::NetworkReceiveThroughput => MetricDefinition {
                name: "Network Receive Throughput",
                unit: MetricUnit::BytesPerSecond,
                display_format: DisplayFormat::Bytes,
                thresholds: None,
                color: Color::LightGreen,
                description: "Network receive throughput in bytes per second",
            },

            MetricType::NetworkTransmitThroughput => MetricDefinition {
                name: "Network Transmit Throughput",
                unit: MetricUnit::BytesPerSecond,
                display_format: DisplayFormat::Bytes,
                thresholds: None,
                color: Color::LightBlue,
                description: "Network transmit throughput in bytes per second",
            },

            MetricType::SwapUsage => MetricDefinition {
                name: "Swap Usage",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 1_073_741_824.0, // 1 GB
                    warning: 536_870_912.0,    // 512 MB
                    reverse_logic: false, // higher is worse
                }),
                color: Color::LightRed,
                description: "Amount of swap space used",
            },

            MetricType::QueueDepth => MetricDefinition {
                name: "Queue Depth",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 10.0,
                    warning: 5.0,
                    reverse_logic: false,
                }),
                color: Color::LightYellow,
                description: "Number of outstanding I/O requests",
            },

            MetricType::BurstBalance => MetricDefinition {
                name: "Burst Balance",
                unit: MetricUnit::Percent,
                display_format: DisplayFormat::Percentage,
                thresholds: Some(HealthThresholds {
                    critical: 20.0,
                    warning: 50.0,
                    reverse_logic: true, // lower is worse
                }),
                color: Color::LightCyan,
                description: "Percentage of burst bucket credits available",
            },

            MetricType::CpuCreditUsage => MetricDefinition {
                name: "CPU Credit Usage",
                unit: MetricUnit::Credits,
                display_format: DisplayFormat::Decimal(2),
                thresholds: None,
                color: Color::LightMagenta,
                description: "CPU credits consumed by the instance",
            },

            MetricType::CpuCreditBalance => MetricDefinition {
                name: "CPU Credit Balance",
                unit: MetricUnit::Credits,
                display_format: DisplayFormat::Decimal(2),
                thresholds: Some(HealthThresholds {
                    critical: 50.0,
                    warning: 100.0,
                    reverse_logic: true, // lower is worse
                }),
                color: Color::Green,
                description: "Number of earned CPU credits accumulated",
            },

            MetricType::CpuSurplusCreditBalance => MetricDefinition {
                name: "CPU Surplus Credit Balance",
                unit: MetricUnit::Credits,
                display_format: DisplayFormat::Decimal(2),
                thresholds: None,
                color: Color::DarkGray,
                description: "Number of surplus CPU credits spent",
            },

            MetricType::CpuSurplusCreditsCharged => MetricDefinition {
                name: "CPU Surplus Credits Charged",
                unit: MetricUnit::Credits,
                display_format: DisplayFormat::Decimal(2),
                thresholds: None,
                color: Color::Gray,
                description: "Number of surplus CPU credits charged",
            },

            MetricType::EbsByteBalance => MetricDefinition {
                name: "EBS Byte Balance",
                unit: MetricUnit::Percent,
                display_format: DisplayFormat::Percentage,
                thresholds: Some(HealthThresholds {
                    critical: 20.0,
                    warning: 50.0,
                    reverse_logic: true, // lower is worse
                }),
                color: Color::Cyan,
                description: "Percentage of throughput credits available",
            },

            MetricType::EbsIoBalance => MetricDefinition {
                name: "EBS IO Balance",
                unit: MetricUnit::Percent,
                display_format: DisplayFormat::Percentage,
                thresholds: Some(HealthThresholds {
                    critical: 20.0,
                    warning: 50.0,
                    reverse_logic: true, // lower is worse
                }),
                color: Color::Blue,
                description: "Percentage of I/O credits available",
            },

            MetricType::BinLogDiskUsage => MetricDefinition {
                name: "Binary Log Disk Usage",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 5_368_709_120.0,  // 5 GB
                    warning: 1_073_741_824.0,   // 1 GB
                    reverse_logic: false,
                }),
                color: Color::LightRed,
                description: "Amount of disk space occupied by binary logs",
            },

            MetricType::ReplicaLag => MetricDefinition {
                name: "Replica Lag",
                unit: MetricUnit::Seconds,
                display_format: DisplayFormat::Duration,
                thresholds: Some(HealthThresholds {
                    critical: 300.0, // 5 minutes
                    warning: 60.0,   // 1 minute
                    reverse_logic: false,
                }),
                color: Color::Red,
                description: "Amount of time a read replica lags behind",
            },

            MetricType::MaximumUsedTransactionIds => MetricDefinition {
                name: "Maximum Used Transaction IDs",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 1_500_000_000.0, // 1.5 billion
                    warning: 1_000_000_000.0,  // 1 billion
                    reverse_logic: false,
                }),
                color: Color::LightRed,
                description: "Maximum transaction IDs used (PostgreSQL)",
            },

            MetricType::OldestReplicationSlotLag => MetricDefinition {
                name: "Oldest Replication Slot Lag",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 10_737_418_240.0, // 10 GB
                    warning: 5_368_709_120.0,   // 5 GB
                    reverse_logic: false,
                }),
                color: Color::LightYellow,
                description: "Lag of the oldest replication slot",
            },

            MetricType::OldestLogicalReplicationSlotLag => MetricDefinition {
                name: "Oldest Logical Replication Slot Lag",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 10_737_418_240.0, // 10 GB
                    warning: 5_368_709_120.0,   // 5 GB
                    reverse_logic: false,
                }),
                color: Color::Yellow,
                description: "Lag of the oldest logical replication slot",
            },

            MetricType::ReplicationSlotDiskUsage => MetricDefinition {
                name: "Replication Slot Disk Usage",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 5_368_709_120.0,  // 5 GB
                    warning: 1_073_741_824.0,   // 1 GB
                    reverse_logic: false,
                }),
                color: Color::LightCyan,
                description: "Disk space used by replication slots",
            },

            MetricType::TransactionLogsDiskUsage => MetricDefinition {
                name: "Transaction Logs Disk Usage",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 10_737_418_240.0, // 10 GB
                    warning: 5_368_709_120.0,   // 5 GB
                    reverse_logic: false,
                }),
                color: Color::Magenta,
                description: "Disk space used by transaction logs",
            },

            MetricType::TransactionLogsGeneration => MetricDefinition {
                name: "Transaction Logs Generation",
                unit: MetricUnit::BytesPerSecond,
                display_format: DisplayFormat::Bytes,
                thresholds: None,
                color: Color::LightMagenta,
                description: "Rate of transaction log generation",
            },

            MetricType::FailedSqlServerAgentJobsCount => MetricDefinition {
                name: "Failed SQL Server Agent Jobs",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 3.0,
                    warning: 1.0,
                    reverse_logic: false,
                }),
                color: Color::Red,
                description: "Number of failed SQL Server Agent jobs",
            },

            MetricType::CheckpointLag => MetricDefinition {
                name: "Checkpoint Lag",
                unit: MetricUnit::Seconds,
                display_format: DisplayFormat::Duration,
                thresholds: Some(HealthThresholds {
                    critical: 300.0, // 5 minutes
                    warning: 120.0,  // 2 minutes
                    reverse_logic: false,
                }),
                color: Color::LightBlue,
                description: "Time since last checkpoint",
            },

            MetricType::ConnectionAttempts => MetricDefinition {
                name: "Connection Attempts",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None,
                color: Color::Blue,
                description: "Number of connection attempts to the database",
            },

            MetricType::FreeableMemory => MetricDefinition {
                name: "Freeable Memory",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 536_870_912.0, // 512 MB
                    warning: 1_073_741_824.0, // 1 GB
                    reverse_logic: true,
                }),
                color: Color::LightBlue,
                description: "Amount of available RAM",
            },

            // === SQS Metrics ===
            MetricType::ApproximateNumberOfMessagesVisible => MetricDefinition {
                name: "Messages Visible",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 1000.0,
                    warning: 100.0,
                    reverse_logic: false,
                }),
                color: Color::LightCyan,
                description: "Messages available for retrieval from the queue",
            },

            MetricType::ApproximateNumberOfMessagesNotVisible => MetricDefinition {
                name: "Messages Not Visible",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 500.0,
                    warning: 50.0,
                    reverse_logic: false,
                }),
                color: Color::Magenta,
                description: "Messages that are in flight",
            },

            MetricType::ApproximateAgeOfOldestMessage => MetricDefinition {
                name: "Oldest Message Age",
                unit: MetricUnit::Seconds,
                display_format: DisplayFormat::Duration,
                thresholds: Some(HealthThresholds {
                    critical: 3600.0, // 1 hour
                    warning: 300.0,   // 5 minutes
                    reverse_logic: false,
                }),
                color: Color::Red,
                description: "Age of the oldest message in the queue",
            },

            MetricType::ApproximateNumberOfMessagesDelayed => MetricDefinition {
                name: "Messages Delayed",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 100.0,
                    warning: 10.0,
                    reverse_logic: false,
                }),
                color: Color::LightRed,
                description: "Messages that are delayed and not yet visible",
            },

            MetricType::NumberOfMessagesSent => MetricDefinition {
                name: "Messages Sent",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None, // Throughput metrics don't have universal thresholds
                color: Color::Green,
                description: "Number of messages sent to the queue",
            },

            MetricType::NumberOfMessagesReceived => MetricDefinition {
                name: "Messages Received", 
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None,
                color: Color::Blue,
                description: "Number of messages received from the queue",
            },

            MetricType::NumberOfMessagesDeleted => MetricDefinition {
                name: "Messages Deleted",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None,
                color: Color::Yellow,
                description: "Number of messages deleted from the queue",
            },

            MetricType::NumberOfMessagesInDlq => MetricDefinition {
                name: "DLQ Messages",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 10.0,
                    warning: 1.0,
                    reverse_logic: false,
                }),
                color: Color::Red,
                description: "Messages in the dead letter queue",
            },

            MetricType::ApproximateNumberOfMessages => MetricDefinition {
                name: "Total Queue Depth",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 1000.0,
                    warning: 100.0,
                    reverse_logic: false,
                }),
                color: Color::Cyan,
                description: "Total number of messages in the queue",
            },

            MetricType::SentMessageSize => MetricDefinition {
                name: "Message Size",
                unit: MetricUnit::Bytes,
                display_format: DisplayFormat::Bytes,
                thresholds: Some(HealthThresholds {
                    critical: 204_800.0, // 200 KB
                    warning: 102_400.0,  // 100 KB
                    reverse_logic: false,
                }),
                color: Color::LightGreen,
                description: "Size of messages sent to the queue",
            },

            MetricType::NumberOfEmptyReceives => MetricDefinition {
                name: "Empty Receives",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 1000.0,
                    warning: 100.0,
                    reverse_logic: false,
                }),
                color: Color::Gray,
                description: "Number of empty receives from the queue",
            },

            MetricType::ApproximateNumberOfGroupsWithInflightMessages => MetricDefinition {
                name: "Groups with In-flight Messages",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: Some(HealthThresholds {
                    critical: 100.0,
                    warning: 50.0,
                    reverse_logic: false,
                }),
                color: Color::LightBlue,
                description: "Number of message groups with in-flight messages",
            },

            MetricType::NumberOfDeduplicatedSentMessages => MetricDefinition {
                name: "Deduplicated Messages",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Integer,
                thresholds: None,
                color: Color::LightBlue,
                description: "Number of messages sent after deduplication",
            },

            // Add more metrics as needed...
            _ => MetricDefinition {
                name: "Unknown Metric",
                unit: MetricUnit::Count,
                display_format: DisplayFormat::Decimal(2),
                thresholds: None,
                color: Color::Gray,
                description: "Unknown metric type",
            },
        }
    }

    /// Get all available metrics for a service
    pub fn get_available_metrics_for_service(service: &crate::models::AwsService) -> Vec<MetricType> {
        match service {
            crate::models::AwsService::Rds => vec![
                MetricType::CpuUtilization,
                MetricType::FreeStorageSpace,
                MetricType::ReadLatency,
                MetricType::WriteLatency,
                MetricType::ReadIops,
                MetricType::WriteIops,
                MetricType::FreeableMemory,
                // Add more RDS metrics...
            ],
            crate::models::AwsService::Sqs => vec![
                MetricType::ApproximateNumberOfMessagesVisible,
                MetricType::ApproximateNumberOfMessagesNotVisible,
                MetricType::ApproximateAgeOfOldestMessage,
                MetricType::ApproximateNumberOfMessagesDelayed,
                MetricType::NumberOfMessagesSent,
                MetricType::NumberOfMessagesReceived,
                MetricType::NumberOfMessagesDeleted,
                MetricType::NumberOfMessagesInDlq,
                // Add more SQS metrics...
            ],
        }
    }
}

// Helper functions for formatting
fn format_bytes(bytes: f64) -> String {
    const UNITS: &[(&str, f64)] = &[
        ("TB", 1024.0 * 1024.0 * 1024.0 * 1024.0),
        ("GB", 1024.0 * 1024.0 * 1024.0),
        ("MB", 1024.0 * 1024.0),
        ("KB", 1024.0),
    ];

    for &(unit, size) in UNITS {
        if bytes >= size {
            return format!("{:.1} {}", bytes / size, unit);
        }
    }
    format!("{:.0} B", bytes)
}

fn format_duration(seconds: f64) -> String {
    if seconds < 0.001 {
        format!("{:.0} μs", seconds * 1_000_000.0)
    } else if seconds < 1.0 {
        format!("{:.0} ms", seconds * 1000.0)
    } else if seconds < 60.0 {
        format!("{:.1} s", seconds)
    } else if seconds < 3600.0 {
        format!("{:.1} min", seconds / 60.0)
    } else {
        format!("{:.1} hr", seconds / 3600.0)
    }
} 