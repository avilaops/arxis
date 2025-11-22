//! HTTP Client implementation

use crate::error::{Error, Result};
use crate::common;
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// HTTP Client for making requests
#[derive(Clone)]
pub struct Client {
    /// Client configuration
    pub config: ClientConfig,
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
        }
    }
}

impl Client {
    /// Create a new client with default configuration
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
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
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for configuring HTTP client
pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
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

    /// Build the client
    pub fn build(self) -> Result<Client> {
        Ok(Client { config: self.config })
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
    pub async fn send(self) -> Result<Response> {
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

        // Parse URL
        let (host, port, _is_https) = common::parse_url(&full_url)?;

        // Connect to server
        let addr = format!("{}:{}", host, port);
        let mut stream = tokio::time::timeout(
            self.timeout.unwrap_or(common::DEFAULT_TIMEOUT),
            TcpStream::connect(&addr)
        )
        .await
        .map_err(|_| Error::Timeout { duration: self.timeout.unwrap_or(common::DEFAULT_TIMEOUT) })?
        .map_err(|e| Error::ConnectionFailed { addr: addr.clone(), source: e })?;

        // Build HTTP request
        let path = full_url
            .find("://")
            .and_then(|pos| full_url[pos + 3..].find('/'))
            .map(|pos| &full_url[full_url.find("://").unwrap() + 3 + pos..])
            .unwrap_or("/");

        let mut request = format!("{} {} HTTP/1.1\r\n", self.method, path);
        request.push_str(&format!("Host: {}\r\n", host));
        request.push_str("Connection: close\r\n");

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

        // Read response
        let mut response_data = Vec::new();
        stream.read_to_end(&mut response_data).await?;

        // Parse response
        parse_response(response_data)
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

fn parse_response(data: Vec<u8>) -> Result<Response> {
    // Find the separator between headers and body
    let separator = b"\r\n\r\n";
    let mut header_end = 0;
    
    for i in 0..data.len().saturating_sub(3) {
        if &data[i..i + 4] == separator {
            header_end = i + 4;
            break;
        }
    }

    if header_end == 0 {
        return Err(Error::Internal {
            message: "Invalid HTTP response: no header/body separator found".to_string(),
        });
    }

    let header_data = &data[..header_end - 4];
    let header_str = String::from_utf8_lossy(header_data);
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

    let status = StatusCode::from_u16(status_code)
        .map_err(|_| Error::Internal {
            message: format!("Invalid status code: {}", status_code),
        })?;

    // Parse headers
    let mut headers = HeaderMap::new();

    for line in lines {
        if line.is_empty() {
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
    let body = if header_end < data.len() {
        Bytes::copy_from_slice(&data[header_end..])
    } else {
        Bytes::new()
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

    #[tokio::test]
    async fn test_parse_response() {
        let response_data = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nHello";
        let response = parse_response(response_data.to_vec()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.is_success());
        assert_eq!(response.body.len(), 5);
    }
}
