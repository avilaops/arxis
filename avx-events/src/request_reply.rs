//! Request/Reply messaging pattern.

use crate::{Event, EventEnvelope, Result};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};
use tracing::{debug, info, warn};

const DEFAULT_CHANNEL_CAPACITY: usize = 100;

/// Request/Reply bus for synchronous-style RPC over events.
///
/// Unlike pub/sub, request/reply has one responder per request type
/// and expects exactly one response.
///
/// # Example
///
/// ```rust,no_run
/// use avx_events::{RequestReplyBus, Event};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct GetUserRequest {
///     user_id: String,
/// }
///
/// impl Event for GetUserRequest {
///     fn event_type(&self) -> &'static str { "get_user_request" }
///     fn aggregate_id(&self) -> String { self.user_id.clone() }
/// }
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct GetUserResponse {
///     user_id: String,
///     email: String,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let bus = RequestReplyBus::new();
///
///     // Responder
///     tokio::spawn({
///         let bus = bus.clone();
///         async move {
///             let mut listener = bus.listen::<GetUserRequest, GetUserResponse>().await;
///             while let Some((req, reply)) = listener.recv().await {
///                 let response = GetUserResponse {
///                     user_id: req.user_id,
///                     email: "user@example.com".into(),
///                 };
///                 reply.send(response).await.ok();
///             }
///         }
///     });
///
///     // Requester
///     let request = GetUserRequest { user_id: "123".into() };
///     let response = bus.request::<GetUserRequest, GetUserResponse>(request).await.unwrap();
///     println!("Got response: {:?}", response);
/// }
/// ```
#[derive(Clone)]
pub struct RequestReplyBus {
    listeners: Arc<RwLock<HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>>>,
    capacity: usize,
}

impl RequestReplyBus {
    /// Creates a new request/reply bus.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CHANNEL_CAPACITY)
    }

    /// Creates a new request/reply bus with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            capacity,
        }
    }

    /// Sends a request and waits for a response.
    ///
    /// Returns an error if no responder is registered or if the responder
    /// drops the reply channel without sending.
    pub async fn request<Req, Res>(&self, request: Req) -> Result<Res>
    where
        Req: Event + Send + Sync + 'static,
        Res: Send + 'static,
    {
        let type_id = TypeId::of::<Req>();
        let listeners = self.listeners.read().await;

        let sender = listeners
            .get(&type_id)
            .ok_or_else(|| crate::Error::EventBus("No listener registered for request type".into()))?
            .downcast_ref::<mpsc::Sender<RequestWithReply<Req, Res>>>()
            .ok_or_else(|| crate::Error::EventBus("Invalid listener type".into()))?;

        let envelope = EventEnvelope::new(request);
        let (reply_tx, reply_rx) = oneshot::channel();

        debug!(
            event_type = %envelope.metadata.event_type,
            event_id = %envelope.metadata.event_id,
            "Sending request"
        );

        let req_with_reply = RequestWithReply {
            envelope,
            reply: reply_tx,
        };

        sender
            .send(req_with_reply)
            .await
            .map_err(|_| crate::Error::EventBus("Failed to send request, listener closed".into()))?;

        reply_rx
            .await
            .map_err(|_| crate::Error::EventBus("Responder dropped reply channel".into()))
    }

    /// Starts listening for requests of a specific type.
    ///
    /// Only one listener can be registered per request type.
    pub async fn listen<Req, Res>(&self) -> RequestListener<Req, Res>
    where
        Req: Event + Send + Sync + 'static,
        Res: Send + 'static,
    {
        let type_id = TypeId::of::<Req>();
        let mut listeners = self.listeners.write().await;

        let (tx, rx) = mpsc::channel(self.capacity);

        if listeners.insert(type_id, Box::new(tx)).is_some() {
            warn!(
                request_type = std::any::type_name::<Req>(),
                "Replacing existing listener for request type"
            );
        }

        info!(
            request_type = std::any::type_name::<Req>(),
            "Request listener registered"
        );

        RequestListener { rx }
    }

    /// Returns the number of registered listeners.
    pub async fn listener_count(&self) -> usize {
        self.listeners.read().await.len()
    }
}

impl Default for RequestReplyBus {
    fn default() -> Self {
        Self::new()
    }
}

/// A request with a reply channel.
struct RequestWithReply<Req, Res> {
    envelope: EventEnvelope<Req>,
    reply: oneshot::Sender<Res>,
}

/// Listener that receives requests and can send replies.
pub struct RequestListener<Req, Res> {
    rx: mpsc::Receiver<RequestWithReply<Req, Res>>,
}

impl<Req: Event, Res> RequestListener<Req, Res> {
    /// Receives the next request with its reply channel.
    pub async fn recv(&mut self) -> Option<(EventEnvelope<Req>, ReplyChannel<Res>)> {
        self.rx.recv().await.map(|req_with_reply| {
            let reply = ReplyChannel {
                sender: req_with_reply.reply,
            };
            (req_with_reply.envelope, reply)
        })
    }
}

/// Reply channel for sending a response back to the requester.
pub struct ReplyChannel<Res> {
    sender: oneshot::Sender<Res>,
}

impl<Res> ReplyChannel<Res> {
    /// Sends a reply back to the requester.
    ///
    /// Returns `Ok(())` if the reply was sent successfully,
    /// or an error if the requester has already dropped the request.
    pub async fn send(self, response: Res) -> Result<()> {
        self.sender
            .send(response)
            .map_err(|_| crate::Error::EventBus("Requester dropped request".into()))
    }
}

/// Timeout configuration for requests.
pub struct RequestTimeout {
    duration: std::time::Duration,
}

impl RequestTimeout {
    /// Creates a new timeout configuration.
    pub fn new(duration: std::time::Duration) -> Self {
        Self { duration }
    }

    /// Executes a request with timeout.
    pub async fn execute<Req, Res>(
        &self,
        bus: &RequestReplyBus,
        request: Req,
    ) -> Result<Res>
    where
        Req: Event + Send + Sync + 'static,
        Res: Send + 'static,
    {
        tokio::time::timeout(self.duration, bus.request(request))
            .await
            .map_err(|_| crate::Error::EventBus("Request timed out".into()))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct EchoRequest {
        message: String,
    }

    impl Event for EchoRequest {
        fn event_type(&self) -> &'static str {
            "echo.request"
        }
        fn aggregate_id(&self) -> String {
            "echo".into()
        }
    }

    #[derive(Debug, Clone)]
    struct EchoResponse {
        message: String,
    }

    #[tokio::test]
    async fn test_request_reply() {
        let bus = RequestReplyBus::new();

        // Start listener
        tokio::spawn({
            let bus = bus.clone();
            async move {
                let mut listener = bus.listen::<EchoRequest, EchoResponse>().await;
                while let Some((req, reply)) = listener.recv().await {
                    let response = EchoResponse {
                        message: format!("Echo: {}", req.event.message),
                    };
                    reply.send(response).await.ok();
                }
            }
        });

        // Give listener time to register
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Send request
        let request = EchoRequest {
            message: "Hello".into(),
        };

        let response = bus.request::<EchoRequest, EchoResponse>(request).await.unwrap();
        assert_eq!(response.message, "Echo: Hello");
    }

    #[tokio::test]
    async fn test_request_timeout() {
        let bus = RequestReplyBus::new();

        // No listener registered
        let request = EchoRequest {
            message: "Test".into(),
        };

        let result = bus.request::<EchoRequest, EchoResponse>(request).await;
        assert!(result.is_err());
    }
}
