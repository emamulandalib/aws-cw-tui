use ratatui::widgets::ListState;
use std::time::{Instant, SystemTime};

#[derive(Debug)]
pub struct RdsInstance {
    pub identifier: String,
    pub engine: String,
    pub status: String,
    pub instance_class: String,
    pub endpoint: Option<String>,
}

#[derive(Debug)]
pub struct MetricData {
    // Core Performance Metrics
    pub cpu_utilization: f64,
    pub database_connections: f64,
    pub free_storage_space: f64,
    pub read_iops: f64,
    pub write_iops: f64,
    pub read_latency: f64,
    pub write_latency: f64,
    
    // Extended RDS Metrics
    pub read_throughput: f64,          // Bytes/second
    pub write_throughput: f64,         // Bytes/second
    pub network_receive_throughput: f64,
    pub network_transmit_throughput: f64,
    pub swap_usage: f64,               // Bytes
    pub freeable_memory: f64,          // Bytes
    pub queue_depth: f64,              // Number of outstanding I/O requests
    
    // Additional Core RDS Metrics (27 total metrics)
    pub burst_balance: f64,            // Percent - GP2 burst bucket credits
    pub cpu_credit_usage: f64,         // Credits - for T2/T3/T4g instances
    pub cpu_credit_balance: f64,       // Credits - for T2/T3/T4g instances
    pub bin_log_disk_usage: f64,       // Bytes - MySQL/MariaDB binary logs
    pub replica_lag: f64,              // Seconds - read replica lag
    pub maximum_used_transaction_ids: f64, // Count - PostgreSQL transaction IDs
    pub oldest_replication_slot_lag: f64,  // Bytes - PostgreSQL replication slot lag
    pub replication_slot_disk_usage: f64,  // Bytes - PostgreSQL replication slots
    pub transaction_logs_disk_usage: f64,  // Bytes - PostgreSQL transaction logs
    pub transaction_logs_generation: f64,  // Bytes/second - PostgreSQL log generation
    pub failed_sql_server_agent_jobs_count: f64, // Count/minute - SQL Server agent jobs
    pub checkpoint_lag: f64,           // Seconds - checkpoint lag
    pub connection_attempts: f64,      // Count - MySQL connection attempts
    
    // Historical data for 3 hours (36 data points at 5min intervals)
    pub timestamps: Vec<SystemTime>,
    pub cpu_history: Vec<f64>,
    pub connections_history: Vec<f64>,
    pub read_iops_history: Vec<f64>,
    pub write_iops_history: Vec<f64>,
    pub read_latency_history: Vec<f64>,
    pub write_latency_history: Vec<f64>,
    pub read_throughput_history: Vec<f64>,
    pub write_throughput_history: Vec<f64>,
    pub network_receive_history: Vec<f64>,
    pub network_transmit_history: Vec<f64>,
    pub freeable_memory_history: Vec<f64>,
    pub swap_usage_history: Vec<f64>,
    pub queue_depth_history: Vec<f64>,
    pub free_storage_space_history: Vec<f64>,
    
    // Additional metric histories
    pub burst_balance_history: Vec<f64>,
    pub cpu_credit_usage_history: Vec<f64>,
    pub cpu_credit_balance_history: Vec<f64>,
    pub bin_log_disk_usage_history: Vec<f64>,
    pub replica_lag_history: Vec<f64>,
    pub maximum_used_transaction_ids_history: Vec<f64>,
    pub oldest_replication_slot_lag_history: Vec<f64>,
    pub replication_slot_disk_usage_history: Vec<f64>,
    pub transaction_logs_disk_usage_history: Vec<f64>,
    pub transaction_logs_generation_history: Vec<f64>,
    pub failed_sql_server_agent_jobs_count_history: Vec<f64>,
    pub checkpoint_lag_history: Vec<f64>,
    pub connection_attempts_history: Vec<f64>,
}

impl Default for MetricData {
    fn default() -> Self {
        Self {
            timestamps: Vec::new(),
            cpu_utilization: 0.0,
            database_connections: 0.0,
            free_storage_space: 0.0,
            read_iops: 0.0,
            write_iops: 0.0,
            read_latency: 0.0,
            write_latency: 0.0,
            read_throughput: 0.0,
            write_throughput: 0.0,
            network_receive_throughput: 0.0,
            network_transmit_throughput: 0.0,
            swap_usage: 0.0,
            freeable_memory: 0.0,
            queue_depth: 0.0,
            
            // Additional Core RDS Metrics
            burst_balance: 0.0,
            cpu_credit_usage: 0.0,
            cpu_credit_balance: 0.0,
            bin_log_disk_usage: 0.0,
            replica_lag: 0.0,
            maximum_used_transaction_ids: 0.0,
            oldest_replication_slot_lag: 0.0,
            replication_slot_disk_usage: 0.0,
            transaction_logs_disk_usage: 0.0,
            transaction_logs_generation: 0.0,
            failed_sql_server_agent_jobs_count: 0.0,
            checkpoint_lag: 0.0,
            connection_attempts: 0.0,
            
            cpu_history: Vec::new(),
            connections_history: Vec::new(),
            read_iops_history: Vec::new(),
            write_iops_history: Vec::new(),
            read_latency_history: Vec::new(),
            write_latency_history: Vec::new(),
            read_throughput_history: Vec::new(),
            write_throughput_history: Vec::new(),
            network_receive_history: Vec::new(),
            network_transmit_history: Vec::new(),
            freeable_memory_history: Vec::new(),
            swap_usage_history: Vec::new(),
            queue_depth_history: Vec::new(),
            free_storage_space_history: Vec::new(),
            
            // Additional metric histories
            burst_balance_history: Vec::new(),
            cpu_credit_usage_history: Vec::new(),
            cpu_credit_balance_history: Vec::new(),
            bin_log_disk_usage_history: Vec::new(),
            replica_lag_history: Vec::new(),
            maximum_used_transaction_ids_history: Vec::new(),
            oldest_replication_slot_lag_history: Vec::new(),
            replication_slot_disk_usage_history: Vec::new(),
            transaction_logs_disk_usage_history: Vec::new(),
            transaction_logs_generation_history: Vec::new(),
            failed_sql_server_agent_jobs_count_history: Vec::new(),
            checkpoint_lag_history: Vec::new(),
            connection_attempts_history: Vec::new(),
        }
    }
}

impl MetricData {
    pub fn count_available_metrics(&self) -> usize {
        let mut count = 0;
        
        // Core metrics (14) - always counted if they have data
        let core_metric_names = [
            "cpu_history",
            "connections_history", 
            "read_iops_history",
            "write_iops_history",
            "read_latency_history",
            "write_latency_history",
            "free_storage_space_history",
            "read_throughput_history",
            "write_throughput_history",
            "network_receive_history",
            "network_transmit_history",
            "freeable_memory_history",
            "swap_usage_history",
            "queue_depth_history",
        ];
        
        let core_histories = [
            &self.cpu_history,
            &self.connections_history,
            &self.read_iops_history,
            &self.write_iops_history,
            &self.read_latency_history,
            &self.write_latency_history,
            &self.free_storage_space_history,
            &self.read_throughput_history,
            &self.write_throughput_history,
            &self.network_receive_history,
            &self.network_transmit_history,
            &self.freeable_memory_history,
            &self.swap_usage_history,
            &self.queue_depth_history,
        ];
        
        for (_, history) in core_metric_names.iter().zip(core_histories.iter()) {
            if !history.is_empty() { 
                count += 1; 
            }
        }
        
        // Advanced metrics (13) - only counted if they have data
        let advanced_metric_names = [
            "burst_balance_history",
            "cpu_credit_usage_history", 
            "cpu_credit_balance_history",
            "bin_log_disk_usage_history",
            "replica_lag_history",
            "maximum_used_transaction_ids_history",
            "oldest_replication_slot_lag_history",
            "replication_slot_disk_usage_history",
            "transaction_logs_disk_usage_history",
            "transaction_logs_generation_history",
            "failed_sql_server_agent_jobs_count_history",
            "checkpoint_lag_history",
            "connection_attempts_history",
        ];
        
        let advanced_histories = [
            &self.burst_balance_history,
            &self.cpu_credit_usage_history,
            &self.cpu_credit_balance_history,
            &self.bin_log_disk_usage_history,
            &self.replica_lag_history,
            &self.maximum_used_transaction_ids_history,
            &self.oldest_replication_slot_lag_history,
            &self.replication_slot_disk_usage_history,
            &self.transaction_logs_disk_usage_history,
            &self.transaction_logs_generation_history,
            &self.failed_sql_server_agent_jobs_count_history,
            &self.checkpoint_lag_history,
            &self.connection_attempts_history,
        ];
        
        for (_, history) in advanced_metric_names.iter().zip(advanced_histories.iter()) {
            if !history.is_empty() { 
                count += 1; 
            }
        }
        
        count
    }
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    RdsList,
    MetricsSummary,
    InstanceDetails,
}

pub struct App {
    pub rds_instances: Vec<RdsInstance>,
    pub list_state: ListState,
    pub loading: bool,
    pub state: AppState,
    pub selected_instance: Option<usize>,
    pub metrics: MetricData,
    pub metrics_loading: bool,
    pub last_refresh: Option<Instant>,
    pub auto_refresh_enabled: bool,
    pub scroll_offset: usize,
    pub metrics_per_screen: usize,
    pub metrics_summary_scroll: usize, // Track metrics summary scroll position separately
}
