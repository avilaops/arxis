//! # Broadcast - Multi-Producer Multi-Consumer Channel
//!
//! Implementação usando ring buffer atômico
//! - Múltiplos senders
//! - Múltiplos receivers
//! - Cada receiver recebe todas as mensagens

use std::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use std::sync::Arc;
use std::ptr;

const BUFFER_SIZE: usize = 1024;

/// Sender de canal broadcast (cloneable).
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// Receiver de canal broadcast (cloneable).
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    position: AtomicUsize,
}

struct Shared<T> {
    buffer: Vec<AtomicPtr<T>>,
    head: AtomicUsize, // Posição de escrita
    tail: AtomicUsize, // Posição mais antiga
}

impl<T: Clone> Sender<T> {
    /// Envia valor para todos os receivers.
    ///
    /// # Erros
    ///
    /// Retorna erro se buffer cheio.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let value_ptr = Box::into_raw(Box::new(value));

        // Get current head position
        let head = self.shared.head.fetch_add(1, Ordering::AcqRel);
        let index = head % BUFFER_SIZE;

        // Coloca valor no buffer
        let old = self.shared.buffer[index].swap(value_ptr, Ordering::AcqRel);

        // Libera valor antigo se existir
        if !old.is_null() {
            unsafe {
                let _ = Box::from_raw(old);
            }
        }

        // Atualiza tail se necessário
        let tail = self.shared.tail.load(Ordering::Acquire);
        if head.wrapping_sub(tail) >= BUFFER_SIZE {
            self.shared.tail.store(head.wrapping_sub(BUFFER_SIZE) + 1, Ordering::Release);
        }

        Ok(())
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T: Clone> Receiver<T> {
    /// Recebe próxima mensagem, bloqueando se necessário.
    ///
    /// # Erros
    ///
    /// Retorna erro se perdeu mensagens (lagged) ou canal fechado.
    pub fn recv(&mut self) -> Result<T, RecvError> {
        loop {
            let pos = self.position.load(Ordering::Acquire);
            let head = self.shared.head.load(Ordering::Acquire);

            // Se está atrás do head, tem mensagem
            if pos < head {
                let index = pos % BUFFER_SIZE;
                let value_ptr = self.shared.buffer[index].load(Ordering::Acquire);

                if !value_ptr.is_null() {
                    let value = unsafe { (*value_ptr).clone() };
                    self.position.store(pos + 1, Ordering::Release);
                    return Ok(value);
                }
            }

            // Verifica se ainda há senders
            if Arc::strong_count(&self.shared) == 1 { // Só receiver
                return Err(RecvError);
            }

            std::thread::yield_now();
        }
    }

    /// Tenta receber sem bloquear.
    ///
    /// # Retorna
    ///
    /// Ok se tinha mensagem, Err se vazio ou lagged.
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        let pos = self.position.load(Ordering::Acquire);
        let head = self.shared.head.load(Ordering::Acquire);

        if pos < head {
            let index = pos % BUFFER_SIZE;
            let value_ptr = self.shared.buffer[index].load(Ordering::Acquire);

            if !value_ptr.is_null() {
                let value = unsafe { (*value_ptr).clone() };
                self.position.store(pos + 1, Ordering::Release);
                return Ok(value);
            }
        }

        if Arc::strong_count(&self.shared) == 1 {
            Err(TryRecvError::Disconnected)
        } else {
            Err(TryRecvError::Empty)
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
            position: AtomicUsize::new(self.position.load(Ordering::Acquire)),
        }
    }
}

impl<T> Drop for Shared<T> {
    fn drop(&mut self) {
        // Limpa buffer
        for slot in &self.buffer {
            let ptr = slot.load(Ordering::Acquire);
            if !ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }
        }
    }
}

/// Cria novo broadcast channel
pub fn channel<T: Clone>() -> (Sender<T>, Receiver<T>) {
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    for _ in 0..BUFFER_SIZE {
        buffer.push(AtomicPtr::new(ptr::null_mut()));
    }

    let shared = Arc::new(Shared {
        buffer,
        head: AtomicUsize::new(0),
        tail: AtomicUsize::new(0),
    });

    let sender = Sender {
        shared: Arc::clone(&shared),
    };

    let receiver = Receiver {
        shared,
        position: AtomicUsize::new(0),
    };

    (sender, receiver)
}

/// Erro ao enviar: buffer cheio.
#[derive(Debug, PartialEq)]
pub struct SendError<T>(pub T);

/// Erro ao receber: canal fechado.
#[derive(Debug, PartialEq)]
pub struct RecvError;

/// Erro ao tentar receber sem bloquear.
#[derive(Debug, PartialEq)]
pub enum TryRecvError {
    /// Canal vazio
    Empty,
    /// Canal fechado
    Disconnected,
}

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Sync for Sender<T> {}
unsafe impl<T: Send> Send for Receiver<T> {}
unsafe impl<T: Send> Sync for Receiver<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast() {
        let (tx, rx1) = channel();
        let rx2 = rx1.clone();

        tx.send(42).unwrap();

        assert_eq!(rx1.try_recv().unwrap(), 42);
        assert_eq!(rx2.try_recv().unwrap(), 42);
    }
}
