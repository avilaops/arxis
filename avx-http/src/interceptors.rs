//! Request and response interceptors
//!
//! Interceptors allow you to modify requests before they are sent
//! and inspect or modify responses after they are received.

use crate::error::Result;
use bytes::Bytes;
use http::{HeaderMap, Method};
use std::sync::Arc;

/// Type alias for request interceptor function
pub type RequestInterceptor = Arc<dyn Fn(&mut RequestData) + Send + Sync>;

/// Type alias for response interceptor function
pub type ResponseInterceptor = Arc<dyn Fn(&ResponseData) + Send + Sync>;

/// Data available to request interceptors
#[derive(Debug)]
pub struct RequestData {
    /// HTTP method
    pub method: Method,
    /// Request URL
    pub url: String,
    /// Request headers
    pub headers: HeaderMap,
    /// Request body
    pub body: Option<Bytes>,
}

impl RequestData {
    /// Create new request data
    pub fn new(method: Method, url: String, headers: HeaderMap, body: Option<Bytes>) -> Self {
        Self {
            method,
            url,
            headers,
            body,
        }
    }

    /// Add or update a header
    pub fn set_header(&mut self, name: &str, value: &str) -> Result<()> {
        use http::header::{HeaderName, HeaderValue};
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| crate::error::Error::InvalidHeader {
                name: name.to_string(),
                value: value.to_string(),
            })?;
        let value = HeaderValue::from_str(value)
            .map_err(|_| crate::error::Error::InvalidHeader {
                name: name.to_string(),
                value: value.to_string(),
            })?;
        self.headers.insert(name, value);
        Ok(())
    }

    /// Remove a header
    pub fn remove_header(&mut self, name: &str) {
        if let Ok(name) = http::HeaderName::from_bytes(name.as_bytes()) {
            self.headers.remove(&name);
        }
    }

    /// Get header value
    pub fn get_header(&self, name: &str) -> Option<&str> {
        let name = http::HeaderName::from_bytes(name.as_bytes()).ok()?;
        self.headers.get(&name)?.to_str().ok()
    }
}

/// Data available to response interceptors
#[derive(Debug)]
pub struct ResponseData {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HeaderMap,
    /// Response body size in bytes
    pub body_size: usize,
    /// Request duration in milliseconds
    pub duration_ms: u64,
}

impl ResponseData {
    /// Create new response data
    pub fn new(status: u16, headers: HeaderMap, body_size: usize, duration_ms: u64) -> Self {
        Self {
            status,
            headers,
            body_size,
            duration_ms,
        }
    }

    /// Get header value
    pub fn get_header(&self, name: &str) -> Option<&str> {
        let name = http::HeaderName::from_bytes(name.as_bytes()).ok()?;
        self.headers.get(&name)?.to_str().ok()
    }

    /// Check if response is successful (2xx)
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Check if response is redirect (3xx)
    pub fn is_redirect(&self) -> bool {
        self.status >= 300 && self.status < 400
    }

    /// Check if response is client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    /// Check if response is server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }
}

/// Collection of interceptors
#[derive(Clone, Default)]
pub struct Interceptors {
    /// Request interceptors
    pub(crate) request: Vec<RequestInterceptor>,
    /// Response interceptors
    pub(crate) response: Vec<ResponseInterceptor>,
}

impl Interceptors {
    /// Create new empty interceptor collection
    pub fn new() -> Self {
        Self {
            request: Vec::new(),
            response: Vec::new(),
        }
    }

    /// Add a request interceptor
    pub fn add_request<F>(&mut self, interceptor: F)
    where
        F: Fn(&mut RequestData) + Send + Sync + 'static,
    {
        self.request.push(Arc::new(interceptor));
    }

    /// Add a response interceptor
    pub fn add_response<F>(&mut self, interceptor: F)
    where
        F: Fn(&ResponseData) + Send + Sync + 'static,
    {
        self.response.push(Arc::new(interceptor));
    }

    /// Apply all request interceptors
    pub(crate) fn apply_request(&self, data: &mut RequestData) {
        for interceptor in &self.request {
            interceptor(data);
        }
    }

    /// Apply all response interceptors
    pub(crate) fn apply_response(&self, data: &ResponseData) {
        for interceptor in &self.response {
            interceptor(data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_data_set_header() {
        let mut data = RequestData::new(
            Method::GET,
            "https://example.com".to_string(),
            HeaderMap::new(),
            None,
        );

        data.set_header("User-Agent", "avx-http/0.2.0").unwrap();
        assert_eq!(data.get_header("User-Agent"), Some("avx-http/0.2.0"));
    }

    #[test]
    fn test_request_data_remove_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::USER_AGENT,
            http::HeaderValue::from_static("test"),
        );

        let mut data = RequestData::new(
            Method::GET,
            "https://example.com".to_string(),
            headers,
            None,
        );

        assert!(data.get_header("User-Agent").is_some());
        data.remove_header("User-Agent");
        assert!(data.get_header("User-Agent").is_none());
    }

    #[test]
    fn test_response_data_status_checks() {
        let data = ResponseData::new(200, HeaderMap::new(), 0, 0);
        assert!(data.is_success());
        assert!(!data.is_client_error());

        let data = ResponseData::new(404, HeaderMap::new(), 0, 0);
        assert!(data.is_client_error());
        assert!(!data.is_success());

        let data = ResponseData::new(500, HeaderMap::new(), 0, 0);
        assert!(data.is_server_error());
        assert!(!data.is_success());
    }

    #[test]
    fn test_interceptors_add() {
        let mut interceptors = Interceptors::new();

        interceptors.add_request(|data| {
            let _ = data.set_header("X-Test", "value");
        });

        interceptors.add_response(|data| {
            println!("Response status: {}", data.status);
        });

        assert_eq!(interceptors.request.len(), 1);
        assert_eq!(interceptors.response.len(), 1);
    }

    #[test]
    fn test_interceptors_apply_request() {
        let mut interceptors = Interceptors::new();

        interceptors.add_request(|data| {
            let _ = data.set_header("X-Custom", "test");
        });

        let mut data = RequestData::new(
            Method::GET,
            "https://example.com".to_string(),
            HeaderMap::new(),
            None,
        );

        interceptors.apply_request(&mut data);
        assert_eq!(data.get_header("X-Custom"), Some("test"));
    }
}
