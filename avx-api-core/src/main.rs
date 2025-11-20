use avx_config::AvxConfig;
use avx_telemetry::{self, AvxContext, AvxMetrics};
use axum::{extract::State, routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

#[derive(Clone)]
struct AppState {
    metrics: AvxMetrics,
}

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

    let metrics = AvxMetrics::new();

    let state = AppState { metrics };

    let app = Router::new()
        .route("/core/ping", get(|| async { "pong from avx-api-core" }))
        .route("/core/status", get(status_handler))
        .route("/core/forecast", get(forecast_handler))
        .with_state(state);

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

async fn forecast_handler(State(state): State<AppState>) -> axum::Json<serde_json::Value> {
    // Simulate historical request rates (requests/second)
    let historical_rates = vec![
        100.0, 105.0, 98.0, 110.0, 115.0, 102.0, 108.0, 112.0, 107.0, 120.0,
    ];

    match state.metrics.forecast_metric(historical_rates.clone(), 5) {
        Ok(forecast) => {
            info!(forecast_steps = 5, "Generated forecast for request rates");
            axum::Json(serde_json::json!({
                "service": "avx-api-core",
                "historical_data": historical_rates,
                "forecast": forecast,
                "forecast_steps": 5,
                "model": "ARIMA(1,1,1)"
            }))
        }
        Err(e) => axum::Json(serde_json::json!({
            "error": e,
            "service": "avx-api-core"
        })),
    }
}
