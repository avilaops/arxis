use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;
use tokio::time::{sleep, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AntiDetectionStrategy {
    user_agents: Vec<String>,
    request_delay_ms: u64,
    randomize_delay: bool,
}

impl Default for AntiDetectionStrategy {
    fn default() -> Self {
        Self {
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.2; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            ],
            request_delay_ms: 1000,
            randomize_delay: true,
        }
    }
}

impl AntiDetectionStrategy {
    pub fn new(delay_ms: u64, randomize: bool) -> Self {
        Self {
            user_agents: Self::default().user_agents,
            request_delay_ms: delay_ms,
            randomize_delay: randomize,
        }
    }

    /// Get a random user agent
    pub fn get_user_agent(&self) -> &str {
        use rand::seq::SliceRandom;
        self.user_agents
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    /// Calculate delay with optional randomization
    pub fn calculate_delay(&self) -> Duration {
        let base = self.request_delay_ms;

        if self.randomize_delay {
            use rand::Rng;
            let jitter = rand::thread_rng().gen_range(0..base / 2);
            Duration::from_millis(base + jitter)
        } else {
            Duration::from_millis(base)
        }
    }

    /// Check if URL is allowed by robots.txt
    pub async fn check_robots_txt(&self, url: &str) -> bool {
        // TODO: Implement proper robots.txt parsing
        // For now, always allow (should be implemented properly)
        true
    }

    /// Build HTTP headers with anti-detection measures
    pub fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            "User-Agent",
            HeaderValue::from_str(self.get_user_agent()).unwrap()
        );
        headers.insert(
            "Accept",
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        );
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("en-US,en;q=0.9,pt;q=0.8")
        );
        headers.insert(
            "Accept-Encoding",
            HeaderValue::from_static("gzip, deflate, br")
        );
        headers.insert(
            "DNT",
            HeaderValue::from_static("1")
        );
        headers.insert(
            "Connection",
            HeaderValue::from_static("keep-alive")
        );
        headers.insert(
            "Upgrade-Insecure-Requests",
            HeaderValue::from_static("1")
        );
        headers.insert(
            "Sec-Fetch-Dest",
            HeaderValue::from_static("document")
        );
        headers.insert(
            "Sec-Fetch-Mode",
            HeaderValue::from_static("navigate")
        );
        headers.insert(
            "Sec-Fetch-Site",
            HeaderValue::from_static("none")
        );

        headers
    }
}

/// Rate limiter using token bucket algorithm
pub struct RateLimiter {
    permits_per_second: u32,
    last_request: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            permits_per_second: requests_per_second,
            last_request: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Wait until a request can be made
    pub async fn wait(&self) {
        let mut last = self.last_request.lock().await;
        let now = Instant::now();
        let elapsed = now.duration_since(*last);

        let min_interval = Duration::from_secs(1) / self.permits_per_second;

        if elapsed < min_interval {
            let wait_time = min_interval - elapsed;
            drop(last); // Release lock before sleeping
            sleep(wait_time).await;

            // Re-acquire lock and update
            let mut last = self.last_request.lock().await;
            *last = Instant::now();
        } else {
            *last = now;
        }
    }
}

/// Proxy pool for rotation
pub struct ProxyPool {
    proxies: Vec<String>,
    current_index: Arc<Mutex<usize>>,
}

impl ProxyPool {
    pub fn new(proxies: Vec<String>) -> Self {
        Self {
            proxies,
            current_index: Arc::new(Mutex::new(0)),
        }
    }

    /// Get next proxy in round-robin fashion
    pub async fn get_proxy(&self) -> Option<String> {
        if self.proxies.is_empty() {
            return None;
        }

        let mut index = self.current_index.lock().await;
        let proxy = self.proxies[*index].clone();

        *index = (*index + 1) % self.proxies.len();

        Some(proxy)
    }

    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent_rotation() {
        let strategy = AntiDetectionStrategy::default();
        let ua1 = strategy.get_user_agent();
        let ua2 = strategy.get_user_agent();

        // Both should be valid user agents
        assert!(!ua1.is_empty());
        assert!(!ua2.is_empty());
    }

    #[test]
    fn test_delay_calculation() {
        let strategy = AntiDetectionStrategy::new(1000, false);
        let delay = strategy.calculate_delay();
        assert_eq!(delay, Duration::from_millis(1000));

        let strategy_random = AntiDetectionStrategy::new(1000, true);
        let delay_random = strategy_random.calculate_delay();
        assert!(delay_random >= Duration::from_millis(1000));
        assert!(delay_random <= Duration::from_millis(1500));
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(10); // 10 requests per second

        let start = Instant::now();
        limiter.wait().await;
        limiter.wait().await;
        let elapsed = start.elapsed();

        // Should take at least 100ms for 2 requests at 10/sec
        assert!(elapsed >= Duration::from_millis(90));
    }

    #[tokio::test]
    async fn test_proxy_pool() {
        let proxies = vec![
            "http://proxy1:8080".to_string(),
            "http://proxy2:8080".to_string(),
        ];

        let pool = ProxyPool::new(proxies);

        let p1 = pool.get_proxy().await;
        let p2 = pool.get_proxy().await;
        let p3 = pool.get_proxy().await;

        assert!(p1.is_some());
        assert!(p2.is_some());
        assert_eq!(p1, p3); // Should wrap around
    }
}
