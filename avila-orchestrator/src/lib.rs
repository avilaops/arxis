//! # avila-orchestrator - Service Orchestration
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Clone, Debug)]
pub struct Service {
    pub id: u64,
    pub name: String,
    pub running: bool,
}

pub struct Orchestrator {
    pub services: Vec<Service>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self { services: Vec::new() }
    }
    
    pub fn register(&mut self, id: u64, name: String) {
        self.services.push(Service { id, name, running: false });
    }
    
    pub fn start(&mut self, id: u64) {
        if let Some(svc) = self.services.iter_mut().find(|s| s.id == id) {
            svc.running = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_orchestrator() {
        let mut orch = Orchestrator::new();
        orch.register(1, "svc1".into());
        orch.start(1);
        assert!(orch.services[0].running);
    }
}
