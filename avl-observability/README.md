# 📊 AVL Observability

**Complete Observability Stack for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-observability.svg)](https://crates.io/crates/avl-observability)
[![Documentation](https://docs.rs/avl-observability/badge.svg)](https://docs.rs/avl-observability)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.inc)

🏛️ **Complete Visibility** | ⚙️ **Real-Time Monitoring** | 📈 **Metrics, Logs, Traces**

---

## Features

- **Metrics**: Prometheus-compatible metrics collection
- **Logs**: Centralized structured logging
- **Traces**: Distributed tracing (OpenTelemetry)
- **Dashboards**: Real-time visualization
- **Alerts**: Configurable alerting rules
- **Integration**: Works with all AVL services

## Quick Start

```rust
use avl_observability::{metrics, tracing};

#[tokio::main]
async fn main() {
    // Initialize observability
    avl_observability::init().await?;

    // Record metrics
    metrics::counter!("requests_total", 1);
    metrics::gauge!("memory_usage_bytes", 1024*1024);

    // Structured logging
    tracing::info!(
        user_id = "user123",
        action = "login",
        "User logged in successfully"
    );

    // Distributed tracing
    let span = tracing::span!(tracing::Level::INFO, "process_order");
    let _enter = span.enter();
    // ... process order
}
```

## Architecture

- **Metrics Collection**: Prometheus exposition format
- **Log Aggregation**: JSON structured logs
- **Trace Context**: W3C Trace Context propagation
- **Storage**: AvilaDB for long-term retention
- **Query API**: PromQL-compatible queries

🏛️ **Built by Avila** - Part of AVL Cloud Platform
