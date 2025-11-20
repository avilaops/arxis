use avx_config::AvxConfig;
use avx_telemetry::{self, AvxContext};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cfg = AvxConfig::load().unwrap_or_else(|_| AvxConfig::with_defaults());
    cfg.http.bind_addr = "0.0.0.0:8081".into(); // porta diferente do gateway

    let ctx = AvxContext {
        stack: cfg.stack.clone(),
        layer: cfg.layer.clone(),
        env: cfg.env.clone(),
        cluster: cfg.cluster.clone(),
        mesh: cfg.mesh.clone(),
    };

    avx_telemetry::init_tracing(&ctx);

    let app = Router::new()
        .route("/core/ping", get(|| async { "pong from avx-api-core" }))
        .route("/core/status", get(status_handler));

    let addr: SocketAddr = cfg.http.bind_addr.parse()?;
    info!(%addr, "avx-api-core listening");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

async fn status_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "service": "avx-api-core",
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
