# ⚖️ AVL LoadBalancer

**L7 Load Balancer and Reverse Proxy for AVL Cloud Platform**

[![Crates.io](https://img.shields.io/crates/v/avl-loadbalancer.svg)](https://crates.io/crates/avl-loadbalancer)
[![Documentation](https://docs.rs/avl-loadbalancer/badge.svg)](https://docs.rs/avl-loadbalancer)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)

🏛️ **High Availability** | ⚙️ **Smart Routing** | 🚀 **TLS Termination**

---

## Features

- **L7 Load Balancing**: HTTP/HTTPS traffic distribution
- **Active Health Checks**: Automatic backend health monitoring with configurable intervals
- **Health Status API**: Built-in `/_health` endpoint for monitoring
- **TLS Termination**: SSL/TLS offloading (coming soon)
- **Rate Limiting**: Per-IP, per-user rate limits (planned)
- **Geographic Routing**: Route based on client location (planned)
- **WebSocket Support**: Long-lived connection proxying (planned)

## Quick Start

```rust
use avl_loadbalancer::{LoadBalancer, Backend, HealthCheck, Algorithm};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let lb = LoadBalancer::builder()
        .add_backend(Backend::new("http://server1:8000"))
        .add_backend(Backend::new("http://server2:8000"))
        .health_check(
            HealthCheck::http("/health")
                .interval(Duration::from_secs(10))
                .timeout(Duration::from_secs(5))
        )
        .algorithm(Algorithm::RoundRobin)
        .build();

    lb.listen("0.0.0.0:80").await.unwrap();
}
```

## Status

**Current Implementation:**

✅ Round-robin load balancing
✅ Active health checks with configurable intervals
✅ Automatic unhealthy backend filtering
✅ Health status monitoring endpoint (`/_health`)
✅ Graceful fallback when backends fail

**Coming Soon:**

- Least Connections, IP Hash, and Weighted algorithms
- TLS termination (requires build dependencies on Windows)
- Rate limiting integration
- Geographic routing
- WebSocket upgrade handling

Early feedback welcome. Expect rapid iteration.

## Algorithms

- **Round Robin**: Equal distribution
- **Least Connections**: Route to least busy backend (planned)
- **IP Hash**: Consistent routing per IP (planned)
- **Weighted**: Priority-based routing (planned)

## Health Monitoring

**Active Health Checks**: The load balancer periodically probes backends at a configured HTTP endpoint. Unhealthy backends are automatically removed from rotation.

**Monitoring Endpoint**: Access `/_health` on the load balancer to view:
- Overall health status
- Individual backend health states
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
    {"url": "http://server1:8000", "healthy": true},
    {"url": "http://server2:8000", "healthy": false}
  ],
  "healthy_count": 1,
  "total_count": 2
}
```

🏛️ **Built by Avila** - Part of AVL Cloud Platform
