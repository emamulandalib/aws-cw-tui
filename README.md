# AWS CloudWatch TUI

A terminal user interface (TUI) for AWS CloudWatch that provides comprehensive RDS monitoring with advanced data visualization.

## Features

### 🚀 Enhanced RDS Monitoring
- **Comprehensive Metrics Collection**: 14 essential AWS RDS metrics including CPU, IOPS, latency, throughput, network, memory, and storage
- **Real CloudWatch Time Series Data**: Actual timestamps from CloudWatch with proper time-based X-axis plotting
- **Extended Historical Data**: 3 hours of historical data (36 data points at 5-minute intervals) for trend analysis
- **Large High-Resolution Charts**: Enhanced Braille plotting with optimized chart areas for maximum visibility
- **Scrollable Interface**: Navigate through 8 metric pairs (16 total metrics) with smooth scrolling
- **2-Metric Layout**: Professional side-by-side layout showing 2 metrics per row for efficient space usage

### 📊 Modern UI Layout
- **7 Organized Sections**: Logically grouped metrics in comprehensive dashboard layout
  1. **Core Performance**: CPU Utilization and Database Connections
  2. **Storage & IOPS**: Read IOPS and Write IOPS
  3. **Latency & Performance**: Read/Write Latency (in milliseconds)
  4. **Storage Throughput**: Read/Write Throughput (MB/s)
  5. **Network Traffic**: Network RX/TX Throughput
  6. **Memory Management**: Freeable Memory and Swap Usage
  7. **Advanced I/O**: Queue Depth and Free Storage Space

### 🎨 CloudWatch-Style Visualization
- **High-Resolution Time Series Charts**: Enhanced Braille plotting with larger chart areas for better visualization
- **2 Metrics per Row Layout**: Optimized display showing 2 metrics side-by-side with larger charts
- **Scrollable Interface**: Navigate through 8 metric pairs using ↑/↓ or k/j keys
- **Real-time Values**: Current metric values displayed cleanly without debug information
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
2. **Scrollable Metrics Display**: 8 metric pairs (16 total metrics) organized in rows:
   - **Core Performance**: CPU Utilization, DB Connections
   - **Storage Performance**: Read IOPS, Write IOPS
   - **Latency Metrics**: Read Latency, Write Latency
   - **Throughput Metrics**: Read Throughput, Write Throughput
   - **Network Metrics**: Network RX, Network TX
   - **Memory Metrics**: Freeable Memory, Swap Usage
   - **I/O Queue Metrics**: Queue Depth, Free Storage
3. **Large High-Resolution Charts**: Each metric shows enhanced Braille charts with 3-hour time series
4. **Intelligent Layout**: 2 metrics per row with optimized chart sizing

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

The application monitors 25+ comprehensive RDS metrics:

### Core Performance
- CPU Utilization, Database Connections, Storage Space
- Read/Write IOPS, Read/Write Latency
- Read/Write Throughput

### Network & Memory
- Network Receive/Transmit Throughput
- Freeable Memory, Swap Usage

### Advanced Metrics  
- Queue Depth, Replica Lag, Burst Balance
- Transaction Logs (Generation & Disk Usage)
- InnoDB Metrics (Buffer Pool Hit Ratio, Row Operations)

## Technical Features

### High-Resolution Line Charts
- **Enhanced Braille Charts**: Large, high-resolution time series charts with optimized spacing
- **Scrollable Metric Display**: Navigate through 8 metric pairs using ↑/↓ or k/j keys
- **Intelligent Layout**: 2 metrics per row with adaptive chart sizing
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
