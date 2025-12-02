//! Avila Web - Framework web nativo
//! Substitui axum/tower

use avila_error::{Error, Result};
use avila_serde::{Deserialize, Serialize};
use avila_async::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::future::Future;
use std::io::{BufRead, BufReader, Write};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;

pub struct Router {
    routes: HashMap<(Method, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn get<F, Fut>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: Handler = Arc::new(move |req| Box::pin(handler(req)));
        self.routes.insert((Method::Get, path.to_string()), handler);
        self
    }

    pub fn post<F, Fut>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: Handler = Arc::new(move |req| Box::pin(handler(req)));
        self.routes.insert((Method::Post, path.to_string()), handler);
        self
    }

    pub fn put<F, Fut>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: Handler = Arc::new(move |req| Box::pin(handler(req)));
        self.routes.insert((Method::Put, path.to_string()), handler);
        self
    }

    pub fn delete<F, Fut>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: Handler = Arc::new(move |req| Box::pin(handler(req)));
        self.routes.insert((Method::Delete, path.to_string()), handler);
        self
    }

    async fn handle_request(&self, req: Request) -> Response {
        let key = (req.method, req.path.clone());

        if let Some(handler) = self.routes.get(&key) {
            handler(req).await
        } else {
            Response::not_found()
        }
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| Error::network(format!("Failed to bind: {}", e)))?;

        println!("🚀 Server running on http://{}", addr);

        let router = Arc::new(self);

        loop {
            let (stream, _) = listener
                .accept()
                .await
                .map_err(|e| Error::network(format!("Failed to accept: {}", e)))?;

            let router = Arc::clone(&router);
            // Use thread pool instead of tokio::spawn
            std::thread::spawn(move || {
                let stream = stream.into_std();
                if let Err(e) = handle_connection_sync(stream, router) {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

fn handle_connection_sync(stream: std::net::TcpStream, router: Arc<Router>) -> Result<()> { let mut reader = BufReader::new(stream.try_clone().map_err(|e| Error::io(e.to_string()))?); let request = parse_request_sync(&mut reader)?; let runtime = avila_async::Runtime::new(); let response = runtime.block_on(async move { router.handle_request(request).await });

    let mut stream = stream;
    let response_str = format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        response.status,
        status_text(response.status),
        format_headers(&response.headers),
        String::from_utf8_lossy(&response.body)
    );

    stream
        .write_all(response_str.as_bytes())
        .map_err(|e| Error::io(format!("Failed to write response: {}", e)))?;

    Ok(())
}

fn parse_request_sync<R: BufRead>(reader: &mut R) -> Result<Request> {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .map_err(|e| Error::parse(format!("Failed to read request line: {}", e)))?;

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(Error::parse("Invalid request line"));
    }

    let method = match parts[0] {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        _ => Method::Get,
    };

    let path = parts[1].to_string();

    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| Error::parse(format!("Failed to read header: {}", e)))?;

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        if let Some(idx) = line.find(':') {
            let key = line[..idx].trim().to_lowercase();
            let value = line[idx + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    let body = Vec::new(); // TODO: Read body based on Content-Length

    Ok(Request {
        method,
        path,
        headers,
        body,
    })
}

fn format_headers(headers: &HashMap<String, String>) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\r\n")
}

fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn json<T: Deserialize>(&self) -> Result<T> {
        let text = String::from_utf8(self.body.clone())
            .map_err(|e| Error::parse(format!("Invalid UTF-8: {}", e)))?;
        T::from_json(&text).map_err(|e| Error::parse(format!("JSON error: {}", e)))
    }

    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }
}

pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn created() -> Self {
        Self {
            status: 201,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn not_found() -> Self {
        Self::new(404).text("Not Found")
    }

    pub fn bad_request() -> Self {
        Self::new(400).text("Bad Request")
    }

    pub fn internal_error() -> Self {
        Self::new(500).text("Internal Server Error")
    }

    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self.body = text.as_bytes().to_vec();
        self
    }

    pub fn json<T: Serialize>(mut self, data: &T) -> Self {
        let json = data.to_json();
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self.body = json.into_bytes();
        self
    }

    pub fn html(mut self, html: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "text/html".to_string());
        self.body = html.as_bytes().to_vec();
        self
    }
}

// Helper functions
pub async fn ok_json<T: Serialize>(data: &T) -> Response {
    Response::ok().json(data)
}

pub async fn created_json<T: Serialize>(data: &T) -> Response {
    Response::created().json(data)
}

pub async fn ok_text(text: &str) -> Response {
    Response::ok().text(text)
}
