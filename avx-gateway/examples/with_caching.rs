//! Caching example
//!
//! This example demonstrates response caching to improve performance.

use avx_gateway::{cache::ResponseCache, Gateway};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,avx_gateway=debug")
        .init();

    // Create response cache
    let cache = ResponseCache::new()
        .with_ttl(Duration::from_secs(300)) // 5 minutes
        .with_max_size(10000); // 10k entries

    // Create gateway
    let gateway = Gateway::builder()
        .route("/api/*", "http://localhost:8001")
        .with_port(8080)
        .build()
        .await?;

    println!("💾 Gateway with caching on http://0.0.0.0:8080");
    println!("\nCache configuration:");
    println!("  - TTL: 5 minutes");
    println!("  - Max size: 10,000 entries");
    println!("  - Strategy: GET requests only");
    println!("\nTest caching:");
    println!("  # First request (cache miss)");
    println!("  curl http://localhost:8080/api/data");
    println!("  # Second request (cache hit - faster!)");
    println!("  curl http://localhost:8080/api/data");

    gateway.serve().await?;

    Ok(())
}
