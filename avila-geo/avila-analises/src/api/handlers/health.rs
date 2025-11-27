use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::{
    api::{response::ApiResponse, state::AppState, ApiResult},
    storage::StorageStats,
};

static START_TIME: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);

/// Resposta de health check
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub storage: StorageHealthStatus,
}

#[derive(Debug, Serialize)]
pub struct StorageHealthStatus {
    pub connected: bool,
    pub latency_ms: Option<u64>,
}

/// Resposta de métricas
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub storage_stats: StorageStats,
    pub api_stats: ApiStats,
}

#[derive(Debug, Serialize)]
pub struct ApiStats {
    pub uptime_seconds: u64,
    pub memory_usage_mb: f64,
}

/// Resposta de versão
#[derive(Debug, Serialize)]
pub struct VersionResponse {
    pub version: String,
    pub api_version: String,
    pub build_timestamp: String,
}

/// GET /health
/// Health check endpoint
pub async fn health_check(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<HealthResponse>>> {
    let start = Instant::now();

    // Verificar conectividade do storage
    let storage_connected = state.event_store.health_check().await.is_ok();
    let latency = start.elapsed().as_millis() as u64;

    let uptime = START_TIME.elapsed().as_secs();

    let response = HealthResponse {
        status: if storage_connected { "healthy" } else { "degraded" }.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        storage: StorageHealthStatus {
            connected: storage_connected,
            latency_ms: Some(latency),
        },
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /metrics
/// Métricas detalhadas do sistema
pub async fn get_metrics(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<MetricsResponse>>> {
    let storage_stats = state.event_store.get_stats().await?;
    let uptime = START_TIME.elapsed().as_secs();

    // Obter uso de memória (simplificado)
    let memory_usage_mb = 0.0; // TODO: Implementar com jemalloc stats

    let response = MetricsResponse {
        storage_stats,
        api_stats: ApiStats {
            uptime_seconds: uptime,
            memory_usage_mb,
        },
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /version
/// Informações de versão
pub async fn get_version(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<VersionResponse>>> {
    let response = VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        api_version: state.config.api_version.clone(),
        build_timestamp: env!("BUILD_TIMESTAMP").unwrap_or("unknown").to_string(),
    };

    Ok(Json(ApiResponse::ok(response)))
}
