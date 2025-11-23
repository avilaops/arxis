//! JWT authentication

use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

/// JWT authentication configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Secret key for JWT validation
    pub secret: String,

    /// Algorithm to use (HS256, RS256, etc.)
    #[serde(default = "default_algorithm")]
    pub algorithm: String,

    /// Token expiration time in seconds
    #[serde(default = "default_expiration")]
    pub expiration_seconds: u64,

    /// Issuer validation
    pub issuer: Option<String>,

    /// Audience validation
    pub audience: Option<String>,
}

fn default_algorithm() -> String {
    "HS256".to_string()
}

fn default_expiration() -> u64 {
    3600 // 1 hour
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: String::new(),
            algorithm: default_algorithm(),
            expiration_seconds: default_expiration(),
            issuer: None,
            audience: None,
        }
    }
}

/// JWT authentication handler
#[derive(Clone)]
pub struct JwtAuth {
    config: JwtConfig,
}

impl JwtAuth {
    /// Create a new JWT authenticator
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// Validate JWT token from request
    pub fn validate(&self, req: &Request) -> bool {
        // Extract Authorization header
        let auth_header = match req.headers().get(AUTHORIZATION) {
            Some(header) => match header.to_str() {
                Ok(s) => s,
                Err(_) => return false,
            },
            None => return false,
        };

        // Check Bearer token format
        if !auth_header.starts_with("Bearer ") {
            return false;
        }

        let token = &auth_header[7..];

        // TODO: Implement proper JWT validation with jsonwebtoken crate
        // For now, just check that token exists and is not empty
        !token.is_empty()
    }

    /// Create a JWT token (for testing/development)
    pub fn create_token(&self, subject: &str) -> Result<String, String> {
        // TODO: Implement JWT creation with jsonwebtoken crate
        Ok(format!("jwt_token_for_{}", subject))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[test]
    fn test_jwt_validation_no_header() {
        let config = JwtConfig::default();
        let jwt_auth = JwtAuth::new(config);
        let req = Request::builder().body(()).unwrap();

        assert!(!jwt_auth.validate(&req));
    }

    #[test]
    fn test_jwt_validation_with_bearer() {
        let config = JwtConfig::default();
        let jwt_auth = JwtAuth::new(config);
        let req = Request::builder()
            .header("Authorization", "Bearer some_token")
            .body(())
            .unwrap();

        assert!(jwt_auth.validate(&req));
    }
}
