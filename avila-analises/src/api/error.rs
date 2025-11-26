use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipo de resultado padrão para API
pub type ApiResult<T> = Result<T, ApiError>;

/// Erros da API
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    TooManyRequests(String),
    InternalServerError(String),
    ServiceUnavailable(String),
    StorageError(String),
    ValidationError(Vec<String>),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::TooManyRequests(msg) => write!(f, "Too Many Requests: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::ServiceUnavailable(msg) => write!(f, "Service Unavailable: {}", msg),
            ApiError::StorageError(msg) => write!(f, "Storage Error: {}", msg),
            ApiError::ValidationError(errors) => {
                write!(f, "Validation Error: {}", errors.join(", "))
            }
        }
    }
}

impl std::error::Error for ApiError {}

/// Resposta de erro padronizada
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
    pub timestamp: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message, details) = match self {
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST",
                msg,
                None,
            ),
            ApiError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                msg,
                None,
            ),
            ApiError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                msg,
                None,
            ),
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                msg,
                None,
            ),
            ApiError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "CONFLICT",
                msg,
                None,
            ),
            ApiError::TooManyRequests(msg) => (
                StatusCode::TOO_MANY_REQUESTS,
                "TOO_MANY_REQUESTS",
                msg,
                None,
            ),
            ApiError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                msg,
                None,
            ),
            ApiError::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                msg,
                None,
            ),
            ApiError::StorageError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "STORAGE_ERROR",
                msg,
                None,
            ),
            ApiError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                "Request validation failed".to_string(),
                Some(errors),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
            details,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });

        (status, body).into_response()
    }
}

impl From<crate::storage::StorageError> for ApiError {
    fn from(err: crate::storage::StorageError) -> Self {
        ApiError::StorageError(err.to_string())
    }
}
