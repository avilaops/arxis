//! Rate limiting nativo - 100% Rust sem dependências externas
//!
//! Sistema de rate limiting usando HashMap + Arc<Mutex> + Tokio

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;

/// Rate limit configuration
#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: usize,
    /// Time window duration
    pub window: Duration,
    /// Cleanup interval
    pub cleanup_interval: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300), // 5 min
        }
    }
}

/// Request record
#[derive(Debug, Clone)]
struct RequestRecord {
    count: usize,
    window_start: Instant,
}

/// Native rate limiter
pub struct RateLimiter {
    config: RateLimitConfig,
    records: Arc<Mutex<HashMap<IpAddr, RequestRecord>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        let records = Arc::new(Mutex::new(HashMap::new()));

        // Spawn cleanup task
        let cleanup_records = Arc::clone(&records);
        let cleanup_window = config.window;
        let cleanup_interval = config.cleanup_interval;

        tokio::spawn(async move {
            let mut interval = time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                Self::cleanup_old_records(&cleanup_records, cleanup_window);
            }
        });

        Self { config, records }
    }

    /// Check if request is allowed
    pub fn check(&self, ip: IpAddr) -> Result<(), RateLimitError> {
        let mut records = self.records.lock()
            .map_err(|_| RateLimitError::LockPoisoned)?;

        let now = Instant::now();

        if let Some(record) = records.get_mut(&ip) {
            // Check if window expired
            if now.duration_since(record.window_start) > self.config.window {
                // Reset window
                record.count = 1;
                record.window_start = now;
                Ok(())
            } else if record.count >= self.config.max_requests {
                // Rate limit exceeded
                let retry_after = self.config.window
                    .saturating_sub(now.duration_since(record.window_start));
                Err(RateLimitError::TooManyRequests { retry_after })
            } else {
                // Increment counter
                record.count += 1;
                Ok(())
            }
        } else {
            // First request from this IP
            records.insert(
                ip,
                RequestRecord {
                    count: 1,
                    window_start: now,
                },
            );
            Ok(())
        }
    }

    /// Get current request count for IP
    pub fn get_count(&self, ip: IpAddr) -> usize {
        self.records.lock()
            .ok()
            .and_then(|records| records.get(&ip).map(|r| r.count))
            .unwrap_or(0)
    }

    /// Get remaining requests for IP
    pub fn get_remaining(&self, ip: IpAddr) -> usize {
        let count = self.get_count(ip);
        self.config.max_requests.saturating_sub(count)
    }

    /// Reset rate limit for IP
    pub fn reset(&self, ip: IpAddr) {
        if let Ok(mut records) = self.records.lock() {
            records.remove(&ip);
        }
    }

    /// Cleanup old records
    fn cleanup_old_records(records: &Arc<Mutex<HashMap<IpAddr, RequestRecord>>>, window: Duration) {
        if let Ok(mut records) = records.lock() {
            let now = Instant::now();
            records.retain(|_, record| {
                now.duration_since(record.window_start) <= window
            });
        }
    }

    /// Get statistics
    pub fn stats(&self) -> RateLimitStats {
        if let Ok(records) = self.records.lock() {
            let total_ips = records.len();
            let active_requests: usize = records.values().map(|r| r.count).sum();

            RateLimitStats {
                total_ips,
                active_requests,
                max_per_window: self.config.max_requests,
                window_seconds: self.config.window.as_secs(),
            }
        } else {
            RateLimitStats::default()
        }
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            config: self.config,
            records: Arc::clone(&self.records),
        }
    }
}

/// Rate limit statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct RateLimitStats {
    pub total_ips: usize,
    pub active_requests: usize,
    pub max_per_window: usize,
    pub window_seconds: u64,
}

/// Rate limit error
#[derive(Debug)]
pub enum RateLimitError {
    TooManyRequests { retry_after: Duration },
    LockPoisoned,
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitError::TooManyRequests { retry_after } => {
                write!(f, "Rate limit exceeded. Retry after {} seconds", retry_after.as_secs())
            }
            RateLimitError::LockPoisoned => write!(f, "Rate limiter lock poisoned"),
        }
    }
}

impl std::error::Error for RateLimitError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_rate_limiting() {
        let config = RateLimitConfig {
            max_requests: 3,
            window: Duration::from_secs(10),
            cleanup_interval: Duration::from_secs(60),
        };

        let limiter = RateLimiter::new(config);
        let ip = IpAddr::from_str("127.0.0.1").unwrap();

        // First 3 requests should succeed
        assert!(limiter.check(ip).is_ok());
        assert!(limiter.check(ip).is_ok());
        assert!(limiter.check(ip).is_ok());

        // 4th request should fail
        assert!(limiter.check(ip).is_err());

        // Check count
        assert_eq!(limiter.get_count(ip), 3);
        assert_eq!(limiter.get_remaining(ip), 0);
    }

    #[tokio::test]
    async fn test_window_reset() {
        let config = RateLimitConfig {
            max_requests: 2,
            window: Duration::from_millis(100),
            cleanup_interval: Duration::from_secs(60),
        };

        let limiter = RateLimiter::new(config);
        let ip = IpAddr::from_str("192.168.1.1").unwrap();

        // Use up quota
        assert!(limiter.check(ip).is_ok());
        assert!(limiter.check(ip).is_ok());
        assert!(limiter.check(ip).is_err());

        // Wait for window to expire
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should work again
        assert!(limiter.check(ip).is_ok());
    }

    #[tokio::test]
    async fn test_reset() {
        let config = RateLimitConfig {
            max_requests: 2,
            window: Duration::from_secs(10),
            cleanup_interval: Duration::from_secs(60),
        };

        let limiter = RateLimiter::new(config);
        let ip = IpAddr::from_str("10.0.0.1").unwrap();

        // Use up quota
        assert!(limiter.check(ip).is_ok());
        assert!(limiter.check(ip).is_ok());
        assert!(limiter.check(ip).is_err());

        // Reset
        limiter.reset(ip);

        // Should work again
        assert!(limiter.check(ip).is_ok());
    }

    #[test]
    fn test_stats() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);

        let ip1 = IpAddr::from_str("1.2.3.4").unwrap();
        let ip2 = IpAddr::from_str("5.6.7.8").unwrap();

        let _ = limiter.check(ip1);
        let _ = limiter.check(ip1);
        let _ = limiter.check(ip2);

        let stats = limiter.stats();
        assert_eq!(stats.total_ips, 2);
        assert_eq!(stats.active_requests, 3);
    }
}
