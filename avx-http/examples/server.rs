//! Basic HTTP server example

use avx_http::{Server, Router};
use avx_http::server::Response;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 avx-http - Server Example\n");

    let router = Router::new()
        .get("/", || async {
            Response::text("Hello from AVL Platform! 🇧🇷")
        })
        .get("/health", || async {
            Response::json(&json!({
                "status": "healthy",
                "service": "avx-http-example",
                "region": "br-saopaulo-1"
            }))
        })
        .post("/echo", |req| async move {
            match req.text() {
                Ok(body) => Response::text(format!("Echo: {}", body)),
                Err(_) => Response::internal_error(),
            }
        });

    println!("🚀 Starting server on http://127.0.0.1:3000");
    println!("\nTry these endpoints:");
    println!("  GET  http://127.0.0.1:3000/");
    println!("  GET  http://127.0.0.1:3000/health");
    println!("  POST http://127.0.0.1:3000/echo -d 'Hello'\n");

    Server::bind("127.0.0.1:3000")
        .router(router)
        .telemetry(true)
        .run()
        .await?;

    Ok(())
}
