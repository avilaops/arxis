//! Rate limiting middleware

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tower::{Layer, Service};

/// Rate limit configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: usize,
    /// Time window
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
        }
    }
}

/// Rate limiter state
struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    fn new(config: RateLimitConfig) -> Self {
        Self {
            requests: HashMap::new(),
            config,
        }
    }

    fn check_rate_limit(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let window_start = now - self.config.window;

        // Clean old entries
        let requests = self.requests.entry(key.to_string()).or_insert_with(Vec::new);
        requests.retain(|&time| time > window_start);

        // Check limit
        if requests.len() >= self.config.max_requests {
            return false;
        }

        // Add new request
        requests.push(now);
        true
    }
}

/// Rate limiting layer
#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<Mutex<RateLimiter>>,
}

impl RateLimitLayer {
    pub fn new() -> Self {
        Self::with_config(RateLimitConfig::default())
    }

    pub fn with_config(config: RateLimitConfig) -> Self {
        Self {
            limiter: Arc::new(Mutex::new(RateLimiter::new(config))),
        }
    }
}

impl Default for RateLimitLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware {
            inner,
            limiter: self.limiter.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    limiter: Arc<Mutex<RateLimiter>>,
}

impl<S> Service<Request> for RateLimitMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
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
        let limiter = self.limiter.clone();

        // Extract client ID before moving req
        let client_id = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let future = self.inner.call(req);

        Box::pin(async move {

            // Check rate limit
            let mut limiter = limiter.lock().await;
            if !limiter.check_rate_limit(&client_id) {
                return Ok((
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded. Please try again later.",
                )
                    .into_response());
            }
            drop(limiter);

            future.await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let config = RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(60),
        };
        let mut limiter = RateLimiter::new(config);

        // Should allow first 5 requests
        for _ in 0..5 {
            assert!(limiter.check_rate_limit("test_user"));
        }

        // 6th request should be denied
        assert!(!limiter.check_rate_limit("test_user"));
    }
}
