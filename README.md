# AWS CloudWatch TUI

A terminal user interface (TUI) for AWS CloudWatch that provides comprehensive RDS monitoring with advanced data visualization.

## Features

### 🚀 Enhanced RDS Monitoring
- **Comprehensive Metrics Collection**: All 27 AWS RDS metrics including CPU, IOPS, latency, throughput, network, memory, storage, and advanced engine-specific metrics
- **Real CloudWatch Time Series Data**: Actual timestamps from CloudWatch with proper time-based X-axis plotting
- **Extended Historical Data**: 3 hours of historical data (36 data points at 5-minute intervals) for trend analysis
- **Large High-Resolution Charts**: Enhanced Braille plotting with optimized chart areas for maximum visibility
- **Scrollable Interface**: Navigate through all 27 individual metrics with smooth scrolling
- **Single-Metric Layout**: Full-width charts for each metric for maximum detail visibility

### 📊 Modern UI Layout
- **27 Comprehensive Metrics**: Complete AWS RDS monitoring coverage organized in logical sections
  1. **Core Performance**: CPU Utilization, DB Connections, IOPS (Read/Write), Latency (Read/Write), Free Storage
  2. **Extended Performance**: Throughput (Read/Write), Network (RX/TX), Memory (Freeable/Swap), Queue Depth
  3. **Advanced Metrics**: Burst Balance, CPU Credits, Bin Log Usage, Replica Lag, Transaction Management
  4. **Engine-Specific**: PostgreSQL replication slots, SQL Server agent jobs, checkpoint lag, connection attempts

### 🎨 CloudWatch-Style Visualization
- **High-Resolution Time Series Charts**: Enhanced Braille plotting with larger chart areas for better visualization
- **Single-Metric Layout**: Full-width individual metric charts for maximum detail and clarity
- **Scrollable Interface**: Navigate through all 27 metrics using ↑/↓ or k/j keys
- **Real-time Values**: Current metric values displayed cleanly with appropriate units and formatting
- **Accurate Time Labels**: X-axis shows relative time ("-3h", "-2h", "-1h", "now") based on actual data timestamps
- **Intelligent Scaling**: Automatic Y-axis scaling with proper bounds handling and 5-10% padding
- **Color-coded Metrics**: Each metric type has distinct colors for easy identification
- **Professional Charts**: CloudWatch-style line charts using ratatui's Chart widget with enhanced Braille markers

### 🔄 Enhanced Data Collection
- **Real CloudWatch Integration**: Fetches actual time series data with timestamps from AWS CloudWatch
- **Comprehensive AWS Integration**: Fetches all major RDS CloudWatch metrics
- **Timestamp Accuracy**: Preserves original CloudWatch timestamps for accurate time-based plotting
- **Error Handling**: Robust error handling for AWS API calls with fallback time calculation
- **Performance Optimized**: Efficient data collection and visualization with concurrent metric fetching

## Installation

```bash
cargo install --path .
```

## Usage

### RDS Instances

To view RDS instances in your current AWS account:

```bash
awscw --rds
```

### Navigation

- **Arrow Keys (Instance List)**: Navigate through RDS instances list
- **Enter**: Select an RDS instance to view detailed metrics
- **Arrow Keys / k/j (Metrics View)**: Scroll through metric pairs (↑/↓ or k/j)
- **Home**: Reset scroll position to top
- **'b'**: Go back to instance list from metrics view
- **'r'**: Refresh metrics data (3-hour historical data collection)
- **'q'**: Quit the application

### Enhanced Metrics Dashboard

When viewing an RDS instance, you'll see:

1. **Instance Information**: Engine type, status, instance class, and endpoint
2. **Comprehensive Metrics Display**: All 27 AWS RDS metrics individually displayed:
   - **Core Performance** (7): CPU Utilization, DB Connections, Read/Write IOPS, Read/Write Latency, Free Storage
   - **Extended Performance** (7): Read/Write Throughput, Network RX/TX, Freeable Memory, Swap Usage, Queue Depth
   - **Advanced Metrics** (13): Burst Balance, CPU Credits, Bin Log Usage, Replica Lag, Transaction Management, Engine-specific metrics
3. **Full-Width High-Resolution Charts**: Each metric displays in a dedicated chart with 3-hour time series data
4. **Scrollable Interface**: Navigate through all 27 metrics with smooth scrolling

### Sample Metrics Display

```
┌─ Core Performance ──────────────────────────────────────────┐
│ ┌─ CPU Utilization ─────┐ ┌─ DB Connections ──────────┐    │
│ │ 65.3%                 │ │ 42                        │    │
│ │ ┌─ 3h Trend ────────┐ │ │ ┌─ 3h Trend ─────────┐  │    │
│ │ │ ⠈⠑⠒⠢⠤⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ │ │ │ ⠈⠉⠒⠢⠤⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ │  │    │
│ │ │ 3h ago      now │ │ │ │ 3h ago       now │  │    │
│ │ └─────────────────┘ │ │ └──────────────────┘  │    │
│ └───────────────────── │ └─────────────────────── │    │
└─────────────────────────────────────────────────────────────┘
```

## AWS Configuration

Ensure your AWS credentials are configured:

```bash
aws configure
```

Or use environment variables:
```bash
export AWS_ACCESS_KEY_ID=your_access_key
export AWS_SECRET_ACCESS_KEY=your_secret_key
export AWS_DEFAULT_REGION=us-east-1
```

## Supported RDS Metrics

The application monitors all 27 comprehensive RDS metrics:

### Core Performance (7 metrics)
- CPU Utilization, Database Connections, Free Storage Space
- Read/Write IOPS, Read/Write Latency

### Extended Performance (7 metrics)  
- Read/Write Throughput, Network Receive/Transmit Throughput
- Freeable Memory, Swap Usage, Queue Depth

### Advanced & Engine-Specific (13 metrics)
- **Storage & Credits**: Burst Balance, CPU Credit Usage/Balance
- **MySQL/MariaDB**: Binary Log Disk Usage, Connection Attempts
- **PostgreSQL**: Transaction IDs, Replication Slot Lag/Usage, Transaction Log Usage/Generation
- **SQL Server**: Failed Agent Jobs, Checkpoint Lag
- **General**: Replica Lag (Read Replicas)

## Technical Features

### High-Resolution Line Charts
- **Enhanced Braille Charts**: Full-width, high-resolution time series charts with optimized spacing
- **Scrollable Metric Display**: Navigate through all 27 metrics individually using ↑/↓ or k/j keys
- **Single-Metric Layout**: Each metric gets a dedicated full-width chart for maximum detail
- **3-Hour Data Window**: 36 data points at 5-minute intervals with precise timestamp plotting
- **Real-time Updates**: Fresh data on every refresh with proper time-series visualization
- **Smart Bounds**: Automatic Y-axis scaling with 5-10% padding for optimal visualization

### Data Collection
- **3-Hour Window**: 36 data points at 5-minute intervals
- **Real-time Updates**: Fresh data on every refresh
- **Unit Conversion**: Automatic formatting (bytes→GB/MB, seconds→ms)

## Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Running in Development
```bash
cargo run -- --rds
```

## Dependencies

- **AWS SDK**: CloudWatch and RDS clients
- **Ratatui**: Terminal UI framework  
- **Tokio**: Async runtime
- **Clap**: Command line parsing

## License

This project is open source. See LICENSE file for details.
