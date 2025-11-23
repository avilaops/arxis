//! Sticky Sessions (Session Affinity) Module
//!
//! Ensures that requests from the same client are routed to the same backend server.

use crate::Backend;
use cookie::Cookie;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

const SESSION_COOKIE_NAME: &str = "AVL_SESSION_AFFINITY";
const SESSION_COOKIE_MAX_AGE: i64 = 86400; // 24 hours

/// Sticky session manager
#[derive(Clone)]
pub struct StickySessionManager {
    secret_key: Arc<Vec<u8>>,
}

impl StickySessionManager {
    /// Create a new sticky session manager with a secret key
    pub fn new(secret_key: Vec<u8>) -> Self {
        Self {
            secret_key: Arc::new(secret_key),
        }
    }

    /// Generate a random secret key
    pub fn generate_secret_key() -> Vec<u8> {
        use sha2::Digest;
        let random_data = uuid::Uuid::new_v4().to_string();
        Sha256::digest(random_data.as_bytes()).to_vec()
    }

    /// Create a session cookie for a backend
    pub fn create_session_cookie(&self, backend: &Backend) -> String {
        let backend_url = backend.url();
        let signature = self.sign(backend_url.as_bytes());
        let value = format!("{}:{}", backend_url, base64::encode(&signature));

        let cookie = Cookie::build((SESSION_COOKIE_NAME, value))
            .path("/")
            .max_age(cookie::time::Duration::seconds(SESSION_COOKIE_MAX_AGE))
            .http_only(true)
            .same_site(cookie::SameSite::Lax)
            .build();

        cookie.to_string()
    }

    /// Extract backend URL from session cookie
    pub fn extract_backend_from_cookie(&self, cookie_header: &str) -> Option<String> {
        // Parse cookies from header
        for cookie_str in cookie_header.split(';') {
            if let Ok(cookie) = Cookie::parse(cookie_str.trim()) {
                if cookie.name() == SESSION_COOKIE_NAME {
                    let value = cookie.value();
                    if let Some((backend_url, signature_b64)) = value.split_once(':') {
                        if let Ok(signature) = base64::decode(signature_b64) {
                            if self.verify(backend_url.as_bytes(), &signature) {
                                return Some(backend_url.to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Sign data with HMAC-SHA256
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.secret_key)
            .expect("HMAC can take key of any size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    /// Verify HMAC signature
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        let expected = self.sign(data);
        signature.len() == expected.len()
            && signature.iter().zip(expected.iter()).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticky_session_roundtrip() {
        let manager = StickySessionManager::new(StickySessionManager::generate_secret_key());
        let backend = Backend::new("http://server1:8000");

        let cookie = manager.create_session_cookie(&backend);
        assert!(cookie.contains(SESSION_COOKIE_NAME));
        assert!(cookie.contains("http://server1:8000"));

        // Extract backend from cookie
        let extracted = manager.extract_backend_from_cookie(&cookie);
        assert_eq!(extracted, Some("http://server1:8000".to_string()));
    }

    #[test]
    fn test_invalid_signature() {
        let manager = StickySessionManager::new(StickySessionManager::generate_secret_key());
        let fake_cookie = format!("{}=http://server1:8000:fakesignature", SESSION_COOKIE_NAME);

        let extracted = manager.extract_backend_from_cookie(&fake_cookie);
        assert!(extracted.is_none());
    }

    #[test]
    fn test_tampered_backend_url() {
        let manager = StickySessionManager::new(StickySessionManager::generate_secret_key());
        let backend = Backend::new("http://server1:8000");

        let cookie = manager.create_session_cookie(&backend);
        let tampered = cookie.replace("server1", "server2");

        let extracted = manager.extract_backend_from_cookie(&tampered);
        assert!(extracted.is_none());
    }
}
