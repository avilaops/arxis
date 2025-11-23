//! Gateway core implementation

use crate::{
    circuit_breaker::CircuitBreaker,
    config::{GatewayConfig, RouteConfig as ConfigRoute, UpstreamConfig},
    error::{GatewayError, Result},
    health::{health_handler, liveness_handler, readiness_handler, HealthChecker},
    load_balancer::{LoadBalancer, Strategy},
    metrics::GatewayMetrics,
    middleware::{LoggingLayer, RateLimitLayer, TimeoutLayer},
    routing::{Route, Router, Upstream},
};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router as AxumRouter,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tower::ServiceBuilder;
use tracing::{error, info, warn};

/// Main gateway struct
pub struct Gateway {
    config: GatewayConfig,
    router: Router,
    health_checker: HealthChecker,
    metrics: GatewayMetrics,
    circuit_breakers: Arc<HashMap<String, CircuitBreaker>>,
}

impl Gateway {
    /// Create a new gateway builder
    pub fn builder() -> GatewayBuilder {
        GatewayBuilder::new()
    }

    /// Create a gateway from configuration
    pub async fn from_config(config: GatewayConfig) -> Result<Self> {
        let mut builder = GatewayBuilder::new();

        // Set server configuration
        builder = builder
            .with_host(&config.server.host)
            .with_port(config.server.port);

        // Add routes
        for route_config in &config.routes {
            let route = Self::route_from_config(route_config)?;
            builder.routes.push(route);
        }

        // Set middleware
        if config.middleware.enable_rate_limiting {
            if let Some(rate_limit) = &config.rate_limiting {
                builder = builder.with_rate_limit(rate_limit.requests_per_second);
            }
        }

        builder.build().await
    }

    /// Convert config route to internal route
    fn route_from_config(config: &ConfigRoute) -> Result<Route> {
        let upstream = match &config.upstream {
            UpstreamConfig::Single(url) => Upstream::Single(url.clone()),
            UpstreamConfig::Multiple { urls, strategy } => {
                let lb_strategy = match strategy {
                    crate::config::LoadBalancingStrategy::RoundRobin => Strategy::RoundRobin,
                    crate::config::LoadBalancingStrategy::LeastConnections => {
                        Strategy::LeastConnections
                    }
                    crate::config::LoadBalancingStrategy::Weighted => Strategy::Weighted,
                    crate::config::LoadBalancingStrategy::Random => Strategy::Random,
                };

                Upstream::LoadBalanced {
                    urls: urls.clone(),
                    strategy: lb_strategy,
                    current_index: 0,
                }
            }
        };

        Ok(Route {
            path: config.path.clone(),
            upstream,
            methods: config.methods.clone(),
            strip_path: config.strip_path,
            timeout_ms: config.timeout_ms,
            auth_required: config.auth_required,
        })
    }

    /// Start the gateway server
    pub async fn serve(self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.server.host, self.config.server.port)
            .parse()
            .map_err(|e| GatewayError::Config(format!("Invalid address: {}", e)))?;

        info!(
            address = %addr,
            routes = self.router.upstream_count(),
            "Starting AVX Gateway"
        );

        // Create shared state
        let shared_state = Arc::new(GatewayState {
            router: self.router,
            health_checker: self.health_checker.clone(),
            metrics: self.metrics.clone(),
            circuit_breakers: self.circuit_breakers,
            http_client: reqwest::Client::new(),
        });

        // Build the application router
        let app = AxumRouter::new()
            // Health endpoints
            .route("/health", get(health_handler))
            .route("/healthz", get(liveness_handler))
            .route("/ready", get(readiness_handler))
            // Proxy all other requests
            .fallback(proxy_handler)
            .with_state(shared_state.clone())
            // Metrics in separate router with same state
            .route("/metrics", get(metrics_handler))
            .layer(ServiceBuilder::new().layer(LoggingLayer::new()));

        // Start server
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| GatewayError::Io(e))?;

        info!("Gateway ready to accept connections");

        axum::serve(listener, app)
            .await
            .map_err(|e| GatewayError::Other(e.to_string()))?;

        Ok(())
    }
}

/// Shared gateway state
struct GatewayState {
    router: Router,
    health_checker: HealthChecker,
    metrics: GatewayMetrics,
    circuit_breakers: Arc<HashMap<String, CircuitBreaker>>,
    http_client: reqwest::Client,
}

/// Proxy handler - forwards requests to upstream services
#[axum::debug_handler]
async fn proxy_handler(
    State(state): State<Arc<GatewayState>>,
    mut req: Request,
) -> Response {
    let start = Instant::now();
    state.metrics.increment_connections();

    // Match route
    let route = match state.router.match_route(&req) {
        Ok(route) => route,
        Err(e) => {
            state.metrics.decrement_connections();
            state.metrics.record_request(404, start.elapsed().as_millis() as u64);
            return (StatusCode::NOT_FOUND, e.to_string()).into_response();
        }
    };

    // Get upstream URL
    let upstream_url = state.router.get_upstream_url(&route);

    // Check circuit breaker
    if let Some(cb) = state.circuit_breakers.get(&upstream_url) {
        if !cb.allow_request().await {
            state.metrics.decrement_connections();
            state.metrics.record_request(503, start.elapsed().as_millis() as u64);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service temporarily unavailable (circuit breaker open)",
            )
                .into_response();
        }
    }

    // Process path
    let original_path = req.uri().path();
    let processed_path = state.router.process_path(&route, original_path);
    let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();

    // Build target URL
    let target_url = format!("{}{}{}", upstream_url, processed_path, query);

    info!(
        method = %req.method(),
        original_path = %original_path,
        target = %target_url,
        "Proxying request"
    );

    // Forward request
    let method = req.method().clone();
    let headers = req.headers().clone();
    let body = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .unwrap_or_default();

    // Build upstream request
    let upstream_req = match state.http_client
        .request(method.clone(), &target_url)
        .headers(headers)
        .body(body.to_vec())
        .build()
    {
        Ok(req) => req,
        Err(e) => {
            error!(error = %e, "Failed to build upstream request");
            state.metrics.decrement_connections();
            state.metrics.record_request(500, start.elapsed().as_millis() as u64);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build request")
                .into_response();
        }
    };

    // Send request to upstream
    let upstream_response = match state.http_client.execute(upstream_req).await {
        Ok(resp) => resp,
        Err(e) => {
            error!(error = %e, target = %target_url, "Upstream request failed");

            // Record failure in circuit breaker
            if let Some(cb) = state.circuit_breakers.get(&upstream_url) {
                cb.record_failure().await;
            }

            state.metrics.decrement_connections();
            state.metrics.record_request(502, start.elapsed().as_millis() as u64);
            return (StatusCode::BAD_GATEWAY, "Upstream service unavailable").into_response();
        }
    };

    // Record success in circuit breaker
    if let Some(cb) = state.circuit_breakers.get(&upstream_url) {
        cb.record_success().await;
    }

    // Build response
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    let body = match upstream_response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!(error = %e, "Failed to read upstream response body");
            state.metrics.decrement_connections();
            state.metrics.record_request(502, start.elapsed().as_millis() as u64);
            return (StatusCode::BAD_GATEWAY, "Failed to read response").into_response();
        }
    };

    // Record metrics
    let duration = start.elapsed().as_millis() as u64;
    state.metrics.record_request(status.as_u16(), duration);
    state.metrics.record_bytes_sent(body.len() as u64);
    state.metrics.decrement_connections();

    // Build and return response
    let mut response = Response::builder().status(status);

    for (key, value) in headers.iter() {
        response = response.header(key, value);
    }

    response.body(Body::from(body)).unwrap_or_else(|e| {
        error!(error = %e, "Failed to build response");
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
    })
}

/// Metrics endpoint handler
#[axum::debug_handler]
async fn metrics_handler(State(state): State<Arc<GatewayState>>) -> impl IntoResponse {
    let snapshot = state.metrics.snapshot();
    (StatusCode::OK, axum::Json(snapshot))
}

/// Gateway builder
pub struct GatewayBuilder {
    host: String,
    port: u16,
    routes: Vec<Route>,
    rate_limit: Option<u32>,
    enable_cors: bool,
    timeout: Option<Duration>,
}

impl GatewayBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            routes: Vec::new(),
            rate_limit: None,
            enable_cors: false,
            timeout: None,
        }
    }

    /// Set the host
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Set the port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Add a route
    pub fn route(mut self, path: impl Into<String>, upstream: impl Into<String>) -> Self {
        self.routes.push(Route::new(path, upstream));
        self
    }

    /// Enable rate limiting
    pub fn with_rate_limit(mut self, requests_per_second: u32) -> Self {
        self.rate_limit = Some(requests_per_second);
        self
    }

    /// Enable CORS
    pub fn with_cors(mut self, enable: bool) -> Self {
        self.enable_cors = enable;
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the gateway
    pub async fn build(self) -> Result<Gateway> {
        if self.routes.is_empty() {
            return Err(GatewayError::Config(
                "At least one route is required".to_string(),
            ));
        }

        let config = GatewayConfig {
            server: crate::config::ServerConfig {
                host: self.host.clone(),
                port: self.port,
                workers: num_cpus::get(),
                timeout_ms: self.timeout.map(|d| d.as_millis() as u64).unwrap_or(30000),
            },
            routes: Vec::new(),
            middleware: crate::config::MiddlewareConfig {
                enable_cors: self.enable_cors,
                enable_compression: false,
                enable_rate_limiting: self.rate_limit.is_some(),
                enable_logging: true,
                enable_metrics: true,
            },
            rate_limiting: self.rate_limit.map(|rps| crate::config::RateLimitConfig {
                requests_per_second: rps,
                burst_size: rps * 2,
            }),
            health_check: crate::config::HealthCheckConfig::default(),
            tls: None,
        };

        let router = Router::new(self.routes.clone());
        let health_checker = HealthChecker::new();
        let metrics = GatewayMetrics::new();

        // Create circuit breakers for each upstream
        let mut circuit_breakers = HashMap::new();
        for route in &self.routes {
            match &route.upstream {
                Upstream::Single(url) => {
                    circuit_breakers.insert(url.clone(), CircuitBreaker::new());
                    health_checker.register_upstream(url.clone()).await;
                }
                Upstream::LoadBalanced { urls, .. } => {
                    for url in urls {
                        circuit_breakers.insert(url.clone(), CircuitBreaker::new());
                        health_checker.register_upstream(url.clone()).await;
                    }
                }
            }
        }

        Ok(Gateway {
            config,
            router,
            health_checker,
            metrics,
            circuit_breakers: Arc::new(circuit_breakers),
        })
    }
}

impl Default for GatewayBuilder {
    fn default() -> Self {
        Self::new()
    }
}
