//! Request and response transformation

use axum::http::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request transformer
#[derive(Debug, Clone)]
pub struct RequestTransform {
    /// Headers to add
    pub add_headers: HashMap<String, String>,

    /// Headers to remove
    pub remove_headers: Vec<String>,

    /// Headers to modify
    pub modify_headers: HashMap<String, String>,

    /// Path prefix to add
    pub add_path_prefix: Option<String>,

    /// Path prefix to remove
    pub remove_path_prefix: Option<String>,
}

impl RequestTransform {
    /// Create a new request transformer
    pub fn new() -> Self {
        Self {
            add_headers: HashMap::new(),
            remove_headers: Vec::new(),
            modify_headers: HashMap::new(),
            add_path_prefix: None,
            remove_path_prefix: None,
        }
    }

    /// Add a header
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_headers.insert(key.into(), value.into());
        self
    }

    /// Remove a header
    pub fn remove_header(mut self, key: impl Into<String>) -> Self {
        self.remove_headers.push(key.into());
        self
    }

    /// Modify a header
    pub fn modify_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.modify_headers.insert(key.into(), value.into());
        self
    }

    /// Add path prefix
    pub fn add_path_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.add_path_prefix = Some(prefix.into());
        self
    }

    /// Remove path prefix
    pub fn remove_path_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.remove_path_prefix = Some(prefix.into());
        self
    }

    /// Apply transformations to headers
    pub fn apply_headers(&self, headers: &mut HeaderMap) {
        // Remove headers
        for key in &self.remove_headers {
            if let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) {
                headers.remove(&header_name);
            }
        }

        // Add headers
        for (key, value) in &self.add_headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                headers.insert(header_name, header_value);
            }
        }

        // Modify headers
        for (key, value) in &self.modify_headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                headers.insert(header_name, header_value);
            }
        }
    }

    /// Transform path
    pub fn transform_path(&self, path: &str) -> String {
        let mut result = path.to_string();

        // Remove prefix
        if let Some(prefix) = &self.remove_path_prefix {
            if result.starts_with(prefix) {
                result = result[prefix.len()..].to_string();
                if !result.starts_with('/') {
                    result = format!("/{}", result);
                }
            }
        }

        // Add prefix
        if let Some(prefix) = &self.add_path_prefix {
            result = format!("{}{}", prefix, result);
        }

        result
    }
}

impl Default for RequestTransform {
    fn default() -> Self {
        Self::new()
    }
}

/// Response transformer
#[derive(Debug, Clone)]
pub struct ResponseTransform {
    /// Headers to add
    pub add_headers: HashMap<String, String>,

    /// Headers to remove
    pub remove_headers: Vec<String>,

    /// Status code mapping
    pub status_mapping: HashMap<u16, u16>,
}

impl ResponseTransform {
    /// Create a new response transformer
    pub fn new() -> Self {
        Self {
            add_headers: HashMap::new(),
            remove_headers: Vec::new(),
            status_mapping: HashMap::new(),
        }
    }

    /// Add a header
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_headers.insert(key.into(), value.into());
        self
    }

    /// Remove a header
    pub fn remove_header(mut self, key: impl Into<String>) -> Self {
        self.remove_headers.push(key.into());
        self
    }

    /// Map status code
    pub fn map_status(mut self, from: u16, to: u16) -> Self {
        self.status_mapping.insert(from, to);
        self
    }

    /// Apply transformations to headers
    pub fn apply_headers(&self, headers: &mut HeaderMap) {
        // Remove headers
        for key in &self.remove_headers {
            if let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) {
                headers.remove(&header_name);
            }
        }

        // Add headers
        for (key, value) in &self.add_headers {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                headers.insert(header_name, header_value);
            }
        }
    }

    /// Transform status code
    pub fn transform_status(&self, status: u16) -> u16 {
        self.status_mapping.get(&status).copied().unwrap_or(status)
    }
}

impl Default for ResponseTransform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_transform_path() {
        let transform = RequestTransform::new()
            .remove_path_prefix("/api")
            .add_path_prefix("/v1");

        assert_eq!(transform.transform_path("/api/users"), "/v1/users");
    }

    #[test]
    fn test_response_transform_status() {
        let transform = ResponseTransform::new()
            .map_status(404, 200);

        assert_eq!(transform.transform_status(404), 200);
        assert_eq!(transform.transform_status(500), 500);
    }
}
