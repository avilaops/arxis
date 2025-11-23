//! Rate Limiting for AI Assistant

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub tokens_per_day: u64,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 30,
            requests_per_hour: 300,
            tokens_per_day: 100_000,
            burst_size: 10,
        }
    }
}

#[derive(Debug, Clone)]
struct UserLimits {
    minute_requests: Vec<Instant>,
    hour_requests: Vec<Instant>,
    daily_tokens: u64,
    daily_reset: Instant,
}

impl UserLimits {
    fn new() -> Self {
        Self {
            minute_requests: Vec::new(),
            hour_requests: Vec::new(),
            daily_tokens: 0,
            daily_reset: Instant::now() + Duration::from_secs(86400),
        }
    }

    fn clean_old_requests(&mut self) {
        let now = Instant::now();
        self.minute_requests.retain(|&t| now.duration_since(t) < Duration::from_secs(60));
        self.hour_requests.retain(|&t| now.duration_since(t) < Duration::from_secs(3600));

        if now > self.daily_reset {
            self.daily_tokens = 0;
            self.daily_reset = now + Duration::from_secs(86400);
        }
    }
}

pub struct RateLimiter {
    config: RateLimitConfig,
    users: Arc<Mutex<HashMap<String, UserLimits>>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_request(&self, user_id: &str) -> Result<(), RateLimitError> {
        let mut users = self.users.lock().unwrap();
        let limits = users.entry(user_id.to_string()).or_insert_with(UserLimits::new);

        limits.clean_old_requests();

        // Check minute limit
        if limits.minute_requests.len() >= self.config.requests_per_minute as usize {
            return Err(RateLimitError::MinuteExceeded {
                limit: self.config.requests_per_minute,
                reset_in: 60 - limits.minute_requests[0].elapsed().as_secs(),
            });
        }

        // Check hour limit
        if limits.hour_requests.len() >= self.config.requests_per_hour as usize {
            return Err(RateLimitError::HourExceeded {
                limit: self.config.requests_per_hour,
                reset_in: 3600 - limits.hour_requests[0].elapsed().as_secs(),
            });
        }

        // Check burst
        let recent_burst = limits.minute_requests.iter()
            .filter(|&&t| t.elapsed() < Duration::from_secs(10))
            .count();
        if recent_burst >= self.config.burst_size as usize {
            return Err(RateLimitError::BurstExceeded {
                limit: self.config.burst_size,
            });
        }

        let now = Instant::now();
        limits.minute_requests.push(now);
        limits.hour_requests.push(now);

        Ok(())
    }

    pub fn check_tokens(&self, user_id: &str, tokens: u64) -> Result<(), RateLimitError> {
        let mut users = self.users.lock().unwrap();
        let limits = users.entry(user_id.to_string()).or_insert_with(UserLimits::new);

        limits.clean_old_requests();

        if limits.daily_tokens + tokens > self.config.tokens_per_day {
            return Err(RateLimitError::TokensExceeded {
                limit: self.config.tokens_per_day,
                used: limits.daily_tokens,
                reset_in: limits.daily_reset.duration_since(Instant::now()).as_secs(),
            });
        }

        limits.daily_tokens += tokens;
        Ok(())
    }

    pub fn get_usage(&self, user_id: &str) -> RateLimitUsage {
        let mut users = self.users.lock().unwrap();
        let limits = users.entry(user_id.to_string()).or_insert_with(UserLimits::new);

        limits.clean_old_requests();

        RateLimitUsage {
            requests_this_minute: limits.minute_requests.len() as u32,
            requests_this_hour: limits.hour_requests.len() as u32,
            tokens_today: limits.daily_tokens,
            minute_limit: self.config.requests_per_minute,
            hour_limit: self.config.requests_per_hour,
            daily_token_limit: self.config.tokens_per_day,
        }
    }

    pub fn reset_user(&self, user_id: &str) {
        let mut users = self.users.lock().unwrap();
        users.remove(user_id);
    }

    pub fn reset_all(&self) {
        let mut users = self.users.lock().unwrap();
        users.clear();
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitUsage {
    pub requests_this_minute: u32,
    pub requests_this_hour: u32,
    pub tokens_today: u64,
    pub minute_limit: u32,
    pub hour_limit: u32,
    pub daily_token_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitError {
    MinuteExceeded { limit: u32, reset_in: u64 },
    HourExceeded { limit: u32, reset_in: u64 },
    TokensExceeded { limit: u64, used: u64, reset_in: u64 },
    BurstExceeded { limit: u32 },
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MinuteExceeded { limit, reset_in } => {
                write!(f, "Rate limit exceeded: {} requests/minute. Reset in {}s", limit, reset_in)
            }
            Self::HourExceeded { limit, reset_in } => {
                write!(f, "Rate limit exceeded: {} requests/hour. Reset in {}s", limit, reset_in)
            }
            Self::TokensExceeded { limit, used, reset_in } => {
                write!(f, "Token quota exceeded: {}/{} tokens. Reset in {}s", used, limit, reset_in)
            }
            Self::BurstExceeded { limit } => {
                write!(f, "Burst limit exceeded: {} requests in 10s", limit)
            }
        }
    }
}

impl std::error::Error for RateLimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rate_limiting() {
        use std::thread;
        use std::time::Duration;

        let config = RateLimitConfig {
            requests_per_minute: 5,
            requests_per_hour: 100,
            tokens_per_day: 10000,
            burst_size: 10, // Higher burst to avoid false failures
        };
        let limiter = RateLimiter::new(config);

        // Should allow 5 requests
        for i in 0..5 {
            let result = limiter.check_request("user1");
            if result.is_err() {
                eprintln!("Request {} failed: {:?}", i, result);
            }
            assert!(result.is_ok());
            thread::sleep(Duration::from_millis(100)); // Small delay to avoid burst detection
        }

        // 6th should fail
        assert!(limiter.check_request("user1").is_err());
    }

    #[test]
    fn test_token_limiting() {
        let config = RateLimitConfig {
            requests_per_minute: 100,
            requests_per_hour: 1000,
            tokens_per_day: 1000,
            burst_size: 50,
        };
        let limiter = RateLimiter::new(config);

        assert!(limiter.check_tokens("user1", 500).is_ok());
        assert!(limiter.check_tokens("user1", 400).is_ok());
        assert!(limiter.check_tokens("user1", 200).is_err()); // Would exceed 1000
    }

    #[test]
    fn test_burst_limiting() {
        let config = RateLimitConfig {
            requests_per_minute: 100,
            requests_per_hour: 1000,
            tokens_per_day: 10000,
            burst_size: 3,
        };
        let limiter = RateLimiter::new(config);

        // 3 rapid requests should work
        for _ in 0..3 {
            assert!(limiter.check_request("user1").is_ok());
        }

        // 4th rapid request should fail
        assert!(limiter.check_request("user1").is_err());
    }

    #[test]
    fn test_usage_tracking() {
        let limiter = RateLimiter::default();

        limiter.check_request("user1").unwrap();
        limiter.check_tokens("user1", 500).unwrap();

        let usage = limiter.get_usage("user1");
        assert_eq!(usage.requests_this_minute, 1);
        assert_eq!(usage.tokens_today, 500);
    }

    #[test]
    fn test_user_isolation() {
        let config = RateLimitConfig {
            requests_per_minute: 2,
            requests_per_hour: 100,
            tokens_per_day: 10000,
            burst_size: 10,
        };
        let limiter = RateLimiter::new(config);

        assert!(limiter.check_request("user1").is_ok());
        assert!(limiter.check_request("user1").is_ok());
        assert!(limiter.check_request("user1").is_err());

        // user2 should have independent limits
        assert!(limiter.check_request("user2").is_ok());
        assert!(limiter.check_request("user2").is_ok());
    }

    #[test]
    fn test_reset() {
        let config = RateLimitConfig {
            requests_per_minute: 2,
            requests_per_hour: 100,
            tokens_per_day: 10000,
            burst_size: 10,
        };
        let limiter = RateLimiter::new(config);

        limiter.check_request("user1").unwrap();
        limiter.check_request("user1").unwrap();
        assert!(limiter.check_request("user1").is_err());

        limiter.reset_user("user1");
        assert!(limiter.check_request("user1").is_ok());
    }
}
