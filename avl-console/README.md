# 🖥️ AVL Console

**World-Class Developer Portal and Web Dashboard for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-console.svg)](https://crates.io/crates/avl-console)
[![Documentation](https://docs.rs/avl-console/badge.svg)](https://docs.rs/avl-console)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

🏛️ **Complete Control** | ⚙️ **Intuitive UI** | 📊 **Real-Time Monitoring** | 🚀 **Sub-10ms Latency**

---

## ✨ Features

### 🎯 Core Capabilities

- **📊 Real-Time Dashboard**: Live metrics, resource overview, activity feed with WebSocket updates
- **🗄️ AvilaDB Explorer**: Interactive query editor, document browser, schema visualization
- **💾 Storage Browser**: S3-compatible file management with drag-and-drop uploads
- **📈 Observability Suite**: Metrics, logs, distributed traces with advanced filtering
- **💰 Billing & Cost Tracking**: Usage analytics, cost breakdown, budget alerts
- **🔐 Security**: JWT authentication, RBAC, audit logs, rate limiting
- **🌍 Multi-Region**: Global deployment support with region-aware routing
- **⚡ Performance**: Sub-10ms latency in Brazil, optimized for LATAM

### 🚀 Advanced Features (NEW!)

- **🎨 Visual Query Builder**: Drag-and-drop SQL query constructor with real-time generation
- **🔬 Advanced Monitoring**: ML-powered anomaly detection, predictive insights, smart alerts
- **👥 Team Management**: Enterprise RBAC with 7 granular permissions, audit log, user invitations

> 📖 **Learn More**: See [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) for detailed documentation

### 🛠️ Developer Experience

- **REST API**: Comprehensive API with OpenAPI/Swagger documentation
- **WebSocket**: Real-time updates for dashboard and logs
- **CLI Integration**: Works seamlessly with `avl` CLI
- **Templates**: Server-side rendered with Askama
- **Responsive**: Mobile-first design with dark mode support
- **i18n**: Full support for pt-BR and en-US

## 🏗️ Architecture

```text
┌───────────────────────────────────────────────────────┐
│              AVL Console Frontend (SSR)               │
│     Modern UI with WebSocket Real-Time Updates        │
└───────────────────────────────────────────────────────┘
                           ↓
┌───────────────────────────────────────────────────────┐
│           Axum REST API + WebSocket Server            │
│   Auth • Rate Limiting • CORS • Compression • Trace   │
└───────────────────────────────────────────────────────┘
                           ↓
┌──────────────┬──────────────┬──────────────┬─────────┐
│   AvilaDB    │   Storage    │ Observability│ Billing │
│   Explorer   │   Browser    │   Dashboard  │ Tracker │
└──────────────┴──────────────┴──────────────┴─────────┘
```

## 🚀 Quick Start

### Installation

```bash
# Add to Cargo.toml
[dependencies]
avl-console = "0.1"
```

### Basic Usage

```rust
use avl_console::{Console, ConsoleConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from environment
    let config = ConsoleConfig::from_env()?;

    // Create and start console
    let console = Console::new(config).await?;
    console.serve("127.0.0.1:8080").await?;

    Ok(())
}
```

### Run Example

```bash
# Start the console server
cargo run --example basic

# Or with custom configuration
AVL_CONSOLE_PORT=3000 cargo run --example basic
```

### Access Console

```
🖥️  Console: http://localhost:8080/dashboard
🗄️  AvilaDB:  http://localhost:8080/databases
💾 Storage:  http://localhost:8080/storage
📈 Metrics:  http://localhost:8080/observability
💰 Billing:  http://localhost:8080/billing
```

## ⚙️ Configuration

### Environment Variables

```bash
# Server
AVL_CONSOLE_BIND=127.0.0.1
AVL_CONSOLE_PORT=8080
AVL_CONSOLE_DEBUG=false

# Endpoints
AVL_AUTH_ENDPOINT=http://localhost:8001
AVL_AVILADB_ENDPOINT=http://localhost:8000
AVL_STORAGE_ENDPOINT=http://localhost:8002
AVL_OBSERVABILITY_ENDPOINT=http://localhost:8003

# Security
AVL_CONSOLE_SECRET=your-secret-key-change-in-production
AVL_CONSOLE_CORS_ORIGINS=http://localhost:8080

# Rate Limiting
AVL_CONSOLE_RATE_LIMIT=100  # requests per minute
```

### Programmatic Configuration

```rust
use avl_console::ConsoleConfig;

let config = ConsoleConfig {
    bind_address: "0.0.0.0".to_string(),
    port: 8080,
    debug: true,
    rate_limit: 100,
    max_ws_connections: 10,
    ..Default::default()
};
```

## 📊 Features in Detail

### Dashboard

- **Resource Overview**: Databases, storage, queues, functions
- **Real-Time Metrics**: CPU, memory, requests/sec, error rate
- **Activity Feed**: Recent operations and events
- **Health Status**: Service status indicators
- **Cost Summary**: Current month usage and projections

### AvilaDB Explorer

- **Query Editor**: Syntax highlighting, auto-complete
- **Document Browser**: JSON viewer with search and filters
- **Schema Visualization**: Collection structure and indexes
- **Performance Metrics**: Query execution time and RU consumption
- **Batch Operations**: Import/export data in JSON/CSV

### Storage Browser

- **File Management**: Upload, download, delete, rename
- **Folder Navigation**: Hierarchical bucket browsing
- **Presigned URLs**: Generate temporary access links
- **Metadata Editor**: Set content-type, cache-control, etc.
- **Batch Operations**: Multi-file uploads with progress

### Observability

- **Metrics Dashboard**: Time-series charts with Chart.js
- **Log Viewer**: Real-time logs with search and filters
- **Distributed Tracing**: Request flow visualization
- **Alert Configuration**: Custom alerts on metrics
- **Export**: Download logs and metrics in various formats

### Billing

- **Usage Dashboard**: Current month costs and trends
- **Service Breakdown**: Cost per service (DB, storage, etc.)
- **Invoice History**: Past invoices with download
- **Budget Alerts**: Email notifications on thresholds
- **Cost Estimation**: Projected costs based on usage

## 🔒 Security

- **Authentication**: JWT-based session management
- **Authorization**: Role-based access control (RBAC)
- **Rate Limiting**: Per-user request throttling
- **CORS**: Configurable cross-origin policies
- **Audit Logs**: Complete action history
- **Secrets Management**: Secure credential storage

## 🌐 API Reference

### Health Check

```bash
GET /api/health
```

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "services": {
    "aviladb": true,
    "storage": true,
    "observability": true
  }
}
```

### Authentication

```bash
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password"
}
```

### List Databases

```bash
GET /databases/list
Authorization: Cookie avl_session=xxx
```

### Execute Query

```bash
POST /databases/{db_id}/query
Content-Type: application/json

{
  "query": "SELECT * FROM users WHERE active = true"
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## 📦 Building

```bash
# Development build
cargo build

# Production build (optimized)
cargo build --release

# Build for specific target
cargo build --target x86_64-unknown-linux-gnu --release
```

## 🐳 Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/avl-console /usr/local/bin/
EXPOSE 8080
CMD ["avl-console"]
```

```bash
docker build -t avl-console .
docker run -p 8080:8080 -e AVL_CONSOLE_SECRET=mysecret avl-console
```

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md).

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -am 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🏛️ Built by Avila

Part of the **AVL Cloud Platform** - Brazil's premier cloud infrastructure.

- **Website**: https://avila.cloud
- **Documentation**: https://docs.avila.cloud
- **Support**: support@avila.cloud
- **Community**: https://discord.gg/avilacloud

---

**AVL Console** - Complete control over your cloud infrastructure 🚀

