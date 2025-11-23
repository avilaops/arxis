# AVX Gateway - Development Guide

## Project Structure

```
avx-gateway/
├── src/
│   ├── lib.rs              # Library entry point & public API
│   ├── main.rs             # Binary entry point
│   ├── gateway.rs          # Core gateway implementation
│   ├── routing.rs          # Route matching and URL processing
│   ├── config.rs           # Configuration types and defaults
│   ├── error.rs            # Error types and handling
│   ├── health.rs           # Health check endpoints
│   ├── metrics.rs          # Metrics collection (Prometheus-compatible)
│   ├── load_balancer.rs    # Load balancing strategies
│   ├── circuit_breaker.rs  # Circuit breaker pattern
│   ├── auth.rs             # Authentication module
│   │   ├── jwt.rs          # JWT authentication
│   │   └── api_key.rs      # API key authentication
│   └── middleware/         # Middleware components
│       ├── mod.rs
│       ├── cors.rs         # CORS handling
│       ├── logging.rs      # Request logging
│       ├── rate_limit.rs   # Rate limiting (token bucket)
│       └── timeout.rs      # Request timeout
├── examples/               # Usage examples
│   ├── basic_gateway.rs
│   ├── with_auth.rs
│   └── load_balancing.rs
├── config/                 # Configuration files
│   └── gateway.toml
└── Cargo.toml
```

## Architecture

### Core Components

1. **Gateway** - Main orchestrator
   - Builder pattern for configuration
   - Manages routes, health checks, metrics
   - Spawns Axum server with middleware stack

2. **Router** - Request routing
   - Path pattern matching (exact, wildcard, parameters)
   - Route matching algorithm
   - Path processing (strip prefix, etc.)

3. **Middleware Stack**
   - Logging (request/response tracing)
   - Rate limiting (token bucket algorithm)
   - CORS (configurable origins/methods)
   - Timeout (per-route or global)
   - Authentication (JWT, API keys)

4. **Load Balancer**
   - Round-robin
   - Random selection
   - Least connections
   - Weighted distribution (planned)

5. **Circuit Breaker**
   - States: Closed, Open, Half-Open
   - Automatic failure detection
   - Configurable thresholds
   - Recovery mechanism

6. **Health Checks**
   - Upstream service monitoring
   - `/health` - Overall health status
   - `/healthz` - Liveness probe
   - `/ready` - Readiness probe

7. **Metrics**
   - Prometheus-compatible format
   - Request counts by status
   - Latency tracking
   - Active connections
   - Bandwidth usage

## Usage Examples

### Basic Gateway

```rust
use avx_gateway::Gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let gateway = Gateway::builder()
        .route("/api/users", "http://user-service:8001")
        .route("/api/products", "http://product-service:8002")
        .with_port(8080)
        .build()
        .await?;

    gateway.serve().await?;
    Ok(())
}
```

### With Configuration File

```rust
let config = GatewayConfig::load("config/gateway.toml")?;
let gateway = Gateway::from_config(config).await?;
gateway.serve().await?;
```

### With Middleware

```rust
let gateway = Gateway::builder()
    .route("/api/*", "http://service:8000")
    .with_rate_limit(100)  // 100 req/s
    .with_cors(true)
    .with_timeout(Duration::from_secs(30))
    .build()
    .await?;
```

## Configuration Format

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
timeout_ms = 30000

[[routes]]
path = "/api/users/*"
strip_path = true
methods = ["GET", "POST", "PUT", "DELETE"]

[routes.upstream]
urls = ["http://service1:8001", "http://service2:8002"]
strategy = "round_robin"

[middleware]
enable_cors = true
enable_rate_limiting = true

[rate_limiting]
requests_per_second = 100
burst_size = 200
```

## Performance Characteristics

- **Throughput**: 50,000+ req/s (simple proxy)
- **Latency (p50)**: < 1ms
- **Latency (p99)**: < 5ms
- **Memory**: ~50MB baseline
- **CPU**: Scales linearly with cores

## Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test --package avx-gateway routing

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Development Workflow

1. Make changes to source files
2. Run `cargo check` for quick syntax validation
3. Run `cargo test` to ensure tests pass
4. Run `cargo clippy` for linting
5. Run `cargo fmt` for formatting
6. Build with `cargo build --release` for production

## Deployment

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/avx-gateway /usr/local/bin/
EXPOSE 8080
CMD ["avx-gateway"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: avx-gateway
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: gateway
        image: avx-gateway:latest
        ports:
        - containerPort: 8080
        livenessProbe:
          httpGet:
            path: /healthz
            port: 8080
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
```

## Roadmap

- [ ] WebSocket proxying
- [ ] gRPC support
- [ ] Request/response transformation
- [ ] Caching layer
- [ ] OAuth2 integration
- [ ] Mutual TLS (mTLS)
- [ ] Request validation (JSON Schema)
- [ ] GraphQL support
- [ ] Plugin system
- [ ] Admin API for runtime configuration

## Contributing

This is part of the AVX (Avila Experience) platform. All code must be:
- 100% Rust (no unsafe code)
- Well-documented
- Thoroughly tested
- Production-ready quality

## License

MIT OR Apache-2.0
