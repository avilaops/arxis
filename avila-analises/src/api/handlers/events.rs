use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    api::{error::ApiError, response::ApiResponse, state::AppState, ApiResult},
    models::BehaviorEvent,
    storage::{EventFilter, QueryOptions},
};

/// Payload para ingestão de evento único
#[derive(Debug, Deserialize)]
pub struct IngestEventRequest {
    pub event: BehaviorEvent,
}

/// Payload para ingestão em batch
#[derive(Debug, Deserialize)]
pub struct IngestBatchRequest {
    pub events: Vec<BehaviorEvent>,
}

/// Payload para query de eventos
#[derive(Debug, Deserialize)]
pub struct QueryEventsRequest {
    pub user_ids: Option<Vec<String>>,
    pub session_ids: Option<Vec<String>>,
    pub event_types: Option<Vec<String>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub countries: Option<Vec<String>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Query parameters para paginação
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Resposta de evento único
#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub event_id: String,
    pub message: String,
}

/// Resposta de batch
#[derive(Debug, Serialize)]
pub struct BatchResponse {
    pub successful: usize,
    pub failed: usize,
    pub message: String,
}

/// POST /api/v1/events
/// Ingerir um único evento
pub async fn ingest_event(
    State(state): State<AppState>,
    Json(payload): Json<IngestEventRequest>,
) -> ApiResult<Json<ApiResponse<EventResponse>>> {
    // Validar evento
    if payload.event.user_id.is_empty() {
        return Err(ApiError::BadRequest("user_id is required".to_string()));
    }

    if payload.event.session_id.is_empty() {
        return Err(ApiError::BadRequest("session_id is required".to_string()));
    }

    // Armazenar evento
    state.event_store.store(payload.event.clone()).await?;

    let response = EventResponse {
        event_id: payload.event.event_id.clone(),
        message: "Event ingested successfully".to_string(),
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/events/batch
/// Ingerir múltiplos eventos
pub async fn ingest_batch(
    State(state): State<AppState>,
    Json(payload): Json<IngestBatchRequest>,
) -> ApiResult<Json<ApiResponse<BatchResponse>>> {
    let total = payload.events.len();

    // Validar tamanho do batch
    if total > state.config.max_batch_size {
        return Err(ApiError::BadRequest(format!(
            "Batch size {} exceeds maximum of {}",
            total, state.config.max_batch_size
        )));
    }

    if total == 0 {
        return Err(ApiError::BadRequest("Empty batch".to_string()));
    }

    // Armazenar eventos
    match state.event_store.store_batch(payload.events).await {
        Ok(_) => {
            let response = BatchResponse {
                successful: total,
                failed: 0,
                message: format!("Successfully ingested {} events", total),
            };
            Ok(Json(ApiResponse::ok(response)))
        }
        Err(e) => Err(ApiError::StorageError(e.to_string())),
    }
}

/// POST /api/v1/events/query
/// Buscar eventos com filtros
pub async fn query_events(
    State(state): State<AppState>,
    Json(payload): Json<QueryEventsRequest>,
) -> ApiResult<Json<ApiResponse<Vec<BehaviorEvent>>>> {
    // Construir filtro
    let filter = EventFilter {
        user_ids: payload.user_ids,
        session_ids: payload.session_ids,
        event_types: payload.event_types,
        start_time: payload.start_time,
        end_time: payload.end_time,
        metadata_filters: None,
        device_types: None,
        countries: payload.countries,
    };

    // Validar limit
    let limit = payload.limit.unwrap_or(state.config.default_page_size);
    if limit > state.config.max_query_limit {
        return Err(ApiError::BadRequest(format!(
            "Limit {} exceeds maximum of {}",
            limit, state.config.max_query_limit
        )));
    }

    // Buscar eventos
    let events = state.event_store.query(filter).await?;

    Ok(Json(ApiResponse::ok(events)))
}

/// GET /api/v1/events/user/:user_id
/// Buscar eventos de um usuário
pub async fn get_user_events(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> ApiResult<Json<ApiResponse<Vec<BehaviorEvent>>>> {
    let options = QueryOptions {
        limit: Some(params.limit.unwrap_or(state.config.default_page_size)),
        offset: params.offset,
        order_by: crate::storage::OrderBy::TimestampDesc,
    };

    let events = state.event_store.get_by_user(&user_id, options).await?;

    Ok(Json(ApiResponse::ok(events)))
}

/// GET /api/v1/events/session/:session_id
/// Buscar eventos de uma sessão
pub async fn get_session_events(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> ApiResult<Json<ApiResponse<Vec<BehaviorEvent>>>> {
    let events = state.event_store.get_by_session(&session_id).await?;

    if events.is_empty() {
        return Err(ApiError::NotFound(format!(
            "No events found for session {}",
            session_id
        )));
    }

    Ok(Json(ApiResponse::ok(events)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_payload_deserialization() {
        let json = r#"{
            "user_ids": ["user1", "user2"],
            "limit": 100
        }"#;

        let payload: QueryEventsRequest = serde_json::from_str(json).unwrap();
        assert_eq!(payload.user_ids.unwrap().len(), 2);
        assert_eq!(payload.limit, Some(100));
    }
}
