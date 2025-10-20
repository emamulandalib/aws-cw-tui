use crate::aws::dynamic_metric_discovery::DynamicMetricData;
use crate::models::{MetricData, MetricType, SqsMetricData};
use crate::ui::components::metric::MetricRegistry;
use ratatui::style::Color;

/// Format dynamic metric values for display
pub fn format_dynamic_metric_value(metric_data: &DynamicMetricData) -> String {
    if metric_data.history.is_empty() {
        return "N/A".to_string();
    }

    let current_value = metric_data.history[metric_data.history.len() - 1];
    let metric_name_lower = metric_data.metric_name.to_lowercase();

    if metric_name_lower.contains("bytes") || metric_name_lower.contains("size") {
        format_bytes_value(current_value)
    } else if metric_name_lower.contains("percent") || metric_name_lower.contains("cpu") {
        format!("{:.2}%", current_value)
    } else if metric_name_lower.contains("count") || metric_name_lower.contains("connections") {
        format!("{:.0}", current_value)
    } else if metric_name_lower.contains("latency") || metric_name_lower.contains("duration") {
        format_duration_value(current_value)
    } else {
        format_generic_metric_value(current_value)
    }
}

/// Format bytes with appropriate units
pub use crate::utils::formatting::format_bytes as format_bytes_value;

/// Format duration values (input in milliseconds)
pub fn format_duration_value(duration: f64) -> String {
    // Convert milliseconds to seconds and use centralized formatting
    crate::utils::formatting::format_duration(duration / 1000.0)
}

/// Format generic metric values
pub fn format_generic_metric_value(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("{:.2}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else if value >= 100.0 {
        format!("{:.0}", value)
    } else if value >= 10.0 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
    }
}

/// Get display information for RDS metrics
pub fn get_rds_metric_display_info<'a>(
    metric_type: &MetricType,
    metrics: &'a MetricData,
) -> (&'static str, String, &'a Vec<f64>, Color, f64) {
    match metric_type {
        MetricType::CpuUtilization => (
            "CPU Utilization (%)",
            format_percentage_with_fallback(&metrics.cpu_history),
            &metrics.cpu_history,
            MetricRegistry::get_definition(metric_type).color,
            100.0,
        ),
        MetricType::DatabaseConnections => (
            "Database Connections",
            format_count_with_fallback(&metrics.connections_history),
            &metrics.connections_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .connections_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::FreeableMemory => (
            "Freeable Memory (MB)",
            format_bytes_mb_with_fallback(&metrics.freeable_memory_history),
            &metrics.freeable_memory_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .freeable_memory_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::FreeStorageSpace => (
            "Free Storage Space (GB)",
            format_bytes_gb_with_fallback(&metrics.free_storage_space_history),
            &metrics.free_storage_space_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .free_storage_space_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::ReadIops => (
            "Read IOPS",
            format_count_with_fallback(&metrics.read_iops_history),
            &metrics.read_iops_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .read_iops_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::WriteIops => (
            "Write IOPS",
            format_count_with_fallback(&metrics.write_iops_history),
            &metrics.write_iops_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .write_iops_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::ReadLatency => (
            "Read Latency (ms)",
            format_latency_with_fallback(&metrics.read_latency_history),
            &metrics.read_latency_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .read_latency_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::WriteLatency => (
            "Write Latency (ms)",
            format_latency_with_fallback(&metrics.write_latency_history),
            &metrics.write_latency_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .write_latency_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::NetworkReceiveThroughput => (
            "Network Receive Throughput (MB/s)",
            format_throughput_with_fallback(&metrics.network_receive_history),
            &metrics.network_receive_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .network_receive_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::NetworkTransmitThroughput => (
            "Network Transmit Throughput (MB/s)",
            format_throughput_with_fallback(&metrics.network_transmit_history),
            &metrics.network_transmit_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .network_transmit_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        _ => {
            static EMPTY_VEC: Vec<f64> = Vec::new();
            let theme = crate::ui::themes::UnifiedTheme::default();
            (
                "Unknown Metric",
                "N/A".to_string(),
                &EMPTY_VEC,
                theme.muted,
                1.0,
            )
        }
    }
}

/// Get display information for SQS metrics
pub fn get_sqs_metric_display_info<'a>(
    metric_type: &MetricType,
    metrics: &'a SqsMetricData,
) -> (&'static str, String, &'a Vec<f64>, Color, f64) {
    match metric_type {
        MetricType::ApproximateNumberOfMessages => (
            "Approx Number of Messages",
            format_count_with_fallback(&metrics.queue_depth_history),
            &metrics.queue_depth_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .queue_depth_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::ApproximateNumberOfMessagesDelayed => (
            "Approx Number of Messages Delayed",
            format_count_with_fallback(&metrics.messages_delayed_history),
            &metrics.messages_delayed_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .messages_delayed_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::ApproximateNumberOfMessagesNotVisible => (
            "Approx Number of Messages Not Visible",
            format_count_with_fallback(&metrics.messages_not_visible_history),
            &metrics.messages_not_visible_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .messages_not_visible_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::NumberOfMessagesSent => (
            "Number of Messages Sent",
            format_count_with_fallback(&metrics.messages_sent_history),
            &metrics.messages_sent_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .messages_sent_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::NumberOfMessagesReceived => (
            "Number of Messages Received",
            format_count_with_fallback(&metrics.messages_received_history),
            &metrics.messages_received_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .messages_received_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        MetricType::NumberOfMessagesDeleted => (
            "Number of Messages Deleted",
            format_count_with_fallback(&metrics.messages_deleted_history),
            &metrics.messages_deleted_history,
            MetricRegistry::get_definition(metric_type).color,
            metrics
                .messages_deleted_history
                .iter()
                .cloned()
                .fold(0.0f64, f64::max),
        ),
        _ => {
            static EMPTY_VEC_SQS: Vec<f64> = Vec::new();
            let theme = crate::ui::themes::UnifiedTheme::default();
            (
                "Unknown Metric",
                "N/A".to_string(),
                &EMPTY_VEC_SQS,
                theme.muted,
                1.0,
            )
        }
    }
}

// Helper formatting functions with fallback to "N/A"
fn format_percentage_with_fallback(history: &[f64]) -> String {
    if let Some(&last_value) = history.last() {
        crate::utils::formatting::format_percentage(last_value)
    } else {
        "N/A".to_string()
    }
}

fn format_count_with_fallback(history: &[f64]) -> String {
    if let Some(&last_value) = history.last() {
        format!("{:.0}", last_value)
    } else {
        "N/A".to_string()
    }
}

fn format_bytes_mb_with_fallback(history: &[f64]) -> String {
    if let Some(&last_value) = history.last() {
        format!("{:.0} MB", last_value / (1024.0 * 1024.0))
    } else {
        "N/A".to_string()
    }
}

fn format_bytes_gb_with_fallback(history: &[f64]) -> String {
    if let Some(&last_value) = history.last() {
        format!("{:.2} GB", last_value / (1024.0 * 1024.0 * 1024.0))
    } else {
        "N/A".to_string()
    }
}

fn format_latency_with_fallback(history: &[f64]) -> String {
    if let Some(&last_value) = history.last() {
        format!("{:.3} ms", last_value * 1000.0) // Convert seconds to milliseconds
    } else {
        "N/A".to_string()
    }
}

fn format_throughput_with_fallback(history: &[f64]) -> String {
    if let Some(&last_value) = history.last() {
        format!("{:.2} MB/s", last_value / (1024.0 * 1024.0))
    } else {
        "N/A".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_value() {
        assert_eq!(format_bytes_value(512.0), "512 B");
        assert_eq!(format_bytes_value(1536.0), "1.5 KB");
        assert_eq!(format_bytes_value(1048576.0), "1.0 MB");
    }

    #[test]
    fn test_format_duration_value() {
        assert_eq!(format_duration_value(500.0), "500 ms");
        assert_eq!(format_duration_value(1500.0), "1.50 s");
        assert_eq!(format_duration_value(65000.0), "1.1 min");
    }

    #[test]
    fn test_format_generic_metric_value() {
        assert_eq!(format_generic_metric_value(5.0), "5.00");
        assert_eq!(format_generic_metric_value(150.0), "150");
        assert_eq!(format_generic_metric_value(1500.0), "1.5K");
        assert_eq!(format_generic_metric_value(1500000.0), "1.50M");
    }

    #[test]
    fn test_format_percentage_with_fallback() {
        assert_eq!(format_percentage_with_fallback(&[75.5]), "75.50%");
        assert_eq!(format_percentage_with_fallback(&[]), "N/A");
    }

    #[test]
    fn test_format_count_with_fallback() {
        assert_eq!(format_count_with_fallback(&[42.7]), "43");
        assert_eq!(format_count_with_fallback(&[]), "N/A");
    }
}
