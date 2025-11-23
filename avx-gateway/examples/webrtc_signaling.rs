//! Exemplo de Signaling Server WebRTC
//!
//! Demonstra servidor de signaling para desktop remoto

use axum::{routing::get, Router};
use avx_gateway::webrtc::{signaling_router, SignalingState};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Iniciando Signaling Server WebRTC");

    // Estado compartilhado
    let state = SignalingState::new();

    // Criar router
    let app = Router::new()
        .route("/", get(|| async { "WebRTC Signaling Server - OK" }))
        .route("/health", get(|| async { "healthy" }))
        .merge(signaling_router())
        .with_state(state);

    // Bind
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("🎧 Escutando em http://{}", addr);
    info!("📡 WebSocket: ws://{}/signal", addr);
    info!("📡 WebSocket com ID: ws://{}/signal/:peer_id", addr);

    // Servir
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
