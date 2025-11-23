//! Testing utilities for event-driven systems.

use crate::{Event, EventEnvelope, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mock event bus for testing.
///
/// Records all published events for inspection in tests.
///
/// # Example
///
/// ```rust
/// use avx_events::testing::MockEventBus;
/// use avx_events::Event;
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
/// #[tokio::test]
/// async fn test_user_service() {
///     let bus = MockEventBus::new();
///
///     // Your service publishes events
///     bus.publish(UserCreated {
///         user_id: "123".into(),
///     }).await.unwrap();
///
///     // Assert events were published
///     let events = bus.published_events::<UserCreated>().await;
///     assert_eq!(events.len(), 1);
///     assert_eq!(events[0].user_id, "123");
/// }
/// ```
#[derive(Clone)]
pub struct MockEventBus {
    events: Arc<RwLock<HashMap<String, Vec<serde_json::Value>>>>,
}

impl MockEventBus {
    /// Creates a new mock event bus.
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Publishes an event (records it for later inspection).
    pub async fn publish<E: Event + serde::Serialize>(&self, event: E) -> Result<()> {
        let event_type = event.event_type().to_string();
        let value = serde_json::to_value(&event)
            .map_err(|e| crate::Error::Serialization(e.to_string()))?;

        let mut events = self.events.write().await;
        events
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(value);

        Ok(())
    }

    /// Returns all published events of a specific type.
    pub async fn published_events<E: Event + for<'de> serde::Deserialize<'de>>(&self) -> Vec<E> {
        let events = self.events.read().await;

        events
            .values()
            .flatten()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect()
    }    /// Returns the number of events published of a specific type.
    pub async fn event_count<E: Event>(&self) -> usize {
        let events = self.events.read().await;
        let event_type = std::any::type_name::<E>();

        events
            .get(event_type)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Clears all recorded events.
    pub async fn clear(&self) {
        let mut events = self.events.write().await;
        events.clear();
    }

    /// Checks if any events were published.
    pub async fn has_events(&self) -> bool {
        let events = self.events.read().await;
        !events.is_empty()
    }
}

impl Default for MockEventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock event store for testing event sourcing.
#[derive(Clone)]
pub struct MockEventStore {
    streams: Arc<RwLock<HashMap<String, Vec<serde_json::Value>>>>,
}

impl MockEventStore {
    /// Creates a new mock event store.
    pub fn new() -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Appends events to a stream.
    pub async fn append<E: Event + serde::Serialize>(
        &self,
        aggregate_id: impl Into<String>,
        events: Vec<E>,
    ) -> Result<()> {
        let aggregate_id = aggregate_id.into();
        let mut streams = self.streams.write().await;

        let stream = streams.entry(aggregate_id).or_insert_with(Vec::new);

        for event in events {
            let value = serde_json::to_value(&event)
                .map_err(|e| crate::Error::Serialization(e.to_string()))?;
            stream.push(value);
        }

        Ok(())
    }

    /// Gets all events for an aggregate.
    pub async fn get_events<E: Event + for<'de> serde::Deserialize<'de>>(
        &self,
        aggregate_id: impl Into<String>,
    ) -> Result<Vec<E>> {
        let aggregate_id = aggregate_id.into();
        let streams = self.streams.read().await;

        let stream = streams
            .get(&aggregate_id)
            .ok_or_else(|| crate::Error::not_found("Aggregate not found"))?;

        let events: Result<Vec<E>> = stream
            .iter()
            .map(|v| {
                serde_json::from_value(v.clone())
                    .map_err(|e| crate::Error::Serialization(e.to_string()))
            })
            .collect();

        events
    }

    /// Returns the version (event count) of an aggregate.
    pub async fn version(&self, aggregate_id: impl Into<String>) -> u64 {
        let aggregate_id = aggregate_id.into();
        let streams = self.streams.read().await;

        streams
            .get(&aggregate_id)
            .map(|s| s.len() as u64)
            .unwrap_or(0)
    }

    /// Clears all stored events.
    pub async fn clear(&self) {
        let mut streams = self.streams.write().await;
        streams.clear();
    }
}

impl Default for MockEventStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Test event builder for creating test events easily.
pub struct EventBuilder<E> {
    event: E,
}

impl<E: Event> EventBuilder<E> {
    /// Creates a new event builder.
    pub fn new(event: E) -> Self {
        Self { event }
    }

    /// Builds the event.
    pub fn build(self) -> E {
        self.event
    }

    /// Builds an event envelope.
    pub fn build_envelope(self) -> EventEnvelope<E> {
        EventEnvelope::new(self.event)
    }
}

/// Assertion helpers for testing events.
pub mod assertions {
    use crate::{Event, EventEnvelope};

    /// Asserts that an event has the expected type.
    pub fn assert_event_type<E: Event>(envelope: &EventEnvelope<E>, expected: &str) {
        assert_eq!(
            envelope.metadata.event_type, expected,
            "Event type mismatch"
        );
    }

    /// Asserts that an event has the expected aggregate ID.
    pub fn assert_aggregate_id<E: Event>(envelope: &EventEnvelope<E>, expected: &str) {
        assert_eq!(
            envelope.metadata.aggregate_id, expected,
            "Aggregate ID mismatch"
        );
    }

    /// Asserts that metadata has a correlation ID.
    pub fn assert_has_correlation_id<E: Event>(envelope: &EventEnvelope<E>) {
        assert!(
            envelope.metadata.correlation_id.is_some(),
            "Expected correlation ID to be present"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        id: String,
        data: String,
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
    async fn test_mock_event_bus() {
        let bus = MockEventBus::new();

        bus.publish(TestEvent {
            id: "1".into(),
            data: "test".into(),
        })
        .await
        .unwrap();

        let events = bus.published_events::<TestEvent>().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, "1");
    }

    #[tokio::test]
    async fn test_mock_event_store() {
        let store = MockEventStore::new();

        store
            .append(
                "agg-1",
                vec![TestEvent {
                    id: "agg-1".into(),
                    data: "event1".into(),
                }],
            )
            .await
            .unwrap();

        let events = store.get_events::<TestEvent>("agg-1").await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(store.version("agg-1").await, 1);
    }
}
