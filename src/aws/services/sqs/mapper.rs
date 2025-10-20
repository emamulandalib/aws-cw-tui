use crate::aws::services::sqs::validator::{
    sanitize_metric_values, validate_sqs_metric_constraints,
};
use crate::models::SqsMetricData;
use std::time::SystemTime;

/// Map CloudWatch metric data to SqsMetricData structure
pub struct MetricMapper {
    /// Track warnings during mapping
    pub warnings: Vec<String>,
}

impl MetricMapper {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
        }
    }

    /// Map CloudWatch metric to SqsMetricData field
    pub fn map_metric(
        &mut self,
        metric_data: &mut SqsMetricData,
        metric_name: &str,
        values: Vec<f64>,
        _timestamps: Vec<SystemTime>,
    ) -> Result<(), String> {
        // Validate and sanitize the data
        let sanitized_values = sanitize_metric_values(&values);

        // Get current value (latest valid value)
        let current_value = self.get_current_value(&sanitized_values);

        // Validate SQS-specific constraints
        let validation = validate_sqs_metric_constraints(metric_name, current_value);
        if !validation.is_valid {
            return Err(validation
                .error_message
                .unwrap_or_else(|| format!("Validation failed for metric {}", metric_name)));
        }

        // Add any warnings
        self.warnings.extend(validation.warnings);

        // Map to appropriate fields
        match metric_name {
            "NumberOfMessagesSent" => {
                metric_data.number_of_messages_sent = current_value;
                metric_data.messages_sent_history = sanitized_values;
            }
            "NumberOfMessagesReceived" => {
                metric_data.number_of_messages_received = current_value;
                metric_data.messages_received_history = sanitized_values;
            }
            "NumberOfMessagesDeleted" => {
                metric_data.number_of_messages_deleted = current_value;
                metric_data.messages_deleted_history = sanitized_values;
            }
            "ApproximateNumberOfMessages" => {
                metric_data.approximate_number_of_messages = current_value;
                metric_data.queue_depth_history = sanitized_values;
            }
            "ApproximateNumberOfMessagesVisible" => {
                metric_data.approximate_number_of_messages_visible = current_value;
                metric_data.messages_visible_history = sanitized_values;
            }
            "ApproximateNumberOfMessagesNotVisible" => {
                metric_data.approximate_number_of_messages_not_visible = current_value;
                metric_data.messages_not_visible_history = sanitized_values;
            }
            "ApproximateAgeOfOldestMessage" => {
                metric_data.approximate_age_of_oldest_message = current_value;
                metric_data.oldest_message_age_history = sanitized_values;
            }
            "NumberOfEmptyReceives" => {
                metric_data.number_of_empty_receives = current_value;
                metric_data.empty_receives_history = sanitized_values;
            }
            "ApproximateNumberOfMessagesDelayed" => {
                metric_data.approximate_number_of_messages_delayed = current_value;
                metric_data.messages_delayed_history = sanitized_values;
            }
            "SentMessageSize" => {
                metric_data.sent_message_size = current_value;
                metric_data.sent_message_size_history = sanitized_values;
            }
            "NumberOfMessagesInDlq" => {
                metric_data.number_of_messages_in_dlq = current_value;
                metric_data.dlq_messages_history = sanitized_values;
            }
            // FIFO-specific metrics
            "ApproximateNumberOfGroupsWithInflightMessages" => {
                metric_data.approximate_number_of_groups_with_inflight_messages = current_value;
                metric_data.groups_with_inflight_messages_history = sanitized_values;
            }
            "NumberOfDeduplicatedSentMessages" => {
                metric_data.number_of_deduplicated_sent_messages = current_value;
                metric_data.deduplicated_sent_messages_history = sanitized_values;
            }
            _ => {
                return Err(format!("Unknown metric name: {}", metric_name));
            }
        }

        Ok(())
    }

    /// Get the current value from sanitized data (last valid value)
    fn get_current_value(&self, values: &[f64]) -> f64 {
        values
            .iter()
            .rev()
            .find(|&&v| v.is_finite())
            .copied()
            .unwrap_or(0.0)
    }

    /// Get all collected warnings
    pub fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.warnings)
    }
}

/// Standard SQS metrics available in CloudWatch
pub struct SqsMetricDefinitions;

impl SqsMetricDefinitions {
    /// Get all standard SQS metrics
    pub fn standard_metrics() -> Vec<(&'static str, &'static str)> {
        vec![
            ("NumberOfMessagesSent", "AWS/SQS"),
            ("NumberOfMessagesReceived", "AWS/SQS"),
            ("NumberOfMessagesDeleted", "AWS/SQS"),
            ("ApproximateNumberOfMessages", "AWS/SQS"),
            ("ApproximateNumberOfMessagesVisible", "AWS/SQS"),
            ("ApproximateNumberOfMessagesNotVisible", "AWS/SQS"),
            ("ApproximateAgeOfOldestMessage", "AWS/SQS"),
            ("NumberOfEmptyReceives", "AWS/SQS"),
            ("ApproximateNumberOfMessagesDelayed", "AWS/SQS"),
            ("SentMessageSize", "AWS/SQS"),
            ("NumberOfMessagesInDlq", "AWS/SQS"),
        ]
    }

    /// Get FIFO-specific metrics
    pub fn fifo_metrics() -> Vec<(&'static str, &'static str)> {
        vec![
            ("ApproximateNumberOfGroupsWithInflightMessages", "AWS/SQS"),
            ("NumberOfDeduplicatedSentMessages", "AWS/SQS"),
        ]
    }

    /// Get metric display name for UI
    pub fn get_display_name(metric_name: &str) -> &'static str {
        match metric_name {
            "NumberOfMessagesSent" => "Messages Sent",
            "NumberOfMessagesReceived" => "Messages Received",
            "NumberOfMessagesDeleted" => "Messages Deleted",
            "ApproximateNumberOfMessages" => "Queue Depth",
            "ApproximateNumberOfMessagesVisible" => "Visible Messages",
            "ApproximateNumberOfMessagesNotVisible" => "In-Flight Messages",
            "ApproximateAgeOfOldestMessage" => "Oldest Message Age",
            "NumberOfEmptyReceives" => "Empty Receives",
            "ApproximateNumberOfMessagesDelayed" => "Delayed Messages",
            "SentMessageSize" => "Message Size",
            "NumberOfMessagesInDlq" => "DLQ Messages",
            "ApproximateNumberOfGroupsWithInflightMessages" => "Groups In-Flight",
            "NumberOfDeduplicatedSentMessages" => "Deduplicated Sent",
            _ => "Unknown Metric",
        }
    }

    /// Get metric description for tooltips/help
    pub fn get_description(metric_name: &str) -> &'static str {
        match metric_name {
            "NumberOfMessagesSent" => "The number of messages sent to the queue",
            "NumberOfMessagesReceived" => "The number of messages received from the queue",
            "NumberOfMessagesDeleted" => "The number of messages deleted from the queue",
            "ApproximateNumberOfMessages" => {
                "The approximate total number of messages in the queue"
            }
            "ApproximateNumberOfMessagesVisible" => {
                "The approximate number of messages available for retrieval"
            }
            "ApproximateNumberOfMessagesNotVisible" => {
                "The approximate number of messages in flight"
            }
            "ApproximateAgeOfOldestMessage" => {
                "The approximate age of the oldest non-deleted message (seconds)"
            }
            "NumberOfEmptyReceives" => {
                "The number of ReceiveMessage API calls that did not return a message"
            }
            "ApproximateNumberOfMessagesDelayed" => "The approximate number of messages delayed",
            "SentMessageSize" => "The size of messages sent to the queue (bytes)",
            "NumberOfMessagesInDlq" => "The number of messages in the dead letter queue",
            "ApproximateNumberOfGroupsWithInflightMessages" => {
                "The approximate number of message groups with in-flight messages (FIFO only)"
            }
            "NumberOfDeduplicatedSentMessages" => {
                "The number of messages sent that were deduplicated (FIFO only)"
            }
            _ => "Unknown metric description",
        }
    }

    /// Get the unit for display formatting
    pub fn get_unit(metric_name: &str) -> &'static str {
        match metric_name {
            "NumberOfMessagesSent"
            | "NumberOfMessagesReceived"
            | "NumberOfMessagesDeleted"
            | "ApproximateNumberOfMessages"
            | "ApproximateNumberOfMessagesVisible"
            | "ApproximateNumberOfMessagesNotVisible"
            | "NumberOfEmptyReceives"
            | "ApproximateNumberOfMessagesDelayed"
            | "NumberOfMessagesInDlq"
            | "ApproximateNumberOfGroupsWithInflightMessages"
            | "NumberOfDeduplicatedSentMessages" => "Count",
            "ApproximateAgeOfOldestMessage" => "Seconds",
            "SentMessageSize" => "Bytes",
            _ => "Unknown",
        }
    }

    /// Check if metric is rate-based (per period) vs cumulative
    pub fn is_rate_metric(metric_name: &str) -> bool {
        matches!(
            metric_name,
            "NumberOfMessagesSent"
                | "NumberOfMessagesReceived"
                | "NumberOfMessagesDeleted"
                | "NumberOfEmptyReceives"
                | "NumberOfDeduplicatedSentMessages"
        )
    }

    /// Check if metric is a gauge (current state) vs counter
    pub fn is_gauge_metric(metric_name: &str) -> bool {
        matches!(
            metric_name,
            "ApproximateNumberOfMessages"
                | "ApproximateNumberOfMessagesVisible"
                | "ApproximateNumberOfMessagesNotVisible"
                | "ApproximateAgeOfOldestMessage"
                | "ApproximateNumberOfMessagesDelayed"
                | "SentMessageSize"
                | "NumberOfMessagesInDlq"
                | "ApproximateNumberOfGroupsWithInflightMessages"
        )
    }
}

/// Metric mapping result with collected data and warnings
#[derive(Debug)]
pub struct MappingResult {
    pub metric_data: SqsMetricData,
    pub warnings: Vec<String>,
    pub timestamps: Vec<SystemTime>,
}

impl MappingResult {
    pub fn new() -> Self {
        Self {
            metric_data: SqsMetricData::default(),
            warnings: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    pub fn with_timestamps(mut self, timestamps: Vec<SystemTime>) -> Self {
        self.timestamps = timestamps;
        self
    }

    pub fn add_warnings(&mut self, warnings: Vec<String>) {
        self.warnings.extend(warnings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_metric_mapper_basic() {
        let mut mapper = MetricMapper::new();
        let mut metric_data = SqsMetricData::default();

        let values = vec![10.0, 20.0, 30.0];
        let timestamps = create_test_timestamps(3);

        let result =
            mapper.map_metric(&mut metric_data, "NumberOfMessagesSent", values, timestamps);
        assert!(result.is_ok());
        assert_eq!(metric_data.number_of_messages_sent, 30.0);
        assert_eq!(metric_data.messages_sent_history, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_metric_mapper_invalid_metric() {
        let mut mapper = MetricMapper::new();
        let mut metric_data = SqsMetricData::default();

        let result = mapper.map_metric(
            &mut metric_data,
            "InvalidMetric",
            vec![1.0],
            create_test_timestamps(1),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown metric"));
    }

    #[test]
    fn test_sqs_metric_definitions() {
        let standard = SqsMetricDefinitions::standard_metrics();
        assert!(!standard.is_empty());
        assert!(standard
            .iter()
            .any(|(name, _)| *name == "NumberOfMessagesSent"));

        let fifo = SqsMetricDefinitions::fifo_metrics();
        assert!(!fifo.is_empty());
        assert!(fifo
            .iter()
            .any(|(name, _)| *name == "NumberOfDeduplicatedSentMessages"));
    }

    #[test]
    fn test_metric_display_names() {
        assert_eq!(
            SqsMetricDefinitions::get_display_name("NumberOfMessagesSent"),
            "Messages Sent"
        );
        assert_eq!(
            SqsMetricDefinitions::get_display_name("ApproximateAgeOfOldestMessage"),
            "Oldest Message Age"
        );
        assert_eq!(
            SqsMetricDefinitions::get_display_name("Unknown"),
            "Unknown Metric"
        );
    }

    #[test]
    fn test_metric_units() {
        assert_eq!(
            SqsMetricDefinitions::get_unit("NumberOfMessagesSent"),
            "Count"
        );
        assert_eq!(
            SqsMetricDefinitions::get_unit("ApproximateAgeOfOldestMessage"),
            "Seconds"
        );
        assert_eq!(SqsMetricDefinitions::get_unit("SentMessageSize"), "Bytes");
    }

    #[test]
    fn test_metric_types() {
        assert!(SqsMetricDefinitions::is_rate_metric("NumberOfMessagesSent"));
        assert!(!SqsMetricDefinitions::is_rate_metric(
            "ApproximateNumberOfMessages"
        ));

        assert!(SqsMetricDefinitions::is_gauge_metric(
            "ApproximateNumberOfMessages"
        ));
        assert!(!SqsMetricDefinitions::is_gauge_metric(
            "NumberOfMessagesSent"
        ));
    }

    fn create_test_timestamps(count: usize) -> Vec<SystemTime> {
        let now = SystemTime::now();
        (0..count)
            .map(|i| now - std::time::Duration::from_secs((count - 1 - i) as u64 * 60))
            .collect()
    }
}
