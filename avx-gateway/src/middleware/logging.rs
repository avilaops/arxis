//! Request logging middleware

use axum::{extract::Request, response::Response};
use std::time::Instant;
use tower::{Layer, Service};
use tracing::{info, Span};
use uuid::Uuid;

/// Logging layer for requests
#[derive(Clone, Default)]
pub struct LoggingLayer;

impl LoggingLayer {
    /// Create a new logging layer
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

/// Logging middleware service
#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for LoggingMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let start = Instant::now();
        let trace_id = Uuid::new_v4();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let version = req.version();

        info!(
            trace_id = %trace_id,
            method = %method,
            uri = %uri,
            version = ?version,
            "Request started"
        );

        let future = self.inner.call(req);

        Box::pin(async move {
            let response = future.await?;
            let duration = start.elapsed();
            let status = response.status();

            info!(
                trace_id = %trace_id,
                method = %method,
                uri = %uri,
                status = %status,
                duration_ms = duration.as_millis(),
                "Request completed"
            );

            Ok(response)
        })
    }
}
