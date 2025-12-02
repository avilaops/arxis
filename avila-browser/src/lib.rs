//! # Avila Browser
//!
//! High-assurance web browser implementing multi-layer onion routing architecture
//!
//! ## Adversarial Model
//!
//! - **Passive Adversary**: Observes network traffic without modification capabilities
//! - **Active Adversary**: Possesses packet manipulation, injection, and dropping capabilities
//! - **Global Adversary**: Exhibits omniscient network monitoring capabilities (nation-state level)
//!
//! ## Cryptographic Security Properties
//!
//! - **Sender-Receiver Anonymity**: Computational unlinkability of communicating parties
//! - **Session Unlinkability**: Infeasibility of correlating distinct protocol sessions
//! - **Communication Unobservability**: Statistical indistinguishability from random noise
//! - **Perfect Forward Secrecy**: Retroactive security guarantee under key compromise
//! - **Traffic Analysis Resistance**: Countermeasures against temporal and volumetric side-channels

pub mod core;
pub mod layers;
pub mod protocols;
pub mod rendering;

pub use core::{Browser, BrowserConfig, Request, Response, BrowserError};
pub use layers::{LayerStack, ProtectionLayer, LayerType};
pub use protocols::{HttpProtocol, QuicProtocol, DohProtocol};
pub use rendering::{Dom, CssParser, LayoutEngine};
