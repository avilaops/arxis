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
use hyper::{Body, Client, Request, Response};
use hyper::client::HttpConnector;
use hyper::http::uri::Authority;
use hyper::service::Service;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tower::util::BoxCloneService;
use tower::{Layer, ServiceBuilder};
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
	client: Client<HttpConnector>,
}

impl LoadBalancer {
	pub fn builder() -> LoadBalancerBuilder { LoadBalancerBuilder::new() }

	fn new(backends: Vec<Backend>, health_check: Option<HealthCheck>, algorithm: Algorithm) -> Self {
		let client = Client::new();
		Self { inner: Arc::new(Inner { backends, algorithm, rr_counter: AtomicUsize::new(0), _health_check: health_check, client }) }
	}

	/// Start listening on the provided address (e.g. `"127.0.0.1:8080"`).
	/// Blocks until server shutdown. Returns an error if binding or serving fails.
	pub async fn listen(&self, addr: &str) -> Result<()> {
		let listener = TcpListener::bind(addr).await?;
		let local_addr = listener.local_addr()?;
		info!(%local_addr, "load balancer listening");

		let svc = ProxyService { inner: self.inner.clone() };
		let make_svc = tower::service_fn(move |req: Request<Body>| {
			let mut svc_clone = svc.clone();
			async move { svc_clone.call(req).await }
		});

		// Hyper server
		let server = hyper::Server::from_tcp(listener.into_std()?)
			.tcp_nodelay(true)
			.serve(tower::make::Shared::new(make_svc));

		info!("server running");
		server.await?;
		Ok(())
	}

	/// Spawn the load balancer on an ephemeral port (useful for tests).
	pub async fn spawn_ephemeral(&self) -> Result<(SocketAddr, JoinHandle<Result<()>>)> {
		let listener = TcpListener::bind("127.0.0.1:0").await?;
		let addr = listener.local_addr()?;
		let inner = self.inner.clone();
		let handle = tokio::spawn(async move {
			let svc = ProxyService { inner };
			let make_svc = tower::service_fn(move |req: Request<Body>| {
				let mut svc_clone = svc.clone();
				async move { svc_clone.call(req).await }
			});
			hyper::Server::from_tcp(listener.into_std()?)
				.tcp_nodelay(true)
				.serve(tower::make::Shared::new(make_svc))
				.await?;
			Ok(())
		});
		Ok((addr, handle))
	}
}

#[derive(Clone)]
struct ProxyService {
	inner: Arc<Inner>,
}

impl Service<Request<Body>> for ProxyService {
	type Response = Response<Body>;
	type Error = hyper::Error;
	type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
		std::task::Poll::Ready(Ok(()))
	}

	fn call(&mut self, req: Request<Body>) -> Self::Future {
		let inner = self.inner.clone();
		let mut new_req = Request::builder()
			.method(req.method())
			.uri(rewrite_uri(&inner, req.uri()))
			.version(req.version());
		// Copy headers
		for (k, v) in req.headers().iter() { new_req = new_req.header(k, v); }
		let body = req.into_body();
		let final_req = match new_req.body(body) { Ok(r) => r, Err(e) => return Box::pin(async move { Err(hyper::Error::new(e)) }) };
		Box::pin(async move {
			let resp = inner.client.request(final_req).await;
			match resp {
				Ok(r) => Ok(r),
				Err(e) => {
					error!(error=%e, "proxy request failed");
					let mut r = Response::new(Body::from("Upstream error"));
					*r.status_mut() = hyper::StatusCode::BAD_GATEWAY;
					Ok(r)
				}
			}
		})
	}
}

fn rewrite_uri(inner: &Inner, incoming: &hyper::Uri) -> hyper::Uri {
	let backend = select_backend(inner);
	let base = &backend.url;
	let mut parts = incoming.clone().into_parts();
	// Parse backend base URL
	let parsed = base.parse::<hyper::Uri>().expect("backend url must be valid");
	let authority = parsed.authority().cloned();
	let scheme = parsed.scheme_str().map(|s| s.to_string());
	if let Some(auth) = authority { parts.authority = Some(auth); }
	if let Some(s) = scheme { parts.scheme = Some(hyper::http::uri::Scheme::from_static(Box::leak(s.into_boxed_str()))); }
	hyper::Uri::from_parts(parts).expect("failed to build uri")
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
		sleep(Duration::from_millis(50)).await;
		let client = Client::new();
		let uri: hyper::Uri = format!("http://{}", lb_addr).parse().unwrap();
		let resp = client.get(uri).await.unwrap();
		assert_eq!(resp.status(), hyper::StatusCode::OK);
		let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
		assert_eq!(&body_bytes[..], b"OK");
		handle.abort();
		Ok(())
	}
}

