# 📬 AVL Queue

**High-Performance Message Queue and Event Streaming for AVL Cloud**

[![Crates.io](https://img.shields.io/crates/v/avl-queue.svg)](https://crates.io/crates/avl-queue)
[![Documentation](https://docs.rs/avl-queue/badge.svg)](https://docs.rs/avl-queue)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.inc)

🏛️ **Reliable Messaging** | ⚙️ **High Throughput** | 🇧🇷 **Low Latency in Brazil**

---

## Features

- **Pub/Sub**: Topic-based publish/subscribe
- **Message Queues**: Guaranteed delivery, FIFO ordering
- **Event Streaming**: Kafka-like event logs
- **Dead Letter Queues**: Automatic retry and failure handling
- **Compression**: Automatic via `avila-compress`
- **Persistence**: AvilaDB backend for durability

## Quick Start

```rust
use avl_queue::{QueueClient, Message};

#[tokio::main]
async fn main() {
    let client = QueueClient::connect("https://queue.avila.cloud").await?;

    // Publish message
    client.publish("events", Message::new()
        .set("event_type", "user.created")
        .set("user_id", "user123")
    ).await?;

    // Subscribe
    let mut subscriber = client.subscribe("events").await?;
    while let Some(msg) = subscriber.recv().await {
        println!("Received: {:?}", msg);
        msg.ack().await?; // Acknowledge processing
    }
}
```

## Use Cases

- **Event-Driven Architecture**: Microservices communication
- **Task Queues**: Background job processing
- **Real-Time Data**: IoT sensor streams, game events
- **Audit Logs**: Centralized logging and audit trails

🏛️ **Built by Avila** - Part of AVL Cloud Platform
