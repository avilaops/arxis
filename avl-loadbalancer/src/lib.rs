//! AVL LoadBalancer Library
//!
//! Minimal initial implementation of the public API sketched in the README.
//! This provides:
//! * `LoadBalancer` with a builder
//! * `Backend` representation
//! * `HealthCheck` (HTTP path variant only)
//! * `Algorithm` enum (RoundRobin implemented, others stubbed)
//! * Basic reverse proxying of HTTP requests using Hyper
//!
//! NOTE: This is an MVP: algorithms other than RoundRobin fall back to
//! RoundRobin logic. Health checks are passive (no background tasks yet).
//! Future work: active health probes, rate limiting, geo routing, WebSockets.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use anyhow::Result;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

/// Load balancing algorithm.
#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
	RoundRobin,
	LeastConnections,
	IpHash,
	Weighted,
}

/// Health check configuration (currently only HTTP path).
#[derive(Debug, Clone)]
pub enum HealthCheck {
	Http(String),
}

/// Backend server definition.
#[derive(Debug, Clone)]
pub struct Backend {
	pub url: String,
	// Future: weight, observed latency, connection count, health state, etc.
}

impl Backend {
	pub fn new<U: Into<String>>(url: U) -> Self { Self { url: url.into() } }
}

/// Builder for `LoadBalancer`.
pub struct LoadBalancerBuilder {
	backends: Vec<Backend>,
	health_check: Option<HealthCheck>,
	algorithm: Algorithm,
}

impl LoadBalancerBuilder {
	fn new() -> Self {
		Self { backends: Vec::new(), health_check: None, algorithm: Algorithm::RoundRobin }
	}

	pub fn add_backend(mut self, backend: Backend) -> Self {
		self.backends.push(backend);
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

	pub fn build(self) -> LoadBalancer { LoadBalancer::new(self.backends, self.health_check, self.algorithm) }
}

/// Core load balancer type.
pub struct LoadBalancer {
	inner: Arc<Inner>,
}

struct Inner {
	backends: Vec<Backend>,
	algorithm: Algorithm,
	rr_counter: AtomicUsize,
	_health_check: Option<HealthCheck>, // reserved for future active probing
	client: reqwest::Client,
}

impl LoadBalancer {
	pub fn builder() -> LoadBalancerBuilder { LoadBalancerBuilder::new() }

	fn new(backends: Vec<Backend>, health_check: Option<HealthCheck>, algorithm: Algorithm) -> Self {
		let client = reqwest::Client::new();
		Self { inner: Arc::new(Inner { backends, algorithm, rr_counter: AtomicUsize::new(0), _health_check: health_check, client }) }
	}

	/// Start listening on the provided address (e.g. `"127.0.0.1:8080"`).
	/// Blocks until server shutdown. Returns an error if binding or serving fails.
	pub async fn listen(&self, addr: &str) -> Result<()> {
		let listener = TcpListener::bind(addr).await?;
		let local_addr = listener.local_addr()?;
		info!(%local_addr, "load balancer listening");
		let app = self.router();
		axum::serve(listener, app).await?;
		Ok(())
	}

	/// Spawn the load balancer on an ephemeral port (useful for tests).
	pub async fn spawn_ephemeral(&self) -> Result<(SocketAddr, JoinHandle<Result<()>>)> {
		let listener = TcpListener::bind("127.0.0.1:0").await?;
		let addr = listener.local_addr()?;
		let app = self.router();
		let handle = tokio::spawn(async move {
			axum::serve(listener, app).await?;
			Ok(())
		});
		Ok((addr, handle))
	}

	fn router(&self) -> Router {
		let inner = self.inner.clone();
		Router::new()
			.fallback(any(proxy_handler))
			.with_state(inner)
	}
}

async fn proxy_handler(State(inner): State<Arc<Inner>>, req: Request<Body>) -> impl IntoResponse {
	let backend = select_backend(&inner);
	let path_and_query = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
	let target = format!("{}{}", backend.url.trim_end_matches('/'), path_and_query);
	debug!(%target, "proxying request");

	let mut builder = inner.client.request(req.method().clone(), &target);
	// headers
	for (k, v) in req.headers().iter() { builder = builder.header(k, v); }
	let body_bytes = axum::body::to_bytes(req.into_body(), 1 * 1024 * 1024).await.unwrap_or_default();
	let upstream = builder.body(body_bytes.to_vec()).send().await;
	match upstream {
		Ok(up) => {
			let status = up.status();
			let headers = up.headers().clone();
			let bytes = up.bytes().await.unwrap_or_default();
			let mut resp = Response::new(Body::from(bytes));
			*resp.status_mut() = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK);
			for (k, v) in headers.iter() { resp.headers_mut().insert(k, v.clone()); }
			resp
		}
		Err(e) => {
			error!(error=%e, "upstream request failed");
			(StatusCode::BAD_GATEWAY, "Bad Gateway").into_response()
		}
	}
}

fn select_backend(inner: &Inner) -> &Backend {
	if inner.backends.is_empty() { panic!("no backends configured"); }
	match inner.algorithm {
		Algorithm::RoundRobin | Algorithm::LeastConnections | Algorithm::IpHash | Algorithm::Weighted => {
			// For MVP all algorithms share RR until implemented.
			let idx = inner.rr_counter.fetch_add(1, Ordering::Relaxed) % inner.backends.len();
			&inner.backends[idx]
		}
	}
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
}

