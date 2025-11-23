//! Error types for AVL Console

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ConsoleError>;

/// Main error type for AVL Console operations
#[derive(Debug, thiserror::Error)]
pub enum ConsoleError {
    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Authorization failed: {0}")]
    Authorization(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Rate limit: {0}")]
    RateLimit(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for ConsoleError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            ConsoleError::Authentication(_) => (
                StatusCode::UNAUTHORIZED,
                "authentication_error",
                self.to_string(),
            ),
            ConsoleError::Authorization(_) => (
                StatusCode::FORBIDDEN,
                "authorization_error",
                self.to_string(),
            ),
            ConsoleError::Database(_) | ConsoleError::Storage(_) => (
                StatusCode::BAD_GATEWAY,
                "service_error",
                self.to_string(),
            ),
            ConsoleError::InvalidInput(_) => (
                StatusCode::BAD_REQUEST,
                "invalid_input",
                self.to_string(),
            ),
            ConsoleError::NotFound(_) => (
                StatusCode::NOT_FOUND,
                "not_found",
                self.to_string(),
            ),
            ConsoleError::RateLimitExceeded | ConsoleError::RateLimit(_) => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limit_exceeded",
                self.to_string(),
            ),
            ConsoleError::Validation(_) => (
                StatusCode::BAD_REQUEST,
                "validation_error",
                self.to_string(),
            ),
            ConsoleError::WebSocket(_) => (
                StatusCode::BAD_REQUEST,
                "websocket_error",
                self.to_string(),
            ),
            ConsoleError::Template(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "template_error",
                "Failed to render page".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "An internal error occurred".to_string(),
            ),
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
            details: None,
        };

        (status, Json(error_response)).into_response()
    }
}

impl From<anyhow::Error> for ConsoleError {
    fn from(err: anyhow::Error) -> Self {
        ConsoleError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for ConsoleError {
    fn from(err: serde_json::Error) -> Self {
        ConsoleError::InvalidInput(err.to_string())
    }
}
