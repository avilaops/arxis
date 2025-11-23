//! Error types for the gateway

use std::fmt;

/// Result type alias for gateway operations
pub type Result<T> = std::result::Result<T, GatewayError>;

/// Error types that can occur in the gateway
#[derive(Debug)]
pub enum GatewayError {
    /// Configuration error
    Config(String),

    /// Routing error
    Routing(String),

    /// Authentication error
    Auth(String),

    /// Rate limit exceeded
    RateLimit(String),

    /// Upstream service error
    Upstream(String),

    /// Circuit breaker is open
    CircuitOpen(String),

    /// HTTP error
    Http(String),

    /// IO error
    Io(std::io::Error),

    /// Invalid configuration
    InvalidConfig(String),

    /// Service unavailable
    ServiceUnavailable(String),

    /// Timeout
    Timeout(String),

    /// Generic error
    Other(String),
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GatewayError::Config(msg) => write!(f, "Configuration error: {}", msg),
            GatewayError::Routing(msg) => write!(f, "Routing error: {}", msg),
            GatewayError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            GatewayError::RateLimit(msg) => write!(f, "Rate limit error: {}", msg),
            GatewayError::Upstream(msg) => write!(f, "Upstream error: {}", msg),
            GatewayError::CircuitOpen(msg) => write!(f, "Circuit breaker open: {}", msg),
            GatewayError::Http(msg) => write!(f, "HTTP error: {}", msg),
            GatewayError::Io(err) => write!(f, "IO error: {}", err),
            GatewayError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            GatewayError::ServiceUnavailable(msg) => {
                write!(f, "Service unavailable: {}", msg)
            }
            GatewayError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            GatewayError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for GatewayError {}

impl From<std::io::Error> for GatewayError {
    fn from(err: std::io::Error) -> Self {
        GatewayError::Io(err)
    }
}

impl From<anyhow::Error> for GatewayError {
    fn from(err: anyhow::Error) -> Self {
        GatewayError::Other(err.to_string())
    }
}

impl From<GatewayError> for axum::response::Response {
    fn from(err: GatewayError) -> Self {
        use axum::http::StatusCode;
        use axum::response::IntoResponse;

        let (status, message) = match err {
            GatewayError::Auth(_) => (StatusCode::UNAUTHORIZED, err.to_string()),
            GatewayError::RateLimit(_) => (StatusCode::TOO_MANY_REQUESTS, err.to_string()),
            GatewayError::CircuitOpen(_) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            GatewayError::ServiceUnavailable(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, err.to_string())
            }
            GatewayError::Timeout(_) => (StatusCode::GATEWAY_TIMEOUT, err.to_string()),
            GatewayError::Routing(_) => (StatusCode::NOT_FOUND, err.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        (status, message).into_response()
    }
}
