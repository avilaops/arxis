//! Middleware system for HTTP server
//!
//! Middlewares allow you to process requests before they reach handlers
//! and modify responses before they are sent to clients.

use crate::server::{Request, Response};
use crate::error::Result;
use async_trait::async_trait;
use http::{HeaderName, HeaderValue, StatusCode};
use std::sync::Arc;

/// Middleware trait for processing HTTP requests and responses
#[async_trait]
pub trait Middleware: Send + Sync {
    /// Process the request and response
    ///
    /// The middleware can:
    /// - Inspect or modify the request
    /// - Short-circuit by returning early
    /// - Call `next.run(request).await` to continue the chain
    async fn handle(&self, request: Request, next: Next) -> Result<Response>;
}

/// Represents the next middleware/handler in the chain
pub struct Next {
    middleware: Vec<Arc<dyn Middleware>>,
    index: usize,
    handler: Option<Arc<dyn Handler>>,
}

impl Next {
    /// Create a new middleware chain
    pub fn new(middleware: Vec<Arc<dyn Middleware>>, handler: Arc<dyn Handler>) -> Self {
        Self {
            middleware,
            index: 0,
            handler: Some(handler),
        }
    }

    /// Run the next middleware or handler
    pub async fn run(mut self, request: Request) -> Result<Response> {
        if self.index < self.middleware.len() {
            let middleware = self.middleware[self.index].clone();
            self.index += 1;
            middleware.handle(request, self).await
        } else if let Some(handler) = &self.handler {
            handler.handle(request).await
        } else {
            Ok(Response::text("Not Found").with_status(StatusCode::NOT_FOUND))
        }
    }
}

/// Handler trait for final request processing
#[async_trait]
pub trait Handler: Send + Sync {
    /// Handle the request
    async fn handle(&self, request: Request) -> Result<Response>;
}

/// Logger middleware - logs all requests
pub struct Logger;

impl Logger {
    /// Create a new logger middleware
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Middleware for Logger {
    async fn handle(&self, request: Request, next: Next) -> Result<Response> {
        let method = request.method.clone();
        let path = request.path.clone();
        let start = std::time::Instant::now();

        println!("[REQUEST] {} {}", method, path);

        let response = next.run(request).await?;

        let duration = start.elapsed();
        println!(
            "[RESPONSE] {} {} - {} ({:?})",
            method, path, response.status(), duration
        );

        Ok(response)
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// CORS middleware - adds CORS headers
pub struct Cors {
    allow_origin: String,
    allow_methods: Vec<String>,
    allow_headers: Vec<String>,
    max_age: u32,
}

impl Cors {
    /// Create a new CORS middleware with permissive settings
    pub fn permissive() -> Self {
        Self {
            allow_origin: "*".to_string(),
            allow_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            allow_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
            ],
            max_age: 86400,
        }
    }

    /// Create a new CORS middleware with custom origin
    pub fn new(allow_origin: impl Into<String>) -> Self {
        Self {
            allow_origin: allow_origin.into(),
            allow_methods: vec!["GET".to_string(), "POST".to_string()],
            allow_headers: vec!["Content-Type".to_string()],
            max_age: 3600,
        }
    }

    /// Set allowed methods
    pub fn allow_methods(mut self, methods: Vec<String>) -> Self {
        self.allow_methods = methods;
        self
    }

    /// Set allowed headers
    pub fn allow_headers(mut self, headers: Vec<String>) -> Self {
        self.allow_headers = headers;
        self
    }

    /// Set max age
    pub fn max_age(mut self, seconds: u32) -> Self {
        self.max_age = seconds;
        self
    }
}

#[async_trait]
impl Middleware for Cors {
    async fn handle(&self, request: Request, next: Next) -> Result<Response> {
        // Handle preflight OPTIONS request
        if request.method.as_str() == "OPTIONS" {
            let response = Response::text("")
                .with_header(
                    HeaderName::from_static("access-control-allow-origin"),
                    HeaderValue::from_str(&self.allow_origin).unwrap(),
                )
                .with_header(
                    HeaderName::from_static("access-control-allow-methods"),
                    HeaderValue::from_str(&self.allow_methods.join(", ")).unwrap(),
                )
                .with_header(
                    HeaderName::from_static("access-control-allow-headers"),
                    HeaderValue::from_str(&self.allow_headers.join(", ")).unwrap(),
                )
                .with_header(
                    HeaderName::from_static("access-control-max-age"),
                    HeaderValue::from_str(&self.max_age.to_string()).unwrap(),
                )
                .with_status(StatusCode::NO_CONTENT);
            return Ok(response);
        }

        // Add CORS headers to response
        let response = next.run(request).await?;
        let response = response.with_header(
            HeaderName::from_static("access-control-allow-origin"),
            HeaderValue::from_str(&self.allow_origin).unwrap(),
        );

        Ok(response)
    }
}

/// Rate limiting middleware
#[allow(dead_code)]
pub struct RateLimit {
    max_requests: usize,
    window_secs: u64,
    // In a real implementation, this would use a proper storage backend
}

impl RateLimit {
    /// Create a new rate limiter
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            max_requests,
            window_secs,
        }
    }
}

#[async_trait]
impl Middleware for RateLimit {
    async fn handle(&self, request: Request, next: Next) -> Result<Response> {
        // TODO: Implement actual rate limiting with storage
        // For now, just pass through
        next.run(request).await
    }
}

/// Authentication middleware
pub struct Auth {
    token: String,
}

impl Auth {
    /// Create a new auth middleware with bearer token
    pub fn bearer(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl Middleware for Auth {
    async fn handle(&self, request: Request, next: Next) -> Result<Response> {
        // Check for Authorization header
        if let Some(auth_header) = request.headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    if token == self.token {
                        return next.run(request).await;
                    }
                }
            }
        }

        Ok(Response::text("Unauthorized")
            .with_status(StatusCode::UNAUTHORIZED)
            .with_header(
                HeaderName::from_static("www-authenticate"),
                HeaderValue::from_static("Bearer"),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Method;

    struct TestHandler;

    #[async_trait]
    impl Handler for TestHandler {
        async fn handle(&self, _request: Request) -> Result<Response> {
            Ok(Response::text("Hello"))
        }
    }

    #[tokio::test]
    async fn test_logger_middleware() {
        let logger = Logger::new();
        let handler = Arc::new(TestHandler);
        let request = Request {
            method: Method::GET,
            path: "/test".to_string(),
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
        };

        let next = Next::new(vec![], handler);
        let response = logger.handle(request, next).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_cors_middleware() {
        let cors = Cors::permissive();
        let handler = Arc::new(TestHandler);
        let request = Request {
            method: Method::GET,
            path: "/test".to_string(),
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
        };

        let next = Next::new(vec![], handler);
        let response = cors.handle(request, next).await.unwrap();

        assert!(response.headers().contains_key("access-control-allow-origin"));
    }

    #[tokio::test]
    async fn test_cors_preflight() {
        let cors = Cors::permissive();
        let handler = Arc::new(TestHandler);
        let request = Request {
            method: Method::OPTIONS,
            path: "/test".to_string(),
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
        };

        let next = Next::new(vec![], handler);
        let response = cors.handle(request, next).await.unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert!(response.headers().contains_key("access-control-allow-methods"));
    }

    #[tokio::test]
    async fn test_auth_middleware_success() {
        let auth = Auth::bearer("secret-token");
        let handler = Arc::new(TestHandler);

        let mut headers = http::HeaderMap::new();
        headers.insert(
            "authorization",
            http::HeaderValue::from_static("Bearer secret-token"),
        );

        let request = Request {
            method: Method::GET,
            path: "/protected".to_string(),
            headers,
            body: bytes::Bytes::new(),
        };

        let next = Next::new(vec![], handler);
        let response = auth.handle(request, next).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_middleware_failure() {
        let auth = Auth::bearer("secret-token");
        let handler = Arc::new(TestHandler);
        let request = Request {
            method: Method::GET,
            path: "/protected".to_string(),
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
        };

        let next = Next::new(vec![], handler);
        let response = auth.handle(request, next).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
