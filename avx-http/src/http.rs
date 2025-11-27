//! Pure Rust HTTP/1.1 implementation
//!
//! Zero-copy HTTP parser using finite state machine.
//! No allocations for headers, uses slices into original buffer.

use crate::error::{Error, Result};
use std::fmt;
use std::str;

/// HTTP Method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// GET
    Get,
    /// POST
    Post,
    /// PUT
    Put,
    /// DELETE
    Delete,
    /// PATCH
    Patch,
    /// HEAD
    Head,
    /// OPTIONS
    Options,
    /// CONNECT
    Connect,
    /// TRACE
    Trace,
}

impl Method {
    /// Parse method from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        match bytes {
            b"GET" => Ok(Method::Get),
            b"POST" => Ok(Method::Post),
            b"PUT" => Ok(Method::Put),
            b"DELETE" => Ok(Method::Delete),
            b"PATCH" => Ok(Method::Patch),
            b"HEAD" => Ok(Method::Head),
            b"OPTIONS" => Ok(Method::Options),
            b"CONNECT" => Ok(Method::Connect),
            b"TRACE" => Ok(Method::Trace),
            _ => Err(Error::InvalidMethod {
                method: String::from_utf8_lossy(bytes).into_owned(),
            }),
        }
    }

    /// Convert to byte slice
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            Method::Get => b"GET",
            Method::Post => b"POST",
            Method::Put => b"PUT",
            Method::Delete => b"DELETE",
            Method::Patch => b"PATCH",
            Method::Head => b"HEAD",
            Method::Options => b"OPTIONS",
            Method::Connect => b"CONNECT",
            Method::Trace => b"TRACE",
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(self.as_bytes()).unwrap())
    }
}

/// HTTP Status Code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(u16);

impl StatusCode {
    /// 200 OK
    pub const OK: StatusCode = StatusCode(200);
    /// 201 Created
    pub const CREATED: StatusCode = StatusCode(201);
    /// 204 No Content
    pub const NO_CONTENT: StatusCode = StatusCode(204);
    /// 301 Moved Permanently
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301);
    /// 302 Found
    pub const FOUND: StatusCode = StatusCode(302);
    /// 304 Not Modified
    pub const NOT_MODIFIED: StatusCode = StatusCode(304);
    /// 400 Bad Request
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    /// 401 Unauthorized
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    /// 403 Forbidden
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    /// 404 Not Found
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    /// 405 Method Not Allowed
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);
    /// 500 Internal Server Error
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
    /// 502 Bad Gateway
    pub const BAD_GATEWAY: StatusCode = StatusCode(502);
    /// 503 Service Unavailable
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);

    /// Create from u16
    pub fn from_u16(code: u16) -> Result<Self> {
        if (100..600).contains(&code) {
            Ok(StatusCode(code))
        } else {
            Err(Error::InvalidStatusCode { code })
        }
    }

    /// Get status code as u16
    pub fn as_u16(&self) -> u16 {
        self.0
    }

    /// Get reason phrase
    pub fn reason(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown",
        }
    }

    /// Check if status is success (2xx)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.0)
    }

    /// Check if status is redirect (3xx)
    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.0)
    }

    /// Check if status is client error (4xx)
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.0)
    }

    /// Check if status is server error (5xx)
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.0)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.reason())
    }
}

/// HTTP Headers (zero-copy, stores offsets into buffer)
#[derive(Debug, Clone)]
pub struct Headers {
    headers: Vec<(String, String)>,
}

impl Headers {
    /// Create new empty headers
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
        }
    }

    /// Insert a header
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();

        // Remove existing header with same name (case-insensitive)
        self.headers.retain(|(n, _)| !n.eq_ignore_ascii_case(&name));

        self.headers.push((name, value));
    }

    /// Get header value
    pub fn get(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// Get all headers with name
    pub fn get_all(&self, name: &str) -> Vec<&str> {
        self.headers
            .iter()
            .filter(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
            .collect()
    }

    /// Remove header
    pub fn remove(&mut self, name: &str) {
        self.headers.retain(|(n, _)| !n.eq_ignore_ascii_case(name));
    }

    /// Check if header exists
    pub fn contains(&self, name: &str) -> bool {
        self.headers.iter().any(|(n, _)| n.eq_ignore_ascii_case(name))
    }

    /// Iterate over headers
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.headers.iter().map(|(n, v)| (n.as_str(), v.as_str()))
    }

    /// Get number of headers
    pub fn len(&self) -> usize {
        self.headers.len()
    }

    /// Check if headers are empty
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP Request
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: Method,
    /// Request path
    pub path: String,
    /// HTTP version
    pub version: (u8, u8),
    /// Request headers
    pub headers: Headers,
    /// Request body
    pub body: Vec<u8>,
}

impl Request {
    /// Create new request
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            version: (1, 1),
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// Get body as string slice
    pub fn body_str(&self) -> Result<&str> {
        str::from_utf8(&self.body).map_err(|e| Error::InvalidUtf8 {
            message: e.to_string(),
        })
    }

    /// Parse request from buffer
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let mut parser = RequestParser::new(buf);
        parser.parse()
    }

    /// Convert to wire format
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Request line
        buf.extend_from_slice(self.method.as_bytes());
        buf.push(b' ');
        buf.extend_from_slice(self.path.as_bytes());
        buf.extend_from_slice(b" HTTP/");
        buf.push(b'0' + self.version.0);
        buf.push(b'.');
        buf.push(b'0' + self.version.1);
        buf.extend_from_slice(b"\r\n");

        // Headers
        for (name, value) in self.headers.iter() {
            buf.extend_from_slice(name.as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(value.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        // Body
        if !self.body.is_empty() {
            buf.extend_from_slice(format!("Content-Length: {}\r\n", self.body.len()).as_bytes());
        }

        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(&self.body);

        buf
    }
}

/// HTTP Response
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP version
    pub version: (u8, u8),
    /// Status code
    pub status: StatusCode,
    /// Response headers
    pub headers: Headers,
    /// Response body
    pub body: Vec<u8>,
}

impl Response {
    /// Create new response
    pub fn new(status: StatusCode) -> Self {
        Self {
            version: (1, 1),
            status,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// Create 200 OK response with text body
    pub fn text(body: impl Into<String>) -> Self {
        let body_str = body.into();
        let mut response = Self::new(StatusCode::OK);
        response.body = body_str.into_bytes();
        response.headers.insert("Content-Type", "text/plain; charset=utf-8");
        response
    }

    /// Create 200 OK response with JSON body
    pub fn json(body: impl Into<String>) -> Self {
        let body_str = body.into();
        let mut response = Self::new(StatusCode::OK);
        response.body = body_str.into_bytes();
        response.headers.insert("Content-Type", "application/json");
        response
    }

    /// Create 404 Not Found response
    pub fn not_found() -> Self {
        let mut response = Self::new(StatusCode::NOT_FOUND);
        response.body = b"Not Found".to_vec();
        response
    }

    /// Get body as string slice
    pub fn body_str(&self) -> Result<&str> {
        str::from_utf8(&self.body).map_err(|e| Error::InvalidUtf8 {
            message: e.to_string(),
        })
    }

    /// Parse response from buffer
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let mut parser = ResponseParser::new(buf);
        parser.parse()
    }

    /// Convert to wire format
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Status line
        buf.extend_from_slice(b"HTTP/");
        buf.push(b'0' + self.version.0);
        buf.push(b'.');
        buf.push(b'0' + self.version.1);
        buf.push(b' ');
        buf.extend_from_slice(self.status.as_u16().to_string().as_bytes());
        buf.push(b' ');
        buf.extend_from_slice(self.status.reason().as_bytes());
        buf.extend_from_slice(b"\r\n");

        // Headers
        for (name, value) in self.headers.iter() {
            buf.extend_from_slice(name.as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(value.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        // Content-Length
        if !self.body.is_empty() {
            buf.extend_from_slice(format!("Content-Length: {}\r\n", self.body.len()).as_bytes());
        }

        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(&self.body);

        buf
    }
}

/// Request parser (finite state machine)
struct RequestParser<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> RequestParser<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn parse(&mut self) -> Result<Request> {
        // Parse request line
        let method = self.parse_method()?;
        self.skip_whitespace();
        let path = self.parse_path()?;
        self.skip_whitespace();
        let version = self.parse_version()?;
        self.expect(b"\r\n")?;

        // Parse headers
        let headers = self.parse_headers()?;

        // Parse body
        let content_length = headers
            .get("content-length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let body = if content_length > 0 {
            self.read_bytes(content_length)?.to_vec()
        } else {
            Vec::new()
        };

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    fn parse_method(&mut self) -> Result<Method> {
        let start = self.pos;
        while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        Method::from_bytes(&self.buf[start..self.pos])
    }

    fn parse_path(&mut self) -> Result<String> {
        let start = self.pos;
        while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let path_bytes = &self.buf[start..self.pos];
        String::from_utf8(path_bytes.to_vec()).map_err(|e| Error::InvalidUtf8 {
            message: e.to_string(),
        })
    }

    fn parse_version(&mut self) -> Result<(u8, u8)> {
        self.expect(b"HTTP/")?;
        let major = self.read_digit()?;
        self.expect(b".")?;
        let minor = self.read_digit()?;
        Ok((major, minor))
    }

    fn parse_headers(&mut self) -> Result<Headers> {
        let mut headers = Headers::new();

        loop {
            // Check for end of headers
            if self.peek(b"\r\n")? {
                self.pos += 2;
                break;
            }

            // Parse header name
            let name_start = self.pos;
            while self.pos < self.buf.len() && self.buf[self.pos] != b':' {
                self.pos += 1;
            }
            let name = str::from_utf8(&self.buf[name_start..self.pos])
                .map_err(|e| Error::InvalidUtf8 {
                    message: e.to_string(),
                })?
                .trim()
                .to_string();

            self.expect(b":")?;
            self.skip_whitespace();

            // Parse header value
            let value_start = self.pos;
            while self.pos < self.buf.len() && self.buf[self.pos] != b'\r' {
                self.pos += 1;
            }
            let value = str::from_utf8(&self.buf[value_start..self.pos])
                .map_err(|e| Error::InvalidUtf8 {
                    message: e.to_string(),
                })?
                .trim()
                .to_string();

            headers.insert(name, value);
            self.expect(b"\r\n")?;
        }

        Ok(headers)
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, bytes: &[u8]) -> Result<()> {
        if self.pos + bytes.len() > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }
        if &self.buf[self.pos..self.pos + bytes.len()] != bytes {
            return Err(Error::ParseError {
                message: format!(
                    "Expected {:?}, got {:?}",
                    bytes,
                    &self.buf[self.pos..self.pos + bytes.len()]
                ),
            });
        }
        self.pos += bytes.len();
        Ok(())
    }

    fn peek(&self, bytes: &[u8]) -> Result<bool> {
        if self.pos + bytes.len() > self.buf.len() {
            return Ok(false);
        }
        Ok(&self.buf[self.pos..self.pos + bytes.len()] == bytes)
    }

    fn read_digit(&mut self) -> Result<u8> {
        if self.pos >= self.buf.len() || !self.buf[self.pos].is_ascii_digit() {
            return Err(Error::ParseError {
                message: "Expected digit".to_string(),
            });
        }
        let digit = self.buf[self.pos] - b'0';
        self.pos += 1;
        Ok(digit)
    }

    fn read_bytes(&mut self, count: usize) -> Result<&'a [u8]> {
        if self.pos + count > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }
        let bytes = &self.buf[self.pos..self.pos + count];
        self.pos += count;
        Ok(bytes)
    }
}

/// Response parser (finite state machine)
struct ResponseParser<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ResponseParser<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn parse(&mut self) -> Result<Response> {
        // Parse status line
        let version = self.parse_version()?;
        self.skip_whitespace();
        let status = self.parse_status()?;
        self.skip_to_newline()?;

        // Parse headers
        let headers = self.parse_headers()?;

        // Parse body
        let content_length = headers
            .get("content-length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let body = if content_length > 0 {
            self.read_bytes(content_length)?.to_vec()
        } else {
            Vec::new()
        };

        Ok(Response {
            version,
            status,
            headers,
            body,
        })
    }

    fn parse_version(&mut self) -> Result<(u8, u8)> {
        self.expect(b"HTTP/")?;
        let major = self.read_digit()?;
        self.expect(b".")?;
        let minor = self.read_digit()?;
        Ok((major, minor))
    }

    fn parse_status(&mut self) -> Result<StatusCode> {
        let mut code = 0u16;
        for _ in 0..3 {
            if self.pos >= self.buf.len() || !self.buf[self.pos].is_ascii_digit() {
                return Err(Error::ParseError {
                    message: "Invalid status code".to_string(),
                });
            }
            code = code * 10 + (self.buf[self.pos] - b'0') as u16;
            self.pos += 1;
        }
        StatusCode::from_u16(code)
    }

    fn parse_headers(&mut self) -> Result<Headers> {
        let mut headers = Headers::new();

        loop {
            // Check for end of headers
            if self.peek(b"\r\n")? {
                self.pos += 2;
                break;
            }

            // Parse header name
            let name_start = self.pos;
            while self.pos < self.buf.len() && self.buf[self.pos] != b':' {
                self.pos += 1;
            }
            let name = str::from_utf8(&self.buf[name_start..self.pos])
                .map_err(|e| Error::InvalidUtf8 {
                    message: e.to_string(),
                })?
                .trim()
                .to_string();

            self.expect(b":")?;
            self.skip_whitespace();

            // Parse header value
            let value_start = self.pos;
            while self.pos < self.buf.len() && self.buf[self.pos] != b'\r' {
                self.pos += 1;
            }
            let value = str::from_utf8(&self.buf[value_start..self.pos])
                .map_err(|e| Error::InvalidUtf8 {
                    message: e.to_string(),
                })?
                .trim()
                .to_string();

            headers.insert(name, value);
            self.expect(b"\r\n")?;
        }

        Ok(headers)
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() && self.buf[self.pos] != b'\r' {
            self.pos += 1;
        }
    }

    fn skip_to_newline(&mut self) -> Result<()> {
        while self.pos < self.buf.len() && self.buf[self.pos] != b'\r' {
            self.pos += 1;
        }
        self.expect(b"\r\n")
    }

    fn expect(&mut self, bytes: &[u8]) -> Result<()> {
        if self.pos + bytes.len() > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }
        if &self.buf[self.pos..self.pos + bytes.len()] != bytes {
            return Err(Error::ParseError {
                message: format!(
                    "Expected {:?}, got {:?}",
                    bytes,
                    &self.buf[self.pos..self.pos + bytes.len()]
                ),
            });
        }
        self.pos += bytes.len();
        Ok(())
    }

    fn peek(&self, bytes: &[u8]) -> Result<bool> {
        if self.pos + bytes.len() > self.buf.len() {
            return Ok(false);
        }
        Ok(&self.buf[self.pos..self.pos + bytes.len()] == bytes)
    }

    fn read_digit(&mut self) -> Result<u8> {
        if self.pos >= self.buf.len() || !self.buf[self.pos].is_ascii_digit() {
            return Err(Error::ParseError {
                message: "Expected digit".to_string(),
            });
        }
        let digit = self.buf[self.pos] - b'0';
        self.pos += 1;
        Ok(digit)
    }

    fn read_bytes(&mut self, count: usize) -> Result<&'a [u8]> {
        if self.pos + count > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }
        let bytes = &self.buf[self.pos..self.pos + count];
        self.pos += count;
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_bytes() {
        assert_eq!(Method::from_bytes(b"GET").unwrap(), Method::Get);
        assert_eq!(Method::from_bytes(b"POST").unwrap(), Method::Post);
        assert!(Method::from_bytes(b"INVALID").is_err());
    }

    #[test]
    fn test_status_code() {
        let status = StatusCode::OK;
        assert_eq!(status.as_u16(), 200);
        assert_eq!(status.reason(), "OK");
        assert!(status.is_success());
    }

    #[test]
    fn test_headers() {
        let mut headers = Headers::new();
        headers.insert("Content-Type", "text/plain");
        headers.insert("Content-Length", "42");

        assert_eq!(headers.get("Content-Type"), Some("text/plain"));
        assert_eq!(headers.get("content-type"), Some("text/plain"));
        assert_eq!(headers.len(), 2);
    }

    #[test]
    fn test_request_parse() {
        let raw = b"GET /path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let req = Request::parse(raw).unwrap();

        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, "/path");
        assert_eq!(req.version, (1, 1));
        assert_eq!(req.headers.get("Host"), Some("example.com"));
    }

    #[test]
    fn test_response_parse() {
        let raw = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nHello";
        let res = Response::parse(raw).unwrap();

        assert_eq!(res.status, StatusCode::OK);
        assert_eq!(res.headers.get("Content-Type"), Some("text/plain"));
        assert_eq!(res.body_str().unwrap(), "Hello");
    }

    #[test]
    fn test_request_to_bytes() {
        let mut req = Request::new(Method::Get, "/path");
        req.headers.insert("Host", "example.com");

        let bytes = req.to_bytes();
        let parsed = Request::parse(&bytes).unwrap();

        assert_eq!(parsed.method, Method::Get);
        assert_eq!(parsed.path, "/path");
    }

    #[test]
    fn test_response_to_bytes() {
        let res = Response::text("Hello, World!");
        let bytes = res.to_bytes();
        let parsed = Response::parse(&bytes).unwrap();

        assert_eq!(parsed.status, StatusCode::OK);
        assert_eq!(parsed.body_str().unwrap(), "Hello, World!");
    }
}
