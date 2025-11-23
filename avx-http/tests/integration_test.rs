//! Integration tests for avx-http client

use avx_http::{Client, Server, Router};
use avx_http::server::Response;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_client_server_integration() {
    // Start server in background
    let server_handle = tokio::spawn(async {
        let router = Router::new()
            .get("/test", || async {
                Response::text("Test response")
            });

        Server::bind("127.0.0.1:8888")
            .router(router)
            .run()
            .await
            .unwrap();
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;

    // Make client request
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8888/test")
        .send()
        .await
        .unwrap();

    assert!(response.is_success());
    let text = response.text().await.unwrap();
    assert_eq!(text, "Test response");

    // Cleanup
    server_handle.abort();
}

#[tokio::test]
async fn test_client_request_methods() {
    let client = Client::new();

    // Test request builders exist
    let get = client.get("http://example.com");
    assert_eq!(get.method, avx_http::Method::GET);

    let post = client.post("http://example.com");
    assert_eq!(post.method, avx_http::Method::POST);

    let put = client.put("http://example.com");
    assert_eq!(put.method, avx_http::Method::PUT);

    let delete = client.delete("http://example.com");
    assert_eq!(delete.method, avx_http::Method::DELETE);
}

#[tokio::test]
async fn test_client_with_auth() {
    let client = Client::builder()
        .avl_auth("test-token-123")
        .build()
        .unwrap();

    assert_eq!(client.config.avl_auth, Some("test-token-123".to_string()));
}

#[tokio::test]
async fn test_client_with_region() {
    let client = Client::builder()
        .region("br-saopaulo-1")
        .build()
        .unwrap();

    assert_eq!(client.config.region, Some("br-saopaulo-1".to_string()));
}

#[tokio::test]
async fn test_router_routing() {
    let router = Router::new()
        .get("/", || async { Response::text("home") })
        .get("/about", || async { Response::text("about") })
        .post("/data", |_| async { Response::text("posted") });

    assert_eq!(router.routes.len(), 3);
}
