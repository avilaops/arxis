//! Request/Reply pattern example.
//!
//! Run with: cargo run --example request_reply

use avx_events::{Event, RequestReplyBus};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

// Request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetUserRequest {
    user_id: String,
}

impl Event for GetUserRequest {
    fn event_type(&self) -> &'static str {
        "get_user.request"
    }
    fn aggregate_id(&self) -> String {
        self.user_id.clone()
    }
}

// Response
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetUserResponse {
    user_id: String,
    name: String,
    email: String,
}

// Another request type
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CalculateRequest {
    operation: String,
    a: i32,
    b: i32,
}

impl Event for CalculateRequest {
    fn event_type(&self) -> &'static str {
        "calculate.request"
    }
    fn aggregate_id(&self) -> String {
        "calculator".into()
    }
}

#[derive(Debug, Clone)]
struct CalculateResponse {
    result: i32,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("🔄 AVX Events - Request/Reply Example\n");

    let bus = RequestReplyBus::new();

    // User service responder
    tokio::spawn({
        let bus = bus.clone();
        async move {
            println!("👤 User service listening for requests...");
            let mut listener = bus.listen::<GetUserRequest, GetUserResponse>().await;

            while let Some((envelope, reply)) = listener.recv().await {
                println!(
                    "👤 Received request for user: {}",
                    envelope.event.user_id
                );

                // Simulate database lookup
                sleep(Duration::from_millis(50)).await;

                let response = GetUserResponse {
                    user_id: envelope.event.user_id.clone(),
                    name: "Alice Silva".into(),
                    email: "alice@example.com".into(),
                };

                reply.send(response).await.ok();
                println!("👤 Sent response");
            }
        }
    });

    // Calculator service responder
    tokio::spawn({
        let bus = bus.clone();
        async move {
            println!("🧮 Calculator service listening for requests...");
            let mut listener = bus.listen::<CalculateRequest, CalculateResponse>().await;

            while let Some((envelope, reply)) = listener.recv().await {
                println!(
                    "🧮 Received calculation: {} {} {}",
                    envelope.event.a, envelope.event.operation, envelope.event.b
                );

                let result = match envelope.event.operation.as_str() {
                    "add" => envelope.event.a + envelope.event.b,
                    "subtract" => envelope.event.a - envelope.event.b,
                    "multiply" => envelope.event.a * envelope.event.b,
                    "divide" => envelope.event.a / envelope.event.b,
                    _ => 0,
                };

                reply.send(CalculateResponse { result }).await.ok();
                println!("🧮 Sent result: {}", result);
            }
        }
    });

    // Wait for services to start
    sleep(Duration::from_millis(100)).await;

    println!("\n--- Sending Requests ---\n");

    // Request 1: Get user
    println!("📤 Requesting user info...");
    let response = bus
        .request::<GetUserRequest, GetUserResponse>(GetUserRequest {
            user_id: "user-123".into(),
        })
        .await
        .unwrap();

    println!(
        "📥 Response: {} - {} ({})\n",
        response.name, response.email, response.user_id
    );

    // Request 2: Calculate
    println!("📤 Requesting calculation: 10 + 5");
    let response = bus
        .request::<CalculateRequest, CalculateResponse>(CalculateRequest {
            operation: "add".into(),
            a: 10,
            b: 5,
        })
        .await
        .unwrap();

    println!("📥 Response: {}\n", response.result);

    // Request 3: Another calculation
    println!("📤 Requesting calculation: 20 * 3");
    let response = bus
        .request::<CalculateRequest, CalculateResponse>(CalculateRequest {
            operation: "multiply".into(),
            a: 20,
            b: 3,
        })
        .await
        .unwrap();

    println!("📥 Response: {}\n", response.result);

    println!("✅ Request/Reply example completed!");
}
