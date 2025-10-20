use crate::models::{MetricData, MetricType};
use ratatui::style::Color;

/// Get the unit string for a given metric type
pub fn get_metric_unit(metric_type: &MetricType) -> &'static str {
    match metric_type {
        MetricType::CpuUtilization | MetricType::BurstBalance | MetricType::Percentage => "Percent",
        MetricType::DatabaseConnections
        | MetricType::ReadIops
        | MetricType::WriteIops
        | MetricType::QueueDepth
        | MetricType::ConnectionAttempts
        | MetricType::MaximumUsedTransactionIds
        | MetricType::FailedSqlServerAgentJobsCount
        | MetricType::NumberOfMessagesSent
        | MetricType::NumberOfMessagesReceived
        | MetricType::NumberOfMessagesDeleted
        | MetricType::ApproximateNumberOfMessages
        | MetricType::ApproximateNumberOfMessagesVisible
        | MetricType::ApproximateNumberOfMessagesNotVisible
        | MetricType::NumberOfEmptyReceives
        | MetricType::ApproximateNumberOfMessagesDelayed
        | MetricType::NumberOfMessagesInDlq
        | MetricType::ApproximateNumberOfGroupsWithInflightMessages
        | MetricType::NumberOfDeduplicatedSentMessages
        | MetricType::Count => "Count",
        MetricType::ReadLatency
        | MetricType::WriteLatency
        | MetricType::ReplicaLag
        | MetricType::CheckpointLag
        | MetricType::ApproximateAgeOfOldestMessage
        | MetricType::Seconds => "Seconds",
        MetricType::FreeStorageSpace
        | MetricType::FreeableMemory
        | MetricType::SwapUsage
        | MetricType::BinLogDiskUsage
        | MetricType::ReplicationSlotDiskUsage
        | MetricType::TransactionLogsDiskUsage
        | MetricType::OldestReplicationSlotLag
        | MetricType::OldestLogicalReplicationSlotLag
        | MetricType::SentMessageSize
        | MetricType::Bytes => "Bytes",
        MetricType::ReadThroughput
        | MetricType::WriteThroughput
        | MetricType::NetworkReceiveThroughput
        | MetricType::NetworkTransmitThroughput
        | MetricType::TransactionLogsGeneration => "Bytes/Second",
        MetricType::CpuCreditUsage
        | MetricType::CpuCreditBalance
        | MetricType::CpuSurplusCreditBalance
        | MetricType::CpuSurplusCreditsCharged => "Credits",
        MetricType::EbsByteBalance | MetricType::EbsIoBalance => "Percent",
    }
}

/// Get metrics data with history for enhanced display
pub fn get_available_metrics_with_history(
    metrics: &MetricData,
) -> Vec<(&'static str, f64, &Vec<f64>, &'static str)> {
    let mut available = Vec::new();

    for metric_type in metrics.get_available_metrics_with_data() {
        let metric_name = metric_type.display_name();
        let history = metrics.get_metric_history(&metric_type);
        let unit = get_metric_unit(&metric_type);

        let current_value = metrics.get_metric_value(&metric_type);

        available.push((metric_name, current_value, history, unit));
    }

    available
}

/// Get color scheme for a metric based on its name and current value
pub fn get_metric_colors(metric_name: &str, current_value: f64) -> (Color, Color) {
    let (value_color, trend_color) = match metric_name {
        "CPU Utilization" => threshold_colors(current_value, 80.0, 60.0),
        "Database Connections" => threshold_colors(current_value, 1000.0, 500.0),
        "Read Latency" | "Write Latency" => threshold_colors(current_value, 0.1, 0.05),
        "Free Storage Space" | "Freeable Memory" => {
            // Lower is worse, invert thresholds (values in bytes)
            if current_value < 1.0 * 1024.0 * 1024.0 * 1024.0 {
                (Color::Red, Color::Red)
            } else if current_value < 5.0 * 1024.0 * 1024.0 * 1024.0 {
                (Color::Yellow, Color::Yellow)
            } else {
                (Color::Green, Color::Green)
            }
        }
        "Burst Balance" => threshold_colors(current_value, 20.0, 50.0),
        "Replica Lag" | "Approximate Age Of Oldest Message" => {
            threshold_colors(current_value, 300.0, 60.0)
        }
        _ => (Color::Cyan, Color::Cyan),
    };

    (value_color, trend_color)
}

fn threshold_colors(value: f64, high: f64, medium: f64) -> (Color, Color) {
    if value > high {
        (Color::Red, Color::Red)
    } else if value > medium {
        (Color::Yellow, Color::Yellow)
    } else {
        (Color::Green, Color::Green)
    }
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
        "Count" | "Credits" => format_count(value),
        _ => format!("{value:.2}"),
    }
}

fn format_count(value: f64) -> String {
    if value.abs() >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else {
        format!("{value:.1}")
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
        if bytes.abs() >= size {
            return format!("{:.1} {}", bytes / size, unit);
        }
    }

    format!("{:.0} B", bytes)
}
