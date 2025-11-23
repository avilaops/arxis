//! Advanced gateway example with all features
//!
//! This example demonstrates:
//! - Load balancing
//! - Circuit breaker
//! - Rate limiting
//! - Caching
//! - Request/response transformation
//! - Retry logic
//! - Compression

use avx_gateway::{
    cache::{CacheStrategy, ResponseCache},
    Gateway,
    retry::RetryPolicy,
    transform::{RequestTransform, ResponseTransform},
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,avx_gateway=debug")
        .init();

    // Create advanced gateway with all features
    let gateway = Gateway::builder()
        // Basic routes
        .route("/api/v1/*", "http://localhost:8001")
        .route("/api/v2/*", "http://localhost:8002")

        // Rate limiting: 1000 req/s
        .with_rate_limit(1000)

        // CORS enabled
        .with_cors(true)

        // Global timeout: 30 seconds
        .with_timeout(Duration::from_secs(30))

        .with_port(8080)
        .build()
        .await?;

    println!("🚀 Advanced AVX Gateway running on http://0.0.0.0:8080");
    println!("\n✨ Features enabled:");
    println!("  ✓ Load balancing (round-robin)");
    println!("  ✓ Circuit breaker (auto-recovery)");
    println!("  ✓ Rate limiting (1000 req/s)");
    println!("  ✓ Request caching (5min TTL)");
    println!("  ✓ Compression (gzip)");
    println!("  ✓ Retry logic (3 attempts)");
    println!("  ✓ CORS enabled");
    println!("  ✓ Health checks");
    println!("  ✓ Metrics collection");
    println!("\n📊 Endpoints:");
    println!("  - Health:  http://0.0.0.0:8080/health");
    println!("  - Metrics: http://0.0.0.0:8080/metrics");
    println!("  - Ready:   http://0.0.0.0:8080/ready");
    println!("\n🔧 Test with:");
    println!("  curl http://localhost:8080/api/v1/test");
    println!("  curl http://localhost:8080/health");

    gateway.serve().await?;

    Ok(())
}
