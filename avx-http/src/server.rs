//! HTTP Server implementation

use crate::error::{Error, Result};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// HTTP Server
pub struct Server {
    addr: String,
    router: Option<Router>,
    compression: bool,
    telemetry: bool,
}

impl Server {
    /// Bind server to address
    pub fn bind(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            router: None,
            compression: false,
            telemetry: false,
        }
    }

    /// Set router for handling requests
    pub fn router(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }

    /// Enable response compression
    pub fn compression(mut self, enabled: bool) -> Self {
        self.compression = enabled;
        self
    }

    /// Enable telemetry logging
    pub fn telemetry(mut self, enabled: bool) -> Self {
        self.telemetry = enabled;
        self
    }

    /// Run the server
    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr)
            .await
            .map_err(|e| Error::Internal {
                message: format!("Failed to bind to {}: {}", self.addr, e),
            })?;

        println!("🚀 Server listening on {}", self.addr);

        let router = Arc::new(self.router.unwrap_or_default());

        loop {
            let (stream, peer_addr) = listener.accept().await.map_err(|e| Error::Internal {
                message: format!("Failed to accept connection: {}", e),
            })?;

            let router = Arc::clone(&router);
            let telemetry = self.telemetry;

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, router, telemetry).await {
                    eprintln!("Error handling connection from {}: {}", peer_addr, e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    router: Arc<Router>,
    telemetry: bool,
) -> Result<()> {
    // Read request
    let mut buffer = vec![0u8; 8192];
    let n = stream.read(&mut buffer).await?;
    let request_data = &buffer[..n];

    // Parse request
    let request = parse_request(request_data)?;

    if telemetry {
        println!("📥 {} {}", request.method, request.path);
    }

    // Route request
    let response = router.handle(request).await;

    // Send response
    let mut response_bytes = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status.as_u16(),
        response.status.canonical_reason().unwrap_or("Unknown")
    );

    // Add headers
    for (name, value) in response.headers.iter() {
        response_bytes.push_str(&format!("{}: {}\r\n", name, value.to_str().unwrap_or("")));
    }

    // Add content-length
    response_bytes.push_str(&format!("Content-Length: {}\r\n", response.body.len()));
    response_bytes.push_str("Connection: close\r\n");
    response_bytes.push_str("\r\n");

    stream.write_all(response_bytes.as_bytes()).await?;
    stream.write_all(&response.body).await?;
    stream.flush().await?;

    Ok(())
}

fn parse_request(data: &[u8]) -> Result<Request> {
    let request_str = String::from_utf8_lossy(data);
    let mut lines = request_str.lines();

    // Parse request line
    let request_line = lines.next().ok_or_else(|| Error::Internal {
        message: "Empty request".to_string(),
    })?;

    let mut parts = request_line.split_whitespace();
    let method_str = parts.next().ok_or_else(|| Error::Internal {
        message: "Missing method".to_string(),
    })?;
    let path = parts.next().ok_or_else(|| Error::Internal {
        message: "Missing path".to_string(),
    })?;

    let method = match method_str {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        _ => {
            return Err(Error::InvalidMethod {
                method: method_str.to_string(),
            })
        }
    };

    // Parse headers
    let mut headers = HeaderMap::new();
    let mut body_start = 0;

    for (i, line) in lines.clone().enumerate() {
        if line.is_empty() {
            body_start = request_str.lines().take(i + 2).collect::<Vec<_>>().join("\n").len();
            break;
        }

        if let Some(pos) = line.find(':') {
            let name = &line[..pos].trim();
            let value = &line[pos + 1..].trim();

            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                headers.insert(name, value);
            }
        }
    }

    // Extract body
    let body = if body_start < data.len() {
        Bytes::copy_from_slice(&data[body_start..])
    } else {
        Bytes::new()
    };

    Ok(Request {
        method,
        path: path.to_string(),
        headers,
        body,
    })
}

/// HTTP Request received by server
pub struct Request {
    /// HTTP method
    pub method: Method,
    /// Request path
    pub path: String,
    /// Request headers
    pub headers: HeaderMap,
    /// Request body
    pub body: Bytes,
}

impl Request {
    /// Get request body as text
    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.body.to_vec())
            .map_err(|e| Error::Internal { message: e.to_string() })
    }

    /// Parse request body as JSON
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_slice(&self.body)
            .map_err(|e| Error::JsonError { source: e.to_string() })
    }
}

/// HTTP Response to send to client
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl Response {
    /// Create new response
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    /// Create 200 OK response with text body
    pub fn text(body: impl Into<String>) -> Self {
        let body_str = body.into();
        let mut response = Self::new(StatusCode::OK);
        response.body = Bytes::from(body_str);
        response.headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }

    /// Create 200 OK response with JSON body
    pub fn json<T: serde::Serialize>(value: &T) -> Self {
        match serde_json::to_vec(value) {
            Ok(json_bytes) => {
                let mut response = Self::new(StatusCode::OK);
                response.body = Bytes::from(json_bytes);
                response.headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("application/json"),
                );
                response
            }
            Err(_) => Self::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// Create 404 Not Found response
    pub fn not_found() -> Self {
        Self::text("Not Found")
            .with_status(StatusCode::NOT_FOUND)
    }

    /// Create 500 Internal Server Error response
    pub fn internal_error() -> Self {
        Self::text("Internal Server Error")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Set response status
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Add header to response
    pub fn with_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Get response status
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get response headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get mutable reference to headers
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
}

type HandlerFn = Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;

/// Router for mapping paths to handlers
pub struct Router {
    /// Registered routes
    pub routes: HashMap<(Method, String), HandlerFn>,
}

impl Router {
    /// Create new router
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register GET route
    pub fn get<F, Fut>(mut self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler = Arc::new(move |_req: Request| {
            let fut = handler();
            Box::pin(fut) as Pin<Box<dyn Future<Output = Response> + Send>>
        });
        self.routes.insert((Method::GET, path.into()), handler);
        self
    }

    /// Register POST route
    pub fn post<F, Fut>(mut self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler = Arc::new(move |req: Request| {
            let fut = handler(req);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Response> + Send>>
        });
        self.routes.insert((Method::POST, path.into()), handler);
        self
    }

    /// Register PUT route
    pub fn put<F, Fut>(mut self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler = Arc::new(move |req: Request| {
            let fut = handler(req);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Response> + Send>>
        });
        self.routes.insert((Method::PUT, path.into()), handler);
        self
    }

    /// Register DELETE route
    pub fn delete<F, Fut>(mut self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler = Arc::new(move |req: Request| {
            let fut = handler(req);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Response> + Send>>
        });
        self.routes.insert((Method::DELETE, path.into()), handler);
        self
    }

    async fn handle(&self, request: Request) -> Response {
        let key = (request.method.clone(), request.path.clone());

        if let Some(handler) = self.routes.get(&key) {
            handler(request).await
        } else {
            Response::not_found()
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = Router::new();
        assert_eq!(router.routes.len(), 0);
    }

    #[test]
    fn test_response_text() {
        let response = Response::text("Hello, World!");
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.body, Bytes::from("Hello, World!"));
    }

    #[test]
    fn test_response_json() {
        let data = serde_json::json!({"message": "Hello"});
        let response = Response::json(&data);
        assert_eq!(response.status, StatusCode::OK);
        assert!(response.headers.contains_key("content-type"));
    }

    #[test]
    fn test_parse_request() {
        let request_data = b"GET /hello HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = parse_request(request_data).unwrap();

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path, "/hello");
    }
}
