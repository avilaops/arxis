//! FIFO Queue example with ordered delivery
//!
//! This example demonstrates:
//! - Ordered message delivery
//! - Priority queues
//! - Message correlation for request-response patterns

use avl_queue::{Message, QueueClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = QueueClient::connect("https://queue.avila.cloud").await?;
    println!("✅ Connected to AVL Queue\n");

    // Subscribe to order processing queue
    let mut subscriber = client.subscribe("order-processing").await?;
    println!("📬 Subscribed to FIFO queue: order-processing\n");

    // Spawn subscriber
    let subscriber_task = tokio::spawn(async move {
        println!("🔄 Processing orders in sequence...\n");

        while let Some(ack_msg) = subscriber.recv().await {
            let message = ack_msg.message();

            if let (Some(order_id), Some(step)) = (
                message.get_str("order_id"),
                message.get_str("step")
            ) {
                println!("📦 Processing Order: {} - Step: {}", order_id, step);
                println!("   Priority: {}", message.metadata.priority);
                println!("   Message ID: {}", message.id);

                // Simulate processing time
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                ack_msg.ack().await.unwrap();
                println!("   ✅ Completed\n");
            }
        }
    });

    // Give subscriber time to set up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Publish order workflow messages in sequence
    println!("📤 Publishing order workflow messages...\n");

    let order_id = "ORD-12345";
    let steps = vec![
        ("validate", 10),
        ("charge_payment", 9),
        ("reserve_inventory", 8),
        ("prepare_shipment", 7),
        ("ship", 6),
    ];

    for (step, priority) in steps {
        let message = Message::new()
            .with_field("order_id", order_id)
            .with_field("step", step)
            .with_field("timestamp", chrono::Utc::now().to_rfc3339())
            .with_priority(priority)
            .with_correlation_id(order_id);

        client.publish("order-processing", message).await?;
        println!("✅ Published: {} (priority: {})", step, priority);

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    // Wait for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Statistics
    let stats = client.stats().await;
    println!("\n📊 Queue Statistics:");
    println!("   Messages Published: {}", stats.messages_published);
    println!("   Avg Latency: {:.2}ms", stats.avg_publish_latency_ms);

    subscriber_task.abort();

    Ok(())
}
