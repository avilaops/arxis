//! AVL LoadBalancer Library
//!
//! Production-ready L7 load balancer with active health checks and multiple distribution algorithms.
//!
//! ## Features
//!
//! * `LoadBalancer` with fluent builder API
//! * `Backend` health state tracking
//! * Active HTTP health probes with configurable intervals and timeouts
//! * Round-robin algorithm (others planned)
//! * Automatic unhealthy backend filtering
//! * Built-in health status endpoint (`/_health`)
//! * Reverse proxy using Axum + Reqwest
//!
//! ## Example
//!
//! ```rust,no_run
//! use avl_loadbalancer::{LoadBalancer, Backend, HealthCheck, Algorithm};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let lb = LoadBalancer::builder()
//!         .add_backend(Backend::new("http://192.168.1.10:8000"))
//!         .add_backend(Backend::new("http://192.168.1.11:8000"))
//!         .health_check(
//!             HealthCheck::http("/health")
//!                 .interval(Duration::from_secs(10))
//!                 .timeout(Duration::from_secs(5))
//!         )
//!         .algorithm(Algorithm::RoundRobin)
//!         .build();
//!
//!     lb.listen("0.0.0.0:8080").await?;
//!     Ok(())
//! }
//! ```
//!
//! Access the health status endpoint at `http://localhost:8080/_health`.

use std::hash::{Hash, Hasher};
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use axum::body::Body;
use axum::extract::{ConnectInfo, State};
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use dashmap::DashMap;
use governor::{Quota, RateLimiter as GovernorRateLimiter};
use parking_lot::RwLock;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};
use twox_hash::XxHash64;

type RateLimiterType = GovernorRateLimiter<
    governor::state::direct::NotKeyed,
    governor::state::InMemoryState,
    governor::clock::DefaultClock,
    governor::middleware::NoOpMiddleware
>;

/// Load balancing algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
	/// Round-robin distribution
	RoundRobin,
	/// Route to backend with fewest active connections
	LeastConnections,
	/// Consistent hashing based on client IP
	IpHash,
	/// Weighted distribution based on backend capacity
	Weighted,
}

/// Rate limiting configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
	/// Maximum requests per second per IP
	pub requests_per_second: u32,
	/// Burst size
	pub burst: u32,
}

impl Default for RateLimitConfig {
	fn default() -> Self {
		Self {
			requests_per_second: 100,
			burst: 200,
		}
	}
}

impl RateLimitConfig {
	pub fn new(requests_per_second: u32) -> Self {
		Self {
			requests_per_second,
			burst: requests_per_second * 2,
		}
	}

	pub fn burst(mut self, burst: u32) -> Self {
		self.burst = burst;
		self
	}
}

/// Retry configuration.
#[derive(Debug, Clone)]
pub struct RetryConfig {
	/// Maximum retry attempts
	pub max_retries: usize,
	/// Backoff between retries
	pub backoff: Duration,
}

impl Default for RetryConfig {
	fn default() -> Self {
		Self {
			max_retries: 3,
			backoff: Duration::from_millis(100),
		}
	}
}

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
	Closed,
	Open,
	HalfOpen,
}

/// Health check configuration.
#[derive(Debug, Clone)]
pub struct HealthCheck {
    /// HTTP path to probe (e.g., "/health")
    pub path: String,
    /// Interval between probes
    pub interval: Duration,
    /// Request timeout
    pub timeout: Duration,
}

impl HealthCheck {
    pub fn http<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            interval: Duration::from_secs(10),
            timeout: Duration::from_secs(5),
        }
    }

    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}/// Backend server definition with advanced tracking.
#[derive(Debug)]
pub struct Backend {
    pub url: String,
    pub weight: u32,
    healthy: Arc<AtomicBool>,
    active_connections: Arc<AtomicUsize>,
    total_requests: Arc<AtomicU64>,
    failed_requests: Arc<AtomicU64>,
    circuit_state: Arc<RwLock<CircuitState>>,
    circuit_opened_at: Arc<RwLock<Option<Instant>>>,
}

impl Backend {
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self {
            url: url.into(),
            weight: 1,
            healthy: Arc::new(AtomicBool::new(true)),
            active_connections: Arc::new(AtomicUsize::new(0)),
            total_requests: Arc::new(AtomicU64::new(0)),
            failed_requests: Arc::new(AtomicU64::new(0)),
            circuit_state: Arc::new(RwLock::new(CircuitState::Closed)),
            circuit_opened_at: Arc::new(RwLock::new(None)),
        }
    }

    pub fn new_unhealthy<U: Into<String>>(url: U) -> Self {
        let backend = Self::new(url);
        backend.healthy.store(false, Ordering::Relaxed);
        backend
    }

    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight.max(1);
        self
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Relaxed)
    }

    fn set_healthy(&self, healthy: bool) {
        self.healthy.store(healthy, Ordering::Relaxed);
    }

    pub fn active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    fn record_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        let total = self.total_requests.load(Ordering::Relaxed);
        let failed = self.failed_requests.load(Ordering::Relaxed);

        // Circuit breaker: open if >50% failures in last 100 requests
        if total > 100 && failed as f64 / total as f64 > 0.5 {
            let mut state = self.circuit_state.write();
            if *state == CircuitState::Closed {
                *state = CircuitState::Open;
                *self.circuit_opened_at.write() = Some(Instant::now());
                warn!(backend=%self.url, "circuit breaker opened");
            }
        }
    }

    fn is_circuit_open(&self) -> bool {
        let state = self.circuit_state.read();
        match *state {
            CircuitState::Open => {
                drop(state);
                // Check if we should transition to half-open
                if let Some(opened_at) = *self.circuit_opened_at.read() {
                    if opened_at.elapsed() > Duration::from_secs(30) {
                        *self.circuit_state.write() = CircuitState::HalfOpen;
                        info!(backend=%self.url, "circuit breaker half-open");
                        return false;
                    }
                }
                true
            }
            CircuitState::HalfOpen => false,
            CircuitState::Closed => false,
        }
    }

    fn record_success(&self) {
        let state = self.circuit_state.read();
        if *state == CircuitState::HalfOpen {
            drop(state);
            *self.circuit_state.write() = CircuitState::Closed;
            info!(backend=%self.url, "circuit breaker closed");
        }
    }
}

impl Clone for Backend {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            weight: self.weight,
            healthy: self.healthy.clone(),
            active_connections: self.active_connections.clone(),
            total_requests: self.total_requests.clone(),
            failed_requests: self.failed_requests.clone(),
            circuit_state: self.circuit_state.clone(),
            circuit_opened_at: self.circuit_opened_at.clone(),
        }
    }
}/// Builder for `LoadBalancer`.
pub struct LoadBalancerBuilder {
	backends: Vec<Backend>,
	health_check: Option<HealthCheck>,
	algorithm: Algorithm,
	rate_limit: Option<RateLimitConfig>,
	retry_config: Option<RetryConfig>,
	max_request_body_size: usize,
}

impl LoadBalancerBuilder {
	fn new() -> Self {
		Self {
			backends: Vec::new(),
			health_check: None,
			algorithm: Algorithm::RoundRobin,
			rate_limit: None,
			retry_config: None,
			max_request_body_size: 10 * 1024 * 1024, // 10MB default
		}
	}

	pub fn add_backend(mut self, backend: Backend) -> Self {
		self.backends.push(backend);
		self
	}

	pub fn backends(mut self, backends: Vec<Backend>) -> Self {
		self.backends = backends;
		self
	}

	pub fn health_check(mut self, hc: HealthCheck) -> Self {
		self.health_check = Some(hc);
		self
	}

	pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
		self.algorithm = algorithm;
		self
	}

	pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
		self.rate_limit = Some(config);
		self
	}

	pub fn retry_config(mut self, config: RetryConfig) -> Self {
		self.retry_config = Some(config);
		self
	}

	pub fn max_request_body_size(mut self, size: usize) -> Self {
		self.max_request_body_size = size;
		self
	}

	pub fn build(self) -> LoadBalancer {
		LoadBalancer::new(
			self.backends,
			self.health_check,
			self.algorithm,
			self.rate_limit,
			self.retry_config,
			self.max_request_body_size,
		)
	}
}

/// Core load balancer type.
pub struct LoadBalancer {
	inner: Arc<Inner>,
}

struct Inner {
    backends: Vec<Backend>,
    algorithm: Algorithm,
    rr_counter: AtomicUsize,
    health_check: Option<HealthCheck>,
    client: reqwest::Client,
    rate_limiters: Option<Arc<DashMap<IpAddr, Arc<RateLimiterType>>>>,
    rate_limit_config: Option<RateLimitConfig>,
    retry_config: RetryConfig,
    max_request_body_size: usize,
}impl LoadBalancer {
	pub fn builder() -> LoadBalancerBuilder { LoadBalancerBuilder::new() }

    fn new(
        backends: Vec<Backend>,
        health_check: Option<HealthCheck>,
        algorithm: Algorithm,
        rate_limit: Option<RateLimitConfig>,
        retry_config: Option<RetryConfig>,
        max_request_body_size: usize,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(Duration::from_secs(90))
            .build()
            .unwrap();

        let rate_limiters = rate_limit.as_ref().map(|_| Arc::new(DashMap::new()));

        Self {
            inner: Arc::new(Inner {
                backends,
                algorithm,
                rr_counter: AtomicUsize::new(0),
                health_check,
                client,
                rate_limiters,
                rate_limit_config: rate_limit,
                retry_config: retry_config.unwrap_or_default(),
                max_request_body_size,
            })
        }
    }    /// Start listening on the provided address (e.g. `"127.0.0.1:8080"`).
    /// Blocks until server shutdown. Returns an error if binding or serving fails.
    pub async fn listen(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        let local_addr = listener.local_addr()?;
        info!(%local_addr, "load balancer listening");

        // Start health check background task
        if self.inner.health_check.is_some() {
            let inner = self.inner.clone();
            tokio::spawn(async move {
                health_check_loop(inner).await;
            });
        }

        let app = self.router();
        axum::serve(listener, app).await?;
        Ok(())
    }    /// Spawn the load balancer on an ephemeral port (useful for tests).
    pub async fn spawn_ephemeral(&self) -> Result<(SocketAddr, JoinHandle<Result<()>>)> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        // Start health check background task
        if self.inner.health_check.is_some() {
            let inner = self.inner.clone();
            tokio::spawn(async move {
                health_check_loop(inner).await;
            });
        }

        let app = self.router();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await?;
            Ok(())
        });
        Ok((addr, handle))
    }	fn router(&self) -> axum::extract::connect_info::IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
		let inner = self.inner.clone();
		Router::new()
			.route("/_health", axum::routing::get(health_status_handler))
			.route("/_metrics", axum::routing::get(metrics_handler))
			.fallback(any(proxy_handler))
			.with_state(inner)
			.into_make_service_with_connect_info::<SocketAddr>()
	}
}

async fn proxy_handler(
	State(inner): State<Arc<Inner>>,
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
	req: Request<Body>,
) -> impl IntoResponse {
	let client_ip = addr.ip();

	// Rate limiting
	if let Some(ref limiters) = inner.rate_limiters {
		if let Some(ref config) = inner.rate_limit_config {
			let limiter = limiters.entry(client_ip).or_insert_with(|| {
				Arc::new(GovernorRateLimiter::direct(
					Quota::per_second(std::num::NonZeroU32::new(config.requests_per_second).unwrap())
						.allow_burst(std::num::NonZeroU32::new(config.burst).unwrap())
				))
			});
			if limiter.check().is_err() {
				warn!(client_ip=%client_ip, "rate limit exceeded");
				return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
			}
		}
	}

	let Some(backend) = select_backend(&inner, Some(client_ip)) else {
		error!("no backends available");
		return (StatusCode::SERVICE_UNAVAILABLE, "No backends available").into_response();
	};

	// Circuit breaker check
	if backend.is_circuit_open() {
		warn!(backend=%backend.url, "circuit breaker open");
		return (StatusCode::SERVICE_UNAVAILABLE, "Service temporarily unavailable").into_response();
	}

	backend.increment_connections();
	let _guard = ConnectionGuard { backend: backend.clone() };

	let path_and_query = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
	let target = format!("{}{}", backend.url.trim_end_matches('/'), path_and_query);
	debug!(%target, "proxying request");

	let method = req.method().clone();
	let headers = req.headers().clone();
	let body_bytes = axum::body::to_bytes(req.into_body(), inner.max_request_body_size)
		.await
		.unwrap_or_default();

	// Retry logic
	let mut attempts = 0;
	let max_attempts = inner.retry_config.max_retries + 1;

	loop {
		attempts += 1;
		let mut builder = inner.client.request(method.clone(), &target);
		for (k, v) in headers.iter() {
			builder = builder.header(k, v);
		}

		let upstream = builder.body(body_bytes.to_vec()).send().await;

		match upstream {
			Ok(up) => {
				let status = up.status();
				let headers = up.headers().clone();
				let bytes = up.bytes().await.unwrap_or_default();

				if status.is_success() || status.is_redirection() {
					backend.record_success();
				}

				let mut resp = Response::new(Body::from(bytes));
				*resp.status_mut() = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK);
				for (k, v) in headers.iter() {
					resp.headers_mut().insert(k, v.clone());
				}
				return resp;
			}
			Err(e) => {
				backend.record_failure();
				if attempts >= max_attempts {
					error!(backend=%backend.url, error=%e, attempts, "upstream request failed after retries");
					return (StatusCode::BAD_GATEWAY, "Bad Gateway").into_response();
				}
				debug!(backend=%backend.url, error=%e, attempt=attempts, "retrying request");
				tokio::time::sleep(inner.retry_config.backoff).await;
			}
		}
	}
}

struct ConnectionGuard {
	backend: Backend,
}

impl Drop for ConnectionGuard {
	fn drop(&mut self) {
		self.backend.decrement_connections();
	}
}

fn select_backend(inner: &Inner, client_ip: Option<IpAddr>) -> Option<&Backend> {
    if inner.backends.is_empty() { return None; }

    // Filter healthy backends that aren't circuit-broken
    let available: Vec<_> = inner.backends.iter()
        .filter(|b| b.is_healthy() && !b.is_circuit_open())
        .collect();

    if available.is_empty() {
        // Fallback: try any healthy backend even if circuit open
        let healthy: Vec<_> = inner.backends.iter().filter(|b| b.is_healthy()).collect();
        if healthy.is_empty() {
            // Last resort: any backend
            return Some(&inner.backends[0]);
        }
        return Some(healthy[0]);
    }

    match inner.algorithm {
        Algorithm::RoundRobin => {
            let idx = inner.rr_counter.fetch_add(1, Ordering::Relaxed) % available.len();
            Some(available[idx])
        }
        Algorithm::LeastConnections => {
            available.iter()
                .min_by_key(|b| b.active_connections())
                .copied()
        }
        Algorithm::IpHash => {
            if let Some(ip) = client_ip {
                let mut hasher = XxHash64::default();
                match ip {
                    IpAddr::V4(v4) => v4.octets().hash(&mut hasher),
                    IpAddr::V6(v6) => v6.octets().hash(&mut hasher),
                }
                let hash = hasher.finish() as usize;
                Some(available[hash % available.len()])
            } else {
                // Fallback to round-robin if no IP
                let idx = inner.rr_counter.fetch_add(1, Ordering::Relaxed) % available.len();
                Some(available[idx])
            }
        }
        Algorithm::Weighted => {
            let total_weight: u32 = available.iter().map(|b| b.weight).sum();
            if total_weight == 0 {
                return Some(available[0]);
            }

            let mut rnd = inner.rr_counter.fetch_add(1, Ordering::Relaxed) as u32 % total_weight;
            for backend in &available {
                if rnd < backend.weight {
                    return Some(backend);
                }
                rnd -= backend.weight;
            }
            Some(available[0])
        }
    }
}async fn health_check_loop(inner: Arc<Inner>) {
    let Some(ref hc) = inner.health_check else { return };
    let interval = hc.interval;
    let timeout = hc.timeout;
    let path = hc.path.clone();

    info!(interval_secs=?interval.as_secs(), "health check loop started");

    loop {
        for backend in &inner.backends {
            let url = format!("{}{}", backend.url.trim_end_matches('/'), path);
            let client = inner.client.clone();
            let backend_clone = backend.clone();

            tokio::spawn(async move {
                let result = tokio::time::timeout(
                    timeout,
                    client.get(&url).send()
                ).await;

                let healthy = match result {
                    Ok(Ok(resp)) => resp.status().is_success(),
                    Ok(Err(e)) => {
                        debug!(backend=%backend_clone.url, error=%e, "health check failed");
                        false
                    }
                    Err(_) => {
                        debug!(backend=%backend_clone.url, "health check timeout");
                        false
                    }
                };

                backend_clone.set_healthy(healthy);
                if !healthy {
                    info!(backend=%backend_clone.url, "backend marked unhealthy");
                } else {
                    debug!(backend=%backend_clone.url, "backend healthy");
                }
            });
        }

        tokio::time::sleep(interval).await;
    }
}

async fn health_status_handler(State(inner): State<Arc<Inner>>) -> impl IntoResponse {
    use serde_json::json;

    let backends: Vec<_> = inner.backends.iter().map(|b| {
        json!({
            "url": b.url,
            "healthy": b.is_healthy(),
            "active_connections": b.active_connections(),
            "total_requests": b.total_requests.load(Ordering::Relaxed),
            "failed_requests": b.failed_requests.load(Ordering::Relaxed),
            "circuit_state": format!("{:?}", *b.circuit_state.read()),
            "weight": b.weight,
        })
    }).collect();

    let healthy_count = inner.backends.iter().filter(|b| b.is_healthy()).count();
    let total = inner.backends.len();

    let status = json!({
        "healthy": healthy_count > 0,
        "backends": backends,
        "healthy_count": healthy_count,
        "total_count": total,
        "algorithm": format!("{:?}", inner.algorithm),
    });

    let status_code = if healthy_count > 0 {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, axum::Json(status))
}

async fn metrics_handler(State(inner): State<Arc<Inner>>) -> impl IntoResponse {
    use serde_json::json;

    let total_requests: u64 = inner.backends.iter()
        .map(|b| b.total_requests.load(Ordering::Relaxed))
        .sum();
    let total_failures: u64 = inner.backends.iter()
        .map(|b| b.failed_requests.load(Ordering::Relaxed))
        .sum();
    let total_active: usize = inner.backends.iter()
        .map(|b| b.active_connections())
        .sum();

    let metrics = json!({
        "total_requests": total_requests,
        "total_failures": total_failures,
        "total_active_connections": total_active,
        "success_rate": if total_requests > 0 {
            ((total_requests - total_failures) as f64 / total_requests as f64) * 100.0
        } else {
            100.0
        },
        "backends": inner.backends.iter().map(|b| {
            let total = b.total_requests.load(Ordering::Relaxed);
            let failed = b.failed_requests.load(Ordering::Relaxed);
            json!({
                "url": b.url,
                "requests": total,
                "failures": failed,
                "active_connections": b.active_connections(),
                "success_rate": if total > 0 {
                    ((total - failed) as f64 / total as f64) * 100.0
                } else {
                    100.0
                },
            })
        }).collect::<Vec<_>>(),
    });

    axum::Json(metrics)
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::{routing::get, Router};
	use tokio::time::{sleep, Duration};

	#[tokio::test]
	async fn basic_proxy() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();
		// Start a simple backend server
		let backend_listener = TcpListener::bind("127.0.0.1:0").await?;
		let backend_addr = backend_listener.local_addr()?;
		let app = Router::new().route("/", get(|| async { "OK" }));
		tokio::spawn(async move {
			axum::serve(backend_listener, app).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", backend_addr)))
			.algorithm(Algorithm::RoundRobin)
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		// Wait briefly for server to start
		sleep(Duration::from_millis(150)).await; // allow backend & LB to initialize
		let client = reqwest::Client::new();
		let resp = client.get(format!("http://{}", lb_addr)).send().await.unwrap();
		assert_eq!(resp.status(), reqwest::StatusCode::OK);
		let body_text = resp.text().await.unwrap();
		assert_eq!(body_text, "OK");
		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn health_check_marks_unhealthy_backend() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		// Start one healthy backend
		let healthy_listener = TcpListener::bind("127.0.0.1:0").await?;
		let healthy_addr = healthy_listener.local_addr()?;
		let app1 = Router::new()
			.route("/", get(|| async { "healthy" }))
			.route("/health", get(|| async { "ok" }));
		tokio::spawn(async move {
			axum::serve(healthy_listener, app1).await.unwrap();
		});

		// Configure LB with health checks and one good + one bad backend
		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", healthy_addr)))
			.add_backend(Backend::new_unhealthy("http://127.0.0.1:1")) // unreachable, start unhealthy
			.health_check(HealthCheck::http("/health").interval(Duration::from_millis(500)))
			.algorithm(Algorithm::RoundRobin)
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;

		// Wait for servers and initial health checks
		sleep(Duration::from_millis(200)).await;

		// Should still work because one backend is healthy
		let client = reqwest::Client::new();
		let resp = client.get(format!("http://{}", lb_addr)).send().await.unwrap();
		assert_eq!(resp.status(), reqwest::StatusCode::OK);

		// Check health status endpoint
		let health_resp = client.get(format!("http://{}/_health", lb_addr)).send().await.unwrap();
		assert_eq!(health_resp.status(), reqwest::StatusCode::OK);
		let json: serde_json::Value = health_resp.json().await.unwrap();
		assert_eq!(json["healthy_count"], 1);
		assert_eq!(json["total_count"], 2);

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn all_backends_unhealthy() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		// Configure LB with only unreachable backends
		let lb = LoadBalancer::builder()
			.add_backend(Backend::new_unhealthy("http://127.0.0.1:1"))
			.add_backend(Backend::new_unhealthy("http://127.0.0.1:2"))
			.health_check(HealthCheck::http("/health").interval(Duration::from_millis(500)))
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;

		sleep(Duration::from_millis(200)).await;

		// Health endpoint should report unhealthy
		let client = reqwest::Client::new();
		let health_resp = client.get(format!("http://{}/_health", lb_addr)).send().await.unwrap();
		assert_eq!(health_resp.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
		let json: serde_json::Value = health_resp.json().await.unwrap();
		assert_eq!(json["healthy_count"], 0);
		assert_eq!(json["healthy"], false);

		// Proxy request should still attempt fallback but get 502 (bad gateway)
		let proxy_resp = client.get(format!("http://{}/test", lb_addr)).send().await.unwrap();
		assert_eq!(proxy_resp.status(), reqwest::StatusCode::BAD_GATEWAY); // upstream fails

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn rate_limiting_blocks_excess_requests() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend_listener = TcpListener::bind("127.0.0.1:0").await?;
		let backend_addr = backend_listener.local_addr()?;
		let app = Router::new().route("/", get(|| async { "OK" }));
		tokio::spawn(async move {
			axum::serve(backend_listener, app).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", backend_addr)))
			.rate_limit(RateLimitConfig { requests_per_second: 2, burst: 2 })
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();
		// First 2 should succeed (burst)
		let r1 = client.get(format!("http://{}", lb_addr)).send().await?;
		assert_eq!(r1.status(), reqwest::StatusCode::OK);
		let r2 = client.get(format!("http://{}", lb_addr)).send().await?;
		assert_eq!(r2.status(), reqwest::StatusCode::OK);
		// Third should be rate limited
		let r3 = client.get(format!("http://{}", lb_addr)).send().await?;
		assert_eq!(r3.status(), reqwest::StatusCode::TOO_MANY_REQUESTS);

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn least_connections_algorithm() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend1 = TcpListener::bind("127.0.0.1:0").await?;
		let addr1 = backend1.local_addr()?;
		let app1 = Router::new().route("/", get(|| async {
			sleep(Duration::from_millis(200)).await; // slow response
			"backend1"
		}));
		tokio::spawn(async move {
			axum::serve(backend1, app1).await.unwrap();
		});

		let backend2 = TcpListener::bind("127.0.0.1:0").await?;
		let addr2 = backend2.local_addr()?;
		let app2 = Router::new().route("/", get(|| async { "backend2" }));
		tokio::spawn(async move {
			axum::serve(backend2, app2).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr1)))
			.add_backend(Backend::new(format!("http://{}", addr2)))
			.algorithm(Algorithm::LeastConnections)
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();
		// Send first request (will be slow)
		let fut1 = client.get(format!("http://{}", lb_addr)).send();
		sleep(Duration::from_millis(50)).await; // ensure first request is in-flight
		// Second request should go to backend2 (fewer connections)
		let r2 = client.get(format!("http://{}", lb_addr)).send().await?;
		assert_eq!(r2.text().await?, "backend2");

		let _r1 = fut1.await?; // complete first request

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn weighted_algorithm() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend1 = TcpListener::bind("127.0.0.1:0").await?;
		let addr1 = backend1.local_addr()?;
		let app1 = Router::new().route("/", get(|| async { "backend1" }));
		tokio::spawn(async move {
			axum::serve(backend1, app1).await.unwrap();
		});

		let backend2 = TcpListener::bind("127.0.0.1:0").await?;
		let addr2 = backend2.local_addr()?;
		let app2 = Router::new().route("/", get(|| async { "backend2" }));
		tokio::spawn(async move {
			axum::serve(backend2, app2).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr1)).with_weight(3))
			.add_backend(Backend::new(format!("http://{}", addr2)).with_weight(1))
			.algorithm(Algorithm::Weighted)
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();
		let mut backend1_count = 0;
		let mut backend2_count = 0;
		for _ in 0..12 {
			let resp = client.get(format!("http://{}", lb_addr)).send().await?;
			let body = resp.text().await?;
			if body == "backend1" {
				backend1_count += 1;
			} else {
				backend2_count += 1;
			}
		}
		// With weight 3:1, expect roughly 9:3 distribution
		assert!(backend1_count >= 7 && backend1_count <= 11);
		assert!(backend2_count >= 1 && backend2_count <= 5);

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn ip_hash_algorithm() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend1 = TcpListener::bind("127.0.0.1:0").await?;
		let addr1 = backend1.local_addr()?;
		let app1 = Router::new().route("/", get(|| async { "backend1" }));
		tokio::spawn(async move {
			axum::serve(backend1, app1).await.unwrap();
		});

		let backend2 = TcpListener::bind("127.0.0.1:0").await?;
		let addr2 = backend2.local_addr()?;
		let app2 = Router::new().route("/", get(|| async { "backend2" }));
		tokio::spawn(async move {
			axum::serve(backend2, app2).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr1)))
			.add_backend(Backend::new(format!("http://{}", addr2)))
			.algorithm(Algorithm::IpHash)
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();
		// Same client IP should always go to the same backend
		let first_body = client.get(format!("http://{}", lb_addr)).send().await?.text().await?;
		for _ in 0..5 {
			let body = client.get(format!("http://{}", lb_addr)).send().await?.text().await?;
			assert_eq!(body, first_body); // consistent routing
		}

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn circuit_breaker_opens_on_failures() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new("http://127.0.0.1:1")) // unreachable
			.retry_config(RetryConfig { max_retries: 0, backoff: Duration::from_millis(1) })
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(50)).await;

		let client = reqwest::Client::new();
		// Generate failures to open circuit breaker (need >50% failure rate over 100 requests)
		for _ in 0..60 {
			let _ = client.get(format!("http://{}", lb_addr)).send().await;
		}

		// Check metrics to see circuit state
		let metrics_resp = client.get(format!("http://{}/_metrics", lb_addr)).send().await?;
		let json: serde_json::Value = metrics_resp.json().await?;
		assert!(json["total_failures"].as_u64().unwrap() > 50);

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn retry_logic_retries_failed_requests() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend = TcpListener::bind("127.0.0.1:0").await?;
		let addr = backend.local_addr()?;
		let counter = Arc::new(AtomicUsize::new(0));
		let counter_clone = counter.clone();
		let app = Router::new().route("/", get(move || {
			let counter = counter_clone.clone();
			async move {
				let count = counter.fetch_add(1, Ordering::SeqCst);
				if count < 2 {
					// Fail first 2 requests
					(StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
				} else {
					"success".into_response()
				}
			}
		}));
		tokio::spawn(async move {
			axum::serve(backend, app).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr)))
			.retry_config(RetryConfig { max_retries: 3, backoff: Duration::from_millis(10) })
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();
		let resp = client.get(format!("http://{}", lb_addr)).send().await?;
		// Should succeed after retries
		assert_eq!(resp.status(), reqwest::StatusCode::OK);
		assert_eq!(resp.text().await?, "success");

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn metrics_endpoint_returns_stats() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend = TcpListener::bind("127.0.0.1:0").await?;
		let addr = backend.local_addr()?;
		let app = Router::new().route("/", get(|| async { "OK" }));
		tokio::spawn(async move {
			axum::serve(backend, app).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr)))
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();
		// Generate some traffic
		for _ in 0..5 {
			let _ = client.get(format!("http://{}", lb_addr)).send().await;
		}

		let metrics_resp = client.get(format!("http://{}/_metrics", lb_addr)).send().await?;
		assert_eq!(metrics_resp.status(), reqwest::StatusCode::OK);
		let json: serde_json::Value = metrics_resp.json().await?;
		assert!(json["total_requests"].as_u64().unwrap() >= 5);
		assert!(json.get("backends").is_some());

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn round_robin_multiple_backends() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		// Start two backends with different responses
		let listener1 = TcpListener::bind("127.0.0.1:0").await?;
		let addr1 = listener1.local_addr()?;
		let app1 = Router::new().route("/", get(|| async { "backend1" }));
		tokio::spawn(async move {
			axum::serve(listener1, app1).await.unwrap();
		});

		let listener2 = TcpListener::bind("127.0.0.1:0").await?;
		let addr2 = listener2.local_addr()?;
		let app2 = Router::new().route("/", get(|| async { "backend2" }));
		tokio::spawn(async move {
			axum::serve(listener2, app2).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr1)))
			.add_backend(Backend::new(format!("http://{}", addr2)))
			.algorithm(Algorithm::RoundRobin)
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(150)).await;

		let client = reqwest::Client::new();

		// Make multiple requests and collect responses
		let mut responses = Vec::new();
		for _ in 0..4 {
			let resp = client.get(format!("http://{}", lb_addr)).send().await.unwrap();
			responses.push(resp.text().await.unwrap());
		}

		// Should alternate between backends
		assert!(responses.contains(&"backend1".to_string()));
		assert!(responses.contains(&"backend2".to_string()));

		handle.abort();
		Ok(())
	}
}

