//! Topic-based event bus with pattern matching.

use crate::{Event, EventEnvelope, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};

const DEFAULT_CHANNEL_CAPACITY: usize = 1000;

/// Topic-based event bus that supports wildcard subscriptions.
///
/// Topics are hierarchical strings separated by dots (e.g., "users.created", "orders.shipped").
/// Subscribers can use wildcards:
/// - `*` matches exactly one segment (e.g., "users.*" matches "users.created" but not "users.profile.updated")
/// - `**` matches zero or more segments (e.g., "users.**" matches all user events)
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::{Event, TopicBus};
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
///     let bus = TopicBus::new();
///
///     // Subscribe to all user events
///     let mut sub = bus.subscribe("users.**").await;
///
///     // Publish to specific topic
///     let event = UserCreated { user_id: "123".into() };
///     bus.publish_to("users.created", event).await.unwrap();
///
///     if let Some((topic, envelope)) = sub.recv().await {
///         println!("Received on topic {}: {:?}", topic, envelope);
///     }
/// }
/// ```
#[derive(Clone)]
pub struct TopicBus {
    // topic -> broadcast channel
    topics: Arc<RwLock<HashMap<String, broadcast::Sender<TopicEvent>>>>,
    capacity: usize,
}

/// A topic event that carries the topic name and serialized event data.
#[derive(Debug, Clone)]
pub struct TopicEvent {
    pub topic: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: crate::EventMetadata,
}

impl TopicBus {
    /// Creates a new topic bus.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CHANNEL_CAPACITY)
    }

    /// Creates a new topic bus with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::new())),
            capacity,
        }
    }

    /// Publishes an event to a specific topic.
    pub async fn publish_to<E: Event + serde::Serialize>(
        &self,
        topic: impl Into<String>,
        event: E,
    ) -> Result<()> {
        let topic = topic.into();
        let envelope = EventEnvelope::new(event);

        let topic_event = TopicEvent {
            topic: topic.clone(),
            event_type: envelope.metadata.event_type.clone(),
            data: serde_json::to_value(&envelope.event)
                .map_err(|e| crate::Error::Serialization(e.to_string()))?,
            metadata: envelope.metadata,
        };

        debug!(
            topic = %topic,
            event_type = %topic_event.event_type,
            "Publishing to topic"
        );

        let topics = self.topics.read().await;

        // Find all matching topic patterns and send to them
        let mut sent_count = 0;
        for (pattern, tx) in topics.iter() {
            if matches_pattern(&topic, pattern) {
                match tx.send(topic_event.clone()) {
                    Ok(count) => sent_count += count,
                    Err(_) => {
                        warn!(pattern = %pattern, "No active subscribers for pattern");
                    }
                }
            }
        }

        if sent_count > 0 {
            info!(
                topic = %topic,
                subscribers = sent_count,
                "Event published to topic"
            );
        } else {
            warn!(topic = %topic, "No subscribers matched topic");
        }

        Ok(())
    }

    /// Subscribes to events matching a topic pattern.
    ///
    /// Patterns support:
    /// - Exact match: "users.created"
    /// - Single wildcard: "users.*" (matches one segment)
    /// - Multi wildcard: "users.**" (matches zero or more segments)
    pub async fn subscribe(&self, pattern: impl Into<String>) -> TopicSubscriber {
        let pattern = pattern.into();
        let mut topics = self.topics.write().await;

        let tx = topics
            .entry(pattern.clone())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(self.capacity);
                tx
            })
            .clone();

        let rx = tx.subscribe();

        info!(pattern = %pattern, "New topic subscriber registered");

        TopicSubscriber { rx }
    }

    /// Returns the number of active topic patterns.
    pub async fn pattern_count(&self) -> usize {
        self.topics.read().await.len()
    }
}

impl Default for TopicBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Subscriber for topic-based events.
pub struct TopicSubscriber {
    rx: broadcast::Receiver<TopicEvent>,
}

impl TopicSubscriber {
    /// Receives the next topic event.
    pub async fn recv(&mut self) -> Option<TopicEvent> {
        loop {
            match self.rx.recv().await {
                Ok(event) => return Some(event),
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    warn!(
                        skipped_events = skipped,
                        "Topic subscriber lagged behind"
                    );
                    // Continue to next iteration
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }

    /// Tries to receive an event without blocking.
    pub fn try_recv(&mut self) -> Option<TopicEvent> {
        match self.rx.try_recv() {
            Ok(event) => Some(event),
            Err(broadcast::error::TryRecvError::Lagged(skipped)) => {
                warn!(skipped_events = skipped, "Topic subscriber lagged");
                self.try_recv()
            }
            Err(_) => None,
        }
    }

    /// Deserializes the received event to a specific type.
    pub fn deserialize<E: Event + for<'de> serde::Deserialize<'de>>(
        &self,
        event: &TopicEvent,
    ) -> Result<E> {
        serde_json::from_value(event.data.clone())
            .map_err(|e| crate::Error::Serialization(e.to_string()))
    }
}

/// Checks if a topic matches a pattern.
///
/// Patterns:
/// - "exact.match" - exact string match
/// - "prefix.*" - matches one segment after prefix
/// - "prefix.**" - matches any segments after prefix
fn matches_pattern(topic: &str, pattern: &str) -> bool {
    if topic == pattern {
        return true;
    }

    // Handle multi-segment wildcard
    if pattern.ends_with(".**") {
        let prefix = &pattern[..pattern.len() - 3]; // Remove ".**"
        return topic.starts_with(prefix);
    }

    // Handle single-segment wildcard
    if pattern.contains('*') {
        let topic_parts: Vec<&str> = topic.split('.').collect();
        let pattern_parts: Vec<&str> = pattern.split('.').collect();

        if topic_parts.len() != pattern_parts.len() {
            return false;
        }

        for (t, p) in topic_parts.iter().zip(pattern_parts.iter()) {
            if p != &"*" && t != p {
                return false;
            }
        }

        return true;
    }

    false
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

    #[test]
    fn test_pattern_matching() {
        assert!(matches_pattern("users.created", "users.created"));
        assert!(matches_pattern("users.created", "users.*"));
        assert!(matches_pattern("users.profile.updated", "users.**"));
        assert!(matches_pattern("users.created", "users.**"));

        assert!(!matches_pattern("users.created", "orders.*"));
        assert!(!matches_pattern("users.profile.updated", "users.*"));
    }

    #[tokio::test]
    async fn test_topic_publish_subscribe() {
        let bus = TopicBus::new();
        let mut sub = bus.subscribe("users.*").await;

        let event = TestEvent { id: "123".into() };
        bus.publish_to("users.created", event).await.unwrap();

        let received = sub.recv().await.unwrap();
        assert_eq!(received.topic, "users.created");
    }

    #[tokio::test]
    async fn test_wildcard_subscription() {
        let bus = TopicBus::new();
        let mut sub = bus.subscribe("users.**").await;

        let event1 = TestEvent { id: "1".into() };
        let event2 = TestEvent { id: "2".into() };

        bus.publish_to("users.created", event1).await.unwrap();
        bus.publish_to("users.profile.updated", event2).await.unwrap();

        let recv1 = sub.recv().await.unwrap();
        let recv2 = sub.recv().await.unwrap();

        assert_eq!(recv1.topic, "users.created");
        assert_eq!(recv2.topic, "users.profile.updated");
    }
}
