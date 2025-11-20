use serde::Serialize;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug, Clone, Serialize)]
pub struct AvxContext {
    pub stack: String,
    pub layer: String,
    pub env: String,
    pub cluster: String,
    pub mesh: String,
}

pub fn init_tracing(ctx: &AvxContext) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_target(true)
        .flatten_event(true)
        .init();

    tracing::info!(
        stack = %ctx.stack,
        layer = %ctx.layer,
        env   = %ctx.env,
        cluster = %ctx.cluster,
        mesh = %ctx.mesh,
        "Avx telemetry initialized"
    );
}
