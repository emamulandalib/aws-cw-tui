use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use crate::models::RdsInstance;
use anyhow::Result;
use aws_sdk_rds::Client as RdsClient;

/// RDS client operations - centralized AWS RDS API calls
pub struct RdsClientManager {
    client: RdsClient,
}

impl RdsClientManager {
    /// Create a new RDS client manager using shared AWS session
    pub async fn new() -> Self {
        let client = AwsSessionManager::rds_client().await;
        Self { client }
    }

    /// Load all RDS instances from AWS
    pub async fn load_instances(&self) -> Result<Vec<RdsInstance>> {
        let resp: aws_sdk_rds::operation::describe_db_instances::DescribeDbInstancesOutput =
            match self.client.describe_db_instances().send().await {
                Ok(resp) => resp,
                Err(e) => {
                    return Err(AwsErrorHandler::handle_aws_error(
                        e,
                        "fetch RDS instances",
                        "RDS describe permissions",
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
}
