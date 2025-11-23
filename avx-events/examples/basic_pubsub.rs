//! Basic pub/sub example with EventBus.
//!
//! Run with: cargo run --example basic_pubsub

use avx_events::{Event, EventBus};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserCreated {
    user_id: String,
    email: String,
    name: String,
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
struct OrderPlaced {
    order_id: String,
    user_id: String,
    total: f64,
}

impl Event for OrderPlaced {
    fn event_type(&self) -> &'static str {
        "order.placed"
    }

    fn aggregate_id(&self) -> String {
        self.order_id.clone()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("🚀 AVX Events - Basic Pub/Sub Example\n");

    let bus = EventBus::new();

    // Service 1: Email notification service
    let mut email_subscriber = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        println!("📧 Email service started");
        while let Some(envelope) = email_subscriber.recv().await {
            println!(
                "📧 Sending welcome email to {} ({})",
                envelope.event.name, envelope.event.email
            );
        }
    });

    // Service 2: User profile service
    let mut profile_subscriber = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        println!("👤 Profile service started");
        while let Some(envelope) = profile_subscriber.recv().await {
            println!("👤 Creating profile for user {}", envelope.event.user_id);
        }
    });

    // Service 3: Analytics service
    let mut analytics_subscriber = bus.subscribe::<UserCreated>().await;
    tokio::spawn(async move {
        println!("📊 Analytics service started");
        while let Some(envelope) = analytics_subscriber.recv().await {
            println!("📊 Tracking signup event for {}", envelope.event.email);
        }
    });

    // Service 4: Order processing
    let mut order_subscriber = bus.subscribe::<OrderPlaced>().await;
    tokio::spawn(async move {
        println!("🛒 Order service started");
        while let Some(envelope) = order_subscriber.recv().await {
            println!(
                "🛒 Processing order {} - Total: R$ {:.2}",
                envelope.event.order_id, envelope.event.total
            );
        }
    });

    sleep(Duration::from_millis(100)).await;

    println!("\n--- Publishing Events ---\n");

    // Publish user created event
    bus.publish(UserCreated {
        user_id: "user-001".into(),
        email: "alice@example.com".into(),
        name: "Alice Silva".into(),
    })
    .await
    .unwrap();

    sleep(Duration::from_millis(100)).await;

    // Publish another user
    bus.publish(UserCreated {
        user_id: "user-002".into(),
        email: "bob@example.com".into(),
        name: "Bob Santos".into(),
    })
    .await
    .unwrap();

    sleep(Duration::from_millis(100)).await;

    // Publish order event
    bus.publish(OrderPlaced {
        order_id: "order-123".into(),
        user_id: "user-001".into(),
        total: 299.90,
    })
    .await
    .unwrap();

    sleep(Duration::from_millis(100)).await;

    println!("\n✅ Example completed!");
}
