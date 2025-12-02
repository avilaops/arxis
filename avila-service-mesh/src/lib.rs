//! # avila-service-mesh
extern crate alloc;
use alloc::vec::Vec;

pub struct ServiceMesh {
    pub services: Vec<u16>,
}

impl ServiceMesh {
    pub fn new() -> Self { Self { services: Vec::new() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { let sm = ServiceMesh::new(); assert!(sm.services.is_empty()); }
}
