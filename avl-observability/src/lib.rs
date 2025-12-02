//! # avl-observability - System Observability
extern crate alloc;
use alloc::vec::Vec;

pub struct ObservabilityConfig {
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub enable_logging: bool,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            enable_tracing: true,
            enable_logging: true,
        }
    }
}

pub struct Observability {
    pub config: ObservabilityConfig,
}

impl Observability {
    pub fn new(config: ObservabilityConfig) -> Self {
        Self { config }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.config.enable_metrics || self.config.enable_tracing || self.config.enable_logging
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_observability() {
        let obs = Observability::new(ObservabilityConfig::default());
        assert!(obs.is_enabled());
    }
}
