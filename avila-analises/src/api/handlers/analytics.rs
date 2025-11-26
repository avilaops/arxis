use axum::{extract::{Path, Query, State}, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    api::{error::ApiError, response::ApiResponse, state::AppState, ApiResult},
    cohort::{CohortAnalyzer, CohortBuilder},
    funnel::{Funnel, FunnelAnalyzer, FunnelStep, FunnelCondition},
    models::BehaviorEvent,
    storage::EventFilter,
};

/// Payload para análise de funil
#[derive(Debug, Deserialize)]
pub struct FunnelAnalysisRequest {
    pub funnel_name: String,
    pub steps: Vec<FunnelStepRequest>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct FunnelStepRequest {
    pub name: String,
    pub condition_type: String,
    pub condition_value: String,
}

/// Resposta de análise de funil
#[derive(Debug, Serialize)]
pub struct FunnelAnalysisResponse {
    pub funnel_name: String,
    pub total_entered: usize,
    pub overall_conversion_rate: f64,
    pub steps: Vec<StepAnalysis>,
}

#[derive(Debug, Serialize)]
pub struct StepAnalysis {
    pub step_name: String,
    pub users_entered: usize,
    pub users_completed: usize,
    pub conversion_rate: f64,
    pub drop_off_rate: f64,
}

/// Payload para análise de cohort
#[derive(Debug, Deserialize)]
pub struct CohortAnalysisRequest {
    pub period: String, // "daily", "weekly", "monthly"
    pub periods_count: usize,
}

/// Resposta de análise de cohort
#[derive(Debug, Serialize)]
pub struct CohortAnalysisResponse {
    pub cohorts: Vec<CohortData>,
    pub retention_matrix: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize)]
pub struct CohortData {
    pub cohort_id: String,
    pub size: usize,
    pub retention_rates: Vec<f64>,
}

/// Query params para filtros de tempo
#[derive(Debug, Deserialize)]
pub struct TimeRangeParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

/// Resposta de overview
#[derive(Debug, Serialize)]
pub struct OverviewResponse {
    pub total_events: usize,
    pub total_users: usize,
    pub total_sessions: usize,
    pub events_by_type: std::collections::HashMap<String, usize>,
    pub top_pages: Vec<(String, usize)>,
    pub conversion_rate: f64,
    pub avg_session_duration: f64,
}

/// POST /api/v1/analytics/funnel
/// Analisar funil de conversão
pub async fn analyze_funnel(
    State(state): State<AppState>,
    Json(payload): Json<FunnelAnalysisRequest>,
) -> ApiResult<Json<ApiResponse<FunnelAnalysisResponse>>> {
    // Validar steps
    if payload.steps.is_empty() {
        return Err(ApiError::BadRequest("Funnel must have at least one step".to_string()));
    }

    // Construir funil
    let funnel_steps: Vec<FunnelStep> = payload
        .steps
        .iter()
        .map(|step| {
            let condition = match step.condition_type.as_str() {
                "page_view" => FunnelCondition::PageView(step.condition_value.clone()),
                "purchase" => FunnelCondition::PurchaseCompleted,
                "add_to_cart" => FunnelCondition::AddedToCart,
                _ => FunnelCondition::EventType(step.condition_value.clone()),
            };

            FunnelStep {
                name: step.name.clone(),
                condition,
            }
        })
        .collect();

    let funnel = Funnel {
        name: payload.funnel_name.clone(),
        steps: funnel_steps,
    };

    // Buscar eventos
    let filter = EventFilter {
        start_time: payload.start_time,
        end_time: payload.end_time,
        ..Default::default()
    };

    let events = state.event_store.query(filter).await?;

    // Analisar funil
    let analyzer = FunnelAnalyzer::new();
    let analysis = analyzer.analyze_funnel(&funnel, &events);

    // Construir resposta
    let mut steps_response = Vec::new();
    let mut prev_completed = analysis.total_entered;

    for step_conv in &analysis.step_conversions {
        let drop_off_rate = if prev_completed > 0 {
            ((prev_completed - step_conv.users_completed) as f64 / prev_completed as f64) * 100.0
        } else {
            0.0
        };

        steps_response.push(StepAnalysis {
            step_name: funnel.steps[step_conv.step_index].name.clone(),
            users_entered: step_conv.users_entered,
            users_completed: step_conv.users_completed,
            conversion_rate: step_conv.conversion_rate * 100.0,
            drop_off_rate,
        });

        prev_completed = step_conv.users_completed;
    }

    let overall_conversion = if analysis.total_entered > 0 && !analysis.step_conversions.is_empty() {
        let last_step = analysis.step_conversions.last().unwrap();
        (last_step.users_completed as f64 / analysis.total_entered as f64) * 100.0
    } else {
        0.0
    };

    let response = FunnelAnalysisResponse {
        funnel_name: payload.funnel_name,
        total_entered: analysis.total_entered,
        overall_conversion_rate: overall_conversion,
        steps: steps_response,
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/analytics/cohort
/// Analisar retenção por cohort
pub async fn analyze_cohort(
    State(state): State<AppState>,
    Json(payload): Json<CohortAnalysisRequest>,
) -> ApiResult<Json<ApiResponse<CohortAnalysisResponse>>> {
    // Buscar todos os eventos
    let events = state.event_store.query(EventFilter::default()).await?;

    // Gerar perfis de usuário (simplificado)
    let user_profiles = crate::generate_user_profiles(&events);

    // Criar cohorts
    let cohort_period = match payload.period.as_str() {
        "daily" => CohortBuilder::daily().build(),
        "weekly" => CohortBuilder::weekly().build(),
        "monthly" => CohortBuilder::monthly().build(),
        _ => return Err(ApiError::BadRequest("Invalid period".to_string())),
    };

    let analyzer = CohortAnalyzer::new();
    let cohorts = analyzer.create_cohorts(&user_profiles, cohort_period);
    let analysis = analyzer.analyze_cohorts(&cohorts, &events, payload.periods_count);

    // Construir resposta
    let cohorts_data: Vec<CohortData> = analysis
        .cohorts
        .iter()
        .map(|c| CohortData {
            cohort_id: c.cohort_id.clone(),
            size: c.size,
            retention_rates: c.retention_by_period.iter().map(|r| r * 100.0).collect(),
        })
        .collect();

    let response = CohortAnalysisResponse {
        cohorts: cohorts_data,
        retention_matrix: analysis
            .retention_rates
            .iter()
            .map(|row| row.iter().map(|r| r * 100.0).collect())
            .collect(),
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/analytics/retention
/// Obter taxa de retenção
pub async fn get_retention(
    State(state): State<AppState>,
    Query(params): Query<TimeRangeParams>,
) -> ApiResult<Json<ApiResponse<f64>>> {
    // Implementação simplificada
    // TODO: Calcular retenção real baseado em cohorts

    Ok(Json(ApiResponse::ok(75.5)))
}

/// GET /api/v1/analytics/conversion
/// Obter taxa de conversão
pub async fn get_conversion_rate(
    State(state): State<AppState>,
    Query(params): Query<TimeRangeParams>,
) -> ApiResult<Json<ApiResponse<f64>>> {
    let filter = EventFilter {
        start_time: params.start,
        end_time: params.end,
        ..Default::default()
    };

    let events = state.event_store.query(filter).await?;

    // Contar eventos de purchase e page views
    let purchases = events
        .iter()
        .filter(|e| matches!(e.event_type, crate::models::EventType::Purchase { .. }))
        .count();

    let page_views = events
        .iter()
        .filter(|e| matches!(e.event_type, crate::models::EventType::PageView { .. }))
        .count();

    let conversion_rate = if page_views > 0 {
        (purchases as f64 / page_views as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(ApiResponse::ok(conversion_rate)))
}

/// GET /api/v1/analytics/overview
/// Overview geral de analytics
pub async fn get_overview(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<OverviewResponse>>> {
    let stats = state.event_store.get_stats().await?;

    // Buscar eventos recentes para calcular métricas
    let events = state.event_store.query(EventFilter::default()).await?;

    // Top pages
    let mut page_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for event in &events {
        if let crate::models::EventType::PageView { url, .. } = &event.event_type {
            *page_counts.entry(url.clone()).or_insert(0) += 1;
        }
    }

    let mut top_pages: Vec<(String, usize)> = page_counts.into_iter().collect();
    top_pages.sort_by(|a, b| b.1.cmp(&a.1));
    top_pages.truncate(10);

    // Taxa de conversão
    let purchases = events
        .iter()
        .filter(|e| matches!(e.event_type, crate::models::EventType::Purchase { .. }))
        .count();
    let total = events.len();
    let conversion_rate = if total > 0 {
        (purchases as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let response = OverviewResponse {
        total_events: stats.total_events,
        total_users: stats.total_users,
        total_sessions: stats.total_sessions,
        events_by_type: stats.events_by_type,
        top_pages,
        conversion_rate,
        avg_session_duration: 120.0, // TODO: calcular de verdade
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/stats
/// Estatísticas gerais
pub async fn get_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<crate::storage::StorageStats>>> {
    let stats = state.event_store.get_stats().await?;
    Ok(Json(ApiResponse::ok(stats)))
}
