/// Format metric name into a human-readable display name
pub fn format_metric_display_name(metric_name: &str) -> String {
    match metric_name {
        // RDS Metrics - Enhanced display names
        "CPUUtilization" => "CPU Utilization".to_string(),
        "CPUCreditUsage" => "CPU Credit Usage".to_string(),
        "CPUCreditBalance" => "CPU Credit Balance".to_string(),
        "CPUSurplusCreditBalance" => "CPU Surplus Credit Balance".to_string(),
        "CPUSurplusCreditsCharged" => "CPU Surplus Credits Charged".to_string(),
        "DatabaseConnections" => "Database Connections".to_string(),
        "ConnectionAttempts" => "Connection Attempts".to_string(),
        "FreeStorageSpace" => "Free Storage Space".to_string(),
        "FreeableMemory" => "Freeable Memory".to_string(),
        "SwapUsage" => "Swap Usage".to_string(),
        "ReadIOPS" => "Read IOPS".to_string(),
        "WriteIOPS" => "Write IOPS".to_string(),
        "ReadLatency" => "Read Latency".to_string(),
        "WriteLatency" => "Write Latency".to_string(),
        "ReadThroughput" => "Read Throughput".to_string(),
        "WriteThroughput" => "Write Throughput".to_string(),
        "NetworkReceiveThroughput" => "Network Receive Throughput".to_string(),
        "NetworkTransmitThroughput" => "Network Transmit Throughput".to_string(),
        "QueueDepth" => "Queue Depth".to_string(),
        "BurstBalance" => "Burst Balance".to_string(),
        "EbsByteBalance" => "EBS Byte Balance".to_string(),
        "EbsIOBalance" => "EBS IO Balance".to_string(),
        "BinLogDiskUsage" => "Binary Log Disk Usage".to_string(),
        "ReplicaLag" => "Replica Lag".to_string(),
        "MaximumUsedTransactionIDs" => "Maximum Used Transaction IDs".to_string(),
        "OldestReplicationSlotLag" => "Oldest Replication Slot Lag".to_string(),
        "OldestLogicalReplicationSlotLag" => "Oldest Logical Replication Slot Lag".to_string(),
        "ReplicationSlotDiskUsage" => "Replication Slot Disk Usage".to_string(),
        "TransactionLogsDiskUsage" => "Transaction Logs Disk Usage".to_string(),
        "TransactionLogsGeneration" => "Transaction Logs Generation".to_string(),
        "FailedSQLServerAgentJobsCount" => "Failed SQL Server Agent Jobs".to_string(),
        "CheckpointLag" => "Checkpoint Lag".to_string(),

        // SQS Metrics - Enhanced display names
        "NumberOfMessagesSent" => "Messages Sent".to_string(),
        "NumberOfMessagesReceived" => "Messages Received".to_string(),
        "NumberOfMessagesDeleted" => "Messages Deleted".to_string(),
        "ApproximateNumberOfMessages" => "Approximate Number of Messages".to_string(),
        "ApproximateNumberOfMessagesVisible" => "Approximate Visible Messages".to_string(),
        "ApproximateNumberOfMessagesNotVisible" => "Approximate Not Visible Messages".to_string(),
        "ApproximateAgeOfOldestMessage" => "Age of Oldest Message".to_string(),
        "NumberOfEmptyReceives" => "Empty Receives".to_string(),
        "ApproximateNumberOfMessagesDelayed" => "Approximate Delayed Messages".to_string(),
        "SentMessageSize" => "Sent Message Size".to_string(),
        "NumberOfMessagesInDLQ" => "Messages in DLQ".to_string(),
        "ApproximateNumberOfGroupsWithInflightMessages" => {
            "Groups with Inflight Messages".to_string()
        }
        "NumberOfDeduplicatedSentMessages" => "Deduplicated Sent Messages".to_string(),

        // Default: Convert CamelCase to Title Case with spaces
        _ => camel_case_to_title_case(metric_name),
    }
}

/// Convert CamelCase string to Title Case with spaces
fn camel_case_to_title_case(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_uppercase() && !result.is_empty() {
            // Add space before uppercase letter (except at start)
            result.push(' ');
        }
        result.push(ch);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_metric_display_name() {
        assert_eq!(
            format_metric_display_name("CPUUtilization"),
            "CPU Utilization"
        );
        assert_eq!(
            format_metric_display_name("DatabaseConnections"),
            "Database Connections"
        );
        assert_eq!(
            format_metric_display_name("NumberOfMessagesSent"),
            "Messages Sent"
        );
        assert_eq!(
            format_metric_display_name("ApproximateAgeOfOldestMessage"),
            "Age of Oldest Message"
        );
    }

    #[test]
    fn test_camel_case_to_title_case() {
        assert_eq!(
            camel_case_to_title_case("CPUUtilization"),
            "C P U Utilization"
        );
        assert_eq!(
            camel_case_to_title_case("DatabaseConnections"),
            "Database Connections"
        );
        assert_eq!(
            camel_case_to_title_case("UnknownMetricName"),
            "Unknown Metric Name"
        );
        assert_eq!(camel_case_to_title_case("singleword"), "singleword");
        assert_eq!(camel_case_to_title_case(""), "");
    }

    #[test]
    fn test_sqs_metric_names() {
        assert_eq!(
            format_metric_display_name("NumberOfMessagesReceived"),
            "Messages Received"
        );
        assert_eq!(
            format_metric_display_name("ApproximateNumberOfMessages"),
            "Approximate Number of Messages"
        );
        assert_eq!(
            format_metric_display_name("NumberOfEmptyReceives"),
            "Empty Receives"
        );
    }

    #[test]
    fn test_rds_metric_names() {
        assert_eq!(format_metric_display_name("ReadIOPS"), "Read IOPS");
        assert_eq!(format_metric_display_name("WriteLatency"), "Write Latency");
        assert_eq!(
            format_metric_display_name("BinLogDiskUsage"),
            "Binary Log Disk Usage"
        );
        assert_eq!(format_metric_display_name("ReplicaLag"), "Replica Lag");
    }
}
