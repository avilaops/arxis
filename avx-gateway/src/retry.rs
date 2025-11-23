//! Retry logic with exponential backoff

use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Retry policy
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,

    /// Initial backoff duration
    pub initial_backoff: Duration,

    /// Maximum backoff duration
    pub max_backoff: Duration,

    /// Backoff multiplier
    pub multiplier: f64,

    /// Add jitter to backoff
    pub jitter: bool,

    /// Retry on specific status codes
    pub retry_on_status: Vec<u16>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(10),
            multiplier: 2.0,
            jitter: true,
            retry_on_status: vec![500, 502, 503, 504],
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum attempts
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set initial backoff
    pub fn with_initial_backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = backoff;
        self
    }

    /// Set maximum backoff
    pub fn with_max_backoff(mut self, backoff: Duration) -> Self {
        self.max_backoff = backoff;
        self
    }

    /// Set backoff multiplier
    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Enable/disable jitter
    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }

    /// Set status codes to retry on
    pub fn retry_on_status(mut self, status_codes: Vec<u16>) -> Self {
        self.retry_on_status = status_codes;
        self
    }

    /// Check if should retry based on status code
    pub fn should_retry(&self, status: u16) -> bool {
        self.retry_on_status.contains(&status)
    }

    /// Calculate backoff duration for attempt
    pub fn backoff_duration(&self, attempt: u32) -> Duration {
        let base_duration = self.initial_backoff.as_secs_f64()
            * self.multiplier.powi(attempt as i32);

        let duration = Duration::from_secs_f64(
            base_duration.min(self.max_backoff.as_secs_f64())
        );

        if self.jitter {
            self.add_jitter(duration)
        } else {
            duration
        }
    }

    /// Add jitter to duration
    fn add_jitter(&self, duration: Duration) -> Duration {
        use rand::Rng;
        let jitter_factor = rand::thread_rng().gen_range(0.5..1.5);
        Duration::from_secs_f64(duration.as_secs_f64() * jitter_factor)
    }
}

/// Retry executor
pub struct RetryExecutor {
    policy: RetryPolicy,
}

impl RetryExecutor {
    /// Create a new retry executor
    pub fn new(policy: RetryPolicy) -> Self {
        Self { policy }
    }

    /// Execute with retry
    pub async fn execute<F, Fut, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;

            debug!("Attempt {}/{}", attempt, self.policy.max_attempts);

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt >= self.policy.max_attempts {
                        warn!("Max retry attempts reached");
                        return Err(e);
                    }

                    let backoff = self.policy.backoff_duration(attempt - 1);
                    debug!("Retry after {:?}", backoff);
                    sleep(backoff).await;
                }
            }
        }
    }

    /// Execute with retry and status code checking
    pub async fn execute_with_status<F, Fut>(
        &self,
        mut operation: F,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;

            debug!("Attempt {}/{}", attempt, self.policy.max_attempts);

            match operation().await {
                Ok(response) => {
                    let status = response.status().as_u16();

                    if self.policy.should_retry(status) && attempt < self.policy.max_attempts {
                        warn!("Retryable status code {}, retrying", status);
                        let backoff = self.policy.backoff_duration(attempt - 1);
                        sleep(backoff).await;
                        continue;
                    }

                    return Ok(response);
                }
                Err(e) => {
                    if attempt >= self.policy.max_attempts {
                        warn!("Max retry attempts reached");
                        return Err(e);
                    }

                    let backoff = self.policy.backoff_duration(attempt - 1);
                    debug!("Retry after {:?}", backoff);
                    sleep(backoff).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_backoff() {
        let policy = RetryPolicy::new()
            .with_jitter(false)
            .with_initial_backoff(Duration::from_millis(100))
            .with_multiplier(2.0);

        assert_eq!(policy.backoff_duration(0), Duration::from_millis(100));
        assert_eq!(policy.backoff_duration(1), Duration::from_millis(200));
        assert_eq!(policy.backoff_duration(2), Duration::from_millis(400));
    }

    #[test]
    fn test_should_retry() {
        let policy = RetryPolicy::new();

        assert!(policy.should_retry(503));
        assert!(policy.should_retry(502));
        assert!(!policy.should_retry(200));
        assert!(!policy.should_retry(404));
    }

    #[tokio::test]
    async fn test_retry_executor() {
        let policy = RetryPolicy::new()
            .with_max_attempts(3)
            .with_initial_backoff(Duration::from_millis(10));

        let executor = RetryExecutor::new(policy);

        let mut attempts = 0;
        let result = executor
            .execute(|| async {
                attempts += 1;
                if attempts < 3 {
                    Err("failed")
                } else {
                    Ok("success")
                }
            })
            .await;

        assert_eq!(result, Ok("success"));
        assert_eq!(attempts, 3);
    }
}
