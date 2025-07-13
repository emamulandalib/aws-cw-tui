use crate::models::MetricType;
use crate::ui::components::metric::{DisplayFormat, HealthThresholds, MetricDefinition};
use ratatui::style::Color;

/// Get SQS metric definitions
pub fn get_sqs_metric_definition(metric_type: &MetricType) -> MetricDefinition {
    match metric_type {
        MetricType::ApproximateNumberOfMessagesVisible => MetricDefinition {
            name: "Messages Visible",
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
            display_format: DisplayFormat::Integer,
            thresholds: None,
            color: Color::Green,
            description: "Number of messages sent to the queue",
        },

        MetricType::NumberOfMessagesReceived => MetricDefinition {
            name: "Messages Received",
            display_format: DisplayFormat::Integer,
            thresholds: None,
            color: Color::Blue,
            description: "Number of messages received from the queue",
        },

        MetricType::NumberOfMessagesDeleted => MetricDefinition {
            name: "Messages Deleted",
            display_format: DisplayFormat::Integer,
            thresholds: None,
            color: Color::Yellow,
            description: "Number of messages deleted from the queue",
        },

        MetricType::NumberOfMessagesInDlq => MetricDefinition {
            name: "DLQ Messages",
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
            display_format: DisplayFormat::Integer,
            thresholds: None,
            color: Color::LightBlue,
            description: "Number of messages sent after deduplication",
        },

        _ => MetricDefinition {
            name: "Unknown Metric",
            display_format: DisplayFormat::Decimal(2),
            thresholds: None,
            color: Color::Gray,
            description: "Unknown metric type",
        },
    }
}