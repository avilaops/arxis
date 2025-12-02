//! # avila-darknet
//!
//! Anonymous networking, Tor protocol, encrypted communication
//!
//! ## Features
//!
//! - Tor onion routing (3-hop circuits)
//! - End-to-end encryption (AES-256-GCM)
//! - Anonymous identity (Ed25519 keys)
//! - Hidden services (.onion addresses)
//! - Deep web crawler & mapper
//! - Full-text search indexer

pub mod tor;
pub mod crypto;
pub mod identity;
pub mod crawler;
pub mod indexer;
pub mod mapper;

pub use tor::{Circuit, OnionRouter, TorNode};
pub use crypto::{EncryptedChannel, CryptoEngine};
pub use identity::{AnonymousIdentity, HiddenService};
pub use crawler::{DeepWebCrawler, HiddenService as CrawledService};
pub use indexer::InvertedIndex;
pub use mapper::{NetworkGraph, DeepWebMap};
