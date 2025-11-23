# ⚖️ AVL LoadBalancer

**L7 Load Balancer and Reverse Proxy for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-loadbalancer.svg)](https://crates.io/crates/avl-loadbalancer)
[![Documentation](https://docs.rs/avl-loadbalancer/badge.svg)](https://docs.rs/avl-loadbalancer)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)

🏛️ **High Availability** | ⚙️ **Smart Routing** | 🚀 **TLS Termination**

---

## Features

- **L7 Load Balancing**: HTTP/HTTPS traffic distribution
- **Health Checks**: Automatic backend health monitoring
- **TLS Termination**: SSL/TLS offloading
- **Rate Limiting**: Per-IP, per-user rate limits
- **Geographic Routing**: Route based on client location
- **WebSocket Support**: Long-lived connection proxying

## Quick Start

```rust
use avl_loadbalancer::{LoadBalancer, Backend, HealthCheck};

#[tokio::main]
async fn main() {
    let lb = LoadBalancer::builder()
        .add_backend(Backend::new("http://server1:8000"))
        .add_backend(Backend::new("http://server2:8000"))
        .health_check(HealthCheck::http("/health"))
        .algorithm(Algorithm::RoundRobin)
        .build();

    lb.listen("0.0.0.0:80").await?;
}
```

## Status

This crate is currently an MVP implementation:

- Only the `RoundRobin` algorithm is functionally implemented (others fall back to it).
- Health checks are passive (no active probing yet).
- TLS termination is not yet enabled on Windows builds (requires installing build dependencies and re-enabling `rustls`).
- Rate limiting, geo routing, WebSocket upgrade handling are planned.

Early feedback is welcome. Expect rapid iteration.

## Algorithms

- **Round Robin**: Equal distribution
- **Least Connections**: Route to least busy backend
- **IP Hash**: Consistent routing per IP
- **Weighted**: Priority-based routing

🏛️ **Built by Avila** - Part of AVL Cloud Platform
