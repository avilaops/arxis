//! Rate limiting middleware

use axum::{extract::Request, http::StatusCode, response::{IntoResponse, Response}};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tower::{Layer, Service};

/// Token bucket rate limiter
#[derive(Clone)]
pub struct TokenBucket {
    /// Maximum number of tokens
    capacity: u32,

    /// Current number of tokens
    tokens: Arc<Mutex<f64>>,

    /// Token refill rate per second
    refill_rate: f64,

    /// Last refill time
    last_refill: Arc<Mutex<Instant>>,
}

impl TokenBucket {
    /// Create a new token bucket
    pub fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: Arc::new(Mutex::new(capacity as f64)),
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Try to consume a token
    pub async fn try_consume(&self) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;

        // Refill tokens based on time elapsed
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;
        *tokens = (*tokens + new_tokens).min(self.capacity as f64);
        *last_refill = now;

        // Try to consume a token
        if *tokens >= 1.0 {
            *tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Rate limit layer
#[derive(Clone)]
pub struct RateLimitLayer {
    bucket: TokenBucket,
}

impl RateLimitLayer {
    /// Create a new rate limit layer
    ///
    /// # Arguments
    /// * `requests_per_second` - Maximum requests per second
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            bucket: TokenBucket::new(requests_per_second * 2, requests_per_second as f64),
        }
    }

    /// Set burst size
    pub fn burst(self, burst_size: u32) -> Self {
        Self {
            bucket: TokenBucket::new(burst_size, self.bucket.refill_rate),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware {
            inner,
            bucket: self.bucket.clone(),
        }
    }
}

/// Rate limit middleware service
#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    bucket: TokenBucket,
}

impl<S> Service<Request> for RateLimitMiddleware<S>
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
        let bucket = self.bucket.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            if bucket.try_consume().await {
                inner.call(req).await
            } else {
                Ok((
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded",
                ).into_response())
            }
        })
    }
}
