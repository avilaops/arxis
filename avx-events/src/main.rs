use avx_config::AvxConfig;
use avx_telemetry::{self, AvxContext};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct AvxEvent {
    pub event_type: String,
    pub payload: String,
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

    info!("avx-events service started");

    // loop de consumo/produção (placeholder)
    loop {
        let evt = AvxEvent {
            event_type: "avx.deep.heartbeat".into(),
            payload: "ok".into(),
        };

        info!(
            event_type = %evt.event_type,
            payload = %evt.payload,
            "sending event (mock - awaiting Kafka/NATS integration)"
        );
        // TODO: integração real com Kafka/NATS/Rabbit/etc.

        sleep(Duration::from_secs(5)).await;
    }
}
