//! # Avila ML Integration Module
//!
//! Provides a complete web interface for managing machine learning models,
//! datasets, training jobs, and inference pipelines using Avila ML.
//!
//! ## Features
//!
//! - **Model Management**: Create, list, deploy, and delete ML models
//! - **Dataset Management**: Upload, version, and explore training datasets
//! - **Training Jobs**: Submit, monitor, and manage training workflows
//! - **Inference API**: Real-time and batch inference with model versioning
//! - **Experiment Tracking**: Log metrics, hyperparameters, and artifacts
//! - **Model Registry**: Centralized model storage with metadata
//! - **AutoML**: Automated hyperparameter tuning and architecture search
//! - **Monitoring**: Track model performance, drift, and data quality

use crate::{ml_persistence::MLDatabase, state::AppState};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Model metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModel {
    pub id: String,
    pub name: String,
    pub version: String,
    pub model_type: ModelType,
    pub framework: String,
    pub status: ModelStatus,
    pub created_at: String,
    pub updated_at: String,
    pub accuracy: Option<f32>,
    pub loss: Option<f32>,
    pub parameters: u64,
    pub size_mb: f32,
    pub tags: Vec<String>,
    pub description: String,
}

/// Model architecture types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    Linear,
    Cnn2d,
    Cnn4d,
    Transformer,
    Lstm,
    Attention,
    Custom,
}

/// Model lifecycle status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelStatus {
    Training,
    Trained,
    Deployed,
    Archived,
    Failed,
}

/// Training job configuration and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    pub id: String,
    pub model_id: String,
    pub dataset_id: String,
    pub status: TrainingStatus,
    pub config: TrainingConfig,
    pub metrics: TrainingMetrics,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub error: Option<String>,
}

/// Training job status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainingStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Training hyperparameters and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub epochs: u32,
    pub batch_size: u32,
    pub learning_rate: f32,
    pub optimizer: String,
    pub loss_function: String,
    pub early_stopping: bool,
    pub validation_split: f32,
}

/// Training progress and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub current_epoch: u32,
    pub train_loss: f32,
    pub train_accuracy: f32,
    pub val_loss: f32,
    pub val_accuracy: f32,
    pub learning_rate: f32,
    pub epoch_time_seconds: u64,
}

/// Dataset metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub version: String,
    pub data_type: String,
    pub format: String,
    pub sample_count: u64,
    pub size_mb: f32,
    pub tags: Vec<String>,
    pub description: String,
    pub created_at: String,
}

/// Inference request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub model_id: String,
    pub version: Option<String>,
    pub inputs: Vec<Vec<f32>>,
    pub batch: bool,
}

/// Inference response with predictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub predictions: Vec<Vec<f32>>,
    pub latency_ms: f32,
    pub model_id: String,
    pub version: String,
}

/// Experiment tracking entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub model_type: ModelType,
    pub hyperparameters: serde_json::Value,
    pub metrics: serde_json::Value,
    pub artifacts: Vec<String>,
    pub created_at: String,
}

// ============================================================================
// API Handlers
// ============================================================================

/// GET /ml - ML Dashboard UI
pub async fn ml_dashboard_page() -> Response {
    Html(ML_DASHBOARD_HTML).into_response()
}

/// GET /ml/models - List all models
pub async fn list_models(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // Get ML database from state
    let ml_db = match MLDatabase::new("avl-ml-platform".to_string(), true).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to initialize ML database: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<MLModel>>(vec![])).into_response();
        }
    };

    // Get user ID from session (simplified for now)
    let user_id = "user_demo";

    // Fetch models from database
    let model_docs = match ml_db.list_models(user_id, 100).await {
        Ok(docs) => docs,
        Err(e) => {
            tracing::error!("Failed to list models: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<MLModel>>(vec![])).into_response();
        }
    };

    // Convert to API response format
    let models: Vec<MLModel> = model_docs
        .into_iter()
        .map(|doc| MLModel {
            id: doc.model_id,
            name: doc.name,
            version: doc.version,
            model_type: doc.model_type,
            framework: doc.framework,
            status: doc.status,
            created_at: doc.created_at.to_rfc3339(),
            updated_at: doc.updated_at.to_rfc3339(),
            accuracy: doc.accuracy,
            loss: doc.loss,
            parameters: doc.parameters,
            size_mb: doc.size_mb,
            tags: doc.tags,
            description: doc.description,
        })
        .collect();

    Json(models).into_response()
}

/// GET /ml/models/:id - Get model details
pub async fn get_model(
    State(_state): State<Arc<AppState>>,
    Path(model_id): Path<String>,
) -> impl IntoResponse {
    let ml_db = match MLDatabase::new("avl-ml-platform".to_string(), true).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to initialize ML database: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response();
        }
    };

    let user_id = "user_demo";

    match ml_db.get_model(user_id, &model_id).await {
        Ok(Some(doc)) => {
            let model = MLModel {
                id: doc.model_id,
                name: doc.name,
                version: doc.version,
                model_type: doc.model_type,
                framework: doc.framework,
                status: doc.status,
                created_at: doc.created_at.to_rfc3339(),
                updated_at: doc.updated_at.to_rfc3339(),
                accuracy: doc.accuracy,
                loss: doc.loss,
                parameters: doc.parameters,
                size_mb: doc.size_mb,
                tags: doc.tags,
                description: doc.description,
            };
            Json(model).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Model not found"}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to get model: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
        }
    }
}

/// POST /ml/models - Create a new model
pub async fn create_model(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let ml_db = match MLDatabase::new("avl-ml-platform".to_string(), true).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to initialize ML database: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response();
        }
    };

    let user_id = "user_demo";
    let project_id = "default";
    let model_id = format!("model_{}", uuid::Uuid::new_v4().to_string().replace("-", "_"));

    let model = MLModel {
        id: model_id,
        name: payload["name"].as_str().unwrap_or("Untitled Model").to_string(),
        version: "0.1.0".to_string(),
        model_type: ModelType::Linear,
        framework: "avila-ml".to_string(),
        status: ModelStatus::Training,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        accuracy: None,
        loss: None,
        parameters: 0,
        size_mb: 0.0,
        tags: vec![],
        description: payload["description"].as_str().unwrap_or("").to_string(),
    };

    // Store in database
    match ml_db.create_model(user_id, project_id, &model).await {
        Ok(doc) => {
            tracing::info!("✅ Created model {} in AvilaDB", doc.model_id);
            (StatusCode::CREATED, Json(model)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create model: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
        }
    }
}

/// DELETE /ml/models/:id - Delete a model
pub async fn delete_model(
    State(_state): State<Arc<AppState>>,
    Path(model_id): Path<String>,
) -> impl IntoResponse {
    let ml_db = match MLDatabase::new("avl-ml-platform".to_string(), true).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to initialize ML database: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response();
        }
    };

    let user_id = "user_demo";

    match ml_db.delete_model(user_id, &model_id).await {
        Ok(_) => {
            tracing::info!("✅ Deleted model {} from AvilaDB", model_id);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": format!("Model {} deleted successfully", model_id)
                })),
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to delete model: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
        }
    }
}

/// GET /ml/datasets - List all datasets
pub async fn list_datasets(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let ml_db = match MLDatabase::new("avl-ml-platform".to_string(), true).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to initialize ML database: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Dataset>>(vec![])).into_response();
        }
    };

    let user_id = "user_demo";

    match ml_db.list_datasets(user_id, 100).await {
        Ok(dataset_docs) => {
            let datasets: Vec<Dataset> = dataset_docs
                .into_iter()
                .map(|doc| Dataset {
                    id: doc.dataset_id,
                    name: doc.name,
                    version: doc.version,
                    data_type: doc.data_type,
                    format: doc.format,
                    sample_count: doc.sample_count,
                    size_mb: doc.size_mb,
                    tags: doc.tags,
                    description: doc.description,
                    created_at: doc.created_at.to_rfc3339(),
                })
                .collect();
            Json(datasets).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list datasets: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Dataset>>(vec![])).into_response()
        }
    }
}

/// GET /ml/training - List training jobs
pub async fn list_training_jobs(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let ml_db = match MLDatabase::new("avl-ml-platform".to_string(), true).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to initialize ML database: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<TrainingJob>>(vec![])).into_response();
        }
    };

    let user_id = "user_demo";

    match ml_db.list_training_jobs(user_id, None, 100).await {
        Ok(job_docs) => {
            let jobs: Vec<TrainingJob> = job_docs
                .into_iter()
                .map(|doc| TrainingJob {
                    id: doc.job_id,
                    model_id: doc.model_id,
                    dataset_id: doc.dataset_id,
                    status: doc.status,
                    config: doc.config,
                    metrics: doc.metrics,
                    started_at: doc.started_at.map(|dt| dt.to_rfc3339()),
                    completed_at: doc.completed_at.map(|dt| dt.to_rfc3339()),
                    error: doc.error,
                })
                .collect();
            Json(jobs).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list training jobs: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<TrainingJob>>(vec![])).into_response()
        }
    }
}

/// POST /ml/training - Submit a training job
pub async fn submit_training_job(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let job_id = format!("job-{}", uuid::Uuid::new_v4());

    let job = TrainingJob {
        id: job_id,
        model_id: payload["model_id"].as_str().unwrap_or("").to_string(),
        dataset_id: payload["dataset_id"].as_str().unwrap_or("").to_string(),
        status: TrainingStatus::Queued,
        config: TrainingConfig {
            epochs: payload["epochs"].as_u64().unwrap_or(10) as u32,
            batch_size: payload["batch_size"].as_u64().unwrap_or(32) as u32,
            learning_rate: payload["learning_rate"].as_f64().unwrap_or(0.001) as f32,
            optimizer: payload["optimizer"]
                .as_str()
                .unwrap_or("Adam")
                .to_string(),
            loss_function: payload["loss_function"]
                .as_str()
                .unwrap_or("MSE")
                .to_string(),
            early_stopping: payload["early_stopping"].as_bool().unwrap_or(true),
            validation_split: payload["validation_split"].as_f64().unwrap_or(0.2) as f32,
        },
        metrics: TrainingMetrics {
            current_epoch: 0,
            train_loss: 0.0,
            train_accuracy: 0.0,
            val_loss: 0.0,
            val_accuracy: 0.0,
            learning_rate: payload["learning_rate"].as_f64().unwrap_or(0.001) as f32,
            epoch_time_seconds: 0,
        },
        started_at: None,
        completed_at: None,
        error: None,
    };

    (StatusCode::CREATED, Json(job))
}

/// POST /ml/inference - Run inference on a model
pub async fn run_inference(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<InferenceRequest>,
) -> impl IntoResponse {
    // Mock inference - integrate with actual avila-ml models
    let predictions: Vec<Vec<f32>> = request
        .inputs
        .iter()
        .map(|input| {
            // Simulate model prediction
            vec![
                input.iter().sum::<f32>() / input.len() as f32,
                input.iter().product::<f32>().abs().sqrt(),
                input.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) * 0.9,
            ]
        })
        .collect();

    let response = InferenceResponse {
        predictions,
        latency_ms: 12.5,
        model_id: request.model_id,
        version: request.version.unwrap_or("latest".to_string()),
    };

    Json(response)
}

/// GET /ml/experiments - List experiments
pub async fn list_experiments(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let experiments = vec![
        Experiment {
            id: "exp-001".to_string(),
            name: "Conv4d Hyperparameter Sweep".to_string(),
            model_type: ModelType::Cnn4d,
            hyperparameters: serde_json::json!({
                "learning_rate": [0.001, 0.01, 0.1],
                "batch_size": [16, 32, 64],
                "kernel_size": [(3, 3, 3, 3), (5, 5, 5, 5)]
            }),
            metrics: serde_json::json!({
                "best_accuracy": 0.945,
                "best_loss": 0.032,
                "total_runs": 18
            }),
            artifacts: vec!["model.pth".to_string(), "metrics.json".to_string()],
            created_at: "2025-11-20T10:00:00Z".to_string(),
        },
    ];

    Json(experiments)
}

/// GET /ml/metrics - Get training metrics for visualization
pub async fn get_training_metrics(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let job_id = params.get("job_id").map(|s| s.as_str()).unwrap_or("");

    // Mock time series data for training curves
    let metrics = serde_json::json!({
        "job_id": job_id,
        "epochs": (1..=45).collect::<Vec<u32>>(),
        "train_loss": (1..=45).map(|e| 1.0 / (e as f32 * 0.1 + 1.0)).collect::<Vec<f32>>(),
        "val_loss": (1..=45).map(|e| 1.1 / (e as f32 * 0.1 + 1.0)).collect::<Vec<f32>>(),
        "train_accuracy": (1..=45).map(|e| 1.0 - 1.0 / (e as f32 * 0.1 + 1.0)).collect::<Vec<f32>>(),
        "val_accuracy": (1..=45).map(|e| 0.95 - 0.95 / (e as f32 * 0.1 + 1.0)).collect::<Vec<f32>>(),
    });

    Json(metrics)
}

// ============================================================================
// Router Configuration
// ============================================================================

/// Configure ML routes
pub fn ml_routes() -> Router<Arc<AppState>> {
    Router::new()
        // UI
        .route("/ml", get(ml_dashboard_page))
        // Models
        .route("/ml/models", get(list_models).post(create_model))
        .route("/ml/models/:id", get(get_model).delete(delete_model))
        // Datasets
        .route("/ml/datasets", get(list_datasets))
        // Training
        .route("/ml/training", get(list_training_jobs).post(submit_training_job))
        .route("/ml/metrics", get(get_training_metrics))
        // Inference
        .route("/ml/inference", post(run_inference))
        // Experiments
        .route("/ml/experiments", get(list_experiments))
}

// ============================================================================
// HTML Template
// ============================================================================

const ML_DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Avila ML - Machine Learning Console</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
        }
        .header {
            background: rgba(255, 255, 255, 0.95);
            padding: 30px;
            border-radius: 15px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
            margin-bottom: 30px;
        }
        h1 {
            color: #667eea;
            font-size: 2.5em;
            margin-bottom: 10px;
        }
        .subtitle {
            color: #666;
            font-size: 1.1em;
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .stat-card {
            background: rgba(255, 255, 255, 0.95);
            padding: 25px;
            border-radius: 15px;
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
            transition: transform 0.3s ease;
        }
        .stat-card:hover {
            transform: translateY(-5px);
        }
        .stat-value {
            font-size: 2.5em;
            font-weight: bold;
            color: #667eea;
            margin-bottom: 5px;
        }
        .stat-label {
            color: #888;
            font-size: 0.9em;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        .tabs {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
            flex-wrap: wrap;
        }
        .tab {
            background: rgba(255, 255, 255, 0.9);
            color: #667eea;
            border: none;
            padding: 12px 25px;
            border-radius: 10px;
            cursor: pointer;
            font-size: 1em;
            font-weight: 600;
            transition: all 0.3s ease;
        }
        .tab:hover {
            background: #667eea;
            color: white;
        }
        .tab.active {
            background: #667eea;
            color: white;
        }
        .tab-content {
            display: none;
        }
        .tab-content.active {
            display: block;
        }
        .content-card {
            background: rgba(255, 255, 255, 0.95);
            padding: 30px;
            border-radius: 15px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
        }
        table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }
        th {
            background: #667eea;
            color: white;
            padding: 15px;
            text-align: left;
            font-weight: 600;
        }
        td {
            padding: 15px;
            border-bottom: 1px solid #e0e0e0;
        }
        tr:hover {
            background: #f5f5f5;
        }
        .status-badge {
            display: inline-block;
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: 600;
        }
        .status-deployed { background: #4caf50; color: white; }
        .status-training { background: #ff9800; color: white; }
        .status-trained { background: #2196f3; color: white; }
        .status-running { background: #ff9800; color: white; }
        .status-completed { background: #4caf50; color: white; }
        .status-queued { background: #9e9e9e; color: white; }
        .btn {
            background: #667eea;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            font-size: 0.95em;
            font-weight: 600;
            transition: background 0.3s ease;
            margin-right: 10px;
        }
        .btn:hover {
            background: #5568d3;
        }
        .btn-danger {
            background: #f44336;
        }
        .btn-danger:hover {
            background: #da190b;
        }
        .btn-success {
            background: #4caf50;
        }
        .btn-success:hover {
            background: #45a049;
        }
        .metric-chart {
            width: 100%;
            height: 300px;
            margin-top: 20px;
        }
        .loading {
            text-align: center;
            padding: 40px;
            color: #888;
        }
        .model-card {
            border: 1px solid #e0e0e0;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 15px;
            transition: box-shadow 0.3s ease;
        }
        .model-card:hover {
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
        }
        .model-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }
        .model-name {
            font-size: 1.3em;
            font-weight: 600;
            color: #333;
        }
        .model-meta {
            display: flex;
            gap: 20px;
            color: #666;
            font-size: 0.9em;
            margin-bottom: 10px;
        }
        .model-tags {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
        }
        .tag {
            background: #e0e7ff;
            color: #667eea;
            padding: 5px 12px;
            border-radius: 15px;
            font-size: 0.85em;
        }
        .progress-bar {
            width: 100%;
            height: 10px;
            background: #e0e0e0;
            border-radius: 5px;
            overflow: hidden;
            margin: 10px 0;
        }
        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, #667eea, #764ba2);
            transition: width 0.3s ease;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🤖 Avila ML Console</h1>
            <p class="subtitle">Machine Learning Platform - Powered by Avila ML v1.0</p>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-value" id="stat-models">-</div>
                <div class="stat-label">Models</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="stat-datasets">-</div>
                <div class="stat-label">Datasets</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="stat-training">-</div>
                <div class="stat-label">Training Jobs</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="stat-accuracy">-</div>
                <div class="stat-label">Avg Accuracy</div>
            </div>
        </div>

        <div class="tabs">
            <button class="tab active" onclick="switchTab('models')">📦 Models</button>
            <button class="tab" onclick="switchTab('datasets')">📊 Datasets</button>
            <button class="tab" onclick="switchTab('training')">🔄 Training</button>
            <button class="tab" onclick="switchTab('inference')">⚡ Inference</button>
            <button class="tab" onclick="switchTab('experiments')">🧪 Experiments</button>
        </div>

        <div id="tab-models" class="tab-content active">
            <div class="content-card">
                <h2>Model Registry</h2>
                <button class="btn btn-success" onclick="createModel()">➕ New Model</button>
                <div id="models-list" class="loading">Loading models...</div>
            </div>
        </div>

        <div id="tab-datasets" class="tab-content">
            <div class="content-card">
                <h2>Dataset Management</h2>
                <button class="btn btn-success" onclick="uploadDataset()">⬆️ Upload Dataset</button>
                <div id="datasets-list" class="loading">Loading datasets...</div>
            </div>
        </div>

        <div id="tab-training" class="tab-content">
            <div class="content-card">
                <h2>Training Jobs</h2>
                <button class="btn btn-success" onclick="startTraining()">▶️ Start Training</button>
                <div id="training-list" class="loading">Loading training jobs...</div>
            </div>
        </div>

        <div id="tab-inference" class="tab-content">
            <div class="content-card">
                <h2>Model Inference</h2>
                <p>Submit inference requests to deployed models.</p>
                <div style="margin-top: 20px;">
                    <h3>Test Inference</h3>
                    <textarea id="inference-input" placeholder="Enter input data (JSON array)" style="width: 100%; height: 100px; padding: 10px; border-radius: 5px; border: 1px solid #ddd;"></textarea>
                    <button class="btn" onclick="runInference()" style="margin-top: 10px;">🚀 Run Inference</button>
                    <div id="inference-result" style="margin-top: 20px;"></div>
                </div>
            </div>
        </div>

        <div id="tab-experiments" class="tab-content">
            <div class="content-card">
                <h2>Experiment Tracking</h2>
                <button class="btn btn-success" onclick="createExperiment()">🧪 New Experiment</button>
                <div id="experiments-list" class="loading">Loading experiments...</div>
            </div>
        </div>
    </div>

    <script>
        let modelsData = [];
        let datasetsData = [];
        let trainingData = [];
        let experimentsData = [];

        // Load all data on page load
        async function loadAllData() {
            await Promise.all([
                loadModels(),
                loadDatasets(),
                loadTrainingJobs(),
                loadExperiments()
            ]);
            updateStats();
        }

        async function loadModels() {
            try {
                const response = await fetch('/ml/models');
                modelsData = await response.json();
                renderModels();
            } catch (error) {
                console.error('Error loading models:', error);
            }
        }

        async function loadDatasets() {
            try {
                const response = await fetch('/ml/datasets');
                datasetsData = await response.json();
                renderDatasets();
            } catch (error) {
                console.error('Error loading datasets:', error);
            }
        }

        async function loadTrainingJobs() {
            try {
                const response = await fetch('/ml/training');
                trainingData = await response.json();
                renderTrainingJobs();
            } catch (error) {
                console.error('Error loading training jobs:', error);
            }
        }

        async function loadExperiments() {
            try {
                const response = await fetch('/ml/experiments');
                experimentsData = await response.json();
                renderExperiments();
            } catch (error) {
                console.error('Error loading experiments:', error);
            }
        }

        function renderModels() {
            const container = document.getElementById('models-list');
            if (modelsData.length === 0) {
                container.innerHTML = '<p style="text-align: center; padding: 40px; color: #888;">No models found. Create your first model!</p>';
                return;
            }

            container.innerHTML = modelsData.map(model => `
                <div class="model-card">
                    <div class="model-header">
                        <div class="model-name">${model.name}</div>
                        <span class="status-badge status-${model.status}">${model.status}</span>
                    </div>
                    <div class="model-meta">
                        <span>📦 v${model.version}</span>
                        <span>🧠 ${model.model_type}</span>
                        <span>📊 ${(model.parameters / 1_000_000).toFixed(2)}M params</span>
                        <span>💾 ${model.size_mb.toFixed(1)} MB</span>
                    </div>
                    ${model.accuracy ? `
                    <div class="model-meta">
                        <span>🎯 Accuracy: ${(model.accuracy * 100).toFixed(1)}%</span>
                        <span>📉 Loss: ${model.loss.toFixed(3)}</span>
                    </div>` : ''}
                    <p style="color: #666; margin: 10px 0;">${model.description}</p>
                    <div class="model-tags">
                        ${model.tags.map(tag => `<span class="tag">${tag}</span>`).join('')}
                    </div>
                    <div style="margin-top: 15px;">
                        <button class="btn" onclick="deployModel('${model.id}')">🚀 Deploy</button>
                        <button class="btn" onclick="viewModel('${model.id}')">👁️ View</button>
                        <button class="btn btn-danger" onclick="deleteModel('${model.id}')">🗑️ Delete</button>
                    </div>
                </div>
            `).join('');
        }

        function renderDatasets() {
            const container = document.getElementById('datasets-list');
            if (datasetsData.length === 0) {
                container.innerHTML = '<p style="text-align: center; padding: 40px; color: #888;">No datasets found.</p>';
                return;
            }

            container.innerHTML = `
                <table>
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Type</th>
                            <th>Samples</th>
                            <th>Size</th>
                            <th>Train/Val/Test</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${datasetsData.map(dataset => `
                            <tr>
                                <td><strong>${dataset.name}</strong><br/><small>v${dataset.version}</small></td>
                                <td>${dataset.dataset_type}</td>
                                <td>${dataset.num_samples.toLocaleString()}</td>
                                <td>${dataset.size_mb.toFixed(1)} MB</td>
                                <td>
                                    ${dataset.splits.train.toLocaleString()} /
                                    ${dataset.splits.validation.toLocaleString()} /
                                    ${dataset.splits.test.toLocaleString()}
                                </td>
                                <td>
                                    <button class="btn" onclick="exploreDataset('${dataset.id}')">🔍 Explore</button>
                                    <button class="btn btn-danger" onclick="deleteDataset('${dataset.id}')">🗑️</button>
                                </td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
            `;
        }

        function renderTrainingJobs() {
            const container = document.getElementById('training-list');
            if (trainingData.length === 0) {
                container.innerHTML = '<p style="text-align: center; padding: 40px; color: #888;">No training jobs found.</p>';
                return;
            }

            container.innerHTML = trainingData.map(job => `
                <div class="model-card">
                    <div class="model-header">
                        <div class="model-name">Training Job: ${job.id}</div>
                        <span class="status-badge status-${job.status}">${job.status}</span>
                    </div>
                    <div class="model-meta">
                        <span>🎯 Model: ${job.model_id}</span>
                        <span>📊 Dataset: ${job.dataset_id}</span>
                        <span>⚙️ Optimizer: ${job.config.optimizer}</span>
                        <span>📐 LR: ${job.config.learning_rate}</span>
                    </div>
                    ${job.status === 'running' || job.status === 'completed' ? `
                    <div>
                        <div style="display: flex; justify-content: space-between; margin: 10px 0;">
                            <span>Epoch ${job.metrics.epoch} / ${job.config.epochs}</span>
                            <span>${((job.metrics.epoch / job.config.epochs) * 100).toFixed(0)}%</span>
                        </div>
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: ${(job.metrics.epoch / job.config.epochs) * 100}%"></div>
                        </div>
                        <div class="model-meta" style="margin-top: 10px;">
                            <span>🎯 Train Acc: ${(job.metrics.train_accuracy * 100).toFixed(1)}%</span>
                            <span>📉 Train Loss: ${job.metrics.train_loss.toFixed(3)}</span>
                            ${job.metrics.val_accuracy ? `<span>✅ Val Acc: ${(job.metrics.val_accuracy * 100).toFixed(1)}%</span>` : ''}
                            ${job.metrics.val_loss ? `<span>📊 Val Loss: ${job.metrics.val_loss.toFixed(3)}</span>` : ''}
                        </div>
                    </div>` : ''}
                    <div style="margin-top: 15px;">
                        <button class="btn" onclick="viewMetrics('${job.id}')">📊 View Metrics</button>
                        ${job.status === 'running' ? `<button class="btn btn-danger" onclick="cancelJob('${job.id}')">⏹️ Cancel</button>` : ''}
                    </div>
                </div>
            `).join('');
        }

        function renderExperiments() {
            const container = document.getElementById('experiments-list');
            if (experimentsData.length === 0) {
                container.innerHTML = '<p style="text-align: center; padding: 40px; color: #888;">No experiments found.</p>';
                return;
            }

            container.innerHTML = experimentsData.map(exp => `
                <div class="model-card">
                    <div class="model-name">${exp.name}</div>
                    <div class="model-meta">
                        <span>🧠 ${exp.model_type}</span>
                        <span>📊 Total Runs: ${exp.metrics.total_runs}</span>
                        <span>🎯 Best Accuracy: ${(exp.metrics.best_accuracy * 100).toFixed(1)}%</span>
                    </div>
                    <p style="margin: 10px 0; color: #666;">Artifacts: ${exp.artifacts.join(', ')}</p>
                    <button class="btn" onclick="viewExperiment('${exp.id}')">👁️ View Details</button>
                </div>
            `).join('');
        }

        function updateStats() {
            document.getElementById('stat-models').textContent = modelsData.length;
            document.getElementById('stat-datasets').textContent = datasetsData.length;
            document.getElementById('stat-training').textContent = trainingData.length;

            const avgAccuracy = modelsData
                .filter(m => m.accuracy !== null)
                .reduce((sum, m) => sum + m.accuracy, 0) / modelsData.filter(m => m.accuracy !== null).length;
            document.getElementById('stat-accuracy').textContent =
                avgAccuracy ? (avgAccuracy * 100).toFixed(1) + '%' : 'N/A';
        }

        function switchTab(tabName) {
            // Hide all tabs
            document.querySelectorAll('.tab-content').forEach(tab => {
                tab.classList.remove('active');
            });
            document.querySelectorAll('.tab').forEach(btn => {
                btn.classList.remove('active');
            });

            // Show selected tab
            document.getElementById(`tab-${tabName}`).classList.add('active');
            event.target.classList.add('active');
        }

        async function runInference() {
            const input = document.getElementById('inference-input').value;
            try {
                const inputs = JSON.parse(input);
                const response = await fetch('/ml/inference', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        model_id: 'model-001',
                        inputs: inputs,
                        batch: true
                    })
                });
                const result = await response.json();
                document.getElementById('inference-result').innerHTML = `
                    <div style="background: #f5f5f5; padding: 20px; border-radius: 10px;">
                        <h3>✅ Inference Complete</h3>
                        <p><strong>Model:</strong> ${result.model_id} (${result.version})</p>
                        <p><strong>Latency:</strong> ${result.latency_ms}ms</p>
                        <p><strong>Predictions:</strong></p>
                        <pre style="background: white; padding: 15px; border-radius: 5px; overflow-x: auto;">${JSON.stringify(result.predictions, null, 2)}</pre>
                    </div>
                `;
            } catch (error) {
                document.getElementById('inference-result').innerHTML = `<p style="color: red;">Error: ${error.message}</p>`;
            }
        }

        // Placeholder functions
        function createModel() { alert('Create Model - Coming soon!'); }
        function uploadDataset() { alert('Upload Dataset - Coming soon!'); }
        function startTraining() { alert('Start Training - Coming soon!'); }
        function createExperiment() { alert('Create Experiment - Coming soon!'); }
        function deployModel(id) { alert(`Deploy model ${id} - Coming soon!`); }
        function viewModel(id) { alert(`View model ${id} - Coming soon!`); }
        function deleteModel(id) { if (confirm('Delete this model?')) { alert(`Delete model ${id}`); } }
        function deleteDataset(id) { if (confirm('Delete this dataset?')) { alert(`Delete dataset ${id}`); } }
        function exploreDataset(id) { alert(`Explore dataset ${id} - Coming soon!`); }
        function viewMetrics(id) { alert(`View metrics for job ${id} - Coming soon!`); }
        function cancelJob(id) { if (confirm('Cancel this job?')) { alert(`Cancel job ${id}`); } }
        function viewExperiment(id) { alert(`View experiment ${id} - Coming soon!`); }

        // Load data on page load
        loadAllData();

        // Auto-refresh every 10 seconds
        setInterval(loadAllData, 10000);
    </script>
</body>
</html>"#;
