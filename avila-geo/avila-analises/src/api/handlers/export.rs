use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    api::{error::ApiError, response::ApiResponse, state::AppState, ApiResult},
    export::{csv::CsvExporter, json::JsonExporter, DataExporter, ExportFormat},
    storage::EventFilter,
};

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub format: String,
    pub filters: Option<ExportFilters>,
}

#[derive(Debug, Deserialize)]
pub struct ExportFilters {
    pub user_ids: Option<Vec<String>>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub file_path: String,
    pub record_count: usize,
    pub file_size_bytes: u64,
    pub format: String,
}

/// POST /api/v1/export
/// Exportar dados
pub async fn export_data(
    State(state): State<AppState>,
    Json(payload): Json<ExportRequest>,
) -> ApiResult<Json<ApiResponse<ExportResponse>>> {
    // Construir filtro
    let filter = if let Some(filters) = payload.filters {
        EventFilter {
            user_ids: filters.user_ids,
            start_time: filters.start_time,
            end_time: filters.end_time,
            ..Default::default()
        }
    } else {
        EventFilter::default()
    };

    // Buscar eventos
    let events = state.event_store.query(filter).await?;

    if events.is_empty() {
        return Err(ApiError::NotFound("No events found to export".to_string()));
    }

    // Determinar exporter
    let output_path = PathBuf::from(format!(
        "./exports/export_{}.{}",
        chrono::Utc::now().timestamp(),
        payload.format
    ));

    // Criar diretório se não existir
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            ApiError::InternalServerError(format!("Failed to create export directory: {}", e))
        })?;
    }

    let result = match payload.format.as_str() {
        "csv" => {
            let exporter = CsvExporter::new();
            exporter.export(events, &output_path).await
        }
        "json" => {
            let exporter = JsonExporter::new().pretty();
            exporter.export(events, &output_path).await
        }
        "jsonl" | "jsonlines" => {
            let exporter = JsonExporter::new().jsonlines();
            exporter.export(events, &output_path).await
        }
        _ => {
            return Err(ApiError::BadRequest(format!(
                "Unsupported format: {}. Supported: csv, json, jsonlines",
                payload.format
            )));
        }
    };

    match result {
        Ok(export_result) => {
            let response = ExportResponse {
                file_path: export_result.file_path,
                record_count: export_result.record_count,
                file_size_bytes: export_result.file_size_bytes,
                format: format!("{:?}", export_result.format),
            };
            Ok(Json(ApiResponse::ok(response)))
        }
        Err(e) => Err(ApiError::InternalServerError(format!(
            "Export failed: {}",
            e
        ))),
    }
}

/// GET /api/v1/export/:filename
/// Download arquivo exportado
pub async fn download_export(
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let file_path = PathBuf::from(format!("./exports/{}", filename));

    if !file_path.exists() {
        return Err(ApiError::NotFound(format!("File {} not found", filename)));
    }

    let content = tokio::fs::read(&file_path).await.map_err(|e| {
        ApiError::InternalServerError(format!("Failed to read file: {}", e))
    })?;

    Ok((StatusCode::OK, content))
}
