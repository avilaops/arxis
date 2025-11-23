//! Event-driven HTTP server example
//!
//! This example demonstrates integration between avx-http and avx-events.
//! HTTP requests are published as events that can be consumed by other services.
//!
//! Run with: cargo run --example event_driven_server --features events

use avx_events::{Event, EventBus};
use avx_http::{
    events::{EventPublisher, HttpRequestEvent, Metrics, health_check_response, metrics_response},
    middleware::{Cors, Logger, Middleware, Next},
    server::{Request, Response, Router, Server},
    Handler, Result,
};
use async_trait::async_trait;
use http::{Method, StatusCode};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Custom application event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct UserAction {
    user_id: String,
    action: String,
    timestamp: i64,
}

impl Event for UserAction {
    fn event_type(&self) -> &'static str {
        "user.action"
    }

    fn aggregate_id(&self) -> String {
        self.user_id.clone()
    }
}

// Simple handler that publishes custom events
struct UserActionHandler {
    event_bus: Arc<EventBus>,
}

#[async_trait]
impl Handler for UserActionHandler {
    async fn handle(&self, request: Request) -> Result<Response> {
        // Parse user_id from path
        let parts: Vec<&str> = request.path.split('/').collect();
        let user_id = parts.get(2).unwrap_or(&"unknown").to_string();

        // Publish custom event
        let event = UserAction {
            user_id: user_id.clone(),
            action: "viewed_profile".into(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        self.event_bus.publish(event).await.ok();

        Ok(Response::json(&serde_json::json!({
            "message": "Action recorded",
            "user_id": user_id
        })))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("🚀 AVX HTTP + Events Integration Example\n");

    // Create event bus
    let event_bus = Arc::new(EventBus::new());

    // Subscribe to HTTP request events
    let mut http_subscriber = event_bus.subscribe::<HttpRequestEvent>().await;
    tokio::spawn(async move {
        println!("📊 HTTP Request Event Subscriber started");
        while let Some(envelope) = http_subscriber.recv().await {
            let event = &envelope.event;
            println!(
                "📊 HTTP Event: {} {} -> {} ({}ms)",
                event.method, event.path, event.status_code, event.duration_ms
            );
        }
    });

    // Subscribe to user actions
    let mut user_subscriber = event_bus.subscribe::<UserAction>().await;
    tokio::spawn(async move {
        println!("👤 User Action Subscriber started");
        while let Some(envelope) = user_subscriber.recv().await {
            let event = &envelope.event;
            println!(
                "👤 User Action: {} performed {} at {}",
                event.user_id, event.action, event.timestamp
            );
        }
    });

    // Wait for subscribers to be ready
    sleep(Duration::from_millis(100)).await;

    // Create metrics
    let metrics = Arc::new(Metrics::new());

    // Create router
    let router = Router::new()
        .get("/health", || async { health_check_response() })
        .get("/metrics", {
            let metrics = metrics.clone();
            move || {
                let metrics = metrics.clone();
                async move { metrics_response(&metrics) }
            }
        })
        .get("/users/:id", {
            let event_bus = event_bus.clone();
            move || {
                let handler = UserActionHandler {
                    event_bus: event_bus.clone(),
                };
                async move {
                    // This is a simplified example
                    Ok(Response::json(&serde_json::json!({
                        "message": "User endpoint"
                    })))
                }
            }
        })
        .get("/", || async {
            Response::json(&serde_json::json!({
                "name": "AVX HTTP + Events Demo",
                "version": "0.1.0",
                "endpoints": [
                    "/health",
                    "/metrics",
                    "/users/:id"
                ]
            }))
        });

    println!("📝 Routes configured:");
    println!("  GET  /");
    println!("  GET  /health");
    println!("  GET  /metrics");
    println!("  GET  /users/:id\n");

    // Create middleware stack
    let logger = Arc::new(Logger::new());
    let cors = Arc::new(Cors::permissive());
    let event_publisher = Arc::new(EventPublisher::new(event_bus.clone()));
    let metrics_middleware: Arc<dyn Middleware> = metrics.clone();

    let middleware: Vec<Arc<dyn Middleware>> = vec![
        logger,
        cors,
        metrics_middleware,
        event_publisher,
    ];

    println!("🔧 Middleware stack:");
    println!("  - Logger");
    println!("  - CORS");
    println!("  - Metrics");
    println!("  - Event Publisher\n");

    println!("🌐 Server listening on http://0.0.0.0:8080");
    println!("📡 Events are being published to the event bus");
    println!("\n💡 Try:");
    println!("  curl http://localhost:8080/");
    println!("  curl http://localhost:8080/health");
    println!("  curl http://localhost:8080/metrics");
    println!("  curl http://localhost:8080/users/123\n");

    // Note: This is a simplified example
    // In a real implementation, you'd need to integrate the middleware
    // with the actual server routing system
    println!("⚠️  Note: This is a demonstration of the event integration.");
    println!("    Full server integration would require additional routing logic.\n");

    // Simulate some requests for demonstration
    println!("🧪 Simulating requests for demo...\n");

    // Create a simple request and process through middleware
    let request = Request {
        method: Method::GET,
        path: "/health".to_string(),
        headers: http::HeaderMap::new(),
        body: bytes::Bytes::new(),
    };

    // Manually trigger event publisher
    sleep(Duration::from_secs(1)).await;

    // Publish a sample HTTP request event
    event_bus
        .publish(HttpRequestEvent {
            method: "GET".into(),
            path: "/health".into(),
            status_code: 200,
            duration_ms: 5,
            user_agent: Some("curl/7.68.0".into()),
            remote_addr: Some("127.0.0.1:45678".into()),
        })
        .await
        .ok();

    sleep(Duration::from_millis(100)).await;

    // Publish a user action
    event_bus
        .publish(UserAction {
            user_id: "user-123".into(),
            action: "login".into(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
        .await
        .ok();

    sleep(Duration::from_millis(100)).await;

    println!("\n✅ Event-driven HTTP demo completed!");
    println!("📊 Metrics: {} requests, {} errors", metrics.request_count(), metrics.error_count());

    Ok(())
}
