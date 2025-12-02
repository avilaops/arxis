//! # avila-raft - Raft Consensus Algorithm
//! 
//! Implementation of the Raft consensus protocol for distributed systems.
extern crate alloc;
use alloc::vec::Vec;

/// Raft node state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeState {
    Follower,
    Candidate,
    Leader,
}

/// Raft log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub data: Vec<u8>,
}

/// Raft node
pub struct RaftNode {
    pub id: u64,
    pub state: NodeState,
    pub current_term: u64,
    pub log: Vec<LogEntry>,
}

impl RaftNode {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: NodeState::Follower,
            current_term: 0,
            log: Vec::new(),
        }
    }
    
    pub fn append_entry(&mut self, data: Vec<u8>) {
        let entry = LogEntry {
            term: self.current_term,
            index: self.log.len() as u64,
            data,
        };
        self.log.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_node() {
        let node = RaftNode::new(1);
        assert_eq!(node.id, 1);
        assert_eq!(node.state, NodeState::Follower);
    }
    
    #[test]
    fn test_append_entry() {
        let mut node = RaftNode::new(1);
        node.append_entry(vec![1, 2, 3]);
        assert_eq!(node.log.len(), 1);
    }
}
