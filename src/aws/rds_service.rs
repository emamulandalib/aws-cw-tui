use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_rds::Client as RdsClient;
use crate::models::RdsInstance;
use crate::aws::cloudwatch_service::{TimeRange, TimeUnit};

pub async fn load_rds_instances() -> Result<Vec<RdsInstance>> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = RdsClient::new(&config);

    let resp = client.describe_db_instances().send().await?;
    
    let mut instances = Vec::new();
    
    if let Some(db_instances) = resp.db_instances {
        for instance in db_instances {
            let rds_instance = RdsInstance {
                identifier: instance.db_instance_identifier.unwrap_or_default(),
                engine: instance.engine.unwrap_or_default(),
                status: instance.db_instance_status.unwrap_or_default(),
                instance_class: instance.db_instance_class.unwrap_or_default(),
                endpoint: instance.endpoint.and_then(|e| e.address),
                time_range: TimeRange::new(3, TimeUnit::Hours, 1).unwrap(),
            };
            instances.push(rds_instance);
        }
    }
    
    Ok(instances)
}
