use axum::{extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api::{error::ApiError, response::ApiResponse, state::AppState, ApiResult},
    industry40::*,
};

/// POST /api/v1/industry40/iot/ingest
/// Ingerir dados de telemetria
#[derive(Debug, Deserialize)]
pub struct TelemetryRequest {
    pub device_id: String,
    pub telemetry: iot::ProductionTelemetry,
}

pub async fn ingest_telemetry(
    State(state): State<AppState>,
    Json(payload): Json<TelemetryRequest>,
) -> ApiResult<Json<ApiResponse<String>>> {
    // Em produção: armazenar no AvilaDB via event_store
    Ok(Json(ApiResponse::ok("Telemetry ingested".to_string())))
}

/// POST /api/v1/industry40/maintenance/predict
/// Prever falhas de máquina
#[derive(Debug, Deserialize)]
pub struct PredictFailureRequest {
    pub device_id: String,
    pub telemetry: iot::ProductionTelemetry,
}

#[derive(Debug, Serialize)]
pub struct PredictFailureResponse {
    pub device_id: String,
    pub failure_probability: f64,
    pub alert: Option<predictive_maintenance::MaintenanceAlert>,
}

pub async fn predict_failure(
    State(state): State<AppState>,
    Json(payload): Json<PredictFailureRequest>,
) -> ApiResult<Json<ApiResponse<PredictFailureResponse>>> {
    let mut engine = predictive_maintenance::PredictiveMaintenanceEngine::new();

    // Simular dados históricos para treinamento
    let historical_data = vec![payload.telemetry.clone()];
    engine.train_model(payload.device_id.clone(), &historical_data);

    let alert = engine.predict_failure(&payload.telemetry);
    let failure_prob = alert.as_ref().map(|a| a.failure_probability).unwrap_or(0.0);

    let response = PredictFailureResponse {
        device_id: payload.device_id,
        failure_probability: failure_prob,
        alert,
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/industry40/oee/calculate
/// Calcular OEE
#[derive(Debug, Deserialize)]
pub struct CalculateOEERequest {
    pub device_id: String,
    pub production_data: oee::ProductionData,
    pub target_cycle_time_ms: u64,
    pub planned_hours: f64,
}

pub async fn calculate_oee(
    State(state): State<AppState>,
    Json(payload): Json<CalculateOEERequest>,
) -> ApiResult<Json<ApiResponse<oee::OEEMetrics>>> {
    let calculator = oee::OEECalculator::new(
        payload.target_cycle_time_ms,
        payload.planned_hours,
    );

    let metrics = calculator.calculate_oee(&payload.production_data);

    Ok(Json(ApiResponse::ok(metrics)))
}

/// GET /api/v1/industry40/twin/:device_id
/// Obter gêmeo digital
pub async fn get_digital_twin(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> ApiResult<Json<ApiResponse<digital_twin::TwinState>>> {
    let twin = digital_twin::DigitalTwin::new(
        format!("twin-{}", device_id),
        device_id,
    );

    Ok(Json(ApiResponse::ok(twin.state)))
}

/// POST /api/v1/industry40/optimize/production
/// Otimizar produção
#[derive(Debug, Deserialize)]
pub struct OptimizeProductionRequest {
    pub orders: Vec<production_optimizer::ProductionOrder>,
    pub constraints: production_optimizer::ProductionConstraints,
}

pub async fn optimize_production(
    State(state): State<AppState>,
    Json(payload): Json<OptimizeProductionRequest>,
) -> ApiResult<Json<ApiResponse<production_optimizer::OptimizationResult>>> {
    let optimizer = production_optimizer::ProductionOptimizer::new(payload.constraints);
    let result = optimizer.optimize_schedule(payload.orders);

    Ok(Json(ApiResponse::ok(result)))
}

/// POST /api/v1/industry40/quality/inspect
/// Inspecionar qualidade
#[derive(Debug, Deserialize)]
pub struct InspectRequest {
    pub product: quality_control::Product,
}

pub async fn inspect_quality(
    State(state): State<AppState>,
    Json(payload): Json<InspectRequest>,
) -> ApiResult<Json<ApiResponse<quality_control::InspectionResult>>> {
    let inspector = quality_control::QualityInspector::new();
    let result = inspector.inspect(&payload.product);

    Ok(Json(ApiResponse::ok(result)))
}

/// GET /api/v1/industry40/energy/consumption
/// Obter consumo de energia
pub async fn get_energy_consumption(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<f64>>> {
    let monitor = energy_management::EnergyMonitor::new();
    let consumption = monitor.get_total_consumption();

    Ok(Json(ApiResponse::ok(consumption)))
}

/// POST /api/v1/industry40/anomaly/detect
/// Detectar anomalias
#[derive(Debug, Deserialize)]
pub struct DetectAnomalyRequest {
    pub values: Vec<f64>,
    pub threshold_sigma: f64,
}

#[derive(Debug, Serialize)]
pub struct DetectAnomalyResponse {
    pub anomaly_indices: Vec<usize>,
    pub count: usize,
}

pub async fn detect_anomalies(
    State(state): State<AppState>,
    Json(payload): Json<DetectAnomalyRequest>,
) -> ApiResult<Json<ApiResponse<DetectAnomalyResponse>>> {
    let detector = time_series::AnomalyDetector::new(payload.threshold_sigma);
    let indices = detector.detect(&payload.values);

    let response = DetectAnomalyResponse {
        count: indices.len(),
        anomaly_indices: indices,
    };

    Ok(Json(ApiResponse::ok(response)))
}
