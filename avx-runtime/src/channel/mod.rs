//! # Channel - Native communication channels
//!
//! Implementações 100% Rust puro:
//! - MPSC: Multi-Producer Single-Consumer
//! - Broadcast: Multi-Producer Multi-Consumer
//! - Oneshot: Single-Use Channel

pub mod mpsc;
pub mod broadcast;
pub mod oneshot;

pub use mpsc::{channel as mpsc_channel, Sender as MpscSender, Receiver as MpscReceiver};
pub use broadcast::{channel as broadcast_channel, Sender as BroadcastSender, Receiver as BroadcastReceiver};
pub use oneshot::{channel as oneshot_channel, Sender as OneshotSender, Receiver as OneshotReceiver};
