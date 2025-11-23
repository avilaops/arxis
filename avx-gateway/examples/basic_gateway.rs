//! Basic gateway example
//!
//! This example demonstrates how to create a simple API gateway
//! that routes requests to different upstream services.

use avx_gateway::Gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,avx_gateway=debug")
        .init();

    // Create a basic gateway with two routes
    let gateway = Gateway::builder()
        .route("/api/users/*", "http://localhost:8001")
        .route("/api/products/*", "http://localhost:8002")
        .with_port(8080)
        .build()
        .await?;

    println!("Gateway listening on http://0.0.0.0:8080");
    println!("Routes configured:");
    println!("  - /api/users/*    -> http://localhost:8001");
    println!("  - /api/products/* -> http://localhost:8002");
    println!("\nHealth check: http://0.0.0.0:8080/health");
    println!("Metrics:      http://0.0.0.0:8080/metrics");

    gateway.serve().await?;

    Ok(())
}
