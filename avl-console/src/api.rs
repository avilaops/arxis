//! REST API routes

use crate::state::AppState;
use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/version", get(version))
        .nest("/auth", crate::auth::routes(state.clone()))
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    services: ServiceStatus,
}

#[derive(Serialize)]
struct ServiceStatus {
    aviladb: bool,
    storage: bool,
    observability: bool,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services: ServiceStatus {
            aviladb: true,
            storage: true,
            observability: true,
        },
    })
}

#[derive(Serialize)]
struct VersionResponse {
    version: String,
    build_date: String,
    git_commit: String,
}

async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_date: "2024-11-23".to_string(),
        git_commit: "dev".to_string(),
    })
}
