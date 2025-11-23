//! Distributed event bus backends.

use crate::{Event, EventEnvelope, Result};
use async_trait::async_trait;

pub mod redis;

#[cfg(feature = "redis")]
pub use redis::RedisEventBus;

/// Trait for distributed event bus implementations.
///
/// Distributed buses allow events to be shared across multiple service instances,
/// enabling true microservices architecture.
#[async_trait]
pub trait DistributedBus: Send + Sync {
    /// Publishes an event to all subscribers across all instances.
    async fn publish<E: Event + serde::Serialize + Send + Sync>(
        &self,
        event: E,
    ) -> Result<()>;

    /// Subscribes to events of a specific type across all instances.
    async fn subscribe<E: Event + for<'de> serde::Deserialize<'de> + Send + Sync + 'static>(
        &self,
    ) -> Box<dyn DistributedSubscriber<E>>;

    /// Closes the connection to the distributed backend.
    async fn close(&self) -> Result<()>;
}

/// Trait for distributed event subscribers.
#[async_trait]
pub trait DistributedSubscriber<E: Event>: Send + Sync {
    /// Receives the next event from the distributed bus.
    async fn recv(&mut self) -> Option<EventEnvelope<E>>;
}
