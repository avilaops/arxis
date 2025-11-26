use crate::types::{ScraperError, ScraperResult};
use crate::anti_detect::{AntiDetectionStrategy, ProxyPool, RateLimiter};
use reqwest::{Client, Response};
use scraper::Html;
use std::time::{Duration, Instant};
use std::sync::Arc;

#[derive(Clone)]
pub struct ScraperEngine {
    client: Client,
    rate_limiter: Arc<RateLimiter>,
    proxy_pool: Arc<ProxyPool>,
    anti_detect: Arc<AntiDetectionStrategy>,
    max_retries: u32,
}

pub struct ScraperEngineBuilder {
    rate_limit: u32,
    max_retries: u32,
    request_timeout: Duration,
    anti_detect: Option<AntiDetectionStrategy>,
    proxies: Vec<String>,
}

impl Default for ScraperEngineBuilder {
    fn default() -> Self {
        Self {
            rate_limit: 10,
            max_retries: 3,
            request_timeout: Duration::from_secs(30),
            anti_detect: None,
            proxies: Vec::new(),
        }
    }
}

impl ScraperEngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rate_limit(mut self, requests_per_second: u32) -> Self {
        self.rate_limit = requests_per_second;
        self
    }

    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    pub fn with_anti_detection(mut self, strategy: AntiDetectionStrategy) -> Self {
        self.anti_detect = Some(strategy);
        self
    }

    pub fn with_proxies(mut self, proxies: Vec<String>) -> Self {
        self.proxies = proxies;
        self
    }

    pub fn build(self) -> ScraperResult<ScraperEngine> {
        let client = Client::builder()
            .timeout(self.request_timeout)
            .cookie_store(true)
            .gzip(true)
            .build()
            .map_err(|e| ScraperError::HttpError(e))?;

        let anti_detect = self.anti_detect.unwrap_or_default();
        let rate_limiter = RateLimiter::new(self.rate_limit);
        let proxy_pool = ProxyPool::new(self.proxies);

        Ok(ScraperEngine {
            client,
            rate_limiter: Arc::new(rate_limiter),
            proxy_pool: Arc::new(proxy_pool),
            anti_detect: Arc::new(anti_detect),
            max_retries: self.max_retries,
        })
    }
}

impl ScraperEngine {
    pub fn builder() -> ScraperEngineBuilder {
        ScraperEngineBuilder::new()
    }

    /// Scrape a single URL with retry logic
    pub async fn scrape_url(&self, url: &str) -> ScraperResult<Html> {
        // Check robots.txt
        if !self.anti_detect.check_robots_txt(url).await {
            return Err(ScraperError::RobotsTxtDisallowed);
        }

        let mut attempts = 0;

        while attempts < self.max_retries {
            // Rate limiting
            self.rate_limiter.wait().await;

            // Apply delay
            let delay = self.anti_detect.calculate_delay();
            tokio::time::sleep(delay).await;

            match self.fetch_url(url).await {
                Ok(html) => {
                    tracing::info!("Successfully scraped: {}", url);
                    return Ok(html);
                },
                Err(e) => {
                    attempts += 1;
                    tracing::warn!(
                        "Attempt {}/{} failed for {}: {}",
                        attempts, self.max_retries, url, e
                    );

                    if attempts < self.max_retries {
                        // Exponential backoff
                        let backoff = Duration::from_secs(2_u64.pow(attempts));
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }

        Err(ScraperError::MaxAttemptsExceeded)
    }

    /// Scrape multiple URLs in parallel
    pub async fn scrape_batch(
        &self,
        urls: Vec<String>,
        max_concurrent: usize,
    ) -> Vec<ScraperResult<Html>> {
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut tasks = Vec::new();

        for url in urls {
            let sem = semaphore.clone();
            let engine = self.clone();

            tasks.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                engine.scrape_url(&url).await
            }));
        }

        let mut results = Vec::new();
        for task in tasks {
            results.push(task.await.unwrap());
        }

        results
    }

    /// Internal fetch with headers and proxy support
    async fn fetch_url(&self, url: &str) -> ScraperResult<Html> {
        let headers = self.anti_detect.build_headers();

        let response = self.client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ScraperError::HttpError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let text = response.text().await?;
        let html = Html::parse_document(&text);

        Ok(html)
    }

    /// Get request with custom headers
    pub async fn get(&self, url: &str) -> ScraperResult<Response> {
        self.rate_limiter.wait().await;

        let headers = self.anti_detect.build_headers();

        let response = self.client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        Ok(response)
    }

    /// POST request with custom headers
    pub async fn post(&self, url: &str, body: String) -> ScraperResult<Response> {
        self.rate_limiter.wait().await;

        let headers = self.anti_detect.build_headers();

        let response = self.client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder() {
        let engine = ScraperEngine::builder()
            .with_rate_limit(5)
            .with_max_retries(2)
            .build();

        assert!(engine.is_ok());
    }
}
