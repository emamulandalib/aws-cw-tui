use super::client::RdsClientManager;
use crate::models::RdsInstance;
use anyhow::Result;

/// RDS instance management operations
pub struct RdsInstanceManager;

impl RdsInstanceManager {
    /// Load RDS instances using the centralized client
    pub async fn load_instances() -> Result<Vec<RdsInstance>> {
        let client_manager = RdsClientManager::new().await;
        client_manager.load_instances().await
    }

    /// Get RDS-specific metrics list
    pub fn available_metrics() -> Vec<&'static str> {
        vec![
            "CPUUtilization",
            "DatabaseConnections",
            "FreeableMemory",
            "ReadLatency",
            "WriteLatency",
            "ReadIOPS",
            "WriteIOPS",
            "ReadThroughput",
            "WriteThroughput",
            "FreeStorageSpace",
            "NetworkReceiveThroughput",
            "NetworkTransmitThroughput",
            "ReplicaLag",
            "BurstBalance",
            "DBLoad",
            "DBLoadCPU",
            "DBLoadNonCPU",
            "EngineUptime",
            "RDSToAuroraPostgreSQLReplicaLag",
            "AuroraReplicaLag",
            "AuroraReplicaLagMinimum",
            "AuroraReplicaLagMaximum",
            "CheckpointLag",
            "TransactionLogsDiskUsage",
            "TransactionLogsGeneration",
            "OldestReplicationSlotLag",
            "MaximumUsedTransactionIDs",
        ]
    }

    /// Get metric unit for RDS metrics
    pub fn get_metric_unit(metric_name: &str) -> &'static str {
        match metric_name {
            "CPUUtilization" => "Percent",
            "DatabaseConnections" => "Count",
            "FreeableMemory" => "Bytes",
            "ReadLatency" | "WriteLatency" => "Seconds",
            "ReadIOPS" | "WriteIOPS" => "Count/Second",
            "ReadThroughput" | "WriteThroughput" => "Bytes/Second",
            "FreeStorageSpace" => "Bytes",
            "NetworkReceiveThroughput" | "NetworkTransmitThroughput" => "Bytes/Second",
            "ReplicaLag" => "Seconds",
            "BurstBalance" => "Percent",
            "DBLoad" | "DBLoadCPU" | "DBLoadNonCPU" => "Count",
            "EngineUptime" => "Seconds",
            "RDSToAuroraPostgreSQLReplicaLag"
            | "AuroraReplicaLag"
            | "AuroraReplicaLagMinimum"
            | "AuroraReplicaLagMaximum" => "Milliseconds",
            "CheckpointLag" => "Seconds",
            "TransactionLogsDiskUsage" => "Bytes",
            "TransactionLogsGeneration" => "Bytes/Second",
            "OldestReplicationSlotLag" => "Bytes",
            "MaximumUsedTransactionIDs" => "Count",
            _ => "None",
        }
    }

    /// Validate RDS instance identifier format
    pub fn validate_instance_id(id: &str) -> bool {
        !id.is_empty() && id.len() <= 63 && id.chars().all(|c| c.is_alphanumeric() || c == '-')
    }
}
