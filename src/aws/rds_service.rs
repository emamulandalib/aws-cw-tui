use crate::models::RdsInstance;
use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_rds::Client as RdsClient;

pub async fn load_rds_instances() -> Result<Vec<RdsInstance>> {
    // Load AWS config - this doesn't return a Result, just loads the config
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = RdsClient::new(&config);

    let resp = match client.describe_db_instances().send().await {
        Ok(resp) => resp,
        Err(e) => {
            // Check if it's a credential error
            let error_msg = e.to_string();
            if error_msg.contains("credential")
                || error_msg.contains("authentication")
                || error_msg.contains("access")
                || error_msg.contains("no providers in chain")
            {
                return Err(anyhow::anyhow!(
                    "AWS credentials error: {}\n\n\
                     Please ensure:\n\
                     - Your AWS credentials are configured correctly\n\
                     - You have the correct AWS_PROFILE set (currently: {})\n\
                     - Your credentials have RDS describe permissions\n\
                     - Try: export AWS_PROFILE=your-profile-name",
                    error_msg,
                    std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string())
                ));
            } else {
                return Err(anyhow::anyhow!("Failed to fetch RDS instances: {}", e));
            }
        }
    };

    let mut instances = Vec::new();

    if let Some(db_instances) = resp.db_instances {
        for instance in db_instances {
            let rds_instance = RdsInstance {
                identifier: instance.db_instance_identifier.unwrap_or_default(),
                engine: instance.engine.unwrap_or_default(),
                status: instance.db_instance_status.unwrap_or_default(),
                instance_class: instance.db_instance_class.unwrap_or_default(),
                endpoint: instance.endpoint.and_then(|e| e.address),
            };
            instances.push(rds_instance);
        }
    }

    Ok(instances)
}
