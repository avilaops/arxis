//! Error types for AVL Auth

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("User already exists: {0}")]
    UserAlreadyExists(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Session expired")]
    SessionExpired,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("MFA required")]
    MfaRequired,

    #[error("Invalid MFA code")]
    InvalidMfaCode,

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Suspicious activity detected: {0}")]
    SuspiciousActivity(String),

    #[error("Account locked: {0}")]
    AccountLocked(String),

    #[error("Invalid OAuth2 state")]
    InvalidOAuth2State,

    #[error("OAuth2 provider error: {0}")]
    OAuth2Error(String),

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("API key expired")]
    ApiKeyExpired,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl AuthError {
    pub fn is_retriable(&self) -> bool {
        matches!(
            self,
            AuthError::DatabaseError(_) | AuthError::Internal(_)
        )
    }

    pub fn status_code(&self) -> u16 {
        match self {
            AuthError::InvalidCredentials => 401,
            AuthError::UserNotFound(_) => 404,
            AuthError::UserAlreadyExists(_) => 409,
            AuthError::InvalidToken(_) => 401,
            AuthError::TokenExpired => 401,
            AuthError::SessionNotFound => 404,
            AuthError::SessionExpired => 401,
            AuthError::PermissionDenied(_) => 403,
            AuthError::MfaRequired => 401,
            AuthError::InvalidMfaCode => 401,
            AuthError::InvalidPassword(_) => 400,
            AuthError::RateLimitExceeded => 429,
            AuthError::SuspiciousActivity(_) => 403,
            AuthError::AccountLocked(_) => 423,
            AuthError::InvalidOAuth2State => 400,
            AuthError::OAuth2Error(_) => 502,
            AuthError::InvalidApiKey => 401,
            AuthError::ApiKeyExpired => 401,
            AuthError::DatabaseError(_) => 500,
            AuthError::CryptoError(_) => 500,
            AuthError::ConfigError(_) => 500,
            AuthError::Internal(_) => 500,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken(err.to_string()),
        }
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AuthError::CryptoError(err.to_string())
    }
}
