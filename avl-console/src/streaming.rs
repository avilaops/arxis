//! Advanced Streaming Enhancements for AI Assistant
//!
//! Adds metadata, progress tracking, and cancellation support to SSE streams.

use serde::{Deserialize, Serialize};
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub token_type: TokenType,
    pub confidence: f32,
    pub position: usize,
    pub total_estimated: Option<usize>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Text,
    Code,
    Keyword,
    Operator,
    Function,
    Comment,
    Delimiter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamToken {
    pub content: String,
    pub metadata: StreamMetadata,
}

impl StreamToken {
    pub fn new(content: String, token_type: TokenType, position: usize) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            content,
            metadata: StreamMetadata {
                token_type,
                confidence: 1.0,
                position,
                total_estimated: None,
                timestamp,
            },
        }
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.metadata.confidence = confidence;
        self
    }

    pub fn with_total_estimated(mut self, total: usize) -> Self {
        self.metadata.total_estimated = Some(total);
        self
    }

    pub fn to_sse_event(&self) -> String {
        format!(
            "event: token\ndata: {}\n\n",
            serde_json::to_string(self).unwrap_or_else(|_| self.content.clone())
        )
    }
}

pub struct EnhancedStream {
    inner: Pin<Box<dyn Stream<Item = String> + Send>>,
    position: usize,
    cancelled: bool,
}

impl EnhancedStream {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = String> + Send + 'static,
    {
        Self {
            inner: Box::pin(stream),
            position: 0,
            cancelled: false,
        }
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl Stream for EnhancedStream {
    type Item = StreamToken;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.cancelled {
            return Poll::Ready(None);
        }

        match self.inner.as_mut().poll_next(cx) {
            Poll::Ready(Some(content)) => {
                let token = StreamToken::new(
                    content,
                    TokenType::Text,
                    self.position,
                );
                self.position += 1;
                Poll::Ready(Some(token))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamProgress {
    pub tokens_generated: usize,
    pub estimated_total: Option<usize>,
    pub elapsed_ms: u64,
    pub tokens_per_second: f32,
    pub status: StreamStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamStatus {
    Starting,
    Streaming,
    Completed,
    Cancelled,
    Error(String),
}

pub struct ProgressTracker {
    start_time: std::time::Instant,
    tokens_generated: usize,
    estimated_total: Option<usize>,
    status: StreamStatus,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            tokens_generated: 0,
            estimated_total: None,
            status: StreamStatus::Starting,
        }
    }

    pub fn increment(&mut self) {
        self.tokens_generated += 1;
        self.status = StreamStatus::Streaming;
    }

    pub fn set_estimated_total(&mut self, total: usize) {
        self.estimated_total = Some(total);
    }

    pub fn complete(&mut self) {
        self.status = StreamStatus::Completed;
    }

    pub fn cancel(&mut self) {
        self.status = StreamStatus::Cancelled;
    }

    pub fn error(&mut self, message: String) {
        self.status = StreamStatus::Error(message);
    }

    pub fn get_progress(&self) -> StreamProgress {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;
        let tokens_per_second = if elapsed_ms > 0 {
            (self.tokens_generated as f64 / elapsed.as_secs_f64()) as f32
        } else {
            0.0
        };

        StreamProgress {
            tokens_generated: self.tokens_generated,
            estimated_total: self.estimated_total,
            elapsed_ms,
            tokens_per_second,
            status: self.status.clone(),
        }
    }

    pub fn to_sse_event(&self) -> String {
        format!(
            "event: progress\ndata: {}\n\n",
            serde_json::to_string(&self.get_progress()).unwrap_or_else(|_| "{}".to_string())
        )
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    #[test]
    fn test_stream_token_creation() {
        let token = StreamToken::new("SELECT".to_string(), TokenType::Keyword, 0);
        assert_eq!(token.content, "SELECT");
        assert_eq!(token.metadata.position, 0);
        assert_eq!(token.metadata.confidence, 1.0);
    }

    #[test]
    fn test_stream_token_with_confidence() {
        let token = StreamToken::new("SELECT".to_string(), TokenType::Keyword, 0)
            .with_confidence(0.95);
        assert_eq!(token.metadata.confidence, 0.95);
    }

    #[test]
    fn test_progress_tracker() {
        let mut tracker = ProgressTracker::new();

        tracker.increment();
        tracker.increment();

        let progress = tracker.get_progress();
        assert_eq!(progress.tokens_generated, 2);
        assert!(matches!(progress.status, StreamStatus::Streaming));
    }

    #[test]
    fn test_progress_tracker_completion() {
        let mut tracker = ProgressTracker::new();
        tracker.increment();
        tracker.complete();

        let progress = tracker.get_progress();
        assert!(matches!(progress.status, StreamStatus::Completed));
    }

    #[test]
    fn test_progress_tracker_cancellation() {
        let mut tracker = ProgressTracker::new();
        tracker.increment();
        tracker.cancel();

        let progress = tracker.get_progress();
        assert!(matches!(progress.status, StreamStatus::Cancelled));
    }

    #[tokio::test]
    async fn test_enhanced_stream() {
        use futures::StreamExt;

        let base_stream = stream::iter(vec![
            "token1".to_string(),
            "token2".to_string(),
            "token3".to_string(),
        ]);

        let mut enhanced = EnhancedStream::new(base_stream);

        let mut count = 0;
        while let Some(token) = enhanced.next().await {
            assert!(token.content.starts_with("token"));
            count += 1;
        }

        assert_eq!(count, 3);
        assert_eq!(enhanced.position(), 3);
    }

    #[tokio::test]
    async fn test_stream_cancellation() {
        use futures::StreamExt;

        let base_stream = stream::iter(vec![
            "token1".to_string(),
            "token2".to_string(),
            "token3".to_string(),
        ]);

        let mut enhanced = EnhancedStream::new(base_stream);

        enhanced.cancel();
        assert!(enhanced.is_cancelled());

        let result = enhanced.next().await;
        assert!(result.is_none());
    }
}
