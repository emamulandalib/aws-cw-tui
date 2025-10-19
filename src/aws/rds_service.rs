use crate::aws::error_utils::AwsErrorHandler;
use crate::aws::session::AwsSessionManager;
use crate::models::RdsInstance;
use anyhow::Result;
use log::{debug, error, info, warn};

pub async fn load_rds_instances() -> Result<Vec<RdsInstance>> {
    debug!("Starting RDS instance loading via AWS SDK...");

    // Use shared AWS session manager for RDS client
    let client = AwsSessionManager::rds_client().await;
    debug!("RDS client obtained from session manager");

    debug!("Sending describe_db_instances API call...");
    let resp = match client.describe_db_instances().send().await {
        Ok(resp) => {
            debug!("Successfully received RDS API response");
            resp
        }
        Err(e) => {
            error!("RDS API call failed: {:?}", e);
            return Err(AwsErrorHandler::handle_aws_error(
                e,
                "fetch RDS instances",
                "RDS describe permissions",
            ));
        }
    };

    let mut instances = Vec::new();

    if let Some(db_instances) = resp.db_instances {
        info!(
            "Processing {} RDS instances from API response",
            db_instances.len()
        );

        for (index, instance) in db_instances.iter().enumerate() {
            debug!(
                "Processing RDS instance {}: {:?}",
                index + 1,
                instance.db_instance_identifier
            );
            let rds_instance = RdsInstance {
                id: instance.db_instance_identifier.clone().unwrap_or_default(),
                name: instance.db_instance_identifier.clone().unwrap_or_default(),
                identifier: instance.db_instance_identifier.clone().unwrap_or_default(),
                engine: instance.engine.clone().unwrap_or_default(),
                status: instance.db_instance_status.clone().unwrap_or_default(),
                instance_class: instance.db_instance_class.clone().unwrap_or_default(),
                endpoint: instance.endpoint.as_ref().and_then(|e| e.address.clone()),
                port: instance.endpoint.as_ref().and_then(|e| e.port),
                engine_version: instance.engine_version.clone(),
                allocated_storage: instance.allocated_storage,
                storage_type: instance.storage_type.clone(),
                availability_zone: instance.availability_zone.clone(),
                backup_retention_period: instance.backup_retention_period,
                multi_az: instance.multi_az,
                storage_encrypted: instance.storage_encrypted,
                performance_insights_enabled: instance.performance_insights_enabled,
                deletion_protection: instance.deletion_protection,
                creation_time: instance.instance_create_time.map(|t| std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(t.secs() as u64)),
            };
            debug!("Created RDS instance: {:?}", rds_instance);
            instances.push(rds_instance);
        }
    } else {
        warn!("No RDS instances found in API response");
    }

    info!("Successfully loaded {} RDS instances", instances.len());
    debug!("Final RDS instances: {:?}", instances);
    Ok(instances)
}
