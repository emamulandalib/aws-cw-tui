use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use aws_sdk_rds::Client as RdsClient;
use std::time::SystemTime;
use crate::models::{RdsInstance, MetricData};

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
            };
            instances.push(rds_instance);
        }
    }
    
    Ok(instances)
}

// Enhanced helper function to get comprehensive metric data
async fn fetch_comprehensive_metric(
    client: &CloudWatchClient,
    metric_name: &str,
    namespace: &str,
    instance_id: &str,
    start_time: SystemTime,
    end_time: SystemTime,
    unit: Option<&str>,
) -> (f64, Vec<f64>, Vec<SystemTime>) {
    let mut request = client
        .get_metric_statistics()
        .namespace(namespace)
        .metric_name(metric_name)
        .dimensions(
            aws_sdk_cloudwatch::types::Dimension::builder()
                .name("DBInstanceIdentifier")
                .value(instance_id)
                .build(),
        )
        .start_time(aws_sdk_cloudwatch::primitives::DateTime::from(start_time))
        .end_time(aws_sdk_cloudwatch::primitives::DateTime::from(end_time))
        .period(300) // 5 minutes = 300 seconds
        .statistics(aws_sdk_cloudwatch::types::Statistic::Average);

    if let Some(u) = unit {
        match u {
            "Percent" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Percent),
            "Count" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Count),
            "Count/Second" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::CountSecond),
            "Bytes" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Bytes),
            "Bytes/Second" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::BytesSecond),
            "Seconds" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds),
            _ => {} // Skip unknown units
        }
    }

    let resp = request.send().await;

    match resp {
        Ok(data) => {
            if let Some(mut datapoints) = data.datapoints {
                // Sort by timestamp
                datapoints.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
                
                let latest_value = datapoints.last()
                    .and_then(|dp| dp.average)
                    .unwrap_or(0.0);
                
                // Get last 36 data points (3 hours at 5min intervals) - take the most recent ones
                let recent_datapoints: Vec<_> = datapoints
                    .iter()
                    .rev()
                    .take(36)
                    .rev()
                    .collect();
                
                let history: Vec<f64> = recent_datapoints
                    .iter()
                    .filter_map(|dp| dp.average)
                    .collect();
                
                let timestamps: Vec<SystemTime> = recent_datapoints
                    .iter()
                    .map(|dp| {
                        dp.timestamp
                            .map(|ts| SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(ts.secs() as u64))
                            .unwrap_or_else(SystemTime::now)
                    })
                    .collect();
                
                (latest_value, history, timestamps)
            } else {
                // No data available - return zero with empty history and timestamps
                (0.0, Vec::new(), Vec::new())
            }
        }
        Err(_) => {
            // No data available on error - return zero with empty history and timestamps
            (0.0, Vec::new(), Vec::new())
        }
    }
}

pub async fn load_metrics(instance_id: &str) -> Result<MetricData> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = CloudWatchClient::new(&config);

    // Get current time and 3 hours ago for comprehensive metrics
    let end_time = SystemTime::now();
    let start_time = end_time - std::time::Duration::from_secs(10800); // 3 hours ago

    let instance_id_owned = instance_id.to_string();
    
    // Fetch all comprehensive metrics concurrently
    let (
        (cpu, cpu_hist, cpu_timestamps),
        (connections, conn_hist, _),
        (free_storage, free_storage_hist, _),
        (read_iops, read_iops_hist, _),
        (write_iops, write_iops_hist, _),
        (read_latency, read_lat_hist, _),
        (write_latency, write_lat_hist, _),
        (read_throughput, read_throughput_hist, _),
        (write_throughput, write_throughput_hist, _),
        (net_receive, net_receive_hist, _),
        (net_transmit, net_transmit_hist, _),
        (swap_usage, swap_hist, _),
        (freeable_memory, memory_hist, _),
        (queue_depth, queue_depth_hist, _),
    ) = tokio::join!(
        fetch_comprehensive_metric(&client, "CPUUtilization", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Percent")),
        fetch_comprehensive_metric(&client, "DatabaseConnections", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Count")),
        fetch_comprehensive_metric(&client, "FreeStorageSpace", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes")),
        fetch_comprehensive_metric(&client, "ReadIOPS", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Count/Second")),
        fetch_comprehensive_metric(&client, "WriteIOPS", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Count/Second")),
        fetch_comprehensive_metric(&client, "ReadLatency", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Seconds")),
        fetch_comprehensive_metric(&client, "WriteLatency", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Seconds")),
        fetch_comprehensive_metric(&client, "ReadThroughput", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes/Second")),
        fetch_comprehensive_metric(&client, "WriteThroughput", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes/Second")),
        fetch_comprehensive_metric(&client, "NetworkReceiveThroughput", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes/Second")),
        fetch_comprehensive_metric(&client, "NetworkTransmitThroughput", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes/Second")),
        fetch_comprehensive_metric(&client, "SwapUsage", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes")),
        fetch_comprehensive_metric(&client, "FreeableMemory", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Bytes")),
        fetch_comprehensive_metric(&client, "DiskQueueDepth", "AWS/RDS", &instance_id_owned, start_time, end_time, Some("Count")),
    );

    Ok(MetricData {
        // Use CPU timestamps as the primary timeline (all metrics should have similar timestamps)
        timestamps: cpu_timestamps,
        cpu_utilization: cpu,
        database_connections: connections,
        free_storage_space: free_storage,
        read_iops,
        write_iops,
        read_latency,
        write_latency,
        read_throughput,
        write_throughput,
        network_receive_throughput: net_receive,
        network_transmit_throughput: net_transmit,
        swap_usage,
        freeable_memory,
        queue_depth,
        cpu_history: cpu_hist,
        connections_history: conn_hist,
        read_iops_history: read_iops_hist,
        write_iops_history: write_iops_hist,
        read_latency_history: read_lat_hist,
        write_latency_history: write_lat_hist,
        read_throughput_history: read_throughput_hist,
        write_throughput_history: write_throughput_hist,
        network_receive_history: net_receive_hist,
        network_transmit_history: net_transmit_hist,
        freeable_memory_history: memory_hist,
        swap_usage_history: swap_hist,
        queue_depth_history: queue_depth_hist,
        free_storage_space_history: free_storage_hist,
    })
}
