use std::time::SystemTime;

// Parameter struct to reduce function argument count
pub struct MetricFetchParams {
    pub metric_name: String,
    pub namespace: String,
    pub instance_id: String,
    pub unit: Option<String>,
}

// Helper structs to organize metric data
pub struct CoreMetrics {
    pub cpu_utilization: f64,
    pub cpu_history: Vec<f64>,
    pub timestamps: Vec<SystemTime>,
    pub database_connections: f64,
    pub connections_history: Vec<f64>,
    pub free_storage_space: f64,
    pub free_storage_space_history: Vec<f64>,
    pub read_iops: f64,
    pub read_iops_history: Vec<f64>,
    pub write_iops: f64,
    pub write_iops_history: Vec<f64>,
    pub read_latency: f64,
    pub read_latency_history: Vec<f64>,
    pub write_latency: f64,
    pub write_latency_history: Vec<f64>,
    pub read_throughput: f64,
    pub read_throughput_history: Vec<f64>,
    pub write_throughput: f64,
    pub write_throughput_history: Vec<f64>,
    pub network_receive_throughput: f64,
    pub network_receive_history: Vec<f64>,
    pub network_transmit_throughput: f64,
    pub network_transmit_history: Vec<f64>,
    pub swap_usage: f64,
    pub swap_usage_history: Vec<f64>,
    pub freeable_memory: f64,
    pub freeable_memory_history: Vec<f64>,
    pub queue_depth: f64,
    pub queue_depth_history: Vec<f64>,
}

pub struct AdvancedMetrics {
    pub burst_balance: f64,
    pub burst_balance_history: Vec<f64>,
    pub cpu_credit_usage: f64,
    pub cpu_credit_usage_history: Vec<f64>,
    pub cpu_credit_balance: f64,
    pub cpu_credit_balance_history: Vec<f64>,
    // Missing CPU surplus credit metrics
    pub cpu_surplus_credit_balance: f64,
    pub cpu_surplus_credit_balance_history: Vec<f64>,
    pub cpu_surplus_credits_charged: f64,
    pub cpu_surplus_credits_charged_history: Vec<f64>,
    // Missing EBS performance metrics
    pub ebs_byte_balance: f64,
    pub ebs_byte_balance_history: Vec<f64>,
    pub ebs_io_balance: f64,
    pub ebs_io_balance_history: Vec<f64>,
    pub bin_log_disk_usage: f64,
    pub bin_log_disk_usage_history: Vec<f64>,
    pub replica_lag: f64,
    pub replica_lag_history: Vec<f64>,
    pub maximum_used_transaction_ids: f64,
    pub maximum_used_transaction_ids_history: Vec<f64>,
    pub oldest_replication_slot_lag: f64,
    pub oldest_replication_slot_lag_history: Vec<f64>,
    // Missing logical replication slot lag metric
    pub oldest_logical_replication_slot_lag: f64,
    pub oldest_logical_replication_slot_lag_history: Vec<f64>,
    pub replication_slot_disk_usage: f64,
    pub replication_slot_disk_usage_history: Vec<f64>,
    pub transaction_logs_disk_usage: f64,
    pub transaction_logs_disk_usage_history: Vec<f64>,
    pub transaction_logs_generation: f64,
    pub transaction_logs_generation_history: Vec<f64>,
    pub failed_sql_server_agent_jobs_count: f64,
    pub failed_sql_server_agent_jobs_count_history: Vec<f64>,
    pub checkpoint_lag: f64,
    pub checkpoint_lag_history: Vec<f64>,
    pub connection_attempts: f64,
    pub connection_attempts_history: Vec<f64>,
}

/// RDS instance characteristics for metric filtering
#[derive(Debug, Clone)]
pub struct RdsInstanceCharacteristics {
    pub engine: String,
    pub instance_class: String,
    pub is_read_replica: bool,
    pub multi_az: bool,
}

impl RdsInstanceCharacteristics {
    /// Create characteristics from RDS instance
    pub fn from_instance(instance: &crate::models::RdsInstance) -> Self {
        Self {
            engine: instance.engine.clone(),
            instance_class: instance.instance_class.clone(),
            is_read_replica: instance.identifier.contains("-replica") || instance.identifier.contains("-read"),
            multi_az: false, // We'll enhance this later when we have more instance details
        }
    }

    /// Determine if this instance supports burstable performance (T-series)
    pub fn is_burstable(&self) -> bool {
        self.instance_class.starts_with("db.t2.") || 
        self.instance_class.starts_with("db.t3.") || 
        self.instance_class.starts_with("db.t4g.")
    }

    /// Determine if this instance supports GP2 burst balance
    pub fn supports_gp2_burst(&self) -> bool {
        // Most RDS instances support GP2 storage
        true
    }

    /// Check if this is a PostgreSQL engine
    pub fn is_postgresql(&self) -> bool {
        self.engine.to_lowercase().contains("postgres")
    }

    /// Check if this is a MySQL/MariaDB engine
    pub fn is_mysql_family(&self) -> bool {
        let engine_lower = self.engine.to_lowercase();
        engine_lower.contains("mysql") || engine_lower.contains("mariadb")
    }

    /// Check if this is a SQL Server engine
    pub fn is_sql_server(&self) -> bool {
        self.engine.to_lowercase().contains("sqlserver")
    }

    /// Get relevant metrics for this instance
    pub fn get_relevant_metrics(&self) -> Vec<crate::models::MetricType> {
        use crate::models::MetricType;
        
        let mut relevant_metrics = Vec::new();

        log::info!("Getting relevant metrics for {} {} instance (burstable: {}, postgresql: {})", 
            self.engine, self.instance_class, self.is_burstable(), self.is_postgresql());

        // Core metrics - always relevant for all RDS instances
        relevant_metrics.extend_from_slice(&[
            MetricType::CpuUtilization,
            MetricType::DatabaseConnections,
            MetricType::FreeStorageSpace,
            MetricType::ReadIops,
            MetricType::WriteIops,
            MetricType::ReadLatency,
            MetricType::WriteLatency,
            MetricType::ReadThroughput,
            MetricType::WriteThroughput,
            MetricType::NetworkReceiveThroughput,
            MetricType::NetworkTransmitThroughput,
            MetricType::FreeableMemory,
            MetricType::SwapUsage,
            MetricType::QueueDepth,
        ]);
        log::info!("Added 14 core metrics");

        // Storage-related metrics (GP2 burst balance)
        if self.supports_gp2_burst() {
            relevant_metrics.push(MetricType::BurstBalance);
            log::info!("Added BurstBalance metric");
        }

        // Burstable instance metrics (CPU credits)
        if self.is_burstable() {
            relevant_metrics.push(MetricType::CpuCreditUsage);
            relevant_metrics.push(MetricType::CpuCreditBalance);
            // Add surplus CPU credit metrics for burstable instances
            relevant_metrics.push(MetricType::CpuSurplusCreditBalance);
            relevant_metrics.push(MetricType::CpuSurplusCreditsCharged);
            log::info!("Added CPU credit and surplus credit metrics for burstable instance");
        }

        // EBS performance metrics (available for all instances)
        relevant_metrics.push(MetricType::EbsByteBalance);
        relevant_metrics.push(MetricType::EbsIoBalance);
        log::info!("Added EBS performance metrics");

        // MySQL/MariaDB specific metrics
        if self.is_mysql_family() {
            relevant_metrics.push(MetricType::BinLogDiskUsage);
            relevant_metrics.push(MetricType::ConnectionAttempts);
            log::info!("Added MySQL/MariaDB specific metrics");
        }

        // PostgreSQL specific metrics
        if self.is_postgresql() {
            relevant_metrics.push(MetricType::MaximumUsedTransactionIds);
            relevant_metrics.push(MetricType::OldestReplicationSlotLag);
            relevant_metrics.push(MetricType::OldestLogicalReplicationSlotLag);
            relevant_metrics.push(MetricType::ReplicationSlotDiskUsage);
            relevant_metrics.push(MetricType::TransactionLogsDiskUsage);
            relevant_metrics.push(MetricType::TransactionLogsGeneration);
            relevant_metrics.push(MetricType::CheckpointLag);
            log::info!("Added 7 PostgreSQL specific metrics");
        }

        // SQL Server specific metrics
        if self.is_sql_server() {
            relevant_metrics.push(MetricType::FailedSqlServerAgentJobsCount);
            log::info!("Added SQL Server specific metrics");
        }

        // Read replica specific metrics
        if self.is_read_replica {
            relevant_metrics.push(MetricType::ReplicaLag);
            log::info!("Added read replica metrics");
        }

        log::info!("Total relevant metrics: {}", relevant_metrics.len());
        relevant_metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgresql_metrics_count() {
        let characteristics = RdsInstanceCharacteristics {
            engine: "postgres".to_string(),
            instance_class: "db.t3.micro".to_string(),
            is_read_replica: false,
            multi_az: false,
        };

        let metrics = characteristics.get_relevant_metrics();
        
        // PostgreSQL db.t3.micro should have:
        // 14 core metrics + 2 CPU credit metrics + 6 PostgreSQL specific = 22 metrics
        assert!(metrics.len() >= 20, "PostgreSQL instance should have at least 20 metrics, got {}", metrics.len());
        
        // Verify core metrics are included
        assert!(metrics.contains(&crate::models::MetricType::CpuUtilization));
        assert!(metrics.contains(&crate::models::MetricType::DatabaseConnections));
        assert!(metrics.contains(&crate::models::MetricType::FreeStorageSpace));
        
        // Verify PostgreSQL specific metrics are included
        assert!(metrics.contains(&crate::models::MetricType::MaximumUsedTransactionIds));
        assert!(metrics.contains(&crate::models::MetricType::TransactionLogsDiskUsage));
        assert!(metrics.contains(&crate::models::MetricType::CheckpointLag));
        
        // Verify burstable instance metrics are included
        assert!(metrics.contains(&crate::models::MetricType::CpuCreditUsage));
        assert!(metrics.contains(&crate::models::MetricType::CpuCreditBalance));
    }

    #[test]
    fn test_mysql_metrics_count() {
        let characteristics = RdsInstanceCharacteristics {
            engine: "mysql".to_string(),
            instance_class: "db.t3.micro".to_string(),
            is_read_replica: false,
            multi_az: false,
        };

        let metrics = characteristics.get_relevant_metrics();
        
        // MySQL db.t3.micro should have:
        // 14 core metrics + 2 CPU credit metrics + 2 MySQL specific = 18 metrics
        assert!(metrics.len() >= 16, "MySQL instance should have at least 16 metrics, got {}", metrics.len());
        
        // Verify MySQL specific metrics are included
        assert!(metrics.contains(&crate::models::MetricType::BinLogDiskUsage));
        assert!(metrics.contains(&crate::models::MetricType::ConnectionAttempts));
    }

    #[test]
    fn test_non_burstable_instance() {
        let characteristics = RdsInstanceCharacteristics {
            engine: "postgres".to_string(),
            instance_class: "db.m5.large".to_string(),
            is_read_replica: false,
            multi_az: false,
        };

        let metrics = characteristics.get_relevant_metrics();
        
        // Should not include CPU credit metrics for non-burstable instances
        assert!(!metrics.contains(&crate::models::MetricType::CpuCreditUsage));
        assert!(!metrics.contains(&crate::models::MetricType::CpuCreditBalance));
    }

    #[test]
    fn test_read_replica_metrics() {
        let characteristics = RdsInstanceCharacteristics {
            engine: "postgres".to_string(),
            instance_class: "db.t3.micro".to_string(),
            is_read_replica: true,
            multi_az: false,
        };

        let metrics = characteristics.get_relevant_metrics();
        
        // Should include replica lag for read replicas
        assert!(metrics.contains(&crate::models::MetricType::ReplicaLag));
    }
}
