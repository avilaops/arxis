//! # MPSC - Multi-Producer Single-Consumer Channel
//!
//! Implementação lock-free usando atomics
//! - Múltiplos senders (cloneable)
//! - Um único receiver
//! - Baseado em linked list atômica

use std::sync::atomic::{AtomicPtr, AtomicBool, Ordering};
use std::sync::Arc;
use std::ptr;

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

/// Sender de canal MPSC (cloneable para múltiplos produtores).
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// Receiver de canal MPSC (exclusivo para um consumidor).
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    tail: *mut Node<T>,
}

struct Shared<T> {
    head: AtomicPtr<Node<T>>,
    closed: AtomicBool,
}

impl<T> Sender<T> {
    /// Envia valor para o canal.
    ///
    /// # Erros
    ///
    /// Retorna `SendError` se receiver foi dropado.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.shared.closed.load(Ordering::Acquire) {
            return Err(SendError(value));
        }

        // Cria novo nó
        let node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        // Adiciona no head (push front)
        loop {
            let head = self.shared.head.load(Ordering::Acquire);
            unsafe { (*node).next.store(head, Ordering::Release) };

            if self.shared.head.compare_exchange(
                head,
                node,
                Ordering::AcqRel,
                Ordering::Acquire,
            ).is_ok() {
                break;
            }
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

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Se é o último sender, marca como closed
        if Arc::strong_count(&self.shared) == 2 { // 1 sender + 1 receiver
            self.shared.closed.store(true, Ordering::Release);
        }
    }
}

impl<T> Receiver<T> {
    /// Recebe valor do canal, bloqueando se necessário.
    ///
    /// # Erros
    ///
    /// Retorna `RecvError` se todos senders foram dropados.
    pub fn recv(&mut self) -> Result<T, RecvError> {
        loop {
            // Tenta pegar da tail
            let next = unsafe { (*self.tail).next.load(Ordering::Acquire) };

            if !next.is_null() {
                // Tem valor disponível
                let value = unsafe { (*next).value.take() };
                self.tail = next;

                if let Some(v) = value {
                    return Ok(v);
                }
                continue;
            }

            // Verifica se channel está fechado
            if self.shared.closed.load(Ordering::Acquire) {
                return Err(RecvError);
            }

            // Yield e tenta novamente
            std::thread::yield_now();
        }
    }

    /// Tenta receber sem bloquear.
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        let next = unsafe { (*self.tail).next.load(Ordering::Acquire) };

        if !next.is_null() {
            let value = unsafe { (*next).value.take() };
            self.tail = next;

            if let Some(v) = value {
                return Ok(v);
            }
        }

        if self.shared.closed.load(Ordering::Acquire) {
            Err(TryRecvError::Disconnected)
        } else {
            Err(TryRecvError::Empty)
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Marca como closed
        self.shared.closed.store(true, Ordering::Release);

        // Limpa lista
        let mut current = self.tail;
        while !current.is_null() {
            let next = unsafe { (*current).next.load(Ordering::Acquire) };
            unsafe {
                let _ = Box::from_raw(current);
            }
            current = next;
        }
    }
}

/// Cria novo MPSC channel
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // Dummy node inicial
    let dummy = Box::into_raw(Box::new(Node {
        value: None,
        next: AtomicPtr::new(ptr::null_mut()),
    }));

    let shared = Arc::new(Shared {
        head: AtomicPtr::new(dummy),
        closed: AtomicBool::new(false),
    });

    let sender = Sender {
        shared: Arc::clone(&shared),
    };

    let receiver = Receiver {
        shared,
        tail: dummy,
    };

    (sender, receiver)
}

/// Erro ao enviar: receiver foi dropado.
#[derive(Debug, PartialEq)]
pub struct SendError<T>(pub T);

/// Erro ao receber: todos senders foram dropados.
#[derive(Debug, PartialEq)]
pub struct RecvError;

/// Erro ao tentar receber sem bloquear.
#[derive(Debug, PartialEq)]
pub enum TryRecvError {
    /// Canal está vazio
    Empty,
    /// Todos senders dropados
    Disconnected,
}

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Sync for Sender<T> {}
unsafe impl<T: Send> Send for Receiver<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_recv() {
        let (tx, mut rx) = channel();

        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_multiple_senders() {
        let (tx, mut rx) = channel();
        let tx2 = tx.clone();

        tx.send(1).unwrap();
        tx2.send(2).unwrap();

        let v1 = rx.recv().unwrap();
        let v2 = rx.recv().unwrap();

        assert!(v1 == 1 || v1 == 2);
        assert!(v2 == 1 || v2 == 2);
        assert_ne!(v1, v2);
    }
}
