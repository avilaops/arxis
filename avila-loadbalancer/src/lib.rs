//! # avila-loadbalancer
extern crate alloc;
use alloc::vec::Vec;

pub struct LoadBalancer {
    pub backends: Vec<u16>,
}

impl LoadBalancer {
    pub fn new() -> Self { Self { backends: Vec::new() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { let lb = LoadBalancer::new(); assert_eq!(lb.backends.len(), 0); }
}
