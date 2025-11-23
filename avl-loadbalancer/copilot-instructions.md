# AVL LoadBalancer - Copilot Instructions

## Project Identity
**AVL LoadBalancer** is the **L7 load balancer and reverse proxy** for AVL Cloud. Embodies Arxis philosophy:
- **ARX (Fortress)**: High availability, health checks, automatic failover
- **AXIS (Engine)**: Smart routing, TLS termination, low latency

## Core Principles
```rust
// ✅ ALWAYS: Health check backends before routing
// ✅ ALWAYS: Terminate TLS at load balancer (offload backends)
// ✅ ALWAYS: Rate limit per IP/user to prevent abuse
// ✅ ALWAYS: Log routing decisions to avx-telemetry
// ✅ NEVER: Route to unhealthy backends
// ✅ NEVER: Expose backend errors to clients (sanitize)
```

## Load Balancing Algorithms

### 1. Round Robin
```rust
// Distribute evenly across all healthy backends
Algorithm::RoundRobin
```

### 2. Least Connections
```rust
// Route to backend with fewest active connections
Algorithm::LeastConnections
```

### 3. IP Hash
```rust
// Consistent routing based on client IP (sticky sessions)
Algorithm::IpHash
```

### 4. Weighted
```rust
// Priority-based routing (backend1: 70%, backend2: 30%)
Algorithm::Weighted(vec![(backend1, 70), (backend2, 30)])
```

## Health Checks
- **HTTP**: GET /health, expect 200 OK
- **TCP**: Connection test on port
- **Interval**: 10 seconds (configurable)
- **Timeout**: 5 seconds
- **Threshold**: 3 consecutive failures = unhealthy

## Features
- **TLS Termination**: rustls for TLS 1.3
- **WebSocket Support**: Long-lived connection proxying
- **Rate Limiting**: Token bucket algorithm via `governor`
- **Circuit Breaker**: Prevent cascade failures
- **Geo Routing**: Route based on client location (Brazil priority)

## Related Crates
- **avx-gateway**: API gateway integration
- **tower**: Service middleware
- **hyper**: HTTP client/server
- **rustls**: TLS implementation
- **avx-telemetry**: Routing metrics

🏛️ Built by Avila | ⚖️ High Availability | ⚙️ Smart Routing
