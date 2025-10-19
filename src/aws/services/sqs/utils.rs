use crate::aws::cloudwatch_service::TimeRange;
use crate::models::SqsQueue;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Calculate time range for metric queries
pub fn calculate_time_range(time_range: &TimeRange) -> (SystemTime, SystemTime) {
    let end_time = SystemTime::now();
    let start_time = end_time - time_range.duration();
    (start_time, end_time)
}

/// Extract DLQ name from queue configuration
pub fn get_dlq_name(queue: &SqsQueue) -> Option<String> {
    // Parse the redrive policy to get DLQ ARN
    if let Some(redrive_policy) = queue.attributes.get("RedrivePolicy") {
        // Simple JSON parsing without serde_json dependency
        // Look for deadLetterTargetArn in the JSON string
        if let Some(start) = redrive_policy.find("deadLetterTargetArn") {
            if let Some(colon_pos) = redrive_policy[start..].find(':') {
                let after_colon = &redrive_policy[start + colon_pos + 1..];
                if let Some(quote_start) = after_colon.find('"') {
                    let after_quote = &after_colon[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        let arn = &after_quote[..quote_end];
                        // Extract queue name from ARN (last part after the last colon)
                        return arn.split(':').last().map(|s| s.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Convert SystemTime to Unix timestamp for AWS API
pub fn system_time_to_unix(time: SystemTime) -> f64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs_f64()
}

/// Validate queue name for CloudWatch metrics
pub fn validate_queue_name(queue_name: &str) -> Result<(), String> {
    if queue_name.is_empty() {
        return Err("Queue name cannot be empty".to_string());
    }

    if queue_name.len() > 80 {
        return Err("Queue name too long for CloudWatch metrics".to_string());
    }

    Ok(())
}

/// Extract queue name from SQS URL
pub fn extract_queue_name_from_url(url: &str) -> String {
    url.split('/').last().unwrap_or("unknown").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::time_range::TimeUnit;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_time_range() {
        let time_range = TimeRange::new(1, TimeUnit::Hours, 1).unwrap();
        let (start, end) = calculate_time_range(&time_range);
        let duration = end.duration_since(start).unwrap();
        assert_eq!(duration.as_secs(), 3600);
    }

    #[test]
    fn test_get_dlq_name() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "RedrivePolicy".to_string(),
            r#"{"deadLetterTargetArn":"arn:aws:sqs:us-east-1:123456789012:my-dlq","maxReceiveCount":"3"}"#.to_string(),
        );

        let queue = SqsQueue {
            url: "https://sqs.us-east-1.amazonaws.com/123456789012/test-queue".to_string(),
            name: "test-queue".to_string(),
            queue_type: "Standard".to_string(),
            attributes,
        };

        assert_eq!(get_dlq_name(&queue), Some("my-dlq".to_string()));
    }

    #[test]
    fn test_validate_queue_name() {
        assert!(validate_queue_name("valid-queue-name").is_ok());
        assert!(validate_queue_name("").is_err());
        assert!(validate_queue_name(&"a".repeat(100)).is_err());
    }

    #[test]
    fn test_extract_queue_name_from_url() {
        let url = "https://sqs.us-east-1.amazonaws.com/123456789012/my-test-queue";
        assert_eq!(extract_queue_name_from_url(url), "my-test-queue");
    }

    #[test]
    fn test_system_time_to_unix() {
        let time = UNIX_EPOCH + Duration::from_secs(1000);
        assert_eq!(system_time_to_unix(time), 1000.0);
    }
}
