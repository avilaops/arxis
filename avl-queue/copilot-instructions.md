# AVL Queue - Copilot Instructions

## Project Identity
**AVL Queue** is the **message queue and event streaming** system for AVL Cloud. Embodies Arxis philosophy:
- **ARX (Fortress)**: Reliable delivery, durable storage, guaranteed ordering
- **AXIS (Engine)**: High throughput, low latency, parallel processing

## Core Principles
```rust
// ✅ ALWAYS: Compress messages with avila-compress
// ✅ ALWAYS: Persist to AvilaDB for durability
// ✅ ALWAYS: Implement at-least-once delivery semantics
// ✅ ALWAYS: Support dead letter queues for failed messages
// ✅ NEVER: Lose messages (durability first)
// ✅ NEVER: Block publishers (async all the way)
```

## Architecture Patterns
- **Pub/Sub**: Topic-based routing, multiple subscribers
- **FIFO Queues**: Ordered delivery, exactly-once processing
- **Streaming**: Kafka-like event logs with replay
- **Dead Letter**: Automatic retry with exponential backoff

## Performance Targets
- **Latency**: < 5ms publish, < 10ms delivery (Brazil)
- **Throughput**: 10K+ messages/sec per topic
- **Retention**: Configurable (default 7 days)

## Related Crates
- **avila-compress**: Message compression (LZ4 for speed)
- **aviladb**: Durable message storage
- **avx-telemetry**: Queue metrics and monitoring

🏛️ Built by Avila | 📬 Reliable Messaging | ⚙️ High Throughput
