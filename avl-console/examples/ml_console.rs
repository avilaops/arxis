//! ML Console - Simple Example
//!
//! Run with: cargo run --example ml_console --features with-ml
//! Then open: http://localhost:3000/ml

use avl_console::{Console, ConsoleConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🧠 Starting AVL Console with ML Integration...\n");

    // Create simple configuration
    let config = ConsoleConfig {
        bind_address: "127.0.0.1".to_string(),
        port: 3000,
        debug: true,
        ..Default::default()
    };

    // Create and start console
    let console = Console::new(config).await?;

    println!("✅ AVL Console is ready!\n");
    println!("📍 Open these URLs in your browser:\n");
    println!("   🖥️  Dashboard:    http://localhost:3000/dashboard");
    println!("   🗄️  AvilaDB:      http://localhost:3000/databases");
    println!("   🧠 ML Platform:  http://localhost:3000/ml");
    println!("   🤖 AI Assistant: http://localhost:3000/ai-assistant");
    println!("   📊 Monitoring:   http://localhost:3000/monitoring");
    println!("   👥 Teams:        http://localhost:3000/teams\n");
    println!("Press Ctrl+C to stop the server.\n");

    // Start server
    console.serve("127.0.0.1:3000").await?;

    Ok(())
}
