//! Load balancing example
//!
//! This example demonstrates how to use load balancing to distribute
//! requests across multiple upstream services.

use avx_gateway::{Gateway, routing::Route, routing::Upstream, load_balancer::Strategy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,avx_gateway=debug")
        .init();

    // Create a gateway with load balanced routes
    let mut route = Route::new("/api/*", "http://localhost:8001");
    route.upstream = Upstream::LoadBalanced {
        urls: vec![
            "http://localhost:8001".to_string(),
            "http://localhost:8002".to_string(),
            "http://localhost:8003".to_string(),
        ],
        strategy: Strategy::RoundRobin,
        current_index: 0,
    };

    let gateway = Gateway::builder()
        .with_port(8080)
        .build()
        .await?;

    println!("Gateway with load balancing listening on http://0.0.0.0:8080");
    println!("Load balanced upstreams:");
    println!("  - http://localhost:8001");
    println!("  - http://localhost:8002");
    println!("  - http://localhost:8003");
    println!("\nStrategy: Round-robin");
    println!("\nTest with:");
    println!("  curl http://localhost:8080/api/test");

    gateway.serve().await?;

    Ok(())
}
