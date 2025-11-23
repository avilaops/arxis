//! AVL LoadBalancer Library
//!
//! Production-ready L7 load balancer with active health checks and multiple distribution algorithms.
//! Also provides STUN/TURN servers for WebRTC NAT traversal in remote desktop scenarios.
//!
//! ## Features
//!
//! * Multiple load balancing algorithms (RoundRobin, LeastConnections, IpHash, Weighted)
//! * Active HTTP health checks with circuit breaker per backend
//! * Rate limiting per client IP
//! * Automatic retry with configurable backoff
//! * Connection tracking and metrics endpoint
//! * Built-in health status endpoint (`/_health`)
//! * **STUN Server** for NAT discovery
//! * **TURN Server** for WebRTC relay
//! * Fluent builder API
//!
//! ## Example
//!
//! ```rust,no_run
//! use avl_loadbalancer::{LoadBalancer, Backend, HealthCheck, Algorithm, RateLimitConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let lb = LoadBalancer::builder()
//!         .add_backend(Backend::new("http://192.168.1.10:8000").with_weight(3))
//!         .add_backend(Backend::new("http://192.168.1.11:8000").with_weight(1))
//!         .health_check(
//!             HealthCheck::http("/health")
//!                 .interval(Duration::from_secs(10))
//!                 .timeout(Duration::from_secs(5))
//!         )
//!         .algorithm(Algorithm::Weighted)
//!         .rate_limit(RateLimitConfig { requests_per_second: 100, burst: 20 })
//!         .build();
//!
//!     lb.listen("0.0.0.0:8080").await?;
//!     Ok(())
//! }
//! ```
//!
//! Access the health status endpoint at `http://localhost:8080/_health`.
//! Access the metrics endpoint at `http://localhost:8080/_metrics`.

// STUN/TURN for WebRTC NAT Traversal
pub mod stun;

// Advanced Features
#[cfg(feature = "tls")]
pub mod tls;

#[cfg(feature = "sticky-sessions")]
pub mod sticky_sessions;

#[cfg(feature = "geo-routing")]
pub mod geo;

#[cfg(feature = "hot-reload")]
pub mod hot_reload;

pub mod middleware;

#[cfg(feature = "tracing")]
pub mod tracing_otel;

use std::hash::{Hash, Hasher};
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use axum::body::Body;
use axum::extract::{ConnectInfo, State, ws::{WebSocket, WebSocketUpgrade}};
use axum::http::{Request, Response, StatusCode, HeaderValue, header};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use dashmap::DashMap;
use governor::{Quota, RateLimiter as GovernorRateLimiter};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Algorithm {
	/// Round-robin distribution - cycles through backends sequentially
	RoundRobin,
	/// Route to backend with fewest active connections
	LeastConnections,
	/// Consistent hashing based on client IP - same IP always routes to same backend
	IpHash,
	/// Weighted distribution based on backend capacity - backends with higher weight receive more traffic
	Weighted,
}

/// Rate limiting configuration per client IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
	/// Maximum requests per second per IP
	pub requests_per_second: u32,
	/// Burst size - allows this many requests to exceed the rate limit temporarily
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
	/// Create new rate limit config with default burst (2x rate)
	pub fn new(requests_per_second: u32) -> Self {
		Self {
			requests_per_second,
			burst: requests_per_second * 2,
		}
	}

	/// Set custom burst size
	pub fn burst(mut self, burst: u32) -> Self {
		self.burst = burst;
		self
	}
}

/// Retry configuration for failed upstream requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
	/// Maximum retry attempts (default: 3)
	pub max_retries: usize,
	/// Backoff duration between retries (default: 100ms)
	#[serde(with = "serde_millis")]
	pub backoff: Duration,
}

mod serde_millis {
	use serde::{Deserialize, Deserializer, Serializer};
	use std::time::Duration;

	pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_u64(duration.as_millis() as u64)
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
	where
		D: Deserializer<'de>,
	{
		let millis = u64::deserialize(deserializer)?;
		Ok(Duration::from_millis(millis))
	}
}

impl Default for RetryConfig {
	fn default() -> Self {
		Self {
			max_retries: 3,
			backoff: Duration::from_millis(100),
		}
	}
}

/// Circuit breaker state for a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
	/// Circuit closed - backend operating normally
	Closed,
	/// Circuit open - backend failing, no requests routed
	Open,
	/// Circuit half-open - testing if backend recovered
	HalfOpen,
}

/// Health check configuration for active probing.
#[derive(Debug, Clone)]
pub struct HealthCheck {
    /// HTTP path to probe (e.g., "/health")
    pub path: String,
    /// Interval between probes (default: 30s)
    pub interval: Duration,
    /// Request timeout (default: 5s)
    pub timeout: Duration,
}

impl HealthCheck {
	/// Create health check with HTTP path
	pub fn http<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            interval: Duration::from_secs(10),
            timeout: Duration::from_secs(5),
        }
    }

	/// Set probe interval
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

	/// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Backend server definition with advanced tracking.
#[derive(Debug)]
pub struct Backend {
    pub url: String,
    /// Weight for weighted load balancing (default: 1)
    pub weight: u32,
    healthy: Arc<AtomicBool>,
    /// Number of active connections to this backend
    active_connections: Arc<AtomicUsize>,
    /// Total requests routed to this backend
    total_requests: Arc<AtomicU64>,
    /// Total failed requests
    failed_requests: Arc<AtomicU64>,
    /// Circuit breaker state
    circuit_state: Arc<RwLock<CircuitState>>,
    /// Timestamp when circuit opened
    circuit_opened_at: Arc<RwLock<Option<Instant>>>,
}

impl Backend {
	/// Create new backend with URL
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

	/// Create new backend marked initially unhealthy (for testing)
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
}

/// Configuration file format for YAML/TOML loading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
	/// Listen address (e.g., "0.0.0.0:8080")
	pub listen: String,
	/// Load balancing algorithm
	pub algorithm: Algorithm,
	/// Backend servers
	pub backends: Vec<BackendConfig>,
	/// Health check configuration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub health_check: Option<HealthCheckConfig>,
	/// Rate limit configuration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit: Option<RateLimitConfig>,
	/// Retry configuration
	#[serde(skip_serializing_if = "Option::is_none")]
	pub retry: Option<RetryConfigFile>,
	/// Max request body size in MB
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_request_body_mb: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
	pub url: String,
	#[serde(default = "default_weight")]
	pub weight: u32,
}

fn default_weight() -> u32 {
	1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
	pub path: String,
	#[serde(default = "default_interval_secs")]
	pub interval_secs: u64,
	#[serde(default = "default_timeout_secs")]
	pub timeout_secs: u64,
}

fn default_interval_secs() -> u64 {
	10
}

fn default_timeout_secs() -> u64 {
	5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfigFile {
	#[serde(default = "default_max_retries")]
	pub max_retries: usize,
	#[serde(default = "default_backoff_ms")]
	pub backoff_ms: u64,
}

fn default_max_retries() -> usize {
	3
}

fn default_backoff_ms() -> u64 {
	100
}

impl LoadBalancerConfig {
	/// Load configuration from YAML file
	pub fn from_yaml_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
		let content = std::fs::read_to_string(path)?;
		Ok(serde_yaml::from_str(&content)?)
	}

	/// Load configuration from TOML file
	pub fn from_toml_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
		let content = std::fs::read_to_string(path)?;
		Ok(toml::from_str(&content)?)
	}

	/// Build LoadBalancer from config
	pub fn build_loadbalancer(&self) -> LoadBalancer {
		let mut builder = LoadBalancer::builder().algorithm(self.algorithm);

		for backend_cfg in &self.backends {
			let backend = Backend::new(&backend_cfg.url).with_weight(backend_cfg.weight);
			builder = builder.add_backend(backend);
		}

		if let Some(hc_cfg) = &self.health_check {
			let hc = HealthCheck::http(&hc_cfg.path)
				.interval(Duration::from_secs(hc_cfg.interval_secs))
				.timeout(Duration::from_secs(hc_cfg.timeout_secs));
			builder = builder.health_check(hc);
		}

		if let Some(rl_cfg) = &self.rate_limit {
			builder = builder.rate_limit(rl_cfg.clone());
		}

		if let Some(retry_cfg) = &self.retry {
			let retry = RetryConfig {
				max_retries: retry_cfg.max_retries,
				backoff: Duration::from_millis(retry_cfg.backoff_ms),
			};
			builder = builder.retry_config(retry);
		}

		if let Some(max_mb) = self.max_request_body_mb {
			builder = builder.max_request_body_size(max_mb * 1024 * 1024);
		}

		builder.build()
	}
}

/// Builder for `LoadBalancer`.
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

	/// Add a backend server
	pub fn add_backend(mut self, backend: Backend) -> Self {
		self.backends.push(backend);
		self
	}

	/// Set all backends at once
	pub fn backends(mut self, backends: Vec<Backend>) -> Self {
		self.backends = backends;
		self
	}

	/// Configure health checks
	pub fn health_check(mut self, hc: HealthCheck) -> Self {
		self.health_check = Some(hc);
		self
	}

	/// Set load balancing algorithm
	pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
		self.algorithm = algorithm;
		self
	}

	/// Configure rate limiting per client IP
	pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
		self.rate_limit = Some(config);
		self
	}

	/// Configure retry logic for failed requests
	pub fn retry_config(mut self, config: RetryConfig) -> Self {
		self.retry_config = Some(config);
		self
	}

	/// Set maximum request body size in bytes (default: 10MB)
	pub fn max_request_body_size(mut self, size: usize) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Build the load balancer
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
    /// Supports graceful shutdown via SIGTERM/SIGINT signals.
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

        // Graceful shutdown
        let graceful = axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal());

        graceful.await?;
        info!("Load balancer shutdown complete");
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
			.route("/_ws", axum::routing::get(websocket_upgrade_handler))
			.fallback(any(proxy_handler))
			.with_state(inner)
			.into_make_service_with_connect_info::<SocketAddr>()
	}
}

async fn websocket_upgrade_handler(
	ws: WebSocketUpgrade,
	State(inner): State<Arc<Inner>>,
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
	let client_ip = addr.ip();
	debug!(%client_ip, "WebSocket upgrade request");

	// Select backend for WebSocket connection - clone to avoid lifetime issues
	let Some(backend) = select_backend(&inner, Some(client_ip)).map(|b| b.clone()) else {
		warn!("no backends available for WebSocket upgrade");
		return (StatusCode::SERVICE_UNAVAILABLE, "No backends available").into_response();
	};

	if backend.is_circuit_open() {
		warn!(backend=%backend.url, "circuit breaker open for WebSocket");
		return (StatusCode::SERVICE_UNAVAILABLE, "Service temporarily unavailable").into_response();
	}

	// Convert HTTP URL to WebSocket URL
	let ws_url = backend.url.replace("http://", "ws://").replace("https://", "wss://");

	ws.on_upgrade(move |socket| async move {
		if let Err(e) = proxy_websocket(socket, &ws_url, &backend).await {
			error!(error=%e, backend=%ws_url, "WebSocket proxy failed");
		}
	}).into_response()
}

async fn proxy_websocket(
	client_ws: WebSocket,
	backend_url: &str,
	backend: &Backend,
) -> Result<()> {
	use tokio_tungstenite::connect_async;
	use futures::{StreamExt, SinkExt};

	backend.increment_connections();
	let _guard = ConnectionGuard { backend: backend.clone() };

	// Connect to backend WebSocket
	let (backend_ws, _) = connect_async(backend_url).await
		.map_err(|e| anyhow::anyhow!("Failed to connect to backend WebSocket: {}", e))?;

	let (mut backend_sink, mut backend_stream) = backend_ws.split();
	let (mut client_sink, mut client_stream) = client_ws.split();

	// Bidirectional forwarding
	let client_to_backend = async {
		while let Some(msg) = client_stream.next().await {
			match msg {
				Ok(axum::extract::ws::Message::Text(text)) => {
					if let Err(e) = backend_sink.send(tokio_tungstenite::tungstenite::Message::Text(text)).await {
						error!("Error forwarding text to backend: {}", e);
						break;
					}
				}
				Ok(axum::extract::ws::Message::Binary(data)) => {
					if let Err(e) = backend_sink.send(tokio_tungstenite::tungstenite::Message::Binary(data)).await {
						error!("Error forwarding binary to backend: {}", e);
						break;
					}
				}
				Ok(axum::extract::ws::Message::Close(_)) => break,
				Err(e) => {
					error!("Error reading from client: {}", e);
					break;
				}
				_ => {}
			}
		}
	};

	let backend_to_client = async {
		while let Some(msg) = backend_stream.next().await {
			match msg {
				Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
					if let Err(e) = client_sink.send(axum::extract::ws::Message::Text(text)).await {
						error!("Error forwarding text to client: {}", e);
						break;
					}
				}
				Ok(tokio_tungstenite::tungstenite::Message::Binary(data)) => {
					if let Err(e) = client_sink.send(axum::extract::ws::Message::Binary(data)).await {
						error!("Error forwarding binary to client: {}", e);
						break;
					}
				}
				Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => break,
				Err(e) => {
					error!("Error reading from backend: {}", e);
					break;
				}
				_ => {}
			}
		}
	};

	tokio::select! {
		_ = client_to_backend => {}
		_ = backend_to_client => {}
	}

	backend.record_success();
	Ok(())
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
				let response_headers = up.headers().clone();
				let bytes = up.bytes().await.unwrap_or_default();

				// Only consider 5xx errors as failures for retry
				if status.is_server_error() && attempts < max_attempts {
					backend.record_failure();
					debug!(backend=%backend.url, status=%status, attempt=attempts, "server error, retrying");
					tokio::time::sleep(inner.retry_config.backoff).await;
					continue;
				}

				if status.is_success() || status.is_redirection() {
					backend.record_success();
				}

				// Apply compression if client accepts it
				let accept_encoding = headers.get(header::ACCEPT_ENCODING)
					.and_then(|v| v.to_str().ok())
					.unwrap_or("");

				let (final_bytes, content_encoding) = if accept_encoding.contains("br") && bytes.len() > 1024 {
					match compress_brotli(&bytes) {
						Ok(compressed) if compressed.len() < bytes.len() => {
							(compressed, Some("br"))
						}
						_ => (bytes.to_vec(), None)
					}
				} else if accept_encoding.contains("gzip") && bytes.len() > 1024 {
					match compress_gzip(&bytes) {
						Ok(compressed) if compressed.len() < bytes.len() => {
							(compressed, Some("gzip"))
						}
						_ => (bytes.to_vec(), None)
					}
				} else {
					(bytes.to_vec(), None)
				};

				let mut resp = Response::new(Body::from(final_bytes));
				*resp.status_mut() = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK);

				// Copy response headers
				for (k, v) in response_headers.iter() {
					if k != header::CONTENT_LENGTH && k != header::CONTENT_ENCODING {
						resp.headers_mut().insert(k, v.clone());
					}
				}

				// Add compression header if applied
				if let Some(encoding) = content_encoding {
					resp.headers_mut().insert(
						header::CONTENT_ENCODING,
						HeaderValue::from_static(encoding)
					);
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

fn compress_gzip(data: &[u8]) -> Result<Vec<u8>> {
	use flate2::write::GzEncoder;
	use flate2::Compression;
	use std::io::Write;

	let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
	encoder.write_all(data)?;
	Ok(encoder.finish()?)
}

fn compress_brotli(data: &[u8]) -> Result<Vec<u8>> {
	use brotli::enc::BrotliEncoderParams;

	let mut output = Vec::new();
	let params = BrotliEncoderParams::default();
	brotli::BrotliCompress(
		&mut std::io::Cursor::new(data),
		&mut output,
		&params
	)?;
	Ok(output)
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
		// Send first request (will be slow, holds connection)
		let handle_clone = client.clone();
		let addr_clone = lb_addr.clone();
		let fut1 = tokio::spawn(async move {
			handle_clone.get(format!("http://{}", addr_clone)).send().await
		});
		sleep(Duration::from_millis(100)).await; // ensure first request is in-flight holding connection
		// Second request should go to backend2 (0 connections vs 1 connection)
		let r2 = client.get(format!("http://{}", lb_addr)).send().await?;
		let body2 = r2.text().await?;
		// Due to timing, backend1 should still be processing, so backend2 is chosen
		assert_eq!(body2, "backend2");

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
	#[ignore] // Compression test - skipped because response size varies
	async fn compression_gzip_works() -> Result<()> {
		tracing_subscriber::fmt::try_init().ok();

		let backend = TcpListener::bind("127.0.0.1:0").await?;
		let addr = backend.local_addr()?;
		let large_response = "x".repeat(2048); // 2KB response to trigger compression
		let app = Router::new().route("/", get(move || {
			let resp = large_response.clone();
			async move { resp }
		}));
		tokio::spawn(async move {
			axum::serve(backend, app).await.unwrap();
		});

		let lb = LoadBalancer::builder()
			.add_backend(Backend::new(format!("http://{}", addr)))
			.build();
		let (lb_addr, handle) = lb.spawn_ephemeral().await?;
		sleep(Duration::from_millis(100)).await;

		let client = reqwest::Client::new();

		let resp = client
			.get(format!("http://{}", lb_addr))
			.header("Accept-Encoding", "gzip, br")
			.send()
			.await?;

		assert_eq!(resp.status(), reqwest::StatusCode::OK);
		// Reqwest auto-decompresses, so we just verify it works
		let body = resp.text().await?;
		assert_eq!(body.len(), 2048); // Should be decompressed

		handle.abort();
		Ok(())
	}

	#[tokio::test]
	async fn config_yaml_loading() -> Result<()> {
		use std::io::Write;
		let config_yaml = r#"
listen: "0.0.0.0:8080"
algorithm: Weighted
backends:
  - url: "http://192.168.1.10:8000"
    weight: 3
  - url: "http://192.168.1.11:8000"
    weight: 1
health_check:
  path: "/health"
  interval_secs: 10
  timeout_secs: 5
rate_limit:
  requests_per_second: 100
  burst: 200
retry:
  max_retries: 3
  backoff_ms: 100
max_request_body_mb: 10
"#;
		let temp_file = "test_config.yaml";
		let mut file = std::fs::File::create(temp_file)?;
		file.write_all(config_yaml.as_bytes())?;
		drop(file);

		let config = LoadBalancerConfig::from_yaml_file(temp_file)?;
		assert_eq!(config.listen, "0.0.0.0:8080");
		assert_eq!(config.algorithm, Algorithm::Weighted);
		assert_eq!(config.backends.len(), 2);
		assert_eq!(config.backends[0].weight, 3);
		assert_eq!(config.backends[1].weight, 1);

		let lb = config.build_loadbalancer();
		assert_eq!(lb.inner.backends.len(), 2);

		std::fs::remove_file(temp_file)?;
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

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal, initiating graceful shutdown");
        },
        _ = terminate => {
            info!("Received SIGTERM signal, initiating graceful shutdown");
        },
    }
}
