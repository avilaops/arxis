//! Topic-based routing with wildcards.
//!
//! Run with: cargo run --example topic_routing

use avx_events::{Event, TopicBus};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenericEvent {
    id: String,
    message: String,
}

impl Event for GenericEvent {
    fn event_type(&self) -> &'static str {
        "generic.event"
    }
    fn aggregate_id(&self) -> String {
        self.id.clone()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("🔀 AVX Events - Topic Routing Example\n");

    let bus = TopicBus::new();

    // Subscribe to all user events
    let mut user_sub = bus.subscribe("users.**").await;
    tokio::spawn(async move {
        println!("👥 Subscribed to: users.**");
        while let Some(event) = user_sub.recv().await {
            println!("👥 Received on {}: {:?}", event.topic, event.event_type);
        }
    });

    // Subscribe to user creation only
    let mut user_created_sub = bus.subscribe("users.created").await;
    tokio::spawn(async move {
        println!("✨ Subscribed to: users.created");
        while let Some(event) = user_created_sub.recv().await {
            println!("✨ New user created on {}", event.topic);
        }
    });

    // Subscribe to all order events with single wildcard
    let mut order_sub = bus.subscribe("orders.*").await;
    tokio::spawn(async move {
        println!("🛒 Subscribed to: orders.*");
        while let Some(event) = order_sub.recv().await {
            println!("🛒 Order event on {}", event.topic);
        }
    });

    // Subscribe to payment completed events
    let mut payment_sub = bus.subscribe("payments.completed").await;
    tokio::spawn(async move {
        println!("💳 Subscribed to: payments.completed");
        while let Some(event) = payment_sub.recv().await {
            println!("💳 Payment completed!");
        }
    });

    // Subscribe to ALL events
    let mut all_sub = bus.subscribe("**").await;
    tokio::spawn(async move {
        println!("📡 Subscribed to: ** (all events)");
        while let Some(event) = all_sub.recv().await {
            println!("📡 Event on topic: {}", event.topic);
        }
    });

    sleep(Duration::from_millis(100)).await;

    println!("\n--- Publishing Events to Topics ---\n");

    // Publish to different topics
    bus.publish_to(
        "users.created",
        GenericEvent {
            id: "1".into(),
            message: "Alice created".into(),
        },
    )
    .await
    .unwrap();

    sleep(Duration::from_millis(50)).await;

    bus.publish_to(
        "users.updated",
        GenericEvent {
            id: "1".into(),
            message: "Alice updated".into(),
        },
    )
    .await
    .unwrap();

    sleep(Duration::from_millis(50)).await;

    bus.publish_to(
        "users.profile.changed",
        GenericEvent {
            id: "1".into(),
            message: "Alice profile changed".into(),
        },
    )
    .await
    .unwrap();

    sleep(Duration::from_millis(50)).await;

    bus.publish_to(
        "orders.placed",
        GenericEvent {
            id: "order-1".into(),
            message: "Order placed".into(),
        },
    )
    .await
    .unwrap();

    sleep(Duration::from_millis(50)).await;

    bus.publish_to(
        "orders.shipped",
        GenericEvent {
            id: "order-1".into(),
            message: "Order shipped".into(),
        },
    )
    .await
    .unwrap();

    sleep(Duration::from_millis(50)).await;

    bus.publish_to(
        "payments.completed",
        GenericEvent {
            id: "payment-1".into(),
            message: "Payment done".into(),
        },
    )
    .await
    .unwrap();

    sleep(Duration::from_millis(100)).await;

    println!("\n✅ Topic routing example completed!");
}
