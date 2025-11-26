use axum::{
    routing::{get, post},
    Router,
};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

use super::{
    handlers::{analytics, events, health, users, industry40, export as export_handler},
    middleware::{
        create_cors_layer, request_logger_middleware, security_headers_middleware,
    },
    state::AppState,
};
use crate::websocket;

/// Criar router principal da API
pub fn create_router(state: AppState) -> Router {
    // Router de eventos
    let events_router = Router::new()
        .route("/", post(events::ingest_event))
        .route("/batch", post(events::ingest_batch))
        .route("/query", post(events::query_events))
        .route("/user/:user_id", get(events::get_user_events))
        .route("/session/:session_id", get(events::get_session_events));

    // Router de analytics
    let analytics_router = Router::new()
        .route("/funnel", post(analytics::analyze_funnel))
        .route("/cohort", post(analytics::analyze_cohort))
        .route("/retention", get(analytics::get_retention))
        .route("/conversion", get(analytics::get_conversion_rate))
        .route("/overview", get(analytics::get_overview));

    // Router de usuários
    let users_router = Router::new()
        .route("/:user_id", get(users::get_user_profile))
        .route("/:user_id/segment", get(users::get_user_segment))
        .route("/:user_id/predictions", get(users::get_user_predictions))
        .route("/segments", get(users::list_segments));

    // Router de Industry 4.0
    let industry40_router = Router::new()
        .route("/sensors", post(industry40::ingest_sensor_data))
        .route("/sensors/:sensor_id", get(industry40::get_sensor_data))
        .route("/maintenance/predict", post(industry40::predict_maintenance))
        .route("/oee", get(industry40::calculate_oee))
        .route("/production/optimize", post(industry40::optimize_production));

    // Router de export
    let export_router = Router::new()
        .route("/", post(export_handler::export_data))
        .route("/:filename", get(export_handler::download_export));

    // Router Industry 4.0
    let industry40_router = Router::new()
        .route("/iot/ingest", post(industry40::ingest_telemetry))
        .route("/maintenance/predict", post(industry40::predict_failure))
        .route("/oee/calculate", post(industry40::calculate_oee))
        .route("/twin/:device_id", get(industry40::get_digital_twin))
        .route("/optimize/production", post(industry40::optimize_production))
        .route("/quality/inspect", post(industry40::inspect_quality))
        .route("/energy/consumption", get(industry40::get_energy_consumption))
        .route("/anomaly/detect", post(industry40::detect_anomalies));

    // Router principal v1
    let v1_router = Router::new()
        .nest("/events", events_router)
        .nest("/analytics", analytics_router)
        .nest("/users", users_router)
        .nest("/industry40", industry40_router)
        .nest("/export", export_router)
        .route("/stats", get(analytics::get_stats));

    // Router público (sem auth)
    let public_router = Router::new()
        .route("/health", get(health::health_check))
        .route("/metrics", get(health::get_metrics))
        .route("/version", get(health::get_version))
        .route("/ws", get(websocket::ws_handler));

    // Combinar todos os routers
    Router::new()
        .nest("/api/v1", v1_router)
        .merge(public_router)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(request_logger_middleware))
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .layer(create_cors_layer())
        .with_state(state)
}
