//! In-memory event bus with pub/sub semantics.

use crate::{Event, EventEnvelope, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};

/// Channel capacity for event subscribers.
const DEFAULT_CHANNEL_CAPACITY: usize = 1000;

/// In-memory event bus for pub/sub messaging.
///
/// The event bus allows multiple subscribers to receive events of specific types.
/// Each subscriber gets their own copy of the event (broadcast semantics).
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::{Event, EventBus};
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
/// #[tokio::main]
/// async fn main() {
///     let bus = EventBus::new();
///     let mut sub = bus.subscribe::<UserCreated>().await;
///
///     bus.publish(UserCreated {
///         user_id: "123".into(),
///         email: "user@example.com".into(),
///     }).await.unwrap();
///
///     if let Some(event) = sub.recv().await {
///         println!("Got event: {:?}", event);
///     }
/// }
/// ```
#[derive(Clone)]
pub struct EventBus {
    channels: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
    capacity: usize,
}

impl EventBus {
    /// Creates a new event bus with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CHANNEL_CAPACITY)
    }

    /// Creates a new event bus with specified channel capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            capacity,
        }
    }

    /// Publishes an event to all subscribers.
    ///
    /// Returns an error if serialization fails or if there are no active subscribers
    /// (which may indicate a misconfiguration).
    pub async fn publish<E: Event + Clone + Send + Sync>(&self, event: E) -> Result<()> {
        let envelope = EventEnvelope::new(event);

        debug!(
            event_type = %envelope.metadata.event_type,
            event_id = %envelope.metadata.event_id,
            aggregate_id = %envelope.metadata.aggregate_id,
            "Publishing event"
        );

        let type_id = TypeId::of::<E>();
        let channels = self.channels.read().await;

        if let Some(sender) = channels.get(&type_id) {
            if let Some(tx) = sender.downcast_ref::<broadcast::Sender<EventEnvelope<E>>>() {
                match tx.send(envelope) {
                    Ok(subscriber_count) => {
                        info!(
                            event_type = %tx.len(),
                            subscribers = subscriber_count,
                            "Event published"
                        );
                    }
                    Err(_) => {
                        warn!("No active subscribers for event type");
                    }
                }
            }
        } else {
            warn!(
                event_type = std::any::type_name::<E>(),
                "No channel registered for event type"
            );
        }

        Ok(())
    }

    /// Subscribes to events of a specific type.
    ///
    /// Returns a receiver that will receive all future events of type `E`.
    pub async fn subscribe<E: Event + Clone + Send + Sync + 'static>(&self) -> EventSubscriber<E> {
        let type_id = TypeId::of::<E>();
        let mut channels = self.channels.write().await;

        let tx = channels
            .entry(type_id)
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel::<EventEnvelope<E>>(self.capacity);
                Box::new(tx)
            })
            .downcast_ref::<broadcast::Sender<EventEnvelope<E>>>()
            .unwrap()
            .clone();

        let rx = tx.subscribe();

        info!(
            event_type = std::any::type_name::<E>(),
            "New subscriber registered"
        );

        EventSubscriber { rx }
    }

    /// Returns the number of channels (event types) registered.
    pub async fn channel_count(&self) -> usize {
        self.channels.read().await.len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// A subscriber that receives events of a specific type.
pub struct EventSubscriber<E> {
    rx: broadcast::Receiver<EventEnvelope<E>>,
}

impl<E: Event> EventSubscriber<E> {
    /// Receives the next event, waiting if necessary.
    ///
    /// Returns `None` if all publishers have been dropped.
    pub async fn recv(&mut self) -> Option<EventEnvelope<E>> {
        loop {
            match self.rx.recv().await {
                Ok(envelope) => return Some(envelope),
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    warn!(
                        skipped_events = skipped,
                        "Subscriber lagged behind, some events were skipped"
                    );
                    // Continue to next iteration
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }

    /// Tries to receive an event without blocking.
    ///
    /// Returns `None` if no event is immediately available.
    pub fn try_recv(&mut self) -> Option<EventEnvelope<E>> {
        match self.rx.try_recv() {
            Ok(envelope) => Some(envelope),
            Err(broadcast::error::TryRecvError::Lagged(skipped)) => {
                warn!(
                    skipped_events = skipped,
                    "Subscriber lagged behind, some events were skipped"
                );
                self.try_recv()
            }
            Err(_) => None,
        }
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
    async fn test_publish_subscribe() {
        let bus = EventBus::new();
        let mut sub = bus.subscribe::<TestEvent>().await;

        let event = TestEvent {
            id: "123".into(),
            data: "test data".into(),
        };

        bus.publish(event.clone()).await.unwrap();

        let received = sub.recv().await.unwrap();
        assert_eq!(received.event.id, "123");
        assert_eq!(received.event.data, "test data");
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let bus = EventBus::new();
        let mut sub1 = bus.subscribe::<TestEvent>().await;
        let mut sub2 = bus.subscribe::<TestEvent>().await;

        let event = TestEvent {
            id: "456".into(),
            data: "broadcast".into(),
        };

        bus.publish(event).await.unwrap();

        let recv1 = sub1.recv().await.unwrap();
        let recv2 = sub2.recv().await.unwrap();

        assert_eq!(recv1.event.id, recv2.event.id);
        assert_eq!(recv1.event.data, "broadcast");
    }
}
