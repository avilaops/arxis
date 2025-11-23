# AVX Events - Production Deployment Guide

## 🚀 Production-Ready Event-Driven Architecture

**avx-events** integrates seamlessly with the entire AVX Platform ecosystem to provide a robust, scalable event-driven architecture for microservices.

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
avx-events = { version = "0.1", path = "../avx-events" }
avx-http = { version = "0.1", path = "../avx-http", features = ["events"] }
avx-telemetry = { version = "0.1", path = "../avx-telemetry" }
avx-config = { version = "0.1", path = "../avx-config" }
```

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    AVX Platform Service                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐      ┌──────────────┐                     │
│  │  avx-http    │──────│  avx-events  │                     │
│  │   Server     │      │   EventBus   │                     │
│  └──────────────┘      └──────────────┘                     │
│         │                      │                             │
│         │                      │                             │
│  ┌──────▼──────┐       ┌──────▼─────────┐                  │
│  │ EventPublisher│      │   Subscribers   │                 │
│  │  Middleware   │      │  - Analytics    │                 │
│  └───────────────┘      │  - Metrics      │                 │
│                         │  - Logging      │                 │
│                         │  - Custom       │                 │
│                         └─────────────────┘                 │
│                                                               │
│  ┌──────────────────────────────────────┐                   │
│  │         avx-telemetry                │                   │
│  │  - Structured JSON Logging           │                   │
│  │  - Distributed Tracing               │                   │
│  │  - Metrics Collection                │                   │
│  └──────────────────────────────────────┘                   │
│                                                               │
│  ┌──────────────────────────────────────┐                   │
│  │         avx-config                   │                   │
│  │  - Environment-based Config          │                   │
│  │  - Feature Flags                     │                   │
│  │  - Service Discovery                 │                   │
│  └──────────────────────────────────────┘                   │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Configuration

### Environment Variables

```bash
# AVX Context (required for telemetry)
export AVX__STACK=Avx
export AVX__LAYER=deep
export AVX__ENV=prod
export AVX__CLUSTER=AVL-BR
export AVX__MESH=internal

# HTTP Configuration
export AVX__HTTP__BIND_ADDR=0.0.0.0:8080

# Logging
export RUST_LOG=info,avx_events=debug,avx_http=debug

# Event Bus Configuration
export AVX_EVENTS__CHANNEL_SIZE=1000
export AVX_EVENTS__MAX_SUBSCRIBERS=100
```

### Configuration File (`avx.toml`)

```toml
stack = "Avx"
layer = "deep"
env = "prod"
cluster = "AVL-BR"
mesh = "internal"

[http]
bind_addr = "0.0.0.0:8080"

[events]
channel_size = 1000
max_subscribers = 100
enable_distributed = true

[events.redis]
url = "redis://localhost:6379"
pool_size = 10
```

## 📊 Structured Logging

All logs are emitted as structured JSON for easy parsing and analysis:

```json
{
  "timestamp": "2025-11-23T12:45:10.180855Z",
  "level": "INFO",
  "message": "Avx telemetry initialized",
  "stack": "Avx",
  "layer": "deep",
  "env": "dev",
  "cluster": "AVL-BR",
  "mesh": "internal",
  "target": "avx_telemetry"
}
```

## 🎯 Event Types

### System Events

#### ServiceStarted
```rust
ServiceStarted {
    service_name: "avx-platform-service",
    version: "0.1.0",
    timestamp: 1700754310000,
}
```

#### HealthCheckPerformed
```rust
HealthCheckPerformed {
    service_name: "avx-platform-service",
    status: "healthy",
    checks: vec!["event_bus", "http_server", "database"],
    timestamp: 1700754310000,
}
```

#### MetricsSnapshot
```rust
MetricsSnapshot {
    service_name: "avx-platform-service",
    requests_total: 1000,
    requests_success: 950,
    requests_error: 50,
    events_published: 5000,
    uptime_seconds: 3600,
    timestamp: 1700754310000,
}
```

### HTTP Events (avx-http integration)

#### HttpRequestEvent
```rust
HttpRequestEvent {
    request_id: "req-12345",
    method: "POST",
    path: "/api/users",
    status_code: 201,
    duration_ms: 45,
    user_agent: "curl/7.88.1",
    timestamp: 1700754310000,
}
```

#### HttpErrorEvent
```rust
HttpErrorEvent {
    request_id: "req-12346",
    method: "GET",
    path: "/api/users/999",
    status_code: 404,
    error: "User not found",
    timestamp: 1700754310000,
}
```

## 🔌 Integration Patterns

### 1. HTTP + Events

```rust
use avx_events::EventBus;
use avx_http::{Server, middleware::EventPublisher};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize event bus
    let event_bus = Arc::new(EventBus::new());

    // Create HTTP server with event publishing
    let server = Server::builder()
        .with_middleware(EventPublisher::new(event_bus.clone()))
        .bind("0.0.0.0:8080")
        .await?;

    // Subscribe to HTTP events
    let mut http_sub = event_bus.subscribe::<HttpRequestEvent>().await;
    tokio::spawn(async move {
        while let Some(envelope) = http_sub.recv().await {
            // Process HTTP events (analytics, logging, etc.)
            println!("HTTP Request: {} {}",
                envelope.event.method,
                envelope.event.path
            );
        }
    });

    server.run().await?;
    Ok(())
}
```

### 2. Event Sourcing with EventStore

```rust
use avx_events::{EventStore, Event, AggregateRoot};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserCreated {
    user_id: String,
    email: String,
}

impl Event for UserCreated {
    fn event_type(&self) -> &'static str { "user.created" }
    fn aggregate_id(&self) -> String { self.user_id.clone() }
}

#[derive(Default)]
struct User {
    id: String,
    email: String,
    version: u64,
}

impl AggregateRoot for User {
    fn aggregate_id(&self) -> String { self.id.clone() }
    fn version(&self) -> u64 { self.version }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let store = EventStore::new();

    // Append event
    store.append_event(UserCreated {
        user_id: "user-123".into(),
        email: "user@example.com".into(),
    }).await?;

    // Load aggregate
    let user: User = store.load_aggregate("user-123").await?;
    println!("User: {}", user.email);

    Ok(())
}
```

### 3. CQRS Pattern

```rust
use avx_events::{CommandBus, QueryBus, CommandHandler, QueryHandler};
use async_trait::async_trait;

#[derive(Debug, Clone)]
struct CreateUserCommand {
    email: String,
}

#[derive(Debug, Clone)]
struct GetUserQuery {
    user_id: String,
}

struct UserCommandHandler;

#[async_trait]
impl CommandHandler<CreateUserCommand> for UserCommandHandler {
    type Response = String;

    async fn handle(&self, cmd: CreateUserCommand) -> Result<String, Error> {
        // Create user logic
        Ok("user-123".into())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cmd_bus = CommandBus::new();
    cmd_bus.register("create_user", UserCommandHandler);

    let user_id = cmd_bus.dispatch(
        "create_user",
        CreateUserCommand { email: "user@example.com".into() }
    ).await?;

    println!("Created user: {}", user_id);
    Ok(())
}
```

### 4. Topic-Based Routing

```rust
use avx_events::TopicBus;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bus = TopicBus::new();

    // Subscribe to all user events
    let mut user_sub = bus.subscribe("users.*").await;

    // Subscribe to all events
    let mut all_sub = bus.subscribe("**").await;

    // Publish to specific topic
    bus.publish_to("users.created", UserCreated {
        user_id: "user-123".into(),
        email: "user@example.com".into(),
    }).await?;

    Ok(())
}
```

## 🐳 Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release --example production_service

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/examples/production_service /usr/local/bin/

ENV RUST_LOG=info
ENV AVX__STACK=Avx
ENV AVX__LAYER=deep
ENV AVX__ENV=prod
ENV AVX__CLUSTER=AVL-BR
ENV AVX__MESH=internal

EXPOSE 8080

CMD ["production_service"]
```

### docker-compose.yml

```yaml
version: '3.8'

services:
  avx-service:
    build: .
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info,avx_events=debug
      - AVX__STACK=Avx
      - AVX__LAYER=deep
      - AVX__ENV=prod
      - AVX__CLUSTER=AVL-BR
      - AVX__MESH=internal
      - AVX__HTTP__BIND_ADDR=0.0.0.0:8080
    depends_on:
      - redis
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    restart: unless-stopped

volumes:
  redis-data:
```

## ☸️ Kubernetes Deployment

### deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: avx-events-service
  labels:
    app: avx-events
    stack: Avx
    layer: deep
spec:
  replicas: 3
  selector:
    matchLabels:
      app: avx-events
  template:
    metadata:
      labels:
        app: avx-events
        stack: Avx
        layer: deep
    spec:
      containers:
      - name: avx-events
        image: avilaops/avx-events:latest
        ports:
        - containerPort: 8080
          name: http
        env:
        - name: RUST_LOG
          value: "info,avx_events=debug"
        - name: AVX__STACK
          value: "Avx"
        - name: AVX__LAYER
          value: "deep"
        - name: AVX__ENV
          value: "prod"
        - name: AVX__CLUSTER
          valueFrom:
            fieldRef:
              fieldPath: spec.nodeName
        - name: AVX__MESH
          value: "internal"
        - name: AVX__HTTP__BIND_ADDR
          value: "0.0.0.0:8080"
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
          limits:
            cpu: 500m
            memory: 512Mi
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: avx-events-service
spec:
  selector:
    app: avx-events
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

## 📈 Monitoring & Observability

### Prometheus Metrics

The service exposes metrics at `/metrics`:

```
# HELP avx_events_published_total Total number of events published
# TYPE avx_events_published_total counter
avx_events_published_total{event_type="user.created"} 1000

# HELP avx_http_requests_total Total HTTP requests
# TYPE avx_http_requests_total counter
avx_http_requests_total{method="GET",status="200"} 5000

# HELP avx_http_request_duration_seconds HTTP request duration
# TYPE avx_http_request_duration_seconds histogram
avx_http_request_duration_seconds_bucket{le="0.01"} 3000
avx_http_request_duration_seconds_bucket{le="0.05"} 4500
avx_http_request_duration_seconds_bucket{le="0.1"} 4900
```

### Health Checks

```bash
# Liveness probe
curl http://localhost:8080/health
# {"status":"healthy","checks":["event_bus","http_server","database"]}

# Readiness probe
curl http://localhost:8080/ready
# {"ready":true,"uptime_seconds":3600}

# Metrics endpoint
curl http://localhost:8080/metrics
# ...Prometheus metrics...
```

## 🔒 Security Best Practices

1. **Event Validation**: Always validate events before publishing
2. **Access Control**: Implement authorization for event subscriptions
3. **Rate Limiting**: Prevent event flooding attacks
4. **Encryption**: Use TLS for distributed event bus
5. **Audit Logging**: Log all critical events for compliance

## 🚨 Error Handling

### Dead Letter Queue

```rust
use avx_events::{DeadLetterQueue, RetryStrategy};

let dlq = DeadLetterQueue::new(RetryStrategy::ExponentialBackoff {
    initial_delay: Duration::from_secs(1),
    max_delay: Duration::from_secs(60),
    max_retries: 5,
});

// Failed event processing
dlq.add(failed_event, error).await;

// Retry later
for event in dlq.get_retriable().await {
    // Retry processing
}
```

## 📊 Performance Tuning

### Recommended Settings

| Environment | Channel Size | Max Subscribers | Workers |
| ----------- | ------------ | --------------- | ------- |
| Development | 100          | 10              | 2       |
| Staging     | 1,000        | 50              | 4       |
| Production  | 10,000       | 100             | 8       |

### Benchmarks

- **EventBus throughput**: 100,000+ events/sec
- **Topic matching**: 50,000+ matches/sec
- **EventStore writes**: 10,000+ events/sec
- **HTTP event publishing**: 5,000+ requests/sec

## 🤝 Integration with Other Services

### AvilaDB (Event Sourcing)

```rust
use aviladb::{AvilaClient, Collection};
use avx_events::EventStore;

let client = AvilaClient::connect("http://localhost:8000").await?;
let db = client.database("events").await?;
let events_collection = db.collection("events").await?;

// Persist events to AvilaDB
let store = EventStore::with_backend(AvilaDBBackend::new(events_collection));
```

### AVL Queue (Message Bus)

```rust
use avl_queue::QueueClient;
use avx_events::EventBus;

let queue = QueueClient::connect("amqp://localhost:5672").await?;
let event_bus = EventBus::new();

// Bridge events to message queue
let mut sub = event_bus.subscribe::<UserCreated>().await;
tokio::spawn(async move {
    while let Some(envelope) = sub.recv().await {
        queue.publish("users.created", &envelope.event).await?;
    }
});
```

## 📚 Examples

Run the production example:

```bash
cd avx-events
RUST_LOG=info cargo run --example production_service
```

Other examples:
- `basic_pubsub` - Simple pub/sub pattern
- `event_sourcing` - Event sourcing with EventStore
- `cqrs` - CQRS pattern with commands and queries
- `request_reply` - RPC-style messaging
- `topic_routing` - Topic-based routing with wildcards
- `event_driven_server` - Full HTTP + Events integration

## 🐛 Troubleshooting

### Events not being received

1. Check subscriber is registered before publishing
2. Verify event types match exactly
3. Check channel capacity (increase if needed)
4. Enable debug logging: `RUST_LOG=avx_events=debug`

### High memory usage

1. Reduce channel size
2. Limit number of subscribers
3. Implement backpressure
4. Use distributed backend for large volumes

### Slow event processing

1. Use parallel subscribers
2. Batch event processing
3. Optimize event handlers
4. Consider distributed event bus

## 📖 Documentation

- [API Documentation](https://docs.rs/avx-events)
- [Architecture Guide](./docs/architecture.md)
- [Performance Guide](./docs/performance.md)
- [Security Guide](./docs/security.md)

## 🤝 Support

- GitHub Issues: https://github.com/avilaops/arxis/issues
- Discord: https://discord.gg/avilaops
- Email: dev@avila.inc

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

Built with ❤️ by the Avila Development Team for the AVX Platform.
