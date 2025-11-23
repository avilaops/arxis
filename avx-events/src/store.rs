//! Event store for event sourcing patterns.

use crate::{Event, EventMetadata, Result};
use crate::event::StoredEvent;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Event store that persists events in append-only fashion.
///
/// Supports event sourcing patterns where state is derived by replaying events.
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::{EventStore, AggregateRoot, Event};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct UserCreated {
///     user_id: String,
///     email: String,
/// }
///
/// impl Event for UserCreated {
///     fn event_type(&self) -> &'static str { "user.created" }
///     fn aggregate_id(&self) -> String { self.user_id.clone() }
/// }
///
/// #[derive(Default)]
/// struct UserAggregate {
///     id: String,
///     email: String,
///     version: u64,
/// }
///
/// impl AggregateRoot for UserAggregate {
///     type Event = UserCreated;
///
///     fn apply(&mut self, event: Self::Event) {
///         self.id = event.user_id;
///         self.email = event.email;
///         self.version += 1;
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let store = EventStore::new();
///
///     // Append events
///     store.append("user-123", vec![
///         UserCreated {
///             user_id: "user-123".into(),
///             email: "user@example.com".into(),
///         }
///     ]).await.unwrap();
///
///     // Replay to rebuild state
///     let events = store.get_events::<UserCreated>("user-123", 0).await.unwrap();
///     let mut user = UserAggregate::default();
///     for event in events {
///         user.apply(event);
///     }
/// }
/// ```
pub struct EventStore {
    // aggregate_id -> list of events
    streams: Arc<RwLock<HashMap<String, Vec<StoredEvent>>>>,
}

impl EventStore {
    /// Creates a new in-memory event store.
    pub fn new() -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Appends events to an aggregate's event stream.
    ///
    /// Events are assigned sequential version numbers starting from the current
    /// stream length.
    pub async fn append<E: Event + serde::Serialize>(
        &self,
        aggregate_id: impl Into<String>,
        events: Vec<E>,
    ) -> Result<()> {
        let aggregate_id = aggregate_id.into();
        let mut streams = self.streams.write().await;

        let stream = streams.entry(aggregate_id.clone()).or_insert_with(Vec::new);
        let mut version = stream.len() as u64;

        for event in events {
            let mut metadata = EventMetadata::new(&event);
            metadata.version = Some(version);

            let data = serde_json::to_value(&event)
                .map_err(|e| crate::Error::Serialization(e.to_string()))?;

            stream.push(StoredEvent { metadata, data });

            debug!(
                aggregate_id = %aggregate_id,
                version = version,
                event_type = %event.event_type(),
                "Event appended to store"
            );

            version += 1;
        }

        info!(
            aggregate_id = %aggregate_id,
            total_events = stream.len(),
            "Events persisted to store"
        );

        Ok(())
    }

    /// Gets all events for an aggregate starting from a specific version.
    pub async fn get_events<E: Event + for<'de> serde::Deserialize<'de>>(
        &self,
        aggregate_id: impl Into<String>,
        from_version: u64,
    ) -> Result<Vec<E>> {
        let aggregate_id = aggregate_id.into();
        let streams = self.streams.read().await;

        let stream = streams
            .get(&aggregate_id)
            .ok_or_else(|| crate::Error::not_found(format!("Aggregate {} not found", aggregate_id)))?;

        // Filter by event type and version
        let _event_type_name = std::any::type_name::<E>();

        let events: Result<Vec<E>> = stream
            .iter()
            .filter(|e| {
                // Check version
                if e.metadata.version.unwrap_or(0) < from_version {
                    return false;
                }
                // Check if this event matches the requested type by trying to deserialize
                true
            })
            .filter_map(|stored| {
                // Try to deserialize, skip if it's a different event type
                serde_json::from_value::<E>(stored.data.clone()).ok()
            })
            .map(Ok)
            .collect();

        events
    }

    /// Gets all stored events for an aggregate (type-erased).
    pub async fn get_stored_events(
        &self,
        aggregate_id: impl Into<String>,
        from_version: u64,
    ) -> Result<Vec<StoredEvent>> {
        let aggregate_id = aggregate_id.into();
        let streams = self.streams.read().await;

        let stream = streams
            .get(&aggregate_id)
            .ok_or_else(|| crate::Error::not_found(format!("Aggregate {} not found", aggregate_id)))?;

        Ok(stream
            .iter()
            .filter(|e| e.metadata.version.unwrap_or(0) >= from_version)
            .cloned()
            .collect())
    }

    /// Returns the current version of an aggregate (number of events).
    pub async fn get_version(&self, aggregate_id: impl Into<String>) -> u64 {
        let aggregate_id = aggregate_id.into();
        let streams = self.streams.read().await;

        streams
            .get(&aggregate_id)
            .map(|s| s.len() as u64)
            .unwrap_or(0)
    }

    /// Returns all aggregate IDs in the store.
    pub async fn list_aggregates(&self) -> Vec<String> {
        let streams = self.streams.read().await;
        streams.keys().cloned().collect()
    }

    /// Checks if an aggregate exists.
    pub async fn exists(&self, aggregate_id: impl Into<String>) -> bool {
        let aggregate_id = aggregate_id.into();
        let streams = self.streams.read().await;
        streams.contains_key(&aggregate_id)
    }

    /// Deletes all events for an aggregate (use with caution).
    pub async fn delete_stream(&self, aggregate_id: impl Into<String>) -> Result<()> {
        let aggregate_id = aggregate_id.into();
        let mut streams = self.streams.write().await;

        streams.remove(&aggregate_id);
        info!(aggregate_id = %aggregate_id, "Event stream deleted");

        Ok(())
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for aggregates that can be rebuilt from events.
///
/// An aggregate is an entity whose state is derived by replaying its event history.
pub trait AggregateRoot: Default {
    /// The event type this aggregate handles.
    type Event: Event;

    /// Applies an event to the aggregate, updating its state.
    fn apply(&mut self, event: Self::Event);

    /// Replays a sequence of events to rebuild the aggregate state.
    fn replay(&mut self, events: Vec<Self::Event>) {
        for event in events {
            self.apply(event);
        }
    }
}

/// Helper to load an aggregate from the event store.
pub async fn load_aggregate<A: AggregateRoot>(
    store: &EventStore,
    aggregate_id: impl Into<String>,
) -> Result<A>
where
    A::Event: for<'de> serde::Deserialize<'de>,
{
    let aggregate_id = aggregate_id.into();
    let events = store.get_events::<A::Event>(&aggregate_id, 0).await?;

    let mut aggregate = A::default();
    aggregate.replay(events);

    Ok(aggregate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct UserCreated {
        user_id: String,
        email: String,
    }

    impl Event for UserCreated {
        fn event_type(&self) -> &'static str {
            "user.created"
        }
        fn aggregate_id(&self) -> String {
            self.user_id.clone()
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct EmailChanged {
        user_id: String,
        new_email: String,
    }

    impl Event for EmailChanged {
        fn event_type(&self) -> &'static str {
            "email.changed"
        }
        fn aggregate_id(&self) -> String {
            self.user_id.clone()
        }
    }

    #[derive(Default)]
    struct UserAggregate {
        id: String,
        email: String,
        version: u64,
    }

    #[tokio::test]
    async fn test_append_and_get_events() {
        let store = EventStore::new();

        let events = vec![
            UserCreated {
                user_id: "user-123".into(),
                email: "user@example.com".into(),
            },
        ];

        store.append("user-123", events).await.unwrap();

        let retrieved = store.get_events::<UserCreated>("user-123", 0).await.unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].email, "user@example.com");
    }

    #[tokio::test]
    async fn test_version_tracking() {
        let store = EventStore::new();

        store.append("user-123", vec![
            UserCreated {
                user_id: "user-123".into(),
                email: "user@example.com".into(),
            },
        ]).await.unwrap();

        assert_eq!(store.get_version("user-123").await, 1);

        store.append("user-123", vec![
            UserCreated {
                user_id: "user-123".into(),
                email: "another@example.com".into(),
            },
        ]).await.unwrap();

        assert_eq!(store.get_version("user-123").await, 2);
    }
}
