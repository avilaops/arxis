//! Basic pub/sub example for AVL Queue
//!
//! This example demonstrates:
//! - Connecting to AVL Queue
//! - Publishing messages to a topic
//! - Subscribing to a topic
//! - Acknowledging messages

use avl_queue::{Message, QueueClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Connect to AVL Queue
    let client = QueueClient::connect("https://queue.avila.cloud").await?;
    println!("✅ Connected to AVL Queue");

    // Subscribe to the topic first
    let mut subscriber = client.subscribe("user-events").await?;
    println!("📬 Subscribed to topic: user-events");

    // Spawn subscriber task
    let subscriber_task = tokio::spawn(async move {
        println!("\n🔄 Waiting for messages...\n");

        while let Some(ack_msg) = subscriber.recv().await {
            let message = ack_msg.message();

            println!("📨 Received message:");
            println!("   ID: {}", message.id);
            println!("   Topic: {}", message.metadata.topic);

            if let Some(event_type) = message.get_str("event_type") {
                println!("   Event Type: {}", event_type);
            }
            if let Some(user_id) = message.get_str("user_id") {
                println!("   User ID: {}", user_id);
            }

            // Acknowledge message
            ack_msg.ack().await.unwrap();
            println!("   ✅ Message acknowledged\n");
        }
    });

    // Give subscriber time to set up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Publish messages
    println!("📤 Publishing messages to topic: user-events\n");

    for i in 1..=5 {
        let message = Message::new()
            .with_field("event_type", "user.created")
            .with_field("user_id", format!("user{}", i))
            .with_field("timestamp", chrono::Utc::now().to_rfc3339())
            .with_priority(if i % 2 == 0 { 5 } else { 1 });

        client.publish("user-events", message).await?;
        println!("✅ Published message {}/5", i);

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    // Wait for messages to be processed
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Display statistics
    let stats = client.stats().await;
    println!("\n📊 Queue Statistics:");
    println!("   Messages Published: {}", stats.messages_published);
    println!("   Messages Delivered: {}", stats.messages_delivered);
    println!("   Avg Publish Latency: {:.2}ms", stats.avg_publish_latency_ms);

    subscriber_task.abort();

    Ok(())
}
