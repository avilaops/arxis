//! Routing system for the gateway

use crate::error::{GatewayError, Result};
use axum::extract::Request;
use axum::http::Uri;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// A route in the gateway
#[derive(Debug, Clone)]
pub struct Route {
    /// Path pattern
    pub path: String,

    /// Upstream service(s)
    pub upstream: Upstream,

    /// HTTP methods allowed
    pub methods: Vec<String>,

    /// Strip path prefix
    pub strip_path: bool,

    /// Route-specific timeout in milliseconds
    pub timeout_ms: Option<u64>,

    /// Authentication required
    pub auth_required: bool,
}

/// Upstream service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Upstream {
    /// Single upstream URL
    Single(String),

    /// Multiple upstreams with load balancing
    LoadBalanced {
        urls: Vec<String>,
        strategy: LoadBalancingStrategy,
        current_index: usize,
    },
}

/// Load balancing strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    RoundRobin,

    /// Least connections (requires connection tracking)
    LeastConnections,

    /// Random selection
    Random,
}

impl Route {
    /// Create a new route with default settings
    pub fn new(path: impl Into<String>, upstream: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            upstream: Upstream::Single(upstream.into()),
            methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
            ],
            strip_path: false,
            timeout_ms: None,
            auth_required: false,
        }
    }

    /// Set allowed HTTP methods
    pub fn with_methods(mut self, methods: Vec<String>) -> Self {
        self.methods = methods;
        self
    }

    /// Enable path stripping
    pub fn with_strip_path(mut self, strip: bool) -> Self {
        self.strip_path = strip;
        self
    }

    /// Set route timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    /// Require authentication for this route
    pub fn with_auth(mut self, required: bool) -> Self {
        self.auth_required = required;
        self
    }
}

/// Router that matches requests to routes
#[derive(Clone)]
pub struct Router {
    routes: Arc<Vec<Route>>,
}

impl Router {
    /// Create a new router
    pub fn new(routes: Vec<Route>) -> Self {
        Self {
            routes: Arc::new(routes),
        }
    }

    /// Match a request to a route
    pub fn match_route(&self, req: &Request) -> Result<Route> {
        let path = req.uri().path();
        let method = req.method().as_str();

        for route in self.routes.iter() {
            if self.path_matches(&route.path, path) && route.methods.contains(&method.to_string()) {
                return Ok(route.clone());
            }
        }

        Err(GatewayError::Routing(format!(
            "No route found for {} {}",
            method, path
        )))
    }

    /// Check if a path matches a pattern
    fn path_matches(&self, pattern: &str, path: &str) -> bool {
        // Simple pattern matching: exact match or wildcard
        if pattern == path {
            return true;
        }

        // Handle wildcard patterns like "/api/*"
        if pattern.ends_with("/*") {
            let prefix = &pattern[..pattern.len() - 2];
            return path.starts_with(prefix);
        }

        // Handle path parameters like "/api/:id"
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return false;
        }

        for (pattern_part, path_part) in pattern_parts.iter().zip(path_parts.iter()) {
            if pattern_part.starts_with(':') {
                // This is a parameter, it matches anything
                continue;
            }
            if pattern_part != path_part {
                return false;
            }
        }

        true
    }

    /// Get the upstream URL for a route
    pub fn get_upstream_url(&self, route: &Route) -> String {
        match &route.upstream {
            Upstream::Single(url) => url.clone(),
            Upstream::LoadBalanced { urls, strategy, .. } => {
                // Simple round-robin for now
                // In production, this would be more sophisticated
                match strategy {
                    LoadBalancingStrategy::RoundRobin => {
                        urls[0].clone() // TODO: Implement proper round-robin
                    }
                    LoadBalancingStrategy::Random => {
                        use rand::Rng;
                        let idx = rand::thread_rng().gen_range(0..urls.len());
                        urls[idx].clone()
                    }
                    LoadBalancingStrategy::LeastConnections => {
                        urls[0].clone() // TODO: Implement least connections
                    }
                }
            }
        }
    }

    /// Process the path according to route configuration
    pub fn process_path(&self, route: &Route, original_path: &str) -> String {
        if route.strip_path {
            // Strip the route prefix from the path
            let prefix = route.path.trim_end_matches("/*");
            if original_path.starts_with(prefix) {
                let stripped = &original_path[prefix.len()..];
                if stripped.is_empty() {
                    "/".to_string()
                } else {
                    stripped.to_string()
                }
            } else {
                original_path.to_string()
            }
        } else {
            original_path.to_string()
        }
    }

    /// Get the number of routes
    pub fn upstream_count(&self) -> usize {
        self.routes.len()
    }
}

/// Route configuration builder
pub struct RouteConfig {
    path: String,
    upstream: Option<String>,
    methods: Vec<String>,
    strip_path: bool,
    timeout_ms: Option<u64>,
    auth_required: bool,
}

impl RouteConfig {
    /// Create a new route configuration
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            upstream: None,
            methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
            ],
            strip_path: false,
            timeout_ms: None,
            auth_required: false,
        }
    }

    /// Set the upstream URL
    pub fn upstream(mut self, url: impl Into<String>) -> Self {
        self.upstream = Some(url.into());
        self
    }

    /// Set allowed methods
    pub fn methods(mut self, methods: Vec<String>) -> Self {
        self.methods = methods;
        self
    }

    /// Enable path stripping
    pub fn strip_path(mut self, strip: bool) -> Self {
        self.strip_path = strip;
        self
    }

    /// Set timeout
    pub fn timeout_ms(mut self, timeout: u64) -> Self {
        self.timeout_ms = Some(timeout);
        self
    }

    /// Require authentication
    pub fn auth_required(mut self, required: bool) -> Self {
        self.auth_required = required;
        self
    }

    /// Build the route
    pub fn build(self) -> Result<Route> {
        let upstream = self
            .upstream
            .ok_or_else(|| GatewayError::Config("Upstream not configured".to_string()))?;

        Ok(Route {
            path: self.path,
            upstream: Upstream::Single(upstream),
            methods: self.methods,
            strip_path: self.strip_path,
            timeout_ms: self.timeout_ms,
            auth_required: self.auth_required,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_path_match() {
        let router = Router::new(vec![Route::new("/api/users", "http://localhost:8001")]);
        assert!(router.path_matches("/api/users", "/api/users"));
        assert!(!router.path_matches("/api/users", "/api/products"));
    }

    #[test]
    fn test_wildcard_match() {
        let router = Router::new(vec![Route::new("/api/*", "http://localhost:8001")]);
        assert!(router.path_matches("/api/*", "/api/users"));
        assert!(router.path_matches("/api/*", "/api/users/123"));
        assert!(!router.path_matches("/api/*", "/other/path"));
    }

    #[test]
    fn test_parameter_match() {
        let router = Router::new(vec![Route::new("/api/users/:id", "http://localhost:8001")]);
        assert!(router.path_matches("/api/users/:id", "/api/users/123"));
        assert!(!router.path_matches("/api/users/:id", "/api/users/123/posts"));
    }
}
