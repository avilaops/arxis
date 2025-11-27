//! # Avila Browser
//!
//! Ultra-secure web browser with 7-layer anonymity protection
//!
//! ## Threat Model
//!
//! - **Passive Adversary**: Can observe traffic but not modify
//! - **Active Adversary**: Can drop/modify/inject packets
//! - **Global Adversary**: Can monitor entire network (NSA-level)
//!
//! ## Security Properties
//!
//! - **Anonymity**: Unlinkability of sender/receiver
//! - **Unlinkability**: Cannot correlate multiple sessions
//! - **Unobservability**: Cannot detect communication is happening
//! - **Forward Secrecy**: Past sessions safe if keys compromised
//! - **Traffic Analysis Resistance**: Timing/volume attacks mitigated

pub mod core;
pub mod layers;
pub mod protocols;
pub mod rendering;

pub use core::{Browser, BrowserConfig, Request, Response, BrowserError};
pub use layers::{LayerStack, ProtectionLayer, LayerType};
pub use protocols::{HttpProtocol, QuicProtocol, DohProtocol};
pub use rendering::{Dom, CssParser, LayoutEngine};
