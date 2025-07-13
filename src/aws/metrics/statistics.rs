/// Determine the best statistic for an RDS metric based on its type
pub fn determine_best_statistic(metric_name: &str) -> String {
    match metric_name {
        // Utilization metrics - use Average
        "CPUUtilization" | "CPUCreditUsage" | "BurstBalance" | "CPUCreditBalance"
        | "EbsByteBalance" | "EbsIOBalance" => "Average".to_string(),

        // Connection counts - use Average
        "DatabaseConnections" | "ConnectionAttempts" => "Average".to_string(),

        // Storage and memory - use Average
        "FreeStorageSpace"
        | "FreeableMemory"
        | "SwapUsage"
        | "BinLogDiskUsage"
        | "ReplicationSlotDiskUsage"
        | "TransactionLogsDiskUsage" => "Average".to_string(),

        // IOPS and throughput - use Average
        "ReadIOPS"
        | "WriteIOPS"
        | "ReadThroughput"
        | "WriteThroughput"
        | "NetworkReceiveThroughput"
        | "NetworkTransmitThroughput"
        | "TransactionLogsGeneration" => "Average".to_string(),

        // Latency metrics - use Average
        "ReadLatency"
        | "WriteLatency"
        | "ReplicaLag"
        | "OldestReplicationSlotLag"
        | "OldestLogicalReplicationSlotLag"
        | "CheckpointLag" => "Average".to_string(),

        // Queue depth - use Average
        "QueueDepth" => "Average".to_string(),

        // Credit balances - use Average
        "CPUSurplusCreditBalance" | "CPUSurplusCreditsCharged" => "Average".to_string(),

        // Count metrics - use Maximum for better visibility of peaks
        "MaximumUsedTransactionIDs" | "FailedSQLServerAgentJobsCount" => "Maximum".to_string(),

        // Default to Average for unknown metrics
        _ => "Average".to_string(),
    }
}

/// Determine the best statistic for an SQS metric
pub fn determine_sqs_statistic(metric_name: &str) -> String {
    match metric_name {
        // Message counts - use Sum for total activity
        "NumberOfMessagesSent"
        | "NumberOfMessagesReceived"
        | "NumberOfMessagesDeleted"
        | "NumberOfEmptyReceives"
        | "NumberOfDeduplicatedSentMessages" => "Sum".to_string(),

        // Queue depth metrics - use Average for typical levels
        "ApproximateNumberOfMessages"
        | "ApproximateNumberOfMessagesVisible"
        | "ApproximateNumberOfMessagesNotVisible"
        | "ApproximateNumberOfMessagesDelayed"
        | "NumberOfMessagesInDLQ"
        | "ApproximateNumberOfGroupsWithInflightMessages" => "Average".to_string(),

        // Age metrics - use Maximum to see worst case
        "ApproximateAgeOfOldestMessage" => "Maximum".to_string(),

        // Size metrics - use Average
        "SentMessageSize" => "Average".to_string(),

        // Default to Average for unknown metrics
        _ => "Average".to_string(),
    }
}

/// Extract the appropriate statistic value from a CloudWatch datapoint
pub fn get_statistic_value(
    datapoint: &aws_sdk_cloudwatch::types::Datapoint,
    statistic: &str,
) -> Option<f64> {
    match statistic {
        "Average" => datapoint.average,
        "Sum" => datapoint.sum,
        "Maximum" => datapoint.maximum,
        "Minimum" => datapoint.minimum,
        "SampleCount" => datapoint.sample_count,
        _ => datapoint.average, // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_best_statistic() {
        assert_eq!(determine_best_statistic("CPUUtilization"), "Average");
        assert_eq!(determine_best_statistic("ReadIOPS"), "Average");
        assert_eq!(
            determine_best_statistic("MaximumUsedTransactionIDs"),
            "Maximum"
        );
        assert_eq!(determine_best_statistic("UnknownMetric"), "Average");
    }

    #[test]
    fn test_determine_sqs_statistic() {
        assert_eq!(determine_sqs_statistic("NumberOfMessagesSent"), "Sum");
        assert_eq!(
            determine_sqs_statistic("ApproximateNumberOfMessages"),
            "Average"
        );
        assert_eq!(
            determine_sqs_statistic("ApproximateAgeOfOldestMessage"),
            "Maximum"
        );
        assert_eq!(determine_sqs_statistic("UnknownMetric"), "Average");
    }

    #[test]
    fn test_get_statistic_value() {
        use aws_sdk_cloudwatch::types::Datapoint;

        let datapoint = Datapoint::builder()
            .average(50.0)
            .sum(100.0)
            .maximum(75.0)
            .minimum(25.0)
            .sample_count(10.0)
            .build();

        assert_eq!(get_statistic_value(&datapoint, "Average"), Some(50.0));
        assert_eq!(get_statistic_value(&datapoint, "Sum"), Some(100.0));
        assert_eq!(get_statistic_value(&datapoint, "Maximum"), Some(75.0));
        assert_eq!(get_statistic_value(&datapoint, "Minimum"), Some(25.0));
        assert_eq!(get_statistic_value(&datapoint, "SampleCount"), Some(10.0));
        assert_eq!(get_statistic_value(&datapoint, "Unknown"), Some(50.0)); // Falls back to average
    }
}
