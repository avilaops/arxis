//! Load balancer for distributing requests across multiple upstream services

use crate::error::{GatewayError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Load balancing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// Round-robin distribution
    RoundRobin,

    /// Random selection
    Random,

    /// Least connections (requires connection tracking)
    LeastConnections,

    /// Weighted round-robin
    Weighted,
}

/// Load balancer for multiple upstream services
#[derive(Clone)]
pub struct LoadBalancer {
    /// List of upstream URLs
    upstreams: Arc<Vec<String>>,

    /// Load balancing strategy
    strategy: Strategy,

    /// Current index for round-robin
    current_index: Arc<AtomicUsize>,

    /// Connection counts per upstream (for least connections)
    connection_counts: Arc<Vec<AtomicUsize>>,
}

impl LoadBalancer {
    /// Create a new load balancer
    pub fn new() -> LoadBalancerBuilder {
        LoadBalancerBuilder::new()
    }

    /// Select the next upstream service
    pub fn next_upstream(&self) -> Result<String> {
        if self.upstreams.is_empty() {
            return Err(GatewayError::Config(
                "No upstream services configured".to_string(),
            ));
        }

        let index = match self.strategy {
            Strategy::RoundRobin => self.round_robin(),
            Strategy::Random => self.random(),
            Strategy::LeastConnections => self.least_connections(),
            Strategy::Weighted => self.weighted(),
        };

        Ok(self.upstreams[index].clone())
    }

    /// Round-robin selection
    fn round_robin(&self) -> usize {
        let index = self.current_index.fetch_add(1, Ordering::Relaxed);
        index % self.upstreams.len()
    }

    /// Random selection
    fn random(&self) -> usize {
        use rand::Rng;
        rand::thread_rng().gen_range(0..self.upstreams.len())
    }

    /// Least connections selection
    fn least_connections(&self) -> usize {
        self.connection_counts
            .iter()
            .enumerate()
            .min_by_key(|(_, count)| count.load(Ordering::Relaxed))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    /// Weighted selection (simplified - uses round-robin for now)
    fn weighted(&self) -> usize {
        // TODO: Implement proper weighted distribution
        self.round_robin()
    }

    /// Increment connection count for an upstream
    pub fn increment_connections(&self, index: usize) {
        if index < self.connection_counts.len() {
            self.connection_counts[index].fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Decrement connection count for an upstream
    pub fn decrement_connections(&self, index: usize) {
        if index < self.connection_counts.len() {
            self.connection_counts[index].fetch_sub(1, Ordering::Relaxed);
        }
    }

    /// Get the number of upstreams
    pub fn upstream_count(&self) -> usize {
        self.upstreams.len()
    }
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self {
            upstreams: Arc::new(Vec::new()),
            strategy: Strategy::RoundRobin,
            current_index: Arc::new(AtomicUsize::new(0)),
            connection_counts: Arc::new(Vec::new()),
        }
    }
}

/// Load balancer builder
pub struct LoadBalancerBuilder {
    upstreams: Vec<String>,
    strategy: Strategy,
}

impl LoadBalancerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            upstreams: Vec::new(),
            strategy: Strategy::RoundRobin,
        }
    }

    /// Add an upstream service
    pub fn upstream(mut self, url: impl Into<String>) -> Self {
        self.upstreams.push(url.into());
        self
    }

    /// Set the load balancing strategy
    pub fn strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Build the load balancer
    pub fn build(self) -> Result<LoadBalancer> {
        if self.upstreams.is_empty() {
            return Err(GatewayError::Config(
                "At least one upstream service is required".to_string(),
            ));
        }

        let upstream_count = self.upstreams.len();

        Ok(LoadBalancer {
            upstreams: Arc::new(self.upstreams),
            strategy: self.strategy,
            current_index: Arc::new(AtomicUsize::new(0)),
            connection_counts: Arc::new(
                (0..upstream_count)
                    .map(|_| AtomicUsize::new(0))
                    .collect(),
            ),
        })
    }
}

impl Default for LoadBalancerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        let lb = LoadBalancer::new()
            .upstream("http://service1:8001")
            .upstream("http://service2:8002")
            .upstream("http://service3:8003")
            .strategy(Strategy::RoundRobin)
            .build()
            .unwrap();

        let first = lb.next_upstream().unwrap();
        let second = lb.next_upstream().unwrap();
        let third = lb.next_upstream().unwrap();
        let fourth = lb.next_upstream().unwrap();

        assert_eq!(first, "http://service1:8001");
        assert_eq!(second, "http://service2:8002");
        assert_eq!(third, "http://service3:8003");
        assert_eq!(fourth, "http://service1:8001"); // Wraps around
    }

    #[test]
    fn test_least_connections() {
        let lb = LoadBalancer::new()
            .upstream("http://service1:8001")
            .upstream("http://service2:8002")
            .strategy(Strategy::LeastConnections)
            .build()
            .unwrap();

        // First request goes to first service
        let first = lb.next_upstream().unwrap();
        assert_eq!(first, "http://service1:8001");

        // Simulate connection
        lb.increment_connections(0);

        // Second request should go to second service (fewer connections)
        let second = lb.next_upstream().unwrap();
        assert_eq!(second, "http://service2:8002");
    }
}
