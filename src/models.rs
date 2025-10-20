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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    RdsList,
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
}
