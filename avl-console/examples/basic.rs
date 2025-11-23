//! AVL Console - Example application
//!
//! This example demonstrates how to start the AVL Console server.

use avl_console::{Console, ConsoleConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "avl_console=debug,tower_http=debug".into()),
        )
        .init();

    // Load configuration from environment
    let config = ConsoleConfig::from_env()?;

    tracing::info!("🏛️  AVL Console starting...");
    tracing::info!("📋 Configuration:");
    tracing::info!("   - Bind: {}:{}", config.bind_address, config.port);
    tracing::info!("   - Debug: {}", config.debug);
    tracing::info!("   - Rate limit: {} req/min", config.rate_limit);
    tracing::info!("   - Max WebSocket connections: {}", config.max_ws_connections);

    // Create and start console
    let console = Console::new(config.clone()).await?;

    let addr: std::net::SocketAddr = format!("{}:{}", config.bind_address, config.port)
        .parse()
        .expect("Invalid address");
    console.serve(addr).await?;

    Ok(())
}
