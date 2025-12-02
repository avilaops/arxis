//! Avila Cloud Platform - Main Entry Point
//!
//! A complete cloud provider implementation in Rust

use avila_cloud::error::Result;
use avila_cloud::{compute, storage, network, billing, auth, monitoring, api};
use tracing::{info, error};
use clap::Parser;

#[derive(Parser)]
#[command(name = "avila-cloud")]
#[command(about = "Avila Cloud Platform - Enterprise Cloud Infrastructure Platform", long_about = None)]
struct Cli {
    /// Port to bind API server
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Data directory
    #[arg(short = 'D', long, default_value = "/var/lib/avila-cloud")]
    data_dir: String,

    /// Config file
    #[arg(short, long, default_value = "/etc/avila-cloud/config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    info!("🌩️  Starting Avila Cloud Platform...");
    info!("📊 Data directory: {}", cli.data_dir);
    info!("🔧 Config file: {}", cli.config);

    // Initialize components
    info!("⚙️  Initializing compute manager...");
    let compute = compute::ComputeManager::new()?;

    info!("💾 Initializing storage service...");
    let storage = storage::StorageService::new(cli.data_dir.into())?;

    info!("🌐 Initializing network manager...");
    let network = network::NetworkManager::new();

    info!("💰 Initializing billing system...");
    let billing = billing::BillingManager::new();

    info!("🔐 Initializing auth manager...");
    let auth = auth::AuthManager::new("your-secret-key".to_string());

    info!("📈 Initializing monitoring...");
    let monitoring = monitoring::MonitoringService::new();

    // Start API server
    info!("🚀 Starting API server on port {}...", cli.port);
    let api = api::CloudApi::new(compute, storage, network, billing, auth, monitoring);

    let addr = format!("0.0.0.0:{}", cli.port);

    info!("✅ Avila Cloud Platform is ready!");
    info!("🌍 API listening on {}", addr);
    info!("📖 Documentation: https://docs.avila.cloud");
    info!("💬 Support: https://support.avila.cloud");

    if let Err(e) = api.serve(&addr).await {
        error!("❌ Server error: {}", e);
        return Err(e);
    }

    Ok(())
}
