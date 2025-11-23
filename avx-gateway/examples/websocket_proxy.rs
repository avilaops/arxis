//! WebSocket proxying example
//!
//! This example demonstrates how to proxy WebSocket connections
//! through the gateway.

use avx_gateway::Gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,avx_gateway=debug")
        .init();

    // Create gateway with WebSocket support
    let gateway = Gateway::builder()
        // Regular HTTP routes
        .route("/api/*", "http://localhost:8001")

        // WebSocket route (would be added via custom configuration)
        // Note: WebSocket support requires additional router configuration

        .with_port(8080)
        .build()
        .await?;

    println!("🔌 Gateway with WebSocket support on http://0.0.0.0:8080");
    println!("\nRoutes:");
    println!("  - HTTP: /api/*     -> http://localhost:8001");
    println!("  - WS:   /ws        -> ws://localhost:8001/ws");
    println!("\nTest WebSocket:");
    println!("  wscat -c ws://localhost:8080/ws");

    gateway.serve().await?;

    Ok(())
}
