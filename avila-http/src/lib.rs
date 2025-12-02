//! Avila HTTP - Cliente HTTP nativo
//! Substitui reqwest - 100% Avila

use avila_error::{Error, ErrorKind, Result};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct Client {
    timeout: Option<std::time::Duration>,
    headers: HashMap<String, String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            timeout: Some(std::time::Duration::from_secs(30)),
            headers: HashMap::new(),
        }
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        self.request(Method::Get, url).await
    }

    pub async fn post(&self, url: &str) -> Result<RequestBuilder> {
        Ok(RequestBuilder::new(self, Method::Post, url))
    }

    pub async fn put(&self, url: &str) -> Result<RequestBuilder> {
        Ok(RequestBuilder::new(self, Method::Put, url))
    }

    pub async fn delete(&self, url: &str) -> Result<Response> {
        self.request(Method::Delete, url).await
    }

    async fn request(&self, method: Method, url: &str) -> Result<Response> {
        let parsed_url = parse_url(url)?;
        let host = parsed_url.host;
        let port = parsed_url.port.unwrap_or(80);
        let path = parsed_url.path;

        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| Error::network(format!("Failed to connect: {}", e)))?;

        let request = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n{}\r\n\r\n",
            method.as_str(),
            path,
            host,
            format_headers(&self.headers)
        );

        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| Error::io(format!("Failed to write: {}", e)))?;

        let mut reader = BufReader::new(stream);
        parse_response(&mut reader).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ClientBuilder {
    timeout: Option<std::time::Duration>,
    headers: HashMap<String, String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            timeout: Some(std::time::Duration::from_secs(30)),
            headers: HashMap::new(),
        }
    }

    pub fn timeout(mut self, duration: std::time::Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> Client {
        Client {
            timeout: self.timeout,
            headers: self.headers,
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RequestBuilder<'a> {
    client: &'a Client,
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl<'a> RequestBuilder<'a> {
    fn new(client: &'a Client, method: Method, url: &str) -> Self {
        Self {
            client,
            method,
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(mut self, data: Vec<u8>) -> Self {
        self.body = Some(data);
        self
    }

    pub fn json<T: avila_serde::Serialize>(mut self, data: &T) -> Self {
        let json = data.to_json();
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self.body = Some(json.into_bytes());
        self
    }

    pub async fn send(self) -> Result<Response> {
        // TODO: Implement full request with body
        self.client.request(self.method, &self.url).await
    }
}

#[derive(Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl Method {
    fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
        }
    }
}

pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| Error::parse(format!("Invalid UTF-8: {}", e)))
    }

    pub fn json<T: avila_serde::Deserialize>(&self) -> Result<T> {
        let text = self.text()?;
        T::from_json(&text).map_err(|e| Error::parse(format!("JSON parse error: {}", e)))
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_error(&self) -> bool {
        self.status >= 400
    }
}

struct ParsedUrl {
    host: String,
    port: Option<u16>,
    path: String,
}

fn parse_url(url: &str) -> Result<ParsedUrl> {
    let url = url.trim();

    let (url, _scheme) = if url.starts_with("http://") {
        (&url[7..], "http")
    } else if url.starts_with("https://") {
        (&url[8..], "https")
    } else {
        (url, "http")
    };

    let (host_port, path) = if let Some(idx) = url.find('/') {
        (&url[..idx], &url[idx..])
    } else {
        (url, "/")
    };

    let (host, port) = if let Some(idx) = host_port.find(':') {
        let host = &host_port[..idx];
        let port_str = &host_port[idx + 1..];
        let port = port_str
            .parse::<u16>()
            .map_err(|_| Error::parse("Invalid port"))?;
        (host.to_string(), Some(port))
    } else {
        (host_port.to_string(), None)
    };

    Ok(ParsedUrl {
        host,
        port,
        path: path.to_string(),
    })
}

fn format_headers(headers: &HashMap<String, String>) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\r\n")
}

async fn parse_response<R: AsyncBufReadExt + Unpin>(reader: &mut R) -> Result<Response> {
    let mut status_line = String::new();
    reader
        .read_line(&mut status_line)
        .await
        .map_err(|e| Error::network(format!("Failed to read status: {}", e)))?;

    let parts: Vec<&str> = status_line.split_whitespace().collect();
    let status = parts
        .get(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| Error::parse("Invalid status line"))?;

    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .map_err(|e| Error::network(format!("Failed to read header: {}", e)))?;

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        if let Some(idx) = line.find(':') {
            let key = line[..idx].trim().to_string();
            let value = line[idx + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    let mut body = Vec::new();
    reader
        .read_to_end(&mut body)
        .await
        .map_err(|e| Error::network(format!("Failed to read body: {}", e)))?;

    Ok(Response {
        status,
        headers,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        let url = parse_url("http://example.com/path").unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/path");

        let url = parse_url("http://example.com:8080/api").unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, Some(8080));
        assert_eq!(url.path, "/api");
    }
}
