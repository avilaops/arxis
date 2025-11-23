//! Compression middleware

use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;
use tower::{Layer, Service};

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    /// Gzip compression
    Gzip,
    /// No compression
    None,
}

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Compression level (0-9)
    pub level: u32,

    /// Minimum size to compress (bytes)
    pub min_size: usize,

    /// Content types to compress
    pub content_types: Vec<String>,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: 6,
            min_size: 1024, // 1KB
            content_types: vec![
                "text/html".to_string(),
                "text/plain".to_string(),
                "text/css".to_string(),
                "text/javascript".to_string(),
                "application/javascript".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
                "text/xml".to_string(),
            ],
        }
    }
}

/// Compression layer
#[derive(Clone)]
pub struct CompressionLayer {
    config: CompressionConfig,
}

impl CompressionLayer {
    /// Create a new compression layer
    pub fn new() -> Self {
        Self {
            config: CompressionConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: CompressionConfig) -> Self {
        Self { config }
    }

    /// Set compression level
    pub fn with_level(mut self, level: u32) -> Self {
        self.config.level = level;
        self
    }
}

impl Default for CompressionLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for CompressionLayer {
    type Service = CompressionMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CompressionMiddleware {
            inner,
            config: self.config.clone(),
        }
    }
}

/// Compression middleware
#[derive(Clone)]
pub struct CompressionMiddleware<S> {
    inner: S,
    config: CompressionConfig,
}

impl<S> Service<Request> for CompressionMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // Check if client accepts gzip
        let accepts_gzip = req
            .headers()
            .get(header::ACCEPT_ENCODING)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.contains("gzip"))
            .unwrap_or(false);

        let config = self.config.clone();
        let future = self.inner.call(req);

        Box::pin(async move {
            let response = future.await?;

            if !accepts_gzip {
                return Ok(response);
            }

            // Check content type
            let content_type = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            let should_compress = config
                .content_types
                .iter()
                .any(|ct| content_type.starts_with(ct));

            if !should_compress {
                return Ok(response);
            }

            // Get response body
            let (parts, body) = response.into_parts();
            let bytes = match axum::body::to_bytes(body, usize::MAX).await {
                Ok(bytes) => bytes,
                Err(_) => return Ok(Response::from_parts(parts, Body::empty())),
            };

            // Check minimum size
            if bytes.len() < config.min_size {
                return Ok(Response::from_parts(parts, Body::from(bytes)));
            }

            // Compress
            let mut encoder = GzEncoder::new(Vec::new(), Compression::new(config.level));
            if encoder.write_all(&bytes).is_err() || encoder.finish().is_err() {
                return Ok(Response::from_parts(parts, Body::from(bytes)));
            }

            let compressed = encoder.finish().unwrap_or_else(|_| bytes.to_vec());

            // Build compressed response
            let mut response = Response::from_parts(parts, Body::from(compressed));
            response
                .headers_mut()
                .insert(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));
            response.headers_mut().remove(header::CONTENT_LENGTH);

            Ok(response)
        })
    }
}

/// Compress data with gzip
pub fn compress_gzip(data: &[u8], level: u32) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(level));
    encoder.write_all(data)?;
    encoder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::default();
        assert_eq!(config.level, 6);
        assert_eq!(config.min_size, 1024);
        assert!(config.content_types.contains(&"application/json".to_string()));
    }

    #[test]
    fn test_compress_gzip() {
        let data = b"Hello, World! This is a test string that should compress well.";
        let compressed = compress_gzip(data, 6).unwrap();

        // Compressed data should be smaller
        assert!(compressed.len() < data.len());
    }
}
