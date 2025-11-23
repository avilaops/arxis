use avx_config::AvxConfig;
use avx_events::{Event, EventBus};
use avx_telemetry::{self, AvxContext};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SystemEvent {
    pub event_type: String,
    pub message: String,
    pub timestamp: i64,
}

impl Event for SystemEvent {
    fn event_type(&self) -> &'static str {
        "avx.system.event"
    }

    fn aggregate_id(&self) -> String {
        "avx-events-system".to_string()
    }
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

    info!("🚀 AVX Events Service Started");
    info!("📦 Event-driven architecture for Avila Experience Fabric");

    // Create event bus
    let bus = EventBus::new();

    // Subscribe to system events
    let mut subscriber = bus.subscribe::<SystemEvent>().await;
    tokio::spawn(async move {
        while let Some(envelope) = subscriber.recv().await {
            info!(
                event_id = %envelope.metadata.event_id,
                event_type = %envelope.event.event_type,
                message = %envelope.event.message,
                "📨 Event received"
            );
        }
    });

    // Wait a bit for subscriber to be ready
    sleep(Duration::from_millis(100)).await;

    // Publish initial event
    bus.publish(SystemEvent {
        event_type: "system.started".into(),
        message: "AVX Events service is running".into(),
        timestamp: chrono::Utc::now().timestamp_millis(),
    })
    .await?;

    info!("✅ System ready. Publishing heartbeat events...");

    // Event loop
    let mut counter = 0;
    loop {
        sleep(Duration::from_secs(10)).await;

        counter += 1;
        let evt = SystemEvent {
            event_type: "system.heartbeat".into(),
            message: format!("Heartbeat #{}", counter),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        bus.publish(evt).await?;
    }
}
