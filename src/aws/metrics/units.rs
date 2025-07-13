/// Determine the appropriate unit for an RDS metric based on its name
pub fn determine_metric_unit(metric_name: &str) -> Option<String> {
    match metric_name {
        "CPUUtilization" | "CPUCreditUsage" => Some("Percent".to_string()),
        "DatabaseConnections" | "ConnectionAttempts" => Some("Count".to_string()),
        "FreeStorageSpace"
        | "FreeableMemory"
        | "SwapUsage"
        | "BinLogDiskUsage"
        | "ReplicationSlotDiskUsage"
        | "TransactionLogsDiskUsage" => Some("Bytes".to_string()),
        "ReadIOPS" | "WriteIOPS" => Some("Count/Second".to_string()),
        "ReadLatency" | "WriteLatency" => Some("Seconds".to_string()),
        "ReadThroughput"
        | "WriteThroughput"
        | "NetworkReceiveThroughput"
        | "NetworkTransmitThroughput" => Some("Bytes/Second".to_string()),
        "QueueDepth" => Some("Count".to_string()),
        "BurstBalance" | "CPUCreditBalance" | "EbsByteBalance" | "EbsIOBalance" => {
            Some("Percent".to_string())
        }
        "CPUSurplusCreditBalance" | "CPUSurplusCreditsCharged" => Some("Count".to_string()),
        "ReplicaLag"
        | "OldestReplicationSlotLag"
        | "OldestLogicalReplicationSlotLag"
        | "CheckpointLag" => Some("Seconds".to_string()),
        "MaximumUsedTransactionIDs" | "FailedSQLServerAgentJobsCount" => Some("Count".to_string()),
        "TransactionLogsGeneration" => Some("Bytes/Second".to_string()),
        _ => None,
    }
}

/// Determine the appropriate unit for an SQS metric based on its name
pub fn determine_sqs_metric_unit(metric_name: &str) -> Option<String> {
    match metric_name {
        "NumberOfMessagesSent"
        | "NumberOfMessagesReceived"
        | "NumberOfMessagesDeleted"
        | "ApproximateNumberOfMessages"
        | "ApproximateNumberOfMessagesVisible"
        | "ApproximateNumberOfMessagesNotVisible"
        | "NumberOfEmptyReceives"
        | "ApproximateNumberOfMessagesDelayed"
        | "NumberOfMessagesInDLQ"
        | "ApproximateNumberOfGroupsWithInflightMessages"
        | "NumberOfDeduplicatedSentMessages" => Some("Count".to_string()),
        "ApproximateAgeOfOldestMessage" => Some("Seconds".to_string()),
        "SentMessageSize" => Some("Bytes".to_string()),
        _ => None,
    }
}

/// Parse CloudWatch unit string to AWS SDK unit enum
pub fn parse_cloudwatch_unit(unit_str: &str) -> Option<aws_sdk_cloudwatch::types::StandardUnit> {
    match unit_str {
        "Percent" => Some(aws_sdk_cloudwatch::types::StandardUnit::Percent),
        "Count" => Some(aws_sdk_cloudwatch::types::StandardUnit::Count),
        "Bytes" => Some(aws_sdk_cloudwatch::types::StandardUnit::Bytes),
        "Seconds" => Some(aws_sdk_cloudwatch::types::StandardUnit::Seconds),
        "Count/Second" => Some(aws_sdk_cloudwatch::types::StandardUnit::CountSecond),
        "Bytes/Second" => Some(aws_sdk_cloudwatch::types::StandardUnit::BytesSecond),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_metric_unit() {
        assert_eq!(
            determine_metric_unit("CPUUtilization"),
            Some("Percent".to_string())
        );
        assert_eq!(
            determine_metric_unit("DatabaseConnections"),
            Some("Count".to_string())
        );
        assert_eq!(
            determine_metric_unit("FreeStorageSpace"),
            Some("Bytes".to_string())
        );
        assert_eq!(
            determine_metric_unit("ReadLatency"),
            Some("Seconds".to_string())
        );
        assert_eq!(determine_metric_unit("UnknownMetric"), None);
    }

    #[test]
    fn test_determine_sqs_metric_unit() {
        assert_eq!(
            determine_sqs_metric_unit("NumberOfMessagesSent"),
            Some("Count".to_string())
        );
        assert_eq!(
            determine_sqs_metric_unit("ApproximateAgeOfOldestMessage"),
            Some("Seconds".to_string())
        );
        assert_eq!(
            determine_sqs_metric_unit("SentMessageSize"),
            Some("Bytes".to_string())
        );
        assert_eq!(determine_sqs_metric_unit("UnknownMetric"), None);
    }

    #[test]
    fn test_parse_cloudwatch_unit() {
        assert!(matches!(
            parse_cloudwatch_unit("Percent"),
            Some(aws_sdk_cloudwatch::types::StandardUnit::Percent)
        ));
        assert!(matches!(
            parse_cloudwatch_unit("Count"),
            Some(aws_sdk_cloudwatch::types::StandardUnit::Count)
        ));
        assert_eq!(parse_cloudwatch_unit("InvalidUnit"), None);
    }
}
