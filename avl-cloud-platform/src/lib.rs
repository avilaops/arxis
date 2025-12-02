//! Avila Cloud Platform Library

pub mod error;
pub mod compute;
pub mod storage;
pub mod network;
pub mod billing;
pub mod auth;
pub mod monitoring;
pub mod api;

pub use error::{Error, Result};
pub use compute::ComputeManager;
pub use storage::StorageService;
pub use network::NetworkManager;
pub use billing::BillingManager;
pub use auth::AuthManager;
pub use monitoring::MonitoringService;
pub use api::CloudApi;
