//! AVL Console - Main Binary Entry Point
//!
//! This is the production entry point for the AVL Console web application.
//! It initializes all services, configures middleware, and starts the HTTP server.

use avl_console::{Console, ConsoleConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = ConsoleConfig::from_env()?;

    // Create console
    let console = Console::new(config.clone()).await?;

    // Bind address
    let addr: SocketAddr = format!("{}:{}", config.bind_address, config.port)
        .parse()
        .expect("Invalid address");

    // Start server
    console.serve(addr).await?;

    Ok(())
}
