use crate::aws::cloudwatch_service::TimeRange;
use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use crate::models::{SqsMetricData, SqsQueue};
use anyhow::Result;
use aws_sdk_cloudwatch::types::{Dimension, MetricDataQuery, MetricStat, Statistic};
use aws_sdk_cloudwatch::primitives::DateTime;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub async fn fetch_sqs_metrics(queue: &SqsQueue, time_range: &TimeRange) -> Result<SqsMetricData> {
    let client = AwsSessionManager::cloudwatch_client().await;
    
    // Extract queue name from URL for CloudWatch metrics
    let queue_name = &queue.name;
    

    
    // Calculate time range
    let (start_time, end_time) = calculate_time_range(time_range);
    

    

    
    // Create dimension for SQS queue
    let dimension = Dimension::builder()
        .name("QueueName")
        .value(queue_name)
        .build();

    // Define SQS metrics to fetch (10 standard AWS metrics)
    let metrics_to_fetch = vec![
        ("NumberOfMessagesSent", "AWS/SQS"),
        ("NumberOfMessagesReceived", "AWS/SQS"), 
        ("NumberOfMessagesDeleted", "AWS/SQS"),
        ("ApproximateNumberOfMessages", "AWS/SQS"),
        ("ApproximateNumberOfMessagesVisible", "AWS/SQS"), // Added missing metric
        ("ApproximateNumberOfMessagesNotVisible", "AWS/SQS"),
        ("ApproximateAgeOfOldestMessage", "AWS/SQS"),
        ("NumberOfEmptyReceives", "AWS/SQS"),
        ("ApproximateNumberOfMessagesDelayed", "AWS/SQS"),
        ("SentMessageSize", "AWS/SQS"), // Added missing metric
    ];

    // FIFO-specific metrics (only fetch for FIFO queues)
    let fifo_metrics = if queue.queue_type == "FIFO" {
        vec![
            ("ApproximateNumberOfGroupsWithInflightMessages", "AWS/SQS"),
            ("NumberOfDeduplicatedSentMessages", "AWS/SQS"),
        ]
    } else {
        vec![]
    };

    let mut metric_data = SqsMetricData::default();
    let mut timestamps = Vec::new();

    // Fetch standard metrics
    for (metric_name, namespace) in metrics_to_fetch {
        match fetch_single_metric(
            &client,
            namespace,
            metric_name,
            &dimension,
            start_time,
            end_time,
        )
        .await
        {
            Ok((values, times)) => {
                if timestamps.is_empty() {
                    timestamps = times;
                }
                
                // Update current value (latest) and history
                let current_value = values.last().copied().unwrap_or(0.0);
                
                match metric_name {
                    "NumberOfMessagesSent" => {
                        metric_data.number_of_messages_sent = current_value;
                        metric_data.messages_sent_history = values;
                    }
                    "NumberOfMessagesReceived" => {
                        metric_data.number_of_messages_received = current_value;
                        metric_data.messages_received_history = values;
                    }
                    "NumberOfMessagesDeleted" => {
                        metric_data.number_of_messages_deleted = current_value;
                        metric_data.messages_deleted_history = values;
                    }
                    "ApproximateNumberOfMessages" => {
                        metric_data.approximate_number_of_messages = current_value;
                        metric_data.queue_depth_history = values;
                    }
                    "ApproximateNumberOfMessagesVisible" => {
                        metric_data.approximate_number_of_messages_visible = current_value;
                        metric_data.messages_visible_history = values;
                    }
                    "ApproximateNumberOfMessagesNotVisible" => {
                        metric_data.approximate_number_of_messages_not_visible = current_value;
                        metric_data.messages_not_visible_history = values;
                    }
                    "ApproximateAgeOfOldestMessage" => {
                        metric_data.approximate_age_of_oldest_message = current_value;
                        metric_data.oldest_message_age_history = values;
                    }
                    "NumberOfEmptyReceives" => {
                        metric_data.number_of_empty_receives = current_value;
                        metric_data.empty_receives_history = values;
                    }
                    "ApproximateNumberOfMessagesDelayed" => {
                        metric_data.approximate_number_of_messages_delayed = current_value;
                        metric_data.messages_delayed_history = values;
                    }
                    "SentMessageSize" => {
                        metric_data.sent_message_size = current_value;
                        metric_data.sent_message_size_history = values;
                    }
                    _ => {}
                }
            }
            Err(_e) => {
                // Log error but continue with other metrics
                // Note: Silently continue to avoid cluttering TUI output
            }
        }
    }

    // Fetch FIFO-specific metrics
    for (metric_name, namespace) in fifo_metrics {
        match fetch_single_metric(
            &client,
            namespace,
            metric_name,
            &dimension,
            start_time,
            end_time,
        )
        .await
        {
            Ok((values, _)) => {
                let current_value = values.last().copied().unwrap_or(0.0);
                
                match metric_name {
                    "ApproximateNumberOfGroupsWithInflightMessages" => {
                        metric_data.approximate_number_of_groups_with_inflight_messages = current_value;
                        metric_data.groups_with_inflight_messages_history = values;
                    }
                    "NumberOfDeduplicatedSentMessages" => {
                        metric_data.number_of_deduplicated_sent_messages = current_value;
                        metric_data.deduplicated_sent_messages_history = values;
                    }
                    _ => {}
                }
            }
            Err(_e) => {
                // Note: Silently continue to avoid cluttering TUI output
            }
        }
    }

    // Check for DLQ metrics if this queue has a DLQ configured
    if let Some(dlq_name) = get_dlq_name(queue) {
        let dlq_dimension = Dimension::builder()
            .name("QueueName")
            .value(&dlq_name)
            .build();

        if let Ok((values, _)) = fetch_single_metric(
            &client,
            "AWS/SQS",
            "ApproximateNumberOfMessages",
            &dlq_dimension,
            start_time,
            end_time,
        )
        .await
        {
            metric_data.number_of_messages_in_dlq = values.last().copied().unwrap_or(0.0);
            metric_data.dlq_messages_history = values;
        }
    }

    metric_data.timestamps = timestamps;
    Ok(metric_data)
}

async fn fetch_single_metric(
    client: &aws_sdk_cloudwatch::Client,
    namespace: &str,
    metric_name: &str,
    dimension: &Dimension,
    start_time: SystemTime,
    end_time: SystemTime,
) -> Result<(Vec<f64>, Vec<SystemTime>)> {
    let start_time_dt = DateTime::from(start_time);
    let end_time_dt = DateTime::from(end_time);

    // Choose appropriate statistic based on metric type
    let statistic = match metric_name {
        // Count-based metrics use Sum
        "NumberOfMessagesSent" | 
        "NumberOfMessagesReceived" | 
        "NumberOfMessagesDeleted" |
        "NumberOfEmptyReceives" |
        "NumberOfMessagesInDlq" |
        "NumberOfDeduplicatedSentMessages" => Statistic::Sum,
        
        // Size metrics use Average
        "SentMessageSize" => Statistic::Average,
        
        // Approximate metrics use Average (they're already aggregated)
        "ApproximateNumberOfMessages" |
        "ApproximateNumberOfMessagesVisible" |
        "ApproximateNumberOfMessagesNotVisible" |
        "ApproximateAgeOfOldestMessage" |
        "ApproximateNumberOfMessagesDelayed" |
        "ApproximateNumberOfGroupsWithInflightMessages" => Statistic::Average,
        
        // Default to Average for unknown metrics
        _ => Statistic::Average,
    };



    let metric_stat = MetricStat::builder()
        .metric(
            aws_sdk_cloudwatch::types::Metric::builder()
                .namespace(namespace)
                .metric_name(metric_name)
                .dimensions(dimension.clone())
                .build(),
        )
        .period(300) // 5 minutes
        .stat(statistic.as_str())
        .build();

    let query = MetricDataQuery::builder()
        .id("m1")
        .metric_stat(metric_stat)
        .return_data(true)
        .build();

    let response = client
        .get_metric_data()
        .start_time(start_time_dt)
        .end_time(end_time_dt)
        .metric_data_queries(query)
        .send()
        .await
        .map_err(|e| {
            AwsErrorHandler::handle_aws_error(
                e,
                &format!("fetch CloudWatch metric {}", metric_name),
                "CloudWatch read permissions",
            )
        })?;

    let mut values = Vec::new();
    let mut timestamps = Vec::new();

    if let Some(results) = response.metric_data_results {
        if let Some(result) = results.first() {
            
            if let (Some(vals), Some(times)) = (&result.values, &result.timestamps) {
                for (value, timestamp) in vals.iter().zip(times.iter()) {
                    values.push(*value);
                    timestamps.push(
                        UNIX_EPOCH + Duration::from_secs(timestamp.secs() as u64)
                    );
                }

            }
        }
    }

    Ok((values, timestamps))
}

fn calculate_time_range(time_range: &TimeRange) -> (SystemTime, SystemTime) {
    let end_time = SystemTime::now();
    let duration = match time_range.unit {
        crate::aws::time_range::TimeUnit::Minutes => Duration::from_secs(time_range.value as u64 * 60),
        crate::aws::time_range::TimeUnit::Hours => Duration::from_secs(time_range.value as u64 * 3600),
        crate::aws::time_range::TimeUnit::Days => Duration::from_secs(time_range.value as u64 * 24 * 3600),
        crate::aws::time_range::TimeUnit::Weeks => Duration::from_secs(time_range.value as u64 * 7 * 24 * 3600),
        crate::aws::time_range::TimeUnit::Months => Duration::from_secs(time_range.value as u64 * 30 * 24 * 3600),
    };
    let start_time = end_time - duration;
    (start_time, end_time)
}

fn get_dlq_name(queue: &SqsQueue) -> Option<String> {
    // Check if queue has a redrive policy that indicates a DLQ
    queue.attributes.get("RedrivePolicy")
        .and_then(|policy| {
            // Parse the JSON to extract DLQ ARN
            // This is a simplified approach - in production you'd want proper JSON parsing
            if policy.contains("deadLetterTargetArn") {
                // Extract DLQ name from ARN in the redrive policy
                // Example: "arn:aws:sqs:region:account:queue-name-dlq"
                let parts: Vec<&str> = policy.split("deadLetterTargetArn").collect();
                if parts.len() > 1 {
                    let arn_part = parts[1];
                    let start = arn_part.find(':')?;
                    let end = arn_part.find('"')?;
                    if start < end {
                        let arn = &arn_part[start+1..end];
                        return arn.split(':').last().map(|s| s.to_string());
                    }
                }
            }
            None
        })
}