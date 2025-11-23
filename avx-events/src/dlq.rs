//! Dead Letter Queue for failed event processing.

use crate::{Event, EventEnvelope, EventMetadata, Result};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Maximum number of events to keep in DLQ by default.
const DEFAULT_MAX_SIZE: usize = 10_000;

/// Dead Letter Queue for storing events that failed processing.
///
/// When event handlers fail repeatedly, events can be moved to the DLQ
/// for later inspection, retry, or manual intervention.
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::{EventBus, DeadLetterQueue, Event};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct UserCreated {
///     user_id: String,
/// }
///
/// impl Event for UserCreated {
///     fn event_type(&self) -> &'static str { "user.created" }
///     fn aggregate_id(&self) -> String { self.user_id.clone() }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let bus = EventBus::new();
///     let dlq = DeadLetterQueue::new();
///
///     let mut subscriber = bus.subscribe::<UserCreated>().await;
///
///     tokio::spawn({
///         let dlq = dlq.clone();
///         async move {
///             while let Some(envelope) = subscriber.recv().await {
///                 match process_event(&envelope.event).await {
///                     Ok(_) => {},
///                     Err(e) => {
///                         // Send to DLQ after max retries
///                         dlq.add(envelope, format!("{}", e)).await;
///                     }
///                 }
///             }
///         }
///     });
/// }
///
/// async fn process_event(event: &UserCreated) -> Result<(), String> {
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct DeadLetterQueue {
    inner: Arc<RwLock<DLQInner>>,
}

struct DLQInner {
    events: VecDeque<DeadLetterEvent>,
    max_size: usize,
}

/// An event in the dead letter queue with failure information.
#[derive(Debug, Clone)]
pub struct DeadLetterEvent {
    /// The original event metadata.
    pub metadata: EventMetadata,

    /// Serialized event data.
    pub data: serde_json::Value,

    /// The error that caused the event to be dead-lettered.
    pub error: String,

    /// Number of times processing was attempted.
    pub retry_count: u32,

    /// Timestamp when added to DLQ.
    pub dead_lettered_at: i64,
}

impl DeadLetterQueue {
    /// Creates a new dead letter queue with default max size.
    pub fn new() -> Self {
        Self::with_max_size(DEFAULT_MAX_SIZE)
    }

    /// Creates a new dead letter queue with specified max size.
    ///
    /// When the queue is full, oldest events are discarded (FIFO).
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(DLQInner {
                events: VecDeque::with_capacity(max_size),
                max_size,
            })),
        }
    }

    /// Adds an event to the dead letter queue.
    pub async fn add<E: Event + serde::Serialize>(
        &self,
        envelope: EventEnvelope<E>,
        error: impl Into<String>,
    ) {
        self.add_with_retries(envelope, error, 0).await;
    }

    /// Adds an event to the DLQ with retry count.
    pub async fn add_with_retries<E: Event + serde::Serialize>(
        &self,
        envelope: EventEnvelope<E>,
        error: impl Into<String>,
        retry_count: u32,
    ) {
        let data = match serde_json::to_value(&envelope.event) {
            Ok(data) => data,
            Err(e) => {
                warn!(error = %e, "Failed to serialize event for DLQ");
                return;
            }
        };

        let dead_letter_event = DeadLetterEvent {
            metadata: envelope.metadata,
            data,
            error: error.into(),
            retry_count,
            dead_lettered_at: chrono::Utc::now().timestamp_millis(),
        };

        let mut inner = self.inner.write().await;

        // Remove oldest if at capacity
        if inner.events.len() >= inner.max_size {
            inner.events.pop_front();
            warn!("DLQ at capacity, discarding oldest event");
        }

        inner.events.push_back(dead_letter_event.clone());

        info!(
            event_type = %dead_letter_event.metadata.event_type,
            event_id = %dead_letter_event.metadata.event_id,
            error = %dead_letter_event.error,
            "Event added to DLQ"
        );
    }

    /// Lists all events in the dead letter queue.
    pub async fn list(&self) -> Vec<DeadLetterEvent> {
        let inner = self.inner.read().await;
        inner.events.iter().cloned().collect()
    }

    /// Returns the number of events in the DLQ.
    pub async fn len(&self) -> usize {
        let inner = self.inner.read().await;
        inner.events.len()
    }

    /// Checks if the DLQ is empty.
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    /// Removes and returns the oldest event from the DLQ.
    pub async fn pop(&self) -> Option<DeadLetterEvent> {
        let mut inner = self.inner.write().await;
        inner.events.pop_front()
    }

    /// Removes a specific event by ID from the DLQ.
    pub async fn remove(&self, event_id: &str) -> Option<DeadLetterEvent> {
        let mut inner = self.inner.write().await;

        if let Some(pos) = inner.events.iter().position(|e| e.metadata.event_id == event_id) {
            inner.events.remove(pos)
        } else {
            None
        }
    }

    /// Clears all events from the DLQ.
    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.events.clear();
        info!("Dead letter queue cleared");
    }

    /// Deserializes a dead letter event to a specific type.
    pub fn deserialize_event<E: Event + for<'de> serde::Deserialize<'de>>(
        &self,
        event: &DeadLetterEvent,
    ) -> Result<E> {
        serde_json::from_value(event.data.clone())
            .map_err(|e| crate::Error::Serialization(e.to_string()))
    }
}

impl Default for DeadLetterQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Retry strategy for failed events.
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    /// Maximum number of retries before sending to DLQ.
    pub max_retries: u32,

    /// Base delay between retries in milliseconds.
    pub base_delay_ms: u64,

    /// Whether to use exponential backoff.
    pub exponential_backoff: bool,
}

impl RetryStrategy {
    /// Creates a new retry strategy with sensible defaults.
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            exponential_backoff: true,
        }
    }

    /// Sets the maximum number of retries.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the base delay between retries.
    pub fn with_base_delay(mut self, delay_ms: u64) -> Self {
        self.base_delay_ms = delay_ms;
        self
    }

    /// Enables or disables exponential backoff.
    pub fn with_exponential_backoff(mut self, enabled: bool) -> Self {
        self.exponential_backoff = enabled;
        self
    }

    /// Calculates the delay for a given retry attempt.
    pub fn delay_for_retry(&self, retry_count: u32) -> std::time::Duration {
        let delay_ms = if self.exponential_backoff {
            self.base_delay_ms * 2_u64.pow(retry_count)
        } else {
            self.base_delay_ms
        };

        std::time::Duration::from_millis(delay_ms)
    }

    /// Checks if another retry should be attempted.
    pub fn should_retry(&self, retry_count: u32) -> bool {
        retry_count < self.max_retries
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        id: String,
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &'static str {
            "test.event"
        }
        fn aggregate_id(&self) -> String {
            self.id.clone()
        }
    }

    #[tokio::test]
    async fn test_dlq_add_and_list() {
        let dlq = DeadLetterQueue::new();
        let event = TestEvent { id: "123".into() };
        let envelope = EventEnvelope::new(event);

        dlq.add(envelope, "Test error").await;

        let events = dlq.list().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].error, "Test error");
    }

    #[tokio::test]
    async fn test_dlq_max_size() {
        let dlq = DeadLetterQueue::with_max_size(2);

        for i in 0..3 {
            let event = TestEvent { id: i.to_string() };
            let envelope = EventEnvelope::new(event);
            dlq.add(envelope, "Error").await;
        }

        let events = dlq.list().await;
        assert_eq!(events.len(), 2);
        // First event should be removed
        assert_eq!(events[0].metadata.aggregate_id, "1");
    }

    #[tokio::test]
    async fn test_retry_strategy() {
        let strategy = RetryStrategy::new()
            .with_max_retries(3)
            .with_base_delay(100);

        assert!(strategy.should_retry(0));
        assert!(strategy.should_retry(2));
        assert!(!strategy.should_retry(3));

        let delay = strategy.delay_for_retry(2);
        assert_eq!(delay.as_millis(), 400); // 100 * 2^2
    }
}
