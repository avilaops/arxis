//! Core event abstractions and metadata.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait that all events must implement.
///
/// Events are immutable records of things that have happened in the system.
pub trait Event: Debug + Clone + Send + Sync + 'static {
    /// Returns the type identifier for this event (e.g., "user.created").
    fn event_type(&self) -> &'static str;

    /// Returns the aggregate/entity ID this event belongs to.
    fn aggregate_id(&self) -> String;

    /// Optional: Returns the correlation ID for distributed tracing.
    fn correlation_id(&self) -> Option<String> {
        None
    }

    /// Optional: Returns the causation ID (ID of the event that caused this one).
    fn causation_id(&self) -> Option<String> {
        None
    }

    /// Optional: Returns the user ID who triggered this event.
    fn user_id(&self) -> Option<String> {
        None
    }
}

/// Metadata attached to every event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Unique identifier for this event instance.
    pub event_id: String,

    /// Type of the event (from Event::event_type).
    pub event_type: String,

    /// Aggregate/entity ID this event belongs to.
    pub aggregate_id: String,

    /// Unix timestamp (milliseconds) when event was created.
    pub timestamp: i64,

    /// Correlation ID for distributed tracing.
    pub correlation_id: Option<String>,

    /// Causation ID (ID of the event that caused this one).
    pub causation_id: Option<String>,

    /// User ID who triggered this event.
    pub user_id: Option<String>,

    /// Version number for optimistic concurrency control.
    pub version: Option<u64>,
}

impl EventMetadata {
    /// Creates new metadata for an event.
    pub fn new<E: Event>(event: &E) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: event.event_type().to_string(),
            aggregate_id: event.aggregate_id(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            correlation_id: event.correlation_id(),
            causation_id: event.causation_id(),
            user_id: event.user_id(),
            version: None,
        }
    }

    /// Sets the version number.
    pub fn with_version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }

    /// Sets the correlation ID.
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

/// Event envelope that wraps an event with its metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<E> {
    /// The actual event data.
    pub event: E,

    /// Metadata about the event.
    pub metadata: EventMetadata,
}

impl<E: Event> EventEnvelope<E> {
    /// Creates a new envelope for an event.
    pub fn new(event: E) -> Self {
        let metadata = EventMetadata::new(&event);
        Self { event, metadata }
    }

    /// Creates a new envelope with existing metadata.
    pub fn with_metadata(event: E, metadata: EventMetadata) -> Self {
        Self { event, metadata }
    }
}

/// Type-erased event for storage and serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub metadata: EventMetadata,
    pub data: serde_json::Value,
}

impl StoredEvent {
    /// Creates a stored event from a typed event.
    pub fn from_event<E: Event + Serialize>(event: E) -> Result<Self, crate::Error> {
        let metadata = EventMetadata::new(&event);
        let data = serde_json::to_value(&event)
            .map_err(|e| crate::Error::Serialization(e.to_string()))?;

        Ok(Self { metadata, data })
    }

    /// Deserializes the event data to a specific type.
    pub fn to_event<E: Event + for<'de> Deserialize<'de>>(&self) -> Result<E, crate::Error> {
        serde_json::from_value(self.data.clone())
            .map_err(|e| crate::Error::Serialization(e.to_string()))
    }
}
