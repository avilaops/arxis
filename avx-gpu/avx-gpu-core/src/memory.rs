//! Memory management utilities

use dashmap::DashMap;
use std::sync::Arc;

/// Memory usage statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryUsage {
    pub allocated_bytes: u64,
    pub free_bytes: u64,
    pub total_bytes: u64,
}

/// GPU memory pool for efficient allocation
pub struct MemoryPool {
    allocations: Arc<DashMap<u64, usize>>,
    total_allocated: Arc<parking_lot::Mutex<u64>>,
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            allocations: Arc::new(DashMap::new()),
            total_allocated: Arc::new(parking_lot::Mutex::new(0)),
        }
    }

    /// Track allocation
    pub fn track_allocation(&self, handle: u64, size: usize) {
        self.allocations.insert(handle, size);
        *self.total_allocated.lock() += size as u64;
    }

    /// Track deallocation
    pub fn track_deallocation(&self, handle: u64) {
        if let Some((_, size)) = self.allocations.remove(&handle) {
            *self.total_allocated.lock() -= size as u64;
        }
    }

    /// Get total allocated bytes
    pub fn total_allocated(&self) -> u64 {
        *self.total_allocated.lock()
    }

    /// Get memory usage statistics
    pub fn usage(&self) -> MemoryUsage {
        let allocated = self.total_allocated();
        MemoryUsage {
            allocated_bytes: allocated,
            free_bytes: 0, // Would need device query
            total_bytes: 0, // Would need device query
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}
