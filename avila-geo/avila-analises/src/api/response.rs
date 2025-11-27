use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// Resposta de sucesso padronizada
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn ok_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data,
            message: Some(message),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Resposta paginada
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: usize, page: usize, page_size: usize) -> Self {
        let total_pages = (total + page_size - 1) / page_size;
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

/// Resposta de operação em batch
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse {
    pub successful: usize,
    pub failed: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

/// Resposta de health check
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub storage: StorageHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageHealth {
    pub connected: bool,
    pub latency_ms: Option<u64>,
}
