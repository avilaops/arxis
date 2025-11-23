//! Indústria 4.0 support for avx-events
//!
//! This module provides integration with industrial protocols and patterns
//! commonly used in Industry 4.0 / Smart Manufacturing environments.

#[cfg(feature = "opcua")]
pub mod opcua;

#[cfg(feature = "mqtt")]
pub mod mqtt;

#[cfg(feature = "timeseries")]
pub mod timeseries;

pub mod machine;
pub mod sensor;
