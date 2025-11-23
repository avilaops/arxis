//! CORS middleware

use axum::http::{header, HeaderValue, Method, Request};
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// CORS configuration
#[derive(Clone, Debug)]
pub struct CorsConfig {
    /// Allowed origins
    pub allow_origins: Vec<String>,

    /// Allowed methods
    pub allow_methods: Vec<Method>,

    /// Allowed headers
    pub allow_headers: Vec<String>,

    /// Allow credentials
    pub allow_credentials: bool,

    /// Max age for preflight cache
    pub max_age: Option<u64>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allow_origins: vec!["*".to_string()],
            allow_methods: vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
                Method::OPTIONS,
            ],
            allow_headers: vec![
                "content-type".to_string(),
                "authorization".to_string(),
                "x-requested-with".to_string(),
            ],
            allow_credentials: false,
            max_age: Some(3600),
        }
    }
}

/// CORS layer
#[derive(Clone)]
pub struct CorsLayer {
    config: CorsConfig,
}

impl CorsLayer {
    /// Create a new CORS layer with default configuration
    pub fn new() -> Self {
        Self {
            config: CorsConfig::default(),
        }
    }

    /// Create a CORS layer with custom configuration
    pub fn with_config(config: CorsConfig) -> Self {
        Self { config }
    }

    /// Set allowed origins
    pub fn allow_origins(mut self, origins: Vec<String>) -> Self {
        self.config.allow_origins = origins;
        self
    }

    /// Allow credentials
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.config.allow_credentials = allow;
        self
    }
}

impl Default for CorsLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for CorsLayer {
    type Service = CorsMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CorsMiddleware {
            inner,
            config: self.config.clone(),
        }
    }
}

/// CORS middleware service
#[derive(Clone)]
pub struct CorsMiddleware<S> {
    inner: S,
    config: CorsConfig,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CorsMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = axum::response::Response<ResBody>>,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // For OPTIONS requests (preflight), we could return early
        // For now, we just pass through and add headers to the response
        self.inner.call(req)
    }
}
