# Tailmon 🖥️📊

A lightweight, real-time system monitoring solution built with Rust, featuring an agent-server architecture for distributed system monitoring.

![image](https://github.com/user-attachments/assets/2aabee50-7b07-43e9-bb29-30be3aab9961)

## Overview

Tailmon is a modern system monitoring tool that helps you keep track of your infrastructure's health across multiple machines. Think of it as a simplified version of monitoring solutions like Nagios or Zabbix, but designed to be lightweight and easy to deploy. The system consists of two main components working together: lightweight agents that collect system metrics from your machines, and a central server that aggregates this data and presents it through an elegant web dashboard.

The name "Tailmon" reflects the tool's ability to "tail" or follow your system metrics continuously, providing you with real-time insights into your infrastructure's performance.

## Architecture

Tailmon follows a distributed agent-server architecture that separates data collection from data presentation, making it both scalable and maintainable.

### Agent Component

The agent is a small, efficient program that runs on each machine you want to monitor. Think of it as a digital health monitor for your computer. Here's what it does:

- **System Data Collection**: Continuously gathers essential system metrics including CPU usage, memory consumption, operating system information, and device identification
- **Intelligent Reporting**: Sends collected data to the central server at regular intervals (every 5 seconds by default)
- **Resilient Communication**: Implements smart retry logic with exponential backoff to handle network interruptions gracefully
- **Minimal Resource Usage**: Designed to have negligible impact on system performance while providing accurate monitoring

### Server Component

The server acts as the central hub that receives, processes, and presents data from all connected agents. Its responsibilities include:

- **Data Aggregation**: Receives and stores metrics from multiple agents simultaneously
- **Web Dashboard**: Provides a beautiful, responsive web interface for visualizing system health
- **REST API**: Offers programmatic access to metrics data for integration with other tools
- **Real-time Updates**: Automatically refreshes dashboard data every 3 seconds for live monitoring

## Features

### Current Capabilities

- **Real-time Monitoring**: Live tracking of CPU usage, memory consumption, and system information
- **Multi-platform Support**: Works across Linux, macOS, and Windows systems
- **Beautiful Dashboard**: Modern, responsive web interface with gradient designs and glassmorphism effects
- **Status Indicators**: Visual alerts when systems exceed warning thresholds (CPU > 60%, RAM > 70%) or critical thresholds (CPU > 80%, RAM > 90%)
- **Device Management**: Automatic device identification and OS detection
- **Network Resilience**: Robust error handling and automatic reconnection capabilities
- **Embedded Assets**: Self-contained server with built-in web assets for easy deployment

### Technical Highlights

- **Memory Efficiency**: Uses DashMap for concurrent, lock-free data storage
- **Async/Await**: Fully asynchronous implementation using Tokio for excellent performance
- **Structured Logging**: Comprehensive logging with tracing for debugging and monitoring
- **Type Safety**: Leverages Rust's type system to prevent common runtime errors
- **Cross-platform Compatibility**: Single codebase that works across major operating systems

## Installation

### Prerequisites

Before you begin, ensure you have Rust installed on your system. If you don't have Rust, you can install it from [rustup.rs](https://rustup.rs/).

### Building from Source

```bash
# Clone the repository
git clone https://github.com/sadopc/tailmon.git
cd tailmon

# Build the entire workspace
cargo build --release

# The binaries will be available in target/release/
# - target/release/server (or server.exe on Windows)
# - target/release/agent (or agent.exe on Windows)
```

## Usage

### Starting the Server

The server component should be started first, as it needs to be running to receive data from agents.

```bash
# Start the server (listens on all interfaces, port 3000)
./target/release/server
```

Once started, the server provides several endpoints:

- **Web Dashboard**: http://localhost:3000 - Beautiful web interface for monitoring
- **Metrics API**: http://localhost:3000/api/all_metrics - JSON endpoint for programmatic access
- **Data Ingestion**: http://localhost:3000/api/metrics - Endpoint where agents send data

### Deploying Agents

Agents can be deployed on any machine you want to monitor. They're designed to be lightweight and can run continuously in the background.

```bash
# Start an agent with default settings (connects to localhost:3000)
./target/release/agent

# Start an agent connecting to a remote server
TAILMON_SERVER_URL=http://your-server:3000/api/metrics ./target/release/agent
```

#### Environment Configuration

Agents support configuration through environment variables:

- `TAILMON_SERVER_URL`: Specifies the server endpoint (default: http://127.0.0.1:3000/api/metrics)

### Running as a Service

For production deployments, you'll want to run both components as system services.

#### Linux (systemd)

Create service files for automatic startup and management:

```ini
# /etc/systemd/system/tailmon-server.service
[Unit]
Description=Tailmon Monitoring Server
After=network.target

[Service]
Type=simple
User=tailmon
ExecStart=/usr/local/bin/tailmon-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```ini
# /etc/systemd/system/tailmon-agent.service
[Unit]
Description=Tailmon Monitoring Agent
After=network.target

[Service]
Type=simple
User=tailmon
Environment=TAILMON_SERVER_URL=http://your-server:3000/api/metrics
ExecStart=/usr/local/bin/tailmon-agent
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start the services:

```bash
sudo systemctl enable tailmon-server tailmon-agent
sudo systemctl start tailmon-server tailmon-agent
```

## API Reference

### GET /api/all_metrics

Returns all currently stored metrics from connected devices.

**Response Format:**
```json
[
  {
    "device_id": "my-laptop",
    "os_info": "Ubuntu 22.04 (Kernel: 5.15.0)",
    "cpu_usage": 15.7,
    "ram_used_mb": 4096,
    "ram_total_mb": 16384,
    "last_seen": "2025-07-10T14:30:00Z"
  }
]
```

### POST /api/metrics

Accepts system metrics from agents. This endpoint is primarily used by the agent software.

**Request Format:**
```json
{
  "device_id": "my-server",
  "os_info": "CentOS 8",
  "cpu_usage": 25.3,
  "ram_used_mb": 2048,
  "ram_total_mb": 8192,
  "last_seen": "2025-07-10T14:30:00Z"
}
```

## Development

### Project Structure

The project uses Cargo's workspace feature to organize code into logical modules:

```
tailmon/
├── Cargo.toml              # Workspace configuration
├── common/                 # Shared data structures and types
│   ├── Cargo.toml
│   └── src/lib.rs          # SystemInfo struct definition
├── agent/                  # Agent component
│   ├── Cargo.toml
│   └── src/main.rs         # System monitoring and data collection
├── server/                 # Server component
│   ├── Cargo.toml
│   ├── src/main.rs         # Web server and API endpoints
│   └── static/             # Embedded web assets
│       ├── index.html      # Dashboard HTML
│       ├── style.css       # Dashboard styling
│       └── script.js       # Dashboard JavaScript
```

### Running in Development

```bash
# Start the server in development mode with detailed logging
RUST_LOG=server=debug cargo run --bin server

# Start an agent in development mode
RUST_LOG=agent=debug cargo run --bin agent

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy for additional linting
cargo clippy -- -D warnings
```

### Adding New Metrics

To extend the system with additional metrics, you'll need to modify the `SystemInfo` struct in the common crate:

```rust
// In common/src/lib.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    // Existing fields...
    
    // Add new metrics here
    pub disk_usage_percent: f32,
    pub network_bytes_sent: u64,
    pub uptime_seconds: u64,
}
```

Then update the agent's data collection logic and the dashboard's display logic accordingly.

## Known Limitations and Future Improvements

While Tailmon provides a solid foundation for system monitoring, there are several areas where it can be enhanced for production use:

### Security Considerations

**Current State**: The system currently has no authentication or authorization mechanisms. Any client can send data to the server or access the dashboard.

**Needed Improvements**:
- Implement API key authentication for agent-server communication
- Add user authentication for dashboard access
- Enable HTTPS/TLS encryption for all communications
- Implement rate limiting to prevent abuse
- Add input validation and sanitization to prevent injection attacks

### Data Persistence and Storage

**Current State**: All metrics are stored in memory using DashMap, which means data is lost when the server restarts.

**Needed Improvements**:
- Integrate with a time-series database like InfluxDB or TimescaleDB
- Implement data retention policies for managing storage growth
- Add historical data analysis and trending capabilities
- Create backup and recovery mechanisms for critical monitoring data

### Configuration Management

**Current State**: Very limited configuration options with only one environment variable supported.

**Needed Improvements**:
- Comprehensive configuration file support (YAML/TOML)
- Environment-specific configuration profiles
- Dynamic configuration updates without restarts
- Configurable monitoring intervals and thresholds
- Plugin system for custom metrics collection

### Monitoring and Alerting

**Current State**: The system provides visual indicators but no active alerting mechanisms.

**Needed Improvements**:
- Email and SMS alert notifications
- Integration with popular alerting systems (PagerDuty, Slack)
- Configurable alert thresholds and escalation policies
- Health checks and self-monitoring capabilities
- Integration with metrics aggregation platforms like Prometheus

### Scalability and Performance

**Current State**: Single-server architecture with in-memory storage limits scalability.

**Needed Improvements**:
- Horizontal scaling support with load balancing
- Database connection pooling and optimization
- Caching layers for frequently accessed data
- Clustering support for high availability
- Performance metrics and optimization tooling

### User Experience and Functionality

**Current State**: Basic dashboard with limited functionality.

**Needed Improvements**:
- Historical data visualization with charts and graphs
- Customizable dashboards and metric selection
- Mobile-responsive design improvements
- Export functionality for reports and data analysis
- Multi-tenant support for organizations

### Deployment and Operations

**Current State**: Manual deployment process without containerization.

**Needed Improvements**:
- Docker containerization for easy deployment
- Kubernetes manifests and Helm charts
- Automated CI/CD pipelines
- Comprehensive documentation and deployment guides
- Monitoring and logging best practices

## Contributing

We welcome contributions to Tailmon! Here's how you can help improve the project:

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your feature or bug fix
4. Make your changes with appropriate tests
5. Ensure code follows Rust formatting standards
6. Submit a pull request with a clear description

### Code Style

- Follow Rust's official style guide
- Use `cargo fmt` to format code consistently
- Run `cargo clippy` to catch common issues
- Write meaningful commit messages
- Include tests for new functionality

### Areas Where Help is Needed

- Security implementations (authentication, encryption)
- Database integration and persistence layers
- Dashboard enhancements and new visualizations
- Mobile app development for monitoring on-the-go
- Documentation improvements and tutorials
- Performance optimization and testing

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Support

If you encounter issues or have questions:

1. Check the existing GitHub issues for similar problems
2. Create a new issue with detailed information about your problem
3. Include system information, error messages, and steps to reproduce
4. Tag issues appropriately (bug, enhancement, question)

---

**Note**: Tailmon is currently in active development and should be considered beta software. While it's functional and useful for development and testing environments, additional hardening is recommended before production deployment.# tailmon
