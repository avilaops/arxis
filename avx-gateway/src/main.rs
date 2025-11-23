//! AVX Gateway - High-performance API gateway for Avila Experience Fabric
//!
//! This is the main entry point for running the gateway as a standalone service.

use avx_gateway::{Gateway, GatewayConfig};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,avx_gateway=debug".into()),
        )
        .init();

    info!("Starting AVX Gateway");

    // Try to load configuration from file, otherwise use default
    let config = load_config().unwrap_or_else(|e| {
        info!("Using default configuration: {}", e);
        create_default_config()
    });

    // Create and start gateway
    let gateway = Gateway::from_config(config).await?;
    gateway.serve().await?;

    Ok(())
}

/// Load configuration from file
fn load_config() -> anyhow::Result<GatewayConfig> {
    let config_path = std::env::var("GATEWAY_CONFIG")
        .unwrap_or_else(|_| "config/gateway.toml".to_string());

    let config_str = std::fs::read_to_string(&config_path)?;
    let config: GatewayConfig = toml::from_str(&config_str)?;

    info!("Loaded configuration from {}", config_path);
    Ok(config)
}

/// Create default configuration with example routes
fn create_default_config() -> GatewayConfig {
    use avx_gateway::config::*;

    GatewayConfig {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: num_cpus::get(),
            timeout_ms: 30_000,
        },
        routes: vec![
            RouteConfig {
                path: "/api/v1/*".to_string(),
                upstream: UpstreamConfig::Single("http://localhost:8001".to_string()),
                methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ],
                strip_path: false,
                timeout_ms: None,
                auth_required: false,
            },
        ],
        middleware: MiddlewareConfig {
            enable_cors: true,
            enable_compression: true,
            enable_rate_limiting: false,
            enable_logging: true,
            enable_metrics: true,
        },
        rate_limiting: None,
        health_check: HealthCheckConfig::default(),
        tls: None,
    }
}


