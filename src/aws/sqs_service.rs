use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use crate::models::SqsQueue;
use anyhow::Result;
use std::collections::HashMap;

pub async fn load_sqs_queues() -> Result<Vec<SqsQueue>> {
    // Use shared AWS session manager for SQS client
    let client = AwsSessionManager::sqs_client().await;

    let resp = match client.list_queues().send().await {
        Ok(resp) => resp,
        Err(e) => {
            return Err(AwsErrorHandler::handle_aws_error(
                e,
                "fetch SQS queues",
                "SQS list permissions",
            ));
        }
    };

    let mut queues = Vec::new();

    if let Some(queue_urls) = resp.queue_urls {
        for url in queue_urls {
            // Extract queue name from URL
            let name = url.split('/').next_back().unwrap_or("unknown").to_string();

            // Determine queue type based on name (.fifo suffix)
            let queue_type = if name.ends_with(".fifo") {
                "FIFO".to_string()
            } else {
                "Standard".to_string()
            };

            // Get queue attributes for additional info
            let mut attributes = HashMap::new();
            if let Ok(attr_resp) = client
                .get_queue_attributes()
                .queue_url(&url)
                .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
                .send()
                .await
            {
                if let Some(attrs) = attr_resp.attributes {
                    for (key, value) in attrs {
                        attributes.insert(key.as_str().to_string(), value);
                    }
                }
            }

            let sqs_queue = SqsQueue {
                url,
                name,
                queue_type,
                attributes,
            };
            queues.push(sqs_queue);
        }
    }

    Ok(queues)
}
