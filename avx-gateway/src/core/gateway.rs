//! avx-gateway - Enterprise API Gateway
//!
//! Features:
//! - Rate limiting (token bucket, sliding window)
//! - Circuit breaker pattern
//! - Request routing
//! - Authentication middleware
//!
//! Competing with: Kong, Nginx Plus, AWS API Gateway

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// Rate limiter using token bucket algorithm
pub struct TokenBucket {
    capacity: u64,
    tokens: AtomicU64,
    refill_rate: u64,      // tokens per second
    last_refill: AtomicU64, // timestamp in ms
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            tokens: AtomicU64::new(capacity),
            refill_rate,
            last_refill: AtomicU64::new(0),
        }
    }

    /// Try to consume tokens
    pub fn try_consume(&self, tokens: u64, now_ms: u64) -> bool {
        // Refill tokens based on elapsed time
        self.refill(now_ms);

        let current = self.tokens.load(Ordering::Relaxed);
        if current >= tokens {
            // Try to atomically subtract tokens
            self.tokens.fetch_sub(tokens, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    fn refill(&self, now_ms: u64) {
        let last = self.last_refill.load(Ordering::Relaxed);
        let elapsed_ms = now_ms.saturating_sub(last);

        if elapsed_ms > 0 {
            let tokens_to_add = (elapsed_ms * self.refill_rate) / 1000;

            if tokens_to_add > 0 {
                let current = self.tokens.load(Ordering::Relaxed);
                let new_tokens = (current + tokens_to_add).min(self.capacity);
                self.tokens.store(new_tokens, Ordering::Release);
                self.last_refill.store(now_ms, Ordering::Release);
            }
        }
    }

    pub fn available_tokens(&self) -> u64 {
        self.tokens.load(Ordering::Relaxed)
    }
}

/// Sliding window rate limiter
pub struct SlidingWindowLimiter {
    max_requests: u32,
    window_ms: u64,
    requests: Vec<u64>, // Timestamps
}

impl SlidingWindowLimiter {
    pub fn new(max_requests: u32, window_ms: u64) -> Self {
        Self {
            max_requests,
            window_ms,
            requests: Vec::new(),
        }
    }

    pub fn try_acquire(&mut self, now_ms: u64) -> bool {
        // Remove old requests outside the window
        let window_start = now_ms.saturating_sub(self.window_ms);
        self.requests.retain(|&ts| ts >= window_start);

        if self.requests.len() < self.max_requests as usize {
            self.requests.push(now_ms);
            true
        } else {
            false
        }
    }

    pub fn current_usage(&self) -> u32 {
        self.requests.len() as u32
    }
}

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,      // Normal operation
    Open,        // Failing, reject requests
    HalfOpen,    // Testing if service recovered
}

/// Circuit breaker pattern for fault tolerance
pub struct CircuitBreaker {
    state: CircuitState,
    failure_threshold: u32,
    success_threshold: u32,
    timeout_ms: u64,

    failure_count: AtomicU32,
    success_count: AtomicU32,
    last_failure_time: AtomicU64,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout_ms: u64) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_threshold,
            success_threshold,
            timeout_ms,
            failure_count: AtomicU32::new(0),
            success_count: AtomicU32::new(0),
            last_failure_time: AtomicU64::new(0),
        }
    }

    /// Check if request is allowed
    pub fn allow_request(&mut self, now_ms: u64) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout elapsed
                let last_fail = self.last_failure_time.load(Ordering::Relaxed);
                if now_ms.saturating_sub(last_fail) >= self.timeout_ms {
                    // Transition to half-open
                    self.state = CircuitState::HalfOpen;
                    self.success_count.store(0, Ordering::Release);
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Record successful request
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count.store(0, Ordering::Release);
            }
            CircuitState::HalfOpen => {
                let count = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count >= self.success_threshold {
                    // Transition back to closed
                    self.state = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::Release);
                }
            }
            CircuitState::Open => {}
        }
    }

    /// Record failed request
    pub fn record_failure(&mut self, now_ms: u64) {
        self.last_failure_time.store(now_ms, Ordering::Release);

        match self.state {
            CircuitState::Closed => {
                let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count >= self.failure_threshold {
                    // Transition to open
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                // Failed during testing, go back to open
                self.state = CircuitState::Open;
                self.success_count.store(0, Ordering::Release);
            }
            CircuitState::Open => {}
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }
}

/// Request router
pub struct Router {
    routes: Vec<Route>,
    default_backend: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub path_prefix: String,
    pub methods: Vec<HttpMethod>,
    pub backend: String,
    pub priority: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            default_backend: None,
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
        // Sort by priority (descending)
        self.routes.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn route(&self, path: &str, method: HttpMethod) -> Option<&str> {
        for route in &self.routes {
            if path.starts_with(&route.path_prefix) && route.methods.contains(&method) {
                return Some(&route.backend);
            }
        }

        self.default_backend.as_deref()
    }
}

/// Authentication token
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub user_id: String,
    pub scopes: Vec<String>,
    pub expires_at: u64, // timestamp
}

impl AuthToken {
    pub fn is_expired(&self, now_ms: u64) -> bool {
        now_ms >= self.expires_at
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
}

/// JWT token parser (simplified)
pub struct JwtValidator {
    secret: Vec<u8>,
}

impl JwtValidator {
    pub fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }

    /// Validate JWT token (simplified - would use proper crypto)
    pub fn validate(&self, token: &str) -> Result<AuthToken, AuthError> {
        // In production: parse JWT, verify signature, extract claims

        // Simplified mock
        Ok(AuthToken {
            user_id: "user123".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
            expires_at: u64::MAX,
        })
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    ExpiredToken,
    MissingToken,
}

/// Request context
pub struct RequestContext {
    pub request_id: String,
    pub path: String,
    pub method: HttpMethod,
    pub headers: BTreeMap<String, String>,
    pub auth_token: Option<AuthToken>,
}

impl RequestContext {
    pub fn new(request_id: String, path: String, method: HttpMethod) -> Self {
        Self {
            request_id,
            path,
            method,
            headers: BTreeMap::new(),
            auth_token: None,
        }
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket() {
        let bucket = TokenBucket::new(10, 1); // 10 tokens, 1/sec refill

        assert!(bucket.try_consume(5, 0));
        assert!(bucket.try_consume(5, 0));
        assert!(!bucket.try_consume(1, 0)); // Exhausted

        // After 5 seconds, should have 5 tokens
        assert!(bucket.try_consume(5, 5000));
    }

    #[test]
    fn test_circuit_breaker() {
        let mut cb = CircuitBreaker::new(3, 2, 5000);

        assert_eq!(cb.state(), CircuitState::Closed);

        // Record 3 failures
        cb.record_failure(0);
        cb.record_failure(100);
        cb.record_failure(200);

        assert_eq!(cb.state(), CircuitState::Open);
        assert!(!cb.allow_request(300)); // Should reject

        // After timeout, should allow test request
        assert!(cb.allow_request(6000));
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // Record 2 successes to close
        cb.record_success();
        cb.record_success();
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_router() {
        let mut router = Router::new();

        router.add_route(Route {
            path_prefix: "/api/users".to_string(),
            methods: vec![HttpMethod::GET, HttpMethod::POST],
            backend: "user-service".to_string(),
            priority: 10,
        });

        router.add_route(Route {
            path_prefix: "/api/orders".to_string(),
            methods: vec![HttpMethod::GET],
            backend: "order-service".to_string(),
            priority: 10,
        });

        assert_eq!(router.route("/api/users", HttpMethod::GET), Some("user-service"));
        assert_eq!(router.route("/api/orders", HttpMethod::GET), Some("order-service"));
        assert_eq!(router.route("/api/unknown", HttpMethod::GET), None);
    }
}
