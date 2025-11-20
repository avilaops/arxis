use std::net::SocketAddr;

use avx_config::AvxConfig;
use avx_telemetry::{self, AvxContext};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request},
    response::Response,
    routing::get,
    Router,
};
use tower::{Layer, Service};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    ctx: AvxContext,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = AvxConfig::load().unwrap_or_else(|_| AvxConfig::with_defaults());

    let ctx = AvxContext {
        stack: cfg.stack.clone(),
        layer: cfg.layer.clone(),
        env: cfg.env.clone(),
        cluster: cfg.cluster.clone(),
        mesh: cfg.mesh.clone(),
    };

    avx_telemetry::init_tracing(&ctx);

    let state = AppState { ctx };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/deep/info", get(deep_info))
        .with_state(state.clone());

    // adiciona layer que injeta headers X-Avx-* e correlation-id
    let app = AvxHeaderLayer::new(state.ctx).layer(app);

    let addr: SocketAddr = cfg.http.bind_addr.parse()?;
    info!(%addr, "avx-gateway listening");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn deep_info(State(state): State<AppState>) -> axum::Json<AvxContext> {
    axum::Json(state.ctx.clone())
}

// ============= Layer p/ headers Avx ============= //

#[derive(Clone)]
struct AvxHeaderLayer {
    ctx: AvxContext,
}

impl AvxHeaderLayer {
    fn new(ctx: AvxContext) -> Self {
        Self { ctx }
    }
}

impl<S> Layer<S> for AvxHeaderLayer {
    type Service = AvxHeaderMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AvxHeaderMiddleware {
            inner,
            ctx: self.ctx.clone(),
        }
    }
}

#[derive(Clone)]
struct AvxHeaderMiddleware<S> {
    inner: S,
    ctx: AvxContext,
}

impl<S> Service<Request<Body>> for AvxHeaderMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<axum::BoxError> + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        // correlation id
        let trace_id = Uuid::new_v4().to_string();

        // carimbos Avx
        let headers = req.headers_mut();
        headers.insert(
            "x-avx-stack",
            HeaderValue::from_str(&self.ctx.stack).unwrap(),
        );
        headers.insert(
            "x-avx-layer",
            HeaderValue::from_str(&self.ctx.layer).unwrap(),
        );
        headers.insert("x-avx-env", HeaderValue::from_str(&self.ctx.env).unwrap());
        headers.insert(
            "x-avx-cluster",
            HeaderValue::from_str(&self.ctx.cluster).unwrap(),
        );
        headers.insert("x-avx-mesh", HeaderValue::from_str(&self.ctx.mesh).unwrap());
        headers.insert("x-avx-trace", HeaderValue::from_str(&trace_id).unwrap());

        tracing::info!(
            avx_trace = %trace_id,
            method = %req.method(),
            uri = %req.uri(),
            "incoming request"
        );

        self.inner.call(req)
    }
}
