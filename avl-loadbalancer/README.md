# ⚖️ AVL LoadBalancer v0.2.0

**Production-Grade L7 Load Balancer with TLS, Sticky Sessions, Geo-Routing & Distributed Tracing**

[![Crates.io](https://img.shields.io/crates/v/avl-loadbalancer.svg)](https://crates.io/crates/avl-loadbalancer)
[![Documentation](https://docs.rs/avl-loadbalancer/badge.svg)](https://docs.rs/avl-loadbalancer)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.inc)

🏛️ **High Availability** | ⚙️ **Smart Routing** | 🔒 **TLS/Circuit Breakers** | 📊 **Distributed Tracing** | 🌍 **Geo-Routing** | 🍪 **Sticky Sessions**

---

## Features

### Core Load Balancing
- **Multiple Load Balancing Algorithms**: Round-robin, least connections, IP hash, weighted distribution
- **Active Health Checks**: Automatic backend health monitoring with configurable intervals
- **Circuit Breakers**: Per-backend failure detection with automatic recovery
- **Retry Logic**: Automatic retry with exponential backoff for failed requests
- **Connection Tracking**: Real-time active connection monitoring per backend

### Security & Performance
- **TLS/SSL Termination**: HTTPS support with rustls for secure connections
- **Rate Limiting**: Per-IP token bucket rate limiting with configurable burst
- **Response Compression**: Automatic gzip/brotli compression (>1KB, only if smaller)
- **Security Headers**: Automatic injection of security headers (HSTS, X-Frame-Options, CSP)

### Advanced Features (Level 4.0)
- **🍪 Sticky Sessions**: Session affinity with HMAC-signed cookies for consistent routing
- **🌍 Geo-Routing**: Geographic-based routing using MaxMind GeoIP2 for optimal latency
- **🔄 Hot-Reload**: Configuration file hot-reload without downtime
- **📊 Distributed Tracing**: OpenTelemetry integration for full request tracing
- **🔌 Middleware Pipeline**: Extensible request/response transformation pipeline
- **⚡ Graceful Shutdown**: SIGTERM/SIGINT handling with connection draining

### Monitoring & Observability
- **Metrics Endpoint**: Built-in `/_metrics` with request counts, failure rates, and success rates
- **Health Status API**: Built-in `/_health` endpoint for monitoring
- **OpenTelemetry**: Distributed tracing with trace context propagation

### WebRTC & Real-time
- **WebSocket Support**: Long-lived bidirectional connection proxying at `/_ws` endpoint
- **STUN/TURN Servers**: WebRTC NAT traversal for peer-to-peer connections

### Configuration
- **Configuration Files**: YAML/TOML configuration loading with hot-reload
- **AVL Platform Integration**: Uses native AVL workspace crates (`avila-compress`, `avx-http`, `avx-config`)

## AVL Platform Dependencies

This loadbalancer is fully integrated with the **AVL Cloud Platform** ecosystem:

- **`avila-compress`** - Native compression library (LZ4, Zstandard) optimized for AvilaDB
- **`avx-http`** - Native HTTP client/server with Brazilian infrastructure optimization
- **`avx-config`** - Type-safe configuration management from multiple sources
- **`avx-telemetry`** - Observability and monitoring integration

🇧🇷 **Optimized for Brazil**: 5-10ms latency in Brazilian datacenters, 40-60% cheaper than AWS/Azure for LATAM workloads.

## Quick Start

```rust
use avl_loadbalancer::{LoadBalancer, Backend, HealthCheck, Algorithm, RateLimitConfig};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let lb = LoadBalancer::builder()
        .add_backend(Backend::new("http://server1:8000").with_weight(3))
        .add_backend(Backend::new("http://server2:8000").with_weight(1))
        .health_check(
            HealthCheck::http("/health")
                .interval(Duration::from_secs(10))
                .timeout(Duration::from_secs(5))
        )
        .algorithm(Algorithm::Weighted)
        .rate_limit(RateLimitConfig::new(100).burst(200)) // 100 req/s with 200 burst
        .build();

    lb.listen("0.0.0.0:80").await.unwrap();
}
```

## Configuration File (YAML)

```yaml
listen: "0.0.0.0:8080"
algorithm: Weighted

backends:
  - url: "http://192.168.1.10:8000"
    weight: 3
  - url: "http://192.168.1.11:8000"
    weight: 1

health_check:
  path: "/health"
  interval_secs: 10
  timeout_secs: 5

rate_limit:
  requests_per_second: 100
  burst: 200

retry:
  max_retries: 3
  backoff_ms: 100

max_request_body_mb: 10
```

See `config.example.yaml` for full configuration reference.

## Status

**v0.2.0 - Production Ready (Level 4.0):**

✅ Round-robin load balancing
✅ Least connections algorithm
✅ IP hash (consistent hashing)
✅ Weighted distribution
✅ Active health checks with configurable intervals
✅ Circuit breakers per backend (opens at >50% failure rate)
✅ Per-IP rate limiting with token bucket
✅ Automatic retry with configurable backoff
✅ Connection tracking per backend
✅ WebSocket bidirectional proxy at `/_ws`
✅ Response compression (gzip/brotli, automatic)
✅ YAML/TOML configuration loading
✅ Metrics endpoint (`/_metrics`)
✅ Health status monitoring endpoint (`/_health`)
✅ STUN server for NAT discovery
✅ TURN server for WebRTC relay
✅ **TLS/SSL termination** with rustls
✅ **Sticky sessions** (session affinity) with HMAC-signed cookies
✅ **Geo-routing** with MaxMind GeoIP2 database
✅ **Hot-reload** of configuration files
✅ **Middleware pipeline** for request/response transformation
✅ **Distributed tracing** with OpenTelemetry
✅ **Graceful shutdown** with SIGTERM/SIGINT handling
✅ **AVL Platform integration** (workspace crates)
✅ Graceful fallback when backends fail

**Future Enhancements:**

- Service mesh integration
- Dynamic backend discovery (Consul, etcd)
- Advanced traffic shaping (canary deployments, blue-green)
- AvilaDB backend health storage
- gRPC load balancing support

## Algorithms

- **Round Robin**: Equal distribution - cycles through backends sequentially
- **Least Connections**: Route to backend with fewest active connections
- **IP Hash**: Consistent hashing based on client IP - same IP always routes to same backend
- **Weighted**: Priority-based routing - backends with higher weight receive proportionally more traffic

## Health Monitoring

**Active Health Checks**: The load balancer periodically probes backends at a configured HTTP endpoint. Unhealthy backends are automatically removed from rotation.

**Circuit Breakers**: If a backend exceeds 50% failure rate over 100 requests, the circuit breaker opens for 30 seconds. After the timeout, it enters half-open state to test recovery.

**Monitoring Endpoint**: Access `/_health` on the load balancer to view:
- Overall health status
- Individual backend health states, circuit state, weight, active connections
- Healthy/total backend counts

**Example**:
```bash
curl http://localhost:8080/_health
```

**Response**:
```json
{
  "healthy": true,
  "backends": [
    {
      "url": "http://server1:8000",
      "healthy": true,
      "circuit_state": "Closed",
      "weight": 3,
      "active_connections": 12
    },
    {
      "url": "http://server2:8000",
      "healthy": false,
      "circuit_state": "Open",
      "weight": 1,
      "active_connections": 0
    }
  ],
  "healthy_count": 1,
  "total_count": 2
}
```

## Metrics

Access `/_metrics` for detailed performance statistics:

```bash
curl http://localhost:8080/_metrics
```

**Response**:
```json
{
  "total_requests": 15234,
  "total_failures": 42,
  "success_rate": 99.72,
  "backends": [
    {
      "url": "http://server1:8000",
      "total_requests": 11425,
      "failed_requests": 8,
      "success_rate": 99.93,
      "active_connections": 12
    },
    {
      "url": "http://server2:8000",
      "total_requests": 3809,
      "failed_requests": 34,
      "success_rate": 99.11,
      "active_connections": 3
    }
  ]
}
```

## Testing

```bash
cargo test --lib
```

All tests include:
- Basic proxy functionality
- Round-robin algorithm
- Least connections algorithm
- IP hash consistency
- Weighted distribution
- Health check marking
- Circuit breaker behavior
- Rate limiting enforcement
- Retry logic with exponential backoff
- Metrics endpoint validation

🏛️ **Built by Avila** - Part of AVL Cloud Platform
