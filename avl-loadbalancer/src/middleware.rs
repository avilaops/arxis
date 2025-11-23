//! Middleware Pipeline Module
//!
//! Flexible request/response transformation pipeline.

use anyhow::Result;
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use std::sync::Arc;
use tracing::debug;

/// Middleware context with request metadata
#[derive(Clone)]
pub struct MiddlewareContext {
    pub request_id: uuid::Uuid,
    pub start_time: std::time::Instant,
    pub client_ip: Option<std::net::IpAddr>,
}

impl MiddlewareContext {
    pub fn new() -> Self {
        Self {
            request_id: uuid::Uuid::new_v4(),
            start_time: std::time::Instant::now(),
            client_ip: None,
        }
    }

    pub fn with_client_ip(mut self, ip: std::net::IpAddr) -> Self {
        self.client_ip = Some(ip);
        self
    }
}

/// Request middleware trait
#[async_trait]
pub trait RequestMiddleware: Send + Sync {
    /// Process request before proxying
    async fn process_request(
        &self,
        req: Request<Body>,
        ctx: &mut MiddlewareContext,
    ) -> Result<Request<Body>>;
}

/// Response middleware trait
#[async_trait]
pub trait ResponseMiddleware: Send + Sync {
    /// Process response before sending to client
    async fn process_response(
        &self,
        res: Response<Body>,
        ctx: &MiddlewareContext,
    ) -> Result<Response<Body>>;
}

/// Middleware pipeline for processing requests and responses
pub struct MiddlewarePipeline {
    request_middlewares: Vec<Arc<dyn RequestMiddleware>>,
    response_middlewares: Vec<Arc<dyn ResponseMiddleware>>,
}

impl MiddlewarePipeline {
    pub fn new() -> Self {
        Self {
            request_middlewares: Vec::new(),
            response_middlewares: Vec::new(),
        }
    }

    /// Add request middleware
    pub fn add_request_middleware<M: RequestMiddleware + 'static>(mut self, middleware: M) -> Self {
        self.request_middlewares.push(Arc::new(middleware));
        self
    }

    /// Add response middleware
    pub fn add_response_middleware<M: ResponseMiddleware + 'static>(mut self, middleware: M) -> Self {
        self.response_middlewares.push(Arc::new(middleware));
        self
    }

    /// Process request through all middlewares
    pub async fn process_request(
        &self,
        mut req: Request<Body>,
        ctx: &mut MiddlewareContext,
    ) -> Result<Request<Body>> {
        for middleware in &self.request_middlewares {
            req = middleware.process_request(req, ctx).await?;
        }
        Ok(req)
    }

    /// Process response through all middlewares
    pub async fn process_response(
        &self,
        mut res: Response<Body>,
        ctx: &MiddlewareContext,
    ) -> Result<Response<Body>> {
        for middleware in &self.response_middlewares {
            res = middleware.process_response(res, ctx).await?;
        }
        Ok(res)
    }
}

impl Default for MiddlewarePipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Request ID injection middleware
pub struct RequestIdMiddleware;

#[async_trait]
impl RequestMiddleware for RequestIdMiddleware {
    async fn process_request(
        &self,
        mut req: Request<Body>,
        ctx: &mut MiddlewareContext,
    ) -> Result<Request<Body>> {
        req.headers_mut().insert(
            "X-Request-ID",
            ctx.request_id.to_string().parse().unwrap(),
        );
        debug!("Request ID: {}", ctx.request_id);
        Ok(req)
    }
}

/// Response timing middleware
pub struct ResponseTimingMiddleware;

#[async_trait]
impl ResponseMiddleware for ResponseTimingMiddleware {
    async fn process_response(
        &self,
        mut res: Response<Body>,
        ctx: &MiddlewareContext,
    ) -> Result<Response<Body>> {
        let elapsed = ctx.start_time.elapsed();
        res.headers_mut().insert(
            "X-Response-Time",
            format!("{}ms", elapsed.as_millis()).parse().unwrap(),
        );
        debug!("Response time: {:?}", elapsed);
        Ok(res)
    }
}

/// CORS middleware
pub struct CorsMiddleware {
    allowed_origins: Vec<String>,
}

impl CorsMiddleware {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self { allowed_origins }
    }

    pub fn allow_all() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
        }
    }
}

#[async_trait]
impl ResponseMiddleware for CorsMiddleware {
    async fn process_response(
        &self,
        mut res: Response<Body>,
        _ctx: &MiddlewareContext,
    ) -> Result<Response<Body>> {
        let headers = res.headers_mut();

        if self.allowed_origins.contains(&"*".to_string()) {
            headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
        } else {
            headers.insert(
                "Access-Control-Allow-Origin",
                self.allowed_origins.join(", ").parse().unwrap(),
            );
        }

        headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
        headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());

        Ok(res)
    }
}

/// Security headers middleware
pub struct SecurityHeadersMiddleware;

#[async_trait]
impl ResponseMiddleware for SecurityHeadersMiddleware {
    async fn process_response(
        &self,
        mut res: Response<Body>,
        _ctx: &MiddlewareContext,
    ) -> Result<Response<Body>> {
        let headers = res.headers_mut();
        headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
        headers.insert("X-Frame-Options", "DENY".parse().unwrap());
        headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
        headers.insert(
            "Strict-Transport-Security",
            "max-age=31536000; includeSubDomains".parse().unwrap(),
        );
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_middleware_pipeline() {
        let pipeline = MiddlewarePipeline::new()
            .add_request_middleware(RequestIdMiddleware)
            .add_response_middleware(ResponseTimingMiddleware)
            .add_response_middleware(SecurityHeadersMiddleware);

        let mut ctx = MiddlewareContext::new();
        let req = Request::builder().body(Body::empty()).unwrap();

        let processed_req = pipeline.process_request(req, &mut ctx).await.unwrap();
        assert!(processed_req.headers().contains_key("X-Request-ID"));

        let res = Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap();

        let processed_res = pipeline.process_response(res, &ctx).await.unwrap();
        assert!(processed_res.headers().contains_key("X-Response-Time"));
        assert!(processed_res.headers().contains_key("X-Content-Type-Options"));
    }
}
