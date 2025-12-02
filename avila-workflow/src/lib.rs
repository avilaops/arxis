//! # avila-workflow - Workflow Engine
extern crate alloc;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct Step {
    pub id: u64,
    pub name: alloc::string::String,
}

pub struct Workflow {
    pub steps: Vec<Step>,
    pub current: usize,
}

impl Workflow {
    pub fn new() -> Self {
        Self { steps: Vec::new(), current: 0 }
    }
    
    pub fn add_step(&mut self, id: u64, name: alloc::string::String) {
        self.steps.push(Step { id, name });
    }
    
    pub fn next(&mut self) -> bool {
        if self.current < self.steps.len() {
            self.current += 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow() {
        let mut wf = Workflow::new();
        wf.add_step(1, "step1".into());
        wf.add_step(2, "step2".into());
        assert!(wf.next());
        assert_eq!(wf.current, 1);
    }
}
