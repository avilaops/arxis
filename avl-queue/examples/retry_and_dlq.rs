//! Retry and Dead Letter Queue example
//!
//! This example demonstrates:
//! - Automatic retry with exponential backoff
//! - Dead letter queue for failed messages
//! - Custom subscriber configuration

use avl_queue::{Message, QueueClient};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing to see retry logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Connect to AVL Queue
    let client = QueueClient::connect("https://queue.avila.cloud").await?;
    println!("✅ Connected to AVL Queue\n");

    // Subscribe to the topic
    let mut subscriber = client.subscribe("critical-events").await?;
    println!("📬 Subscribed to topic: critical-events");
    println!("   Max Retries: {}", subscriber.config().max_retries);
    println!("   Base Retry Delay: {}ms", subscriber.config().base_retry_delay_ms);
    println!("   DLQ Enabled: {}\n", subscriber.config().dlq_enabled);

    // Publish some test messages
    println!("📤 Publishing test messages...\n");

    for i in 1..=3 {
        let message = Message::new()
            .with_field("event_type", "payment.process")
            .with_field("payment_id", format!("pay_{}", i))
            .with_field("amount", 100.0 * i as f64)
            .with_priority(10);

        client.publish("critical-events", message).await?;
        println!("✅ Published payment message {}", i);
    }

    // Process messages with retry logic
    println!("\n🔄 Processing messages with automatic retry...\n");

    let attempt_counter = Arc::new(AtomicU32::new(0));
    let counter_clone = Arc::clone(&attempt_counter);

    // Simulate a handler that fails initially then succeeds
    let handler_task = tokio::spawn(async move {
        subscriber.process_with_retry(|message| {
            let counter = Arc::clone(&counter_clone);
            async move {
                let attempt = counter.fetch_add(1, Ordering::SeqCst) + 1;

                println!("🔧 Processing message: {}", message.id);
                println!("   Payment ID: {}", message.get_str("payment_id").unwrap_or("unknown"));
                println!("   Attempt: {}", attempt);

                // Simulate failure on first 2 attempts
                if attempt % 3 != 0 {
                    println!("   ❌ Processing failed (simulated error)\n");
                    return Err(avl_queue::QueueError::InvalidMessage(
                        "Simulated processing error".to_string()
                    ));
                }

                // Success on 3rd attempt
                println!("   ✅ Processing succeeded!\n");
                Ok(())
            }
        }).await
    });

    // Wait for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Display statistics
    let stats = client.stats().await;
    println!("\n📊 Final Statistics:");
    println!("   Messages Published: {}", stats.messages_published);
    println!("   Total Processing Attempts: {}", attempt_counter.load(Ordering::SeqCst));
    println!("   Avg Publish Latency: {:.2}ms", stats.avg_publish_latency_ms);

    handler_task.abort();

    Ok(())
}
