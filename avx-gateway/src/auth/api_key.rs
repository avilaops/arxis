//! API key authentication

use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use std::collections::HashSet;

/// API key authentication
#[derive(Clone)]
pub struct ApiKeyAuth {
    /// Valid API keys
    valid_keys: HashSet<String>,
}

impl ApiKeyAuth {
    /// Create a new API key authenticator
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            valid_keys: keys.into_iter().collect(),
        }
    }

    /// Add an API key
    pub fn add_key(&mut self, key: String) {
        self.valid_keys.insert(key);
    }

    /// Remove an API key
    pub fn remove_key(&mut self, key: &str) -> bool {
        self.valid_keys.remove(key)
    }

    /// Validate API key from request
    ///
    /// Supports multiple headers:
    /// - Authorization: Bearer <key>
    /// - Authorization: ApiKey <key>
    /// - X-API-Key: <key>
    pub fn validate(&self, req: &Request) -> bool {
        // Check Authorization header
        if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                // Support "Bearer <key>" format
                if let Some(key) = auth_str.strip_prefix("Bearer ") {
                    if self.valid_keys.contains(key) {
                        return true;
                    }
                }

                // Support "ApiKey <key>" format
                if let Some(key) = auth_str.strip_prefix("ApiKey ") {
                    if self.valid_keys.contains(key) {
                        return true;
                    }
                }
            }
        }

        // Check X-API-Key header
        if let Some(api_key_header) = req.headers().get("X-API-Key") {
            if let Ok(key) = api_key_header.to_str() {
                if self.valid_keys.contains(key) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[test]
    fn test_api_key_validation_bearer() {
        let auth = ApiKeyAuth::new(vec!["test_key_123".to_string()]);
        let req = Request::builder()
            .header("Authorization", "Bearer test_key_123")
            .body(())
            .unwrap();

        assert!(auth.validate(&req));
    }

    #[test]
    fn test_api_key_validation_x_api_key() {
        let auth = ApiKeyAuth::new(vec!["test_key_123".to_string()]);
        let req = Request::builder()
            .header("X-API-Key", "test_key_123")
            .body(())
            .unwrap();

        assert!(auth.validate(&req));
    }

    #[test]
    fn test_api_key_validation_invalid() {
        let auth = ApiKeyAuth::new(vec!["test_key_123".to_string()]);
        let req = Request::builder()
            .header("Authorization", "Bearer wrong_key")
            .body(())
            .unwrap();

        assert!(!auth.validate(&req));
    }
}
