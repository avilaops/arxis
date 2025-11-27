//! Async HTTP/1.1 Server Example
//!
//! Demonstrates async TCP with reactor-based I/O

use avx_http::async_net::AsyncTcpListener;
use avx_http::runtime;
use std::time::Duration;

async fn handle_connection(mut stream: avx_http::async_net::AsyncTcpStream, id: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Connection {}] Accepted", id);

    // Read HTTP request (simplified)
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;

    if n == 0 {
        println!("[Connection {}] Client disconnected", id);
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buf[..n]);
    println!("[Connection {}] Request:\n{}", id, request.lines().next().unwrap_or(""));

    // Send HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Server: avx-http/0.4.0\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        RESPONSE_BODY.len(),
        RESPONSE_BODY
    );

    stream.write_all(response.as_bytes()).await?;
    println!("[Connection {}] Response sent", id);

    Ok(())
}

const RESPONSE_BODY: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>AVX-HTTP Async Server</title>
    <style>
        body {
            font-family: monospace;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: #0d1117;
            color: #c9d1d9;
        }
        h1 { color: #58a6ff; }
        .info { background: #161b22; padding: 15px; border-radius: 6px; }
        .success { color: #3fb950; }
    </style>
</head>
<body>
    <h1>🚀 AVX-HTTP Async Server</h1>
    <div class="info">
        <p class="success">✅ ZERO external dependencies!</p>
        <ul>
            <li>✅ Custom async runtime</li>
            <li>✅ epoll/kqueue/IOCP reactor</li>
            <li>✅ Hierarchical timer wheel</li>
            <li>✅ Non-blocking TCP</li>
            <li>✅ HTTP/1.1 + HTTP/2</li>
        </ul>
        <p>100% Pure Rust. Maximum Control. 🦀</p>
    </div>
</body>
</html>"#;

async fn server_loop() -> Result<(), Box<dyn std::error::Error>> {
    let listener = AsyncTcpListener::bind("127.0.0.1:8080")?;
    let addr = listener.local_addr()?;

    println!("🚀 AVX-HTTP Async Server listening on http://{}", addr);
    println!("📊 Press Ctrl+C to stop\n");

    let mut connection_id = 0;

    loop {
        // Accept connection
        match listener.accept().await {
            Ok((stream, peer_addr)) => {
                connection_id += 1;
                let id = connection_id;

                println!("[Connection {}] New from {}", id, peer_addr);

                // Spawn handler task
                runtime::spawn(async move {
                    if let Err(e) = handle_connection(stream, id).await {
                        eprintln!("[Connection {}] Error: {}", id, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
                runtime::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

fn main() {
    println!("AVX-HTTP Async Server Demo");
    println!("===========================\n");

    // Run server on runtime
    if let Err(e) = runtime::block_on(server_loop()) {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
