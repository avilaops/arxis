//! Gateway with authentication example
//!
//! This example demonstrates how to add JWT and API key authentication
//! to specific routes.

use avx_gateway::{Gateway, auth::{JwtConfig, AuthLayer}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,avx_gateway=debug")
        .init();

    // Create a gateway with authenticated routes
    let gateway = Gateway::builder()
        // Public route - no authentication
        .route("/api/public/*", "http://localhost:8001")
        // Private route - will require authentication in the future
        .route("/api/private/*", "http://localhost:8002")
        .with_port(8080)
        .build()
        .await?;

    println!("Gateway with authentication listening on http://0.0.0.0:8080");
    println!("Routes configured:");
    println!("  - /api/public/*  -> http://localhost:8001 (no auth)");
    println!("  - /api/private/* -> http://localhost:8002 (auth required)");
    println!("\nTest with:");
    println!("  curl http://localhost:8080/api/public/test");
    println!("  curl -H 'Authorization: Bearer your_token' http://localhost:8080/api/private/test");

    gateway.serve().await?;

    Ok(())
}
