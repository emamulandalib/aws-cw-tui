use crate::models::RdsInstance;
use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use anyhow::Result;

pub async fn load_rds_instances() -> Result<Vec<RdsInstance>> {
    // Use shared AWS session manager for RDS client
    let client = AwsSessionManager::rds_client().await;

    let resp = match client.describe_db_instances().send().await {
        Ok(resp) => resp,
        Err(e) => {
            return Err(AwsErrorHandler::handle_aws_error(
                e,
                "fetch RDS instances",
                "RDS describe permissions"
            ));
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
