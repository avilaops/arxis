# avx-events

**Event-driven architecture for Avila Experience Fabric**

[![Crates.io](https://img.shields.io/crates/v/avx-events.svg)](https://crates.io/crates/avx-events)
[![Documentation](https://docs.rs/avx-events/badge.svg)](https://docs.rs/avx-events)
[![License](https://img.shields.io/crates/l/avx-events.svg)](https://github.com/avilaops/arxis#license)

Pub/sub event bus, event sourcing, and message-driven patterns for building distributed AVX (Avila Experience) platform applications.

## Features

- **Event Bus**: In-memory and distributed pub/sub
- **Event Sourcing**: Append-only event store with replay
- **CQRS Support**: Command/Query separation patterns
- **Message Patterns**: Request/reply, fire-and-forget, broadcast
- **Dead Letter Queue**: Failed event handling
- **Event Replay**: Time-travel debugging and audit trails
- **Async/Await**: Built on Tokio for high concurrency
- **Type-Safe**: Strongly typed events with serde

## Installation

```toml
[dependencies]
avx-events = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Define Events

```rust
use avx_events::{Event, EventMetadata};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreated {
    pub user_id: String,
    pub email: String,
    pub name: String,
}

impl Event for UserCreated {
    fn event_type(&self) -> &'static str {
        "user.created"
    }

    fn aggregate_id(&self) -> String {
        self.user_id.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPlaced {
    pub order_id: String,
    pub user_id: String,
    pub total: f64,
}

impl Event for OrderPlaced {
    fn event_type(&self) -> &'static str {
        "order.placed"
    }

    fn aggregate_id(&self) -> String {
        self.order_id.clone()
    }
}
```

### Create Event Bus

```rust
use avx_events::EventBus;

#[tokio::main]
async fn main() {
    let bus = EventBus::new();

    // Subscribe to events
    let mut subscriber = bus.subscribe::<UserCreated>().await;

    // Publish event
    bus.publish(UserCreated {
        user_id: "123".into(),
        email: "user@example.com".into(),
        name: "John Doe".into(),
    }).await.unwrap();

    // Receive event
    if let Some(event) = subscriber.recv().await {
        println!("Received: {:?}", event);
    }
}
```

### Multiple Subscribers

```rust
use avx_events::EventBus;

#[tokio::main]
async fn main() {
    let bus = EventBus::new();

    // Service 1: Send email
    let mut email_sub = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        while let Some(event) = email_sub.recv().await {
            send_welcome_email(&event.email).await;
        }
    });

    // Service 2: Create profile
    let mut profile_sub = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        while let Some(event) = profile_sub.recv().await {
            create_user_profile(&event.user_id).await;
        }
    });

    // Service 3: Analytics
    let mut analytics_sub = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        while let Some(event) = analytics_sub.recv().await {
            track_signup_event(&event).await;
        }
    });

    // Publish event - all subscribers receive it
    bus.publish(UserCreated {
        user_id: "456".into(),
        email: "jane@example.com".into(),
        name: "Jane Smith".into(),
    }).await.unwrap();
}
```

### Event Sourcing

```rust
use avx_events::{EventStore, AggregateRoot};

// Define aggregate
pub struct UserAggregate {
    pub id: String,
    pub email: String,
    pub name: String,
    pub version: u64,
}

impl AggregateRoot for UserAggregate {
    type Event = UserEvent;

    fn apply(&mut self, event: Self::Event) {
        match event {
            UserEvent::Created(e) => {
                self.id = e.user_id;
                self.email = e.email;
                self.name = e.name;
            },
            UserEvent::EmailChanged(e) => {
                self.email = e.new_email;
            },
        }
        self.version += 1;
    }
}

// Use event store
#[tokio::main]
async fn main() {
    let store = EventStore::new();

    // Save events
    store.append("user-123", vec![
        UserEvent::Created(UserCreated { /* ... */ }),
        UserEvent::EmailChanged(EmailChanged { /* ... */ }),
    ]).await.unwrap();

    // Replay events to rebuild state
    let events = store.get_events("user-123", 0).await.unwrap();
    let mut user = UserAggregate::default();
    for event in events {
        user.apply(event);
    }

    println!("User state: {:?}", user);
}
```

### CQRS Pattern

```rust
use avx_events::{CommandHandler, QueryHandler};

// Commands (write side)
pub struct CreateUserCommand {
    pub email: String,
    pub name: String,
}

pub struct CreateUserHandler {
    event_bus: EventBus,
}

impl CommandHandler<CreateUserCommand> for CreateUserHandler {
    type Result = String; // user_id

    async fn handle(&self, cmd: CreateUserCommand) -> Result<Self::Result, Error> {
        let user_id = uuid::Uuid::new_v4().to_string();

        // Validate
        if cmd.email.is_empty() {
            return Err(Error::validation("Email required"));
        }

        // Publish event
        self.event_bus.publish(UserCreated {
            user_id: user_id.clone(),
            email: cmd.email,
            name: cmd.name,
        }).await?;

        Ok(user_id)
    }
}

// Queries (read side)
pub struct GetUserQuery {
    pub user_id: String,
}

pub struct GetUserHandler {
    read_model: UserReadModel,
}

impl QueryHandler<GetUserQuery> for GetUserHandler {
    type Result = UserView;

    async fn handle(&self, query: GetUserQuery) -> Result<Self::Result, Error> {
        self.read_model.find_by_id(&query.user_id).await
    }
}
```

### Topic-based Routing

```rust
use avx_events::TopicBus;

#[tokio::main]
async fn main() {
    let bus = TopicBus::new();

    // Subscribe to specific topics
    let mut user_sub = bus.subscribe("users.*").await;
    let mut order_sub = bus.subscribe("orders.*").await;
    let mut all_sub = bus.subscribe("*").await; // All events

    // Publish to topics
    bus.publish_to("users.created", UserCreated { /* ... */ }).await;
    bus.publish_to("orders.placed", OrderPlaced { /* ... */ }).await;

    // user_sub receives UserCreated only
    // order_sub receives OrderPlaced only
    // all_sub receives both
}
```

### Dead Letter Queue

```rust
use avx_events::{EventBus, DeadLetterQueue};

#[tokio::main]
async fn main() {
    let bus = EventBus::with_dlq(DeadLetterQueue::new());

    let mut subscriber = bus.subscribe::<UserCreated>().await;

    tokio::spawn(async move {
        while let Some(event) = subscriber.recv().await {
            if let Err(e) = process_event(event).await {
                // Event automatically goes to DLQ after retries
                eprintln!("Failed to process: {}", e);
            }
        }
    });

    // View DLQ
    let dlq_events = bus.dead_letter_queue().list().await;
    println!("Failed events: {}", dlq_events.len());

    // Retry from DLQ
    for event in dlq_events {
        bus.republish(event).await;
    }
}
```

### Request/Reply Pattern

```rust
use avx_events::RequestReplyBus;

#[tokio::main]
async fn main() {
    let bus = RequestReplyBus::new();

    // Responder
    tokio::spawn(async move {
        let mut requests = bus.listen::<GetUserRequest>().await;
        while let Some((req, reply)) = requests.recv().await {
            let user = fetch_user(&req.user_id).await;
            reply.send(user).await;
        }
    });

    // Requester
    let response = bus.request(GetUserRequest {
        user_id: "123".into(),
    }).await.unwrap();

    println!("User: {:?}", response);
}
```

## Distributed Event Bus

Use with Redis, NATS, or Kafka:

```rust
use avx_events::distributed::RedisEventBus;

#[tokio::main]
async fn main() {
    let bus = RedisEventBus::connect("redis://localhost:6379")
        .await
        .unwrap();

    // Now events are distributed across services
    bus.publish(UserCreated { /* ... */ }).await;
}
```

## Event Metadata

All events carry metadata:

```rust
pub struct EventEnvelope<T> {
    pub event: T,
    pub metadata: EventMetadata,
}

pub struct EventMetadata {
    pub event_id: String,
    pub event_type: String,
    pub aggregate_id: String,
    pub timestamp: i64,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub user_id: Option<String>,
}
```

## Integration with AVX Ecosystem

```rust
use avx_events::EventBus;
use avx_telemetry::init_tracing;
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let bus = EventBus::new();

    let mut subscriber = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        while let Some(event) = subscriber.recv().await {
            info!(
                user_id = %event.user_id,
                email = %event.email,
                "User created event received"
            );
        }
    });
}
```

## Testing

```rust
use avx_events::testing::MockEventBus;

#[tokio::test]
async fn test_user_service() {
    let bus = MockEventBus::new();
    let service = UserService::new(bus.clone());

    service.create_user("user@example.com", "User").await.unwrap();

    // Assert event was published
    let events = bus.published_events::<UserCreated>().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].email, "user@example.com");
}
```

## Part of AVX Ecosystem

`avx-events` enables event-driven microservices:

- **avx-gateway**: Event-driven request processing
- **avx-telemetry**: Event logging and tracing
- **avx-api-core**: Domain events from business logic

## Examples

Check the `examples/` directory for complete working examples:

- `basic_pubsub.rs` - Simple publish/subscribe pattern
- `event_sourcing.rs` - Event sourcing with EventStore
- `cqrs.rs` - Command/Query separation
- `request_reply.rs` - RPC-style messaging
- `topic_routing.rs` - Topic-based routing with wildcards
- `production_service.rs` - **Production-ready service with full AVX integration**

Run an example:
```bash
cargo run --example basic_pubsub
```

Run the production service:
```bash
RUST_LOG=info cargo run --example production_service
```

**📚 See [PRODUCTION_GUIDE.md](./PRODUCTION_GUIDE.md) for deployment documentation.**

## Performance

- **In-memory**: 100,000+ events/sec
- **Redis**: 10,000+ events/sec
- **Overhead**: < 1ms per event

## License

MIT OR Apache-2.0

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## Links

- **Repository**: https://github.com/avilaops/arxis
- **Documentation**: https://docs.rs/avx-events
- **Crates.io**: https://crates.io/crates/avx-events
- **AVX Platform**: https://avilaops.com
