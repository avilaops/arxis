//! Integration tests for the gateway

use axum::{routing::get, Router};
use std::net::SocketAddr;

/// Start a mock upstream server for testing
pub async fn start_mock_server(port: u16) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let app = Router::new()
            .route("/test", get(|| async { "test response" }))
            .route("/json", get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }))
            .route("/delay", get(|| async {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                "delayed response"
            }))
            .route("/error", get(|| async {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "error")
            }));

        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_server() {
        let _handle = start_mock_server(9001).await;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let response = reqwest::get("http://127.0.0.1:9001/test")
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, "test response");
    }
}
