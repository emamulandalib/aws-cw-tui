use crate::models::{MetricData, MetricType};
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
        MetricType::CpuCreditUsage | MetricType::CpuCreditBalance => "Credits",
    }
}

/// Get metrics data with history for enhanced display
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
        _ => (Color::Cyan, Color::Cyan), // Default neutral color
    };

    (value_color, trend_color)
}

/// Format a metric value based on its unit
pub fn format_value(value: f64, unit: &str) -> String {
    match unit {
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
            } else {
                format!("{value:.1}")
            }
        }
        _ => format!("{value:.2}"),
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
