//! # Queue - Lock-Free MPSC Queue

use crate::deque::{WorkStealingDeque, Steal};
use std::sync::Arc;

/// Fila MPSC lock-free usando work-stealing deque.
pub struct MPSCQueue<T> {
    deque: Arc<WorkStealingDeque<T>>,
}

impl<T> MPSCQueue<T> {
    /// Cria nova fila vazia.
    pub fn new() -> Self {
        Self {
            deque: Arc::new(WorkStealingDeque::new()),
        }
    }

    /// Adiciona valor na fila.
    pub fn push(&self, value: T) {
        self.deque.push(value);
    }

    /// Tenta roubar valor da fila (não-bloqueante).
    pub fn steal(&self) -> Option<T> {
        match self.deque.steal() {
            Steal::Success(v) => Some(v),
            Steal::Empty | Steal::Retry => None,
        }
    }
}

impl<T> Default for MPSCQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
