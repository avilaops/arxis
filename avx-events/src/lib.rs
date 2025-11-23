//! # avx-events
//!
//! Event-driven architecture for Avila Experience Fabric.
//!
//! Provides pub/sub event bus, event sourcing, CQRS patterns, and message-driven
//! patterns for building distributed AVX platform applications.
//!
//! ## Features
//!
//! - **Event Bus**: In-memory and distributed pub/sub
//! - **Event Sourcing**: Append-only event store with replay
//! - **CQRS Support**: Command/Query separation patterns
//! - **Message Patterns**: Request/reply, fire-and-forget, broadcast
//! - **Dead Letter Queue**: Failed event handling
//! - **Type-Safe**: Strongly typed events with serde
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avx_events::{Event, EventBus};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct UserCreated {
//!     pub user_id: String,
//!     pub email: String,
//! }
//!
//! impl Event for UserCreated {
//!     fn event_type(&self) -> &'static str {
//!         "user.created"
//!     }
//!
//!     fn aggregate_id(&self) -> String {
//!         self.user_id.clone()
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let bus = EventBus::new();
//!
//!     let mut subscriber = bus.subscribe::<UserCreated>().await;
//!
//!     bus.publish(UserCreated {
//!         user_id: "123".into(),
//!         email: "user@example.com".into(),
//!     }).await.unwrap();
//!
//!     if let Some(event) = subscriber.recv().await {
//!         println!("Received: {:?}", event);
//!     }
//! }
//! ```

pub mod event;
pub mod bus;
pub mod topic;
pub mod store;
pub mod cqrs;
pub mod dlq;
pub mod request_reply;
pub mod distributed;
pub mod testing;

// Industry 4.0 support - always compile machine/sensor abstractions
pub mod industry40;

pub use event::{Event, EventEnvelope, EventMetadata};
pub use bus::EventBus;
pub use topic::TopicBus;
pub use store::{EventStore, AggregateRoot};
pub use cqrs::{CommandHandler, QueryHandler};
pub use dlq::DeadLetterQueue;
pub use request_reply::RequestReplyBus;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("Event store error: {0}")]
    EventStore(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Distributed backend error: {0}")]
    Distributed(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn validation(msg: impl Into<String>) -> Self {
        Error::Validation(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Error::NotFound(msg.into())
    }

    pub fn serialization(msg: impl Into<String>) -> Self {
        Error::Serialization(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Error::Other(anyhow::anyhow!(msg.into()))
    }
}
