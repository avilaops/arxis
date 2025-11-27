# avx-telemetry

**Observability and tracing layer for Avila Experience Fabric**

[![Crates.io](https://img.shields.io/crates/v/avx-telemetry.svg)](https://crates.io/crates/avx-telemetry)
[![Documentation](https://docs.rs/avx-telemetry/badge.svg)](https://docs.rs/avx-telemetry)
[![License](https://img.shields.io/crates/l/avx-telemetry.svg)](https://github.com/avilaops/arxis#license)

Production-grade observability for Rust applications. Integrates structured logging, distributed tracing, metrics collection, and scientific time series analysis from `avila-telemetry`.

## Features

- **Structured logging**: JSON and pretty-print formats
- **Distributed tracing**: Span hierarchies with context propagation
- **Metrics integration**: Performance counters and gauges
- **HTTP middleware**: Automatic request tracing for axum/tower
- **Scientific analysis**: Leverage `avila-telemetry` for time series anomaly detection
- **Multiple outputs**: Console, file, OpenTelemetry, cloud services
- **Environment-aware**: Development vs production configurations

## Installation

```toml
[dependencies]
avx-telemetry = "0.1"

# For HTTP middleware
avx-telemetry = { version = "0.1", features = ["middleware"] }
```

## Quick Start

### Basic Setup

```rust
use avx_telemetry::init_tracing;

fn main() {
    // Initialize with default settings
    init_tracing();

    tracing::info!("Application started");
}
```

### Structured Logging

```rust
use tracing::{info, warn, error};

#[tracing::instrument]
fn process_data(user_id: &str, count: usize) {
    info!(user_id, count, "Processing data");

    if count > 1000 {
        warn!(count, threshold = 1000, "High data volume");
    }
}

// Output (JSON):
// {"level":"INFO","fields":{"user_id":"123","count":500},"message":"Processing data"}
```

### HTTP Middleware (Axum)

```rust
use axum::{Router, routing::get};
use avx_telemetry::middleware::TraceLayer;

#[tokio::main]
async fn main() {
    avx_telemetry::init_tracing();

    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, AVX!"
}
```

Each request automatically gets:
- Request ID generation
- Timing information
- Status code tracking
- Error logging

### Time Series Analysis

Combine with `avila-telemetry` for scientific-grade analysis:

```rust
use avx_telemetry::TimeSeries;
use avila_telemetry::AnomalyDetector;

#[tracing::instrument]
fn analyze_metrics(values: Vec<f64>) {
    let ts = TimeSeries::new(values);
    let detector = AnomalyDetector::new(3.0, 1.5);

    if let Ok(anomalies) = detector.detect_zscore(&ts) {
        if !anomalies.is_empty() {
            tracing::warn!(
                count = anomalies.len(),
                "Anomalies detected in metrics"
            );
        }
    }
}
```

## Configuration

### Development Mode

```rust
use avx_telemetry::{TracingConfig, Format};

let config = TracingConfig {
    format: Format::Pretty,
    level: "debug",
    ..Default::default()
};

avx_telemetry::init_with_config(config);
```

### Production Mode

```rust
let config = TracingConfig {
    format: Format::Json,
    level: "info",
    output_file: Some("logs/app.log"),
    ..Default::default()
};
```

## Span Context

Create hierarchical traces:

```rust
use tracing::{info_span, info};

fn handle_request(user_id: &str) {
    let _span = info_span!("handle_request", user_id).entered();

    authenticate(user_id);
    fetch_data(user_id);
    process_response();
}

#[tracing::instrument]
fn authenticate(user_id: &str) {
    info!("Authenticating user");
}
```

Output shows parent-child relationship:
```
handle_request{user_id="123"}
  ├─ authenticate{user_id="123"}
  ├─ fetch_data{user_id="123"}
  └─ process_response
```

## Metrics Collection

```rust
use avx_telemetry::metrics;

// Counters
metrics::counter!("requests_total", 1);

// Gauges
metrics::gauge!("active_connections", 42.0);

// Histograms
let start = std::time::Instant::now();
// ... do work ...
metrics::histogram!("request_duration_ms", start.elapsed().as_millis() as f64);
```

## Integration with AVX Ecosystem

Works seamlessly with other AVX components:

```rust
use avx_gateway::Gateway;
use avx_telemetry::middleware::TraceLayer;

let gateway = Gateway::builder()
    .with_telemetry(TraceLayer::new())
    .build();
```

## Features Flags

- **`middleware`**: HTTP tracing middleware for axum/tower
- **`metrics`**: Metrics collection and export

## Part of AVX Ecosystem

`avx-telemetry` is part of the Avila Experience (AVX) platform:

- **avx-gateway**: Uses telemetry for request tracing
- **avx-quantum-render**: Performance diagnostics
- **avila-telemetry**: Scientific time series analysis

## Examples

See the `examples/` directory:

```bash
cargo run --example basic_tracing
cargo run --example http_middleware --features middleware
cargo run --example time_series_integration
```

## License

MIT OR Apache-2.0

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## Links

- **Repository**: https://github.com/avilaops/arxis
- **Documentation**: https://docs.rs/avx-telemetry
- **Crates.io**: https://crates.io/crates/avx-telemetry
- **AVX Platform**: https://avila.inc
- **Documentation**: https://docs.avila.inc
