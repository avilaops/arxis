//! HTTP/1.1 request/response example
//!
//! Demonstrates basic HTTP parsing

use avx_http::error::Result;
use avx_http::http::{Request, Response, Method, StatusCode};

fn main() -> Result<()> {
    println!("🚀 avx-http HTTP/1.1 Example");
    println!("============================\n");

    // Create request
    println!("✅ Building HTTP request:");
    let mut req = Request::new(Method::Get, "/api/users");
    req.headers.insert("Host", "api.example.com");
    req.headers.insert("User-Agent", "avx-http/0.4.0");
    req.headers.insert("Accept", "application/json");

    // Serialize to wire format
    let request_bytes = req.to_bytes();
    println!("Request ({} bytes):", request_bytes.len());
    println!("{}", String::from_utf8_lossy(&request_bytes));

    // Create response
    println!("\n✅ Building HTTP response:");
    let resp = Response::text("Hello from avx-http!");
    let response_bytes = resp.to_bytes();
    println!("Response ({} bytes):", response_bytes.len());
    println!("{}", String::from_utf8_lossy(&response_bytes));

    // Parse request
    println!("\n✅ Parsing HTTP request:");
    let raw_request = b"GET /api/data HTTP/1.1\r\nHost: api.avila.cloud\r\nUser-Agent: curl/7.68.0\r\n\r\n";
    let parsed_req = Request::parse(raw_request)?;
    println!("   Method: {}", parsed_req.method);
    println!("   Path: {}", parsed_req.path);
    println!("   Headers: {}", parsed_req.headers.len());

    // Parse response
    println!("\n✅ Parsing HTTP response:");
    let raw_response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";
    let parsed_resp = Response::parse(raw_response)?;
    println!("   Status: {}", parsed_resp.status);
    println!("   Body: {}", parsed_resp.body_str()?);

    println!("\n✨ Pure Rust, zero dependencies!");
    println!("   FSM-based parser, zero-copy headers");

    Ok(())
}
