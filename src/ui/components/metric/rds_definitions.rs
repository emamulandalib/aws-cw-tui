use crate::models::MetricType;
use crate::ui::components::metric::{DisplayFormat, HealthThresholds, MetricDefinition};
use ratatui::style::Color;

/// Get RDS metric definitions
pub fn get_rds_metric_definition(metric_type: &MetricType) -> MetricDefinition {
    match metric_type {
        MetricType::CpuUtilization => MetricDefinition {
            name: "CPU Utilization",
            display_format: DisplayFormat::Percentage,
            thresholds: Some(HealthThresholds {
                critical: 80.0,
                warning: 60.0,
                reverse_logic: false,
            }),
            color: Color::Blue,
            description: "Percentage of CPU utilization",
        },

        MetricType::DatabaseConnections => MetricDefinition {
            name: "Database Connections",
            display_format: DisplayFormat::Integer,
            thresholds: Some(HealthThresholds {
                critical: 1000.0,
                warning: 500.0,
                reverse_logic: false,
            }),
            color: Color::Green,
            description: "Number of client network connections to the database",
        },

        MetricType::FreeStorageSpace => MetricDefinition {
            name: "Free Storage Space",
            display_format: DisplayFormat::Bytes,
            thresholds: Some(HealthThresholds {
                critical: 1_073_741_824.0, // 1 GB
                warning: 5_368_709_120.0,  // 5 GB
                reverse_logic: true,       // lower is worse
            }),
            color: Color::Cyan,
            description: "Amount of available storage space",
        },

        MetricType::ReadLatency => MetricDefinition {
            name: "Read Latency",
            display_format: DisplayFormat::Duration,
            thresholds: Some(HealthThresholds {
                critical: 0.1, // 100ms
                warning: 0.05, // 50ms
                reverse_logic: false,
            }),
            color: Color::Red,
            description: "Average time taken for read operations",
        },

        MetricType::WriteLatency => MetricDefinition {
            name: "Write Latency",
            display_format: DisplayFormat::Duration,
            thresholds: Some(HealthThresholds {
                critical: 0.1, // 100ms
                warning: 0.05, // 50ms
                reverse_logic: false,
            }),
            color: Color::Magenta,
            description: "Average time taken for write operations",
        },

        MetricType::ReadIops => MetricDefinition {
            name: "Read IOPS",
            display_format: DisplayFormat::Integer,
            thresholds: None,
            color: Color::Green,
            description: "Read input/output operations per second",
        },

        MetricType::WriteIops => MetricDefinition {
            name: "Write IOPS",
            display_format: DisplayFormat::Integer,
            thresholds: None,
            color: Color::Yellow,
            description: "Write input/output operations per second",
        },

        MetricType::ReadThroughput => MetricDefinition {
            name: "Read Throughput",
            display_format: DisplayFormat::Bytes,
            thresholds: None,
            color: Color::Cyan,
            description: "Read throughput in bytes per second",
        },

        MetricType::WriteThroughput => MetricDefinition {
            name: "Write Throughput",
            display_format: DisplayFormat::Bytes,
            thresholds: None,
            color: Color::Magenta,
            description: "Write throughput in bytes per second",
        },

        MetricType::NetworkReceiveThroughput => MetricDefinition {
            name: "Network Receive Throughput",
            display_format: DisplayFormat::Bytes,
            thresholds: None,
            color: Color::LightGreen,
            description: "Network receive throughput in bytes per second",
        },

        MetricType::NetworkTransmitThroughput => MetricDefinition {
            name: "Network Transmit Throughput",
            display_format: DisplayFormat::Bytes,
            thresholds: None,
            color: Color::LightBlue,
            description: "Network transmit throughput in bytes per second",
        },

        MetricType::SwapUsage => MetricDefinition {
            name: "Swap Usage",
            display_format: DisplayFormat::Bytes,
            thresholds: Some(HealthThresholds {
                critical: 1_073_741_824.0, // 1 GB
                warning: 536_870_912.0,    // 512 MB
                reverse_logic: false,      // higher is worse
            }),
            color: Color::LightRed,
            description: "Amount of swap space used",
        },

        MetricType::QueueDepth => MetricDefinition {
            name: "Queue Depth",
            display_format: DisplayFormat::Integer,
            thresholds: Some(HealthThresholds {
                critical: 10.0,
                warning: 5.0,
                reverse_logic: false,
            }),
            color: Color::LightYellow,
            description: "Number of outstanding I/O requests",
        },

        MetricType::FreeableMemory => MetricDefinition {
            name: "Freeable Memory",
            display_format: DisplayFormat::Bytes,
            thresholds: Some(HealthThresholds {
                critical: 536_870_912.0,  // 512 MB
                warning: 1_073_741_824.0, // 1 GB
                reverse_logic: true,
            }),
            color: Color::LightBlue,
            description: "Amount of available RAM",
        },

        MetricType::BurstBalance => MetricDefinition {
            name: "Burst Balance",
            display_format: DisplayFormat::Percentage,
            thresholds: Some(HealthThresholds {
                critical: 20.0,
                warning: 50.0,
                reverse_logic: true, // lower is worse
            }),
            color: Color::LightCyan,
            description: "Percentage of burst bucket credits available",
        },

        MetricType::CpuCreditUsage => MetricDefinition {
            name: "CPU Credit Usage",
            display_format: DisplayFormat::Decimal(2),
            thresholds: None,
            color: Color::LightMagenta,
            description: "CPU credits consumed by the instance",
        },

        MetricType::CpuCreditBalance => MetricDefinition {
            name: "CPU Credit Balance",
            display_format: DisplayFormat::Decimal(2),
            thresholds: Some(HealthThresholds {
                critical: 50.0,
                warning: 100.0,
                reverse_logic: true, // lower is worse
            }),
            color: Color::Green,
            description: "Number of earned CPU credits accumulated",
        },

        // Add more RDS metrics as needed...
        _ => MetricDefinition {
            name: "Unknown Metric",
            display_format: DisplayFormat::Decimal(2),
            thresholds: None,
            color: Color::Gray,
            description: "Unknown metric type",
        },
    }
}