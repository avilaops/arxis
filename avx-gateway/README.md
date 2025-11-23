# avx-gateway

**High-performance API gateway for Avila Experience Fabric**

[![Crates.io](https://img.shields.io/crates/v/avx-gateway.svg)](https://crates.io/crates/avx-gateway)
[![Documentation](https://docs.rs/avx-gateway/badge.svg)](https://docs.rs/avx-gateway)
[![License](https://img.shields.io/crates/l/avx-gateway.svg)](https://github.com/avilaops/arxis#license)

Production-ready API gateway built with Axum and Tower. Handles routing, authentication, rate limiting, load balancing, and observability for microservices architectures.

## Features

### Core Features
- **High Performance**: Built on Tokio + Axum for async I/O (50,000+ req/s)
- **Routing**: Dynamic route configuration with path parameters and wildcards
- **Load Balancing**: Round-robin, least connections, random, weighted strategies
- **Circuit Breaker**: Automatic failure detection and recovery with half-open state
- **Rate Limiting**: Token bucket algorithm with configurable burst
- **Health Checks**: Automatic service health monitoring with readiness probes

### Advanced Features
- **Response Caching**: In-memory cache with TTL, size limits, and strategies
- **Request/Response Transformation**: Header manipulation, path rewriting, status mapping
- **Retry Logic**: Exponential backoff with jitter and customizable policies
- **Compression**: Gzip compression with content-type filtering
- **WebSocket Proxying**: Full-duplex WebSocket connection forwarding
- **Authentication**: JWT validation and API key authentication
- **Observability**: Prometheus metrics, structured logging, distributed tracing
- **TLS/HTTPS**: Production-grade security (configurable)

## Installation

```toml
[dependencies]
avx-gateway = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Basic Gateway

```rust
use avx_gateway::{Gateway, RouteConfig};

#[tokio::main]
async fn main() {
    let gateway = Gateway::builder()
        .route("/api/users", "http://user-service:8001")
        .route("/api/products", "http://product-service:8002")
        .with_port(8080)
        .build()
        .await
        .unwrap();

    gateway.serve().await.unwrap();
}
```

### With Configuration

```rust
use avx_gateway::{Gateway, GatewayConfig};
use avx_config::Config;

#[tokio::main]
async fn main() {
    let config: GatewayConfig = Config::load("config/gateway.toml")
        .expect("Failed to load config");

    let gateway = Gateway::from_config(config)
        .await
        .unwrap();

    gateway.serve().await.unwrap();
}
```

### Configuration File

```toml
# config/gateway.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[[routes]]
path = "/api/users/*"
upstream = "http://user-service:8001"
strip_path = true
methods = ["GET", "POST", "PUT", "DELETE"]

[[routes]]
path = "/api/products/*"
upstream = "http://product-service:8002"
timeout_ms = 5000

[middleware]
enable_cors = true
enable_compression = true
enable_rate_limiting = true

[rate_limiting]
requests_per_second = 100
burst_size = 20

[health_check]
enabled = true
interval_seconds = 30
timeout_ms = 3000
```

## Features

### Authentication

```rust
use avx_gateway::middleware::{AuthLayer, JwtValidator};

let gateway = Gateway::builder()
    .route("/api/public", "http://service:8000")
    .route("/api/private", "http://service:8000")
    .layer(
        "/api/private",
        AuthLayer::jwt(JwtValidator::new("secret"))
    )
    .build()
    .await?;
```

### Rate Limiting

```rust
use avx_gateway::middleware::RateLimitLayer;

let gateway = Gateway::builder()
    .route("/api", "http://service:8000")
    .layer(
        "/api",
        RateLimitLayer::new(100) // 100 requests per second
            .burst(20)
    )
    .build()
    .await?;
```

### Load Balancing

```rust
use avx_gateway::{LoadBalancer, Strategy};

let gateway = Gateway::builder()
    .route("/api", LoadBalancer::new()
        .strategy(Strategy::RoundRobin)
        .upstream("http://service-1:8001")
        .upstream("http://service-2:8002")
        .upstream("http://service-3:8003")
    )
    .build()
    .await?;
```

### Circuit Breaker

```rust
use avx_gateway::middleware::CircuitBreakerLayer;

let gateway = Gateway::builder()
    .route("/api", "http://service:8000")
    .layer(
        "/api",
        CircuitBreakerLayer::new()
            .failure_threshold(5)
            .timeout_duration(Duration::from_secs(60))
    )
    .build()
    .await?;
```

### Request/Response Transformation

```rust
use avx_gateway::transform::{RequestTransform, ResponseTransform};

let gateway = Gateway::builder()
    .route("/api", "http://service:8000")
    .transform(
        RequestTransform::add_header("X-Gateway-Id", "avx-gateway-1")
    )
    .transform(
        ResponseTransform::remove_header("X-Internal-Detail")
    )
    .build()
    .await?;
```

## Observability

Full integration with `avx-telemetry`:

```rust
use avx_gateway::Gateway;
use avx_telemetry::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();

    let gateway = Gateway::builder()
        .with_telemetry() // Automatic request tracing
        .route("/api", "http://service:8000")
        .build()
        .await
        .unwrap();

    gateway.serve().await.unwrap();
}
```

Automatically logs:
- Request ID
- Method and path
- Response status
- Latency
- Upstream service
- Error details

## Health Checks

```rust
let gateway = Gateway::builder()
    .route("/api", "http://service:8000")
    .health_check("/health")
    .readiness_check("/ready")
    .build()
    .await?;

// GET /health -> {"status": "healthy", "upstreams": {"service": "up"}}
// GET /ready -> {"ready": true}
```

## WebSocket Proxying

```rust
let gateway = Gateway::builder()
    .websocket("/ws", "ws://realtime-service:8080")
    .build()
    .await?;
```

## TLS/HTTPS

```rust
use avx_gateway::tls::TlsConfig;

let gateway = Gateway::builder()
    .route("/api", "http://service:8000")
    .tls(TlsConfig {
        cert_path: "certs/cert.pem",
        key_path: "certs/key.pem",
    })
    .build()
    .await?;
```

## Metrics

Exposed Prometheus-compatible metrics:

- `gateway_requests_total{method,path,status}`
- `gateway_request_duration_seconds{method,path}`
- `gateway_active_connections`
- `gateway_upstream_status{upstream,status}`

```bash
curl http://localhost:8080/metrics
```

## Part of AVX Ecosystem

`avx-gateway` is the entry point for the Avila Experience (AVX) platform:

- **avx-config**: Configuration management
- **avx-telemetry**: Observability and tracing
- **avx-api-core**: Core API types and utilities

## Examples

Run the included examples:

```bash
# Basic gateway with routing
cargo run --example basic_gateway

# Gateway with authentication
cargo run --example with_auth

# Load balancing across multiple upstreams
cargo run --example load_balancing

# Advanced features (caching, retry, compression)
cargo run --example advanced_features

# WebSocket proxying
cargo run --example websocket_proxy

# Response caching
cargo run --example with_caching
```

## Benchmarks

Run performance benchmarks:

```bash
cargo bench
```

## Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# All tests with coverage
cargo test --all-features
```

## Performance

Benchmarks on AMD Ryzen 9 5900X:

- **Throughput**: 50,000+ req/s (simple proxy)
- **Latency (p50)**: < 1ms
- **Latency (p99)**: < 5ms
- **Memory**: ~50MB baseline

## License

MIT OR Apache-2.0

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## Links

- **Repository**: https://github.com/avilaops/arxis
- **Documentation**: https://docs.rs/avx-gateway
- **Crates.io**: https://crates.io/crates/avx-gateway
- **AVX Platform**: https://avilaops.com
