//! HTTP Client implementation

use crate::error::{Error, Result};
use crate::common;
use crate::pool::{ConnectionPool, PoolConfig};
use crate::interceptors::{Interceptors, RequestData, ResponseData};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// HTTP Client for making requests
#[derive(Clone)]
pub struct Client {
    /// Client configuration
    pub config: ClientConfig,
    /// Connection pool
    pool: Arc<ConnectionPool>,
    /// Request and response interceptors
    interceptors: Arc<Interceptors>,
}

/// Client configuration
#[derive(Clone)]
pub struct ClientConfig {
    /// Request timeout
    pub timeout: Duration,
    /// Default headers
    pub default_headers: HeaderMap,
    /// AVL Platform auth token
    pub avl_auth: Option<String>,
    /// Preferred region
    pub region: Option<String>,
    /// Enable compression
    pub compression: bool,
    /// Maximum redirects
    pub max_redirects: usize,
    /// Connection pool configuration
    pub pool_config: PoolConfig,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: common::DEFAULT_TIMEOUT,
            default_headers: HeaderMap::new(),
            avl_auth: None,
            region: None,
            compression: false,
            max_redirects: 5,
            pool_config: PoolConfig::default(),
        }
    }
}

impl Client {
    /// Create a new client with default configuration
    pub fn new() -> Self {
        let config = ClientConfig::default();
        let pool = Arc::new(ConnectionPool::with_config(config.pool_config.clone()));
        Self {
            config,
            pool,
            interceptors: Arc::new(Interceptors::new()),
        }
    }

    /// Create a client builder for custom configuration
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Make a GET request
    pub fn get(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    /// Make a POST request
    pub fn post(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    /// Make a PUT request
    pub fn put(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    /// Make a DELETE request
    pub fn delete(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }

    /// Make a PATCH request
    pub fn patch(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }

    /// Make a HEAD request
    pub fn head(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::HEAD, url)
    }

    /// Create a request with custom method
    pub fn request(&self, method: Method, url: impl Into<String>) -> RequestBuilder {
        RequestBuilder {
            client: self.clone(),
            method,
            url: url.into(),
            headers: self.config.default_headers.clone(),
            body: None,
            query_params: Vec::new(),
            timeout: Some(self.config.timeout),
        }
    }

    /// Get connection pool statistics
    pub async fn pool_stats(&self) -> crate::pool::PoolStats {
        self.pool.stats().await
    }

    /// Clean up expired connections from the pool
    pub async fn cleanup_pool(&self) {
        self.pool.cleanup_expired().await
    }

    /// Add a request interceptor
    ///
    /// The interceptor will be called before each request is sent
    pub fn on_request<F>(&mut self, interceptor: F)
    where
        F: Fn(&mut RequestData) + Send + Sync + 'static,
    {
        Arc::get_mut(&mut self.interceptors)
            .expect("Cannot modify interceptors while client is cloned")
            .add_request(interceptor);
    }

    /// Add a response interceptor
    ///
    /// The interceptor will be called after each response is received
    pub fn on_response<F>(&mut self, interceptor: F)
    where
        F: Fn(&ResponseData) + Send + Sync + 'static,
    {
        Arc::get_mut(&mut self.interceptors)
            .expect("Cannot modify interceptors while client is cloned")
            .add_response(interceptor);
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for configuring HTTP client
pub struct ClientBuilder {
    config: ClientConfig,
    interceptors: Interceptors,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
            interceptors: Interceptors::new(),
        }
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Add a default header for all requests
    pub fn default_header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        let name = HeaderName::from_bytes(name.as_ref().as_bytes())
            .map_err(|_| Error::InvalidHeader {
                name: name.as_ref().to_string(),
                value: value.as_ref().to_string(),
            })?;
        let value = HeaderValue::from_str(value.as_ref())
            .map_err(|_| Error::InvalidHeader {
                name: name.to_string(),
                value: value.as_ref().to_string(),
            })?;
        self.config.default_headers.insert(name, value);
        Ok(self)
    }

    /// Set AVL Platform authentication token
    pub fn avl_auth(mut self, token: impl Into<String>) -> Self {
        self.config.avl_auth = Some(token.into());
        self
    }

    /// Set preferred region for regional routing
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.config.region = Some(region.into());
        self
    }

    /// Enable automatic compression
    pub fn compression(mut self, enabled: bool) -> Self {
        self.config.compression = enabled;
        self
    }

    /// Set maximum number of redirects to follow
    pub fn max_redirects(mut self, max: usize) -> Self {
        self.config.max_redirects = max;
        self
    }

    /// Set connection pool max connections per host
    pub fn pool_max_connections(mut self, max: usize) -> Self {
        self.config.pool_config.max_connections_per_host = max;
        self
    }

    /// Set connection pool idle timeout
    pub fn pool_idle_timeout(mut self, timeout: Duration) -> Self {
        self.config.pool_config.idle_timeout = timeout;
        self
    }

    /// Set connection pool connection timeout
    pub fn pool_connection_timeout(mut self, timeout: Duration) -> Self {
        self.config.pool_config.connection_timeout = timeout;
        self
    }

    /// Enable or disable connection keep-alive
    pub fn pool_keep_alive(mut self, enabled: bool) -> Self {
        self.config.pool_config.keep_alive = enabled;
        self
    }

    /// Build the client
    pub fn build(self) -> Result<Client> {
        let pool = Arc::new(ConnectionPool::with_config(self.config.pool_config.clone()));
        Ok(Client {
            config: self.config,
            pool,
            interceptors: Arc::new(self.interceptors),
        })
    }

    /// Add a request interceptor
    pub fn on_request<F>(mut self, interceptor: F) -> Self
    where
        F: Fn(&mut RequestData) + Send + Sync + 'static,
    {
        self.interceptors.add_request(interceptor);
        self
    }

    /// Add a response interceptor
    pub fn on_response<F>(mut self, interceptor: F) -> Self
    where
        F: Fn(&ResponseData) + Send + Sync + 'static,
    {
        self.interceptors.add_response(interceptor);
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing HTTP requests
pub struct RequestBuilder {
    client: Client,
    /// HTTP method
    pub method: Method,
    url: String,
    headers: HeaderMap,
    body: Option<Bytes>,
    query_params: Vec<(String, String)>,
    timeout: Option<Duration>,
}

impl RequestBuilder {
    /// Add a header to the request
    pub fn header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        let name = HeaderName::from_bytes(name.as_ref().as_bytes())
            .map_err(|_| Error::InvalidHeader {
                name: name.as_ref().to_string(),
                value: value.as_ref().to_string(),
            })?;
        let value = HeaderValue::from_str(value.as_ref())
            .map_err(|_| Error::InvalidHeader {
                name: name.to_string(),
                value: value.as_ref().to_string(),
            })?;
        self.headers.insert(name, value);
        Ok(self)
    }

    /// Set request body
    pub fn body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set JSON body
    pub fn json<T: serde::Serialize>(mut self, json: &T) -> Result<Self> {
        let json_str = serde_json::to_string(json)
            .map_err(|e| Error::JsonError { source: e.to_string() })?;
        self.body = Some(Bytes::from(json_str));
        self = self.header("Content-Type", "application/json")?;
        Ok(self)
    }

    /// Add query parameter
    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Send the request
    pub async fn send(mut self) -> Result<Response> {
        let start_time = Instant::now();

        // Build full URL with query params
        let mut full_url = self.url.clone();
        if !self.query_params.is_empty() {
            let query_string = self.query_params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            full_url = format!("{}?{}", full_url, query_string);
        }

        // Apply request interceptors
        let mut request_data = RequestData::new(
            self.method.clone(),
            full_url.clone(),
            self.headers.clone(),
            self.body.clone(),
        );
        self.client.interceptors.apply_request(&mut request_data);

        // Update request with interceptor changes
        self.headers = request_data.headers;
        self.body = request_data.body;

        // Parse URL
        let (host, port, _is_https) = common::parse_url(&full_url)?;

        // Get connection from pool
        let mut stream = self.client.pool.get_connection(&host, port).await?;

        // Build HTTP request
        let path = full_url
            .find("://")
            .and_then(|pos| full_url[pos + 3..].find('/'))
            .map(|pos| &full_url[full_url.find("://").unwrap() + 3 + pos..])
            .unwrap_or("/");

        let mut request = format!("{} {} HTTP/1.1\r\n", self.method, path);
        request.push_str(&format!("Host: {}\r\n", host));
        request.push_str("Connection: keep-alive\r\n");

        // Add headers
        for (name, value) in self.headers.iter() {
            request.push_str(&format!("{}: {}\r\n", name, value.to_str().unwrap_or("")));
        }

        // Add AVL auth if configured
        if let Some(auth) = &self.client.config.avl_auth {
            request.push_str(&format!("Authorization: Bearer {}\r\n", auth));
        }

        // Add body
        if let Some(body) = &self.body {
            request.push_str(&format!("Content-Length: {}\r\n", body.len()));
            request.push_str("\r\n");
        } else {
            request.push_str("\r\n");
        }

        // Send request
        stream.write_all(request.as_bytes()).await?;
        if let Some(body) = &self.body {
            stream.write_all(body).await?;
        }

        // Read response with connection pooling support
        let response = read_response_with_pool(&mut stream).await?;

        // Return connection to pool for reuse
        self.client.pool.return_connection(&host, port, stream).await;

        // Apply response interceptors
        let duration_ms = start_time.elapsed().as_millis() as u64;
        let response_data = ResponseData::new(
            response.status.as_u16(),
            response.headers.clone(),
            response.body.len(),
            duration_ms,
        );
        self.client.interceptors.apply_response(&response_data);

        Ok(response)
    }
}

/// HTTP Response
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl Response {
    /// Get response status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get response headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get response body as bytes
    pub fn bytes(&self) -> &Bytes {
        &self.body
    }

    /// Get response body as text
    pub async fn text(self) -> Result<String> {
        String::from_utf8(self.body.to_vec())
            .map_err(|e| Error::Internal { message: e.to_string() })
    }

    /// Parse response body as JSON
    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        serde_json::from_slice(&self.body)
            .map_err(|e| Error::JsonError { source: e.to_string() })
    }

    /// Check if response status is success (2xx)
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }
}

async fn read_response_with_pool(stream: &mut TcpStream) -> Result<Response> {
    // Read status line and headers
    let mut headers_buf = Vec::new();
    let mut byte_buf = [0u8; 1];

    // Read until we find \r\n\r\n (end of headers)
    loop {
        stream.read_exact(&mut byte_buf).await?;
        headers_buf.push(byte_buf[0]);

        // Check for \r\n\r\n pattern
        let len = headers_buf.len();
        if len >= 4 {
            if &headers_buf[len - 4..] == b"\r\n\r\n" {
                break;
            }
        }

        // Prevent infinite loop
        if headers_buf.len() > 8192 {
            return Err(Error::Internal {
                message: "Headers too large".to_string(),
            });
        }
    }

    let header_str = String::from_utf8_lossy(&headers_buf[..headers_buf.len() - 4]);
    let mut lines = header_str.lines();

    // Parse status line
    let status_line = lines.next().ok_or_else(|| Error::Internal {
        message: "Empty response".to_string(),
    })?;

    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| Error::Internal {
            message: format!("Invalid status line: {}", status_line),
        })?;

    let status = StatusCode::from_u16(status_code).map_err(|_| Error::Internal {
        message: format!("Invalid status code: {}", status_code),
    })?;

    // Parse headers
    let mut headers = HeaderMap::new();
    let mut content_length: Option<usize> = None;

    for line in lines {
        if line.is_empty() {
            break;
        }

        if let Some(pos) = line.find(':') {
            let name = line[..pos].trim();
            let value = line[pos + 1..].trim();

            // Check for Content-Length
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().ok();
            }

            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                headers.insert(name, value);
            }
        }
    }

    // Read body based on Content-Length
    let body = if let Some(length) = content_length {
        if length > 0 {
            let mut body_buf = vec![0u8; length];
            stream.read_exact(&mut body_buf).await?;
            Bytes::from(body_buf)
        } else {
            Bytes::new()
        }
    } else {
        // No Content-Length, read until connection closes (fallback)
        let mut body_buf = Vec::new();
        let _ = stream.read_to_end(&mut body_buf).await;
        Bytes::from(body_buf)
    };

    Ok(Response {
        status,
        headers,
        body,
    })
}



/// Request type (re-export for convenience)
pub type Request = RequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .avl_auth("test-token")
            .region("br-saopaulo-1")
            .compression(true)
            .build()
            .unwrap();

        assert_eq!(client.config.timeout, Duration::from_secs(10));
        assert_eq!(client.config.avl_auth, Some("test-token".to_string()));
        assert_eq!(client.config.region, Some("br-saopaulo-1".to_string()));
        assert!(client.config.compression);
    }

    #[test]
    fn test_request_builder_methods() {
        let client = Client::new();

        let get_req = client.get("https://example.com");
        assert_eq!(get_req.method, Method::GET);

        let post_req = client.post("https://example.com");
        assert_eq!(post_req.method, Method::POST);
    }

    #[test]
    fn test_request_with_query_params() {
        let client = Client::new();
        let req = client
            .get("https://api.example.com/data")
            .query("limit", "100")
            .query("offset", "0");

        assert_eq!(req.query_params.len(), 2);
        assert_eq!(req.query_params[0], ("limit".to_string(), "100".to_string()));
    }

}
