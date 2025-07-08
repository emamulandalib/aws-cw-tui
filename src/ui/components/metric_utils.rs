use crate::models::{App, MetricData, MetricType, SqsMetricData};
use ratatui::style::Color;

/// Get the unit string for a given metric type
pub fn get_metric_unit(metric_type: &MetricType) -> &'static str {
    match metric_type {
        MetricType::CpuUtilization | MetricType::BurstBalance => "Percent",
        MetricType::DatabaseConnections
        | MetricType::ReadIops
        | MetricType::WriteIops
        | MetricType::QueueDepth
        | MetricType::ConnectionAttempts
        | MetricType::MaximumUsedTransactionIds
        | MetricType::FailedSqlServerAgentJobsCount => "Count",
        MetricType::ReadLatency
        | MetricType::WriteLatency
        | MetricType::ReplicaLag
        | MetricType::CheckpointLag => "Seconds",
        MetricType::FreeStorageSpace
        | MetricType::FreeableMemory
        | MetricType::SwapUsage
        | MetricType::BinLogDiskUsage
        | MetricType::ReplicationSlotDiskUsage
        | MetricType::TransactionLogsDiskUsage
        | MetricType::OldestReplicationSlotLag => "Bytes",
        MetricType::ReadThroughput
        | MetricType::WriteThroughput
        | MetricType::NetworkReceiveThroughput
        | MetricType::NetworkTransmitThroughput
        | MetricType::TransactionLogsGeneration => "Bytes/Second",
        MetricType::CpuCreditUsage | MetricType::CpuCreditBalance 
        | MetricType::CpuSurplusCreditBalance | MetricType::CpuSurplusCreditsCharged => "Credits",
        MetricType::EbsByteBalance | MetricType::EbsIoBalance => "Percent",
        MetricType::OldestLogicalReplicationSlotLag => "Bytes",
        // SQS Metrics
        MetricType::NumberOfMessagesSent
        | MetricType::NumberOfMessagesReceived
        | MetricType::NumberOfMessagesDeleted
        | MetricType::ApproximateNumberOfMessages
        | MetricType::ApproximateNumberOfMessagesVisible
        | MetricType::ApproximateNumberOfMessagesNotVisible
        | MetricType::NumberOfEmptyReceives
        | MetricType::ApproximateNumberOfMessagesDelayed
        | MetricType::NumberOfMessagesInDlq
        | MetricType::ApproximateNumberOfGroupsWithInflightMessages
        | MetricType::NumberOfDeduplicatedSentMessages => "Count",
        MetricType::ApproximateAgeOfOldestMessage => "Seconds",
        MetricType::SentMessageSize => "Bytes",
    }
}

/// Get metrics data with history for enhanced display (unified for all services)
pub fn get_available_metrics_with_history_unified(
    app: &App,
) -> Vec<(&'static str, f64, &Vec<f64>, &'static str)> {
    match app.selected_service.as_ref().unwrap_or(&crate::models::AwsService::Rds) {
        crate::models::AwsService::Rds => get_available_metrics_with_history(&app.metrics),
        crate::models::AwsService::Sqs => get_available_sqs_metrics_with_history(&app.sqs_metrics),
    }
}

/// Get metrics data with history for enhanced display (RDS)
pub fn get_available_metrics_with_history(
    metrics: &MetricData,
) -> Vec<(&'static str, f64, &Vec<f64>, &'static str)> {
    let mut available = Vec::new();

    // Use the same logic as MetricData.get_available_metrics() for consistency
    let available_metric_types = metrics.get_available_metrics();

    for metric_type in available_metric_types {
        let metric_name = metric_type.display_name();
        let history = metrics.get_metric_history(&metric_type);
        let unit = get_metric_unit(&metric_type);

        // Get current value based on metric type
        let current_value = match metric_type {
            MetricType::CpuUtilization => metrics.cpu_utilization,
            MetricType::DatabaseConnections => metrics.database_connections,
            MetricType::FreeStorageSpace => metrics.free_storage_space,
            MetricType::ReadIops => metrics.read_iops,
            MetricType::WriteIops => metrics.write_iops,
            MetricType::ReadLatency => metrics.read_latency,
            MetricType::WriteLatency => metrics.write_latency,
            MetricType::ReadThroughput => metrics.read_throughput,
            MetricType::WriteThroughput => metrics.write_throughput,
            MetricType::NetworkReceiveThroughput => metrics.network_receive_throughput,
            MetricType::NetworkTransmitThroughput => metrics.network_transmit_throughput,
            MetricType::FreeableMemory => metrics.freeable_memory,
            MetricType::SwapUsage => metrics.swap_usage,
            MetricType::QueueDepth => metrics.queue_depth,
            MetricType::BurstBalance => metrics.burst_balance,
            MetricType::CpuCreditUsage => metrics.cpu_credit_usage,
            MetricType::CpuCreditBalance => metrics.cpu_credit_balance,
            MetricType::BinLogDiskUsage => metrics.bin_log_disk_usage,
            MetricType::ReplicaLag => metrics.replica_lag,
            MetricType::MaximumUsedTransactionIds => metrics.maximum_used_transaction_ids,
            MetricType::OldestReplicationSlotLag => metrics.oldest_replication_slot_lag,
            MetricType::ReplicationSlotDiskUsage => metrics.replication_slot_disk_usage,
            MetricType::TransactionLogsDiskUsage => metrics.transaction_logs_disk_usage,
            MetricType::TransactionLogsGeneration => metrics.transaction_logs_generation,
            MetricType::FailedSqlServerAgentJobsCount => metrics.failed_sql_server_agent_jobs_count,
            MetricType::CheckpointLag => metrics.checkpoint_lag,
            MetricType::ConnectionAttempts => metrics.connection_attempts,
            // New missing metrics
            MetricType::CpuSurplusCreditBalance => metrics.cpu_surplus_credit_balance,
            MetricType::CpuSurplusCreditsCharged => metrics.cpu_surplus_credits_charged,
            MetricType::EbsByteBalance => metrics.ebs_byte_balance,
            MetricType::EbsIoBalance => metrics.ebs_io_balance,
            MetricType::OldestLogicalReplicationSlotLag => metrics.oldest_logical_replication_slot_lag,
            // SQS Metrics - return 0.0 for RDS metrics function
            MetricType::NumberOfMessagesSent => 0.0,
            MetricType::NumberOfMessagesReceived => 0.0,
            MetricType::NumberOfMessagesDeleted => 0.0,
            MetricType::ApproximateNumberOfMessages => 0.0,
            MetricType::ApproximateNumberOfMessagesVisible => 0.0,
            MetricType::ApproximateNumberOfMessagesNotVisible => 0.0,
            MetricType::ApproximateAgeOfOldestMessage => 0.0,
            MetricType::NumberOfEmptyReceives => 0.0,
            MetricType::ApproximateNumberOfMessagesDelayed => 0.0,
            MetricType::SentMessageSize => 0.0,
            MetricType::NumberOfMessagesInDlq => 0.0,
            MetricType::ApproximateNumberOfGroupsWithInflightMessages => 0.0,
            MetricType::NumberOfDeduplicatedSentMessages => 0.0,
        };

        available.push((metric_name, current_value, history, unit));
    }

    available
}

/// Get color scheme for a metric based on its name and current value
pub fn get_metric_colors(metric_name: &str, current_value: f64) -> (Color, Color) {
    let (value_color, trend_color) = match metric_name {
        "CPU Utilization" => {
            if current_value > 80.0 {
                (Color::Red, Color::Red)
            } else if current_value > 60.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Database Connections" => {
            // Assume > 1000 is high, > 500 is moderate
            if current_value > 1000.0 {
                (Color::Red, Color::Red)
            } else if current_value > 500.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Read Latency" | "Write Latency" => {
            // Latency in seconds - > 0.1s is bad, > 0.05s is moderate
            if current_value > 0.1 {
                (Color::Red, Color::Red)
            } else if current_value > 0.05 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Free Storage Space" | "Freeable Memory" => {
            // For storage/memory, lower is worse (inverted logic)
            if current_value < 1024.0 * 1024.0 * 1024.0 {
                (Color::Red, Color::Red)
            }
            // < 1GB
            else if current_value < 5.0 * 1024.0 * 1024.0 * 1024.0 {
                (Color::Yellow, Color::Yellow)
            }
            // < 5GB
            else {
                (Color::Green, Color::Green)
            }
        }
        "Burst Balance" => {
            if current_value < 20.0 {
                (Color::Red, Color::Red)
            } else if current_value < 50.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Replica Lag" => {
            if current_value > 300.0 {
                (Color::Red, Color::Red)
            }
            // > 5 minutes
            else if current_value > 60.0 {
                (Color::Yellow, Color::Yellow)
            }
            // > 1 minute
            else {
                (Color::Green, Color::Green)
            }
        }
        // SQS-specific metrics
        "Messages Visible" => {
            if current_value > 1000.0 {
                (Color::Red, Color::Red)
            } else if current_value > 100.0 {
                (Color::Yellow, Color::Yellow)
            } else if current_value > 0.0 {
                (Color::Green, Color::Green)
            } else {
                (Color::Gray, Color::Gray)
            }
        }
        "Messages Sent" | "Messages Received" | "Messages Deleted" => {
            // For throughput metrics, any activity is good
            if current_value > 0.0 {
                (Color::Green, Color::Green)
            } else {
                (Color::Gray, Color::Gray)
            }
        }
        "Messages Not Visible" => {
            // In-flight messages - too many might indicate processing issues
            if current_value > 500.0 {
                (Color::Red, Color::Red)
            } else if current_value > 50.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Oldest Message Age" => {
            // Age in seconds - older messages indicate processing delays
            if current_value > 3600.0 {
                (Color::Red, Color::Red)
            } // > 1 hour
            else if current_value > 300.0 {
                (Color::Yellow, Color::Yellow)
            } // > 5 minutes
            else {
                (Color::Green, Color::Green)
            }
        }
        "Empty Receives" => {
            // Too many empty receives might indicate polling issues
            if current_value > 100.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "DLQ Messages" => {
            // Any messages in DLQ is concerning
            if current_value > 10.0 {
                (Color::Red, Color::Red)
            } else if current_value > 0.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Messages Delayed" => {
            // Delayed messages might indicate processing issues
            if current_value > 100.0 {
                (Color::Red, Color::Red)
            } else if current_value > 10.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        _ => (Color::Cyan, Color::Cyan), // Default neutral color
    };

    (value_color, trend_color)
}

/// Format a metric value based on its unit
pub fn format_value(value: f64, unit: &str) -> String {
    // Debug: Force display for debugging
    let formatted = match unit {
        "Bytes" | "Bytes/Second" => format_bytes(value),
        "Percent" => format!("{value:.1}%"),
        "Seconds" => {
            if value < 0.001 {
                format!("{:.2} μs", value * 1_000_000.0)
            } else if value < 1.0 {
                format!("{:.2} ms", value * 1000.0)
            } else {
                format!("{value:.2} s")
            }
        }
        "Count" | "Count/Second" | "Credits" => {
            if value >= 1_000_000.0 {
                format!("{:.1}M", value / 1_000_000.0)
            } else if value >= 1_000.0 {
                format!("{:.1}K", value / 1_000.0)
            } else if value == 0.0 {
                "0".to_string() // Show zero values clearly
            } else {
                format!("{value:.1}")
            }
        }
        _ => format!("{value:.2}"),
    };
    
    // Ensure we always return something visible (minimum 1 character)
    if formatted.is_empty() {
        "0".to_string()
    } else {
        formatted
    }
}

/// Format bytes with appropriate unit (B, KB, MB, GB, TB)
pub fn format_bytes(bytes: f64) -> String {
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

    format!("{bytes:.0} B")
}

/// Get SQS metrics data with history for enhanced display
pub fn get_available_sqs_metrics_with_history(
    metrics: &SqsMetricData,
) -> Vec<(&'static str, f64, &Vec<f64>, &'static str)> {
    let mut available = Vec::new();

    // Use the same logic as SqsMetricData.get_available_metrics() for consistency
    let available_metric_types = metrics.get_available_metrics();

    for metric_type in available_metric_types {
        let metric_name = metric_type.display_name();
        let history = metrics.get_metric_history(&metric_type);
        let unit = get_metric_unit(&metric_type);

        // Get current value based on metric type
        let stored_current_value = match metric_type {
            MetricType::NumberOfMessagesSent => metrics.number_of_messages_sent,
            MetricType::NumberOfMessagesReceived => metrics.number_of_messages_received,
            MetricType::NumberOfMessagesDeleted => metrics.number_of_messages_deleted,
            MetricType::ApproximateNumberOfMessages => metrics.approximate_number_of_messages,
            MetricType::ApproximateNumberOfMessagesVisible => metrics.approximate_number_of_messages_visible,
            MetricType::ApproximateNumberOfMessagesNotVisible => metrics.approximate_number_of_messages_not_visible,
            MetricType::ApproximateAgeOfOldestMessage => metrics.approximate_age_of_oldest_message,
            MetricType::NumberOfEmptyReceives => metrics.number_of_empty_receives,
            MetricType::ApproximateNumberOfMessagesDelayed => metrics.approximate_number_of_messages_delayed,
            MetricType::SentMessageSize => metrics.sent_message_size,
            MetricType::NumberOfMessagesInDlq => metrics.number_of_messages_in_dlq,
            MetricType::ApproximateNumberOfGroupsWithInflightMessages => metrics.approximate_number_of_groups_with_inflight_messages,
            MetricType::NumberOfDeduplicatedSentMessages => metrics.number_of_deduplicated_sent_messages,
            // RDS Metrics - return 0.0 for SQS metrics function
            _ => 0.0,
        };

        // For SQS metrics, use the historical data's last value if it's more recent or reliable
        // This ensures consistency between the displayed value and the sparkline
        let current_value = if !history.is_empty() {
            let history_last = history.last().copied().unwrap_or(0.0);
            // Use the history's last value if it's different from stored value
            // This helps with metrics that might have fresher data in history
            if history_last != stored_current_value && history_last > 0.0 {
                history_last
            } else {
                stored_current_value
            }
        } else {
            stored_current_value
        };

        available.push((metric_name, current_value, history, unit));
    }

    available
}
