use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

mod models;
mod reconstruction;
mod storage;

use models::*;
use reconstruction::ReconstructionEngine;
use storage::CaseStorage;

#[derive(Clone)]
struct AppState {
    engine: Arc<ReconstructionEngine>,
    storage: Arc<RwLock<CaseStorage>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("burn_reconstruction_api=debug,tower_http=debug")
        .init();

    info!("🏥 Initializing Burn Reconstruction API...");

    // Initialize reconstruction engine
    let engine = Arc::new(ReconstructionEngine::new()?);
    let storage = Arc::new(RwLock::new(CaseStorage::new("./data/cases")?));

    let state = AppState { engine, storage };

    // Build router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/cases", post(create_case))
        .route("/api/cases/:id", get(get_case))
        .route("/api/cases/:id/upload", post(upload_photos))
        .route("/api/cases/:id/reconstruct", post(reconstruct_face))
        .route("/api/cases/:id/simulate", post(simulate_surgery))
        .route("/api/cases/:id/export", get(export_model))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    info!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "burn-reconstruction-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn create_case(
    State(state): State<AppState>,
    Json(payload): Json<CreateCaseRequest>,
) -> Result<Json<CaseResponse>, AppError> {
    info!("Creating new case: patient_id={}", payload.patient_id);

    let case_id = Uuid::new_v4();
    let case = MedicalCase {
        id: case_id,
        patient_id: payload.patient_id,
        created_at: chrono::Utc::now(),
        status: CaseStatus::Created,
        photos: vec![],
        reconstruction: None,
        surgical_plan: None,
    };

    state.storage.write().await.save_case(&case).await?;

    Ok(Json(CaseResponse { case }))
}

async fn get_case(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CaseResponse>, AppError> {
    let case = state.storage.read().await.load_case(id).await?;
    Ok(Json(CaseResponse { case }))
}

async fn upload_photos(
    State(state): State<AppState>,
    Path(case_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    info!("Uploading photos for case {}", case_id);

    let mut photo_ids = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.bytes().await?;

        info!("Received photo: {} ({} bytes)", name, data.len());

        let photo_id = Uuid::new_v4();
        state
            .storage
            .write()
            .await
            .save_photo(case_id, photo_id, &data)
            .await?;

        photo_ids.push(photo_id);
    }

    Ok(Json(UploadResponse { photo_ids }))
}

async fn reconstruct_face(
    State(state): State<AppState>,
    Path(case_id): Path<Uuid>,
) -> Result<Json<ReconstructionResponse>, AppError> {
    info!("Starting 3D reconstruction for case {}", case_id);

    let case = state.storage.read().await.load_case(case_id).await?;

    if case.photos.is_empty() {
        return Err(AppError::BadRequest(
            "No photos uploaded for this case".to_string(),
        ));
    }

    // Load photos
    let mut photos = Vec::new();
    for photo_id in &case.photos {
        let photo = state.storage.read().await.load_photo(case_id, *photo_id).await?;
        photos.push(photo);
    }

    // Run 3D reconstruction
    let reconstruction = state.engine.reconstruct_from_photos(&photos).await?;

    // Save reconstruction
    let mut storage = state.storage.write().await;
    storage
        .update_case_reconstruction(case_id, reconstruction.clone())
        .await?;

    Ok(Json(ReconstructionResponse { reconstruction }))
}

async fn simulate_surgery(
    State(state): State<AppState>,
    Path(case_id): Path<Uuid>,
    Json(payload): Json<SurgicalPlanRequest>,
) -> Result<Json<SimulationResponse>, AppError> {
    info!("Simulating surgery for case {}", case_id);

    let case = state.storage.read().await.load_case(case_id).await?;

    let reconstruction = case
        .reconstruction
        .ok_or_else(|| AppError::BadRequest("Case not reconstructed yet".to_string()))?;

    // Run surgical simulation
    let simulation = state
        .engine
        .simulate_surgical_plan(&reconstruction, &payload)
        .await?;

    Ok(Json(SimulationResponse { simulation }))
}

async fn export_model(
    State(state): State<AppState>,
    Path(case_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!("Exporting 3D model for case {}", case_id);

    let case = state.storage.read().await.load_case(case_id).await?;

    let reconstruction = case
        .reconstruction
        .ok_or_else(|| AppError::BadRequest("Case not reconstructed yet".to_string()))?;

    // Export to OBJ format
    let obj_data = state.engine.export_obj(&reconstruction)?;

    Ok((
        StatusCode::OK,
        [("Content-Type", "model/obj")],
        obj_data,
    ))
}

// Error handling
#[derive(Debug)]
enum AppError {
    Internal(anyhow::Error),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Internal(err) => {
                warn!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}
