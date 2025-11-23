//! Request timeout middleware

use axum::{extract::Request, http::StatusCode, response::{IntoResponse, Response}};
use std::time::Duration;
use tower::{Layer, Service};

/// Timeout layer
#[derive(Clone)]
pub struct TimeoutLayer {
    duration: Duration,
}

impl TimeoutLayer {
    /// Create a new timeout layer
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = TimeoutMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutMiddleware {
            inner,
            duration: self.duration,
        }
    }
}

/// Timeout middleware service
#[derive(Clone)]
pub struct TimeoutMiddleware<S> {
    inner: S,
    duration: Duration,
}

impl<S> Service<Request> for TimeoutMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
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
        let future = self.inner.call(req);
        let duration = self.duration;

        Box::pin(async move {
            match tokio::time::timeout(duration, future).await {
                Ok(result) => result,
                Err(_) => Ok((
                    StatusCode::GATEWAY_TIMEOUT,
                    "Request timeout",
                ).into_response()),
            }
        })
    }
}
