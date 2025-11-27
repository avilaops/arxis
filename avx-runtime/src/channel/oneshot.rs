//! # Oneshot - Single-Use Channel
//!
//! Implementação usando atomics
//! - Um sender, um receiver
//! - Apenas uma mensagem
//! - Otimizado para latência

use std::sync::atomic::{AtomicPtr, AtomicU8, Ordering};
use std::sync::Arc;
use std::ptr;

const EMPTY: u8 = 0;
const SENDING: u8 = 1;
const SENT: u8 = 2;
const CLOSED: u8 = 3;

/// Sender de canal oneshot (uso único).
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// Receiver de canal oneshot (uso único).
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

struct Shared<T> {
    value: AtomicPtr<T>,
    state: AtomicU8,
}

impl<T> Sender<T> {
    /// Envia valor único para o receiver.
    ///
    /// # Erros
    ///
    /// Retorna valor se receiver foi dropado.
    pub fn send(self, value: T) -> Result<(), T> {
        // Verifica se receiver ainda existe
        if Arc::strong_count(&self.shared) < 2 {
            return Err(value);
        }

        let value_ptr = Box::into_raw(Box::new(value));

        // Marca como sending
        if self.shared.state.compare_exchange(
            EMPTY,
            SENDING,
            Ordering::AcqRel,
            Ordering::Acquire,
        ).is_err() {
            unsafe {
                let _ = Box::from_raw(value_ptr);
            }
            return Err(unsafe { *Box::from_raw(value_ptr) });
        }

        // Armazena valor
        self.shared.value.store(value_ptr, Ordering::Release);

        // Marca como sent
        self.shared.state.store(SENT, Ordering::Release);

        Ok(())
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.state.store(CLOSED, Ordering::Release);
    }
}

impl<T> Receiver<T> {
    /// Recebe valor único, bloqueando se necessário.
    ///
    /// # Erros
    ///
    /// Retorna erro se sender foi dropado antes de enviar.
    pub fn recv(self) -> Result<T, RecvError> {
        loop {
            let state = self.shared.state.load(Ordering::Acquire);

            match state {
                SENT => {
                    let value_ptr = self.shared.value.load(Ordering::Acquire);
                    if !value_ptr.is_null() {
                        let value = unsafe { *Box::from_raw(value_ptr) };
                        return Ok(value);
                    }
                }
                CLOSED => return Err(RecvError),
                EMPTY | SENDING => {
                    // Espera
                    std::thread::yield_now();
                    continue;
                }
                _ => return Err(RecvError),
            }
        }
    }

    /// Tenta receber sem bloquear.
    ///
    /// # Erros
    ///
    /// Retorna `Empty` se ainda não enviado, `Disconnected` se sender dropado.
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        let state = self.shared.state.load(Ordering::Acquire);

        match state {
            SENT => {
                let value_ptr = self.shared.value.load(Ordering::Acquire);
                if !value_ptr.is_null() {
                    let value = unsafe { *Box::from_raw(value_ptr) };
                    return Ok(value);
                }
                Err(TryRecvError::Empty)
            }
            CLOSED => Err(TryRecvError::Disconnected),
            EMPTY | SENDING => Err(TryRecvError::Empty),
            _ => Err(TryRecvError::Disconnected),
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.state.store(CLOSED, Ordering::Release);

        // Limpa valor se existir
        let value_ptr = self.shared.value.load(Ordering::Acquire);
        if !value_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(value_ptr);
            }
        }
    }
}

/// Cria novo oneshot channel
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        value: AtomicPtr::new(ptr::null_mut()),
        state: AtomicU8::new(EMPTY),
    });

    let sender = Sender {
        shared: Arc::clone(&shared),
    };

    let receiver = Receiver {
        shared,
    };

    (sender, receiver)
}

/// Erro ao receber: sender dropado sem enviar.
#[derive(Debug, PartialEq)]
pub struct RecvError;

/// Erro ao tentar receber sem bloquear.
#[derive(Debug, PartialEq)]
pub enum TryRecvError {
    /// Ainda não enviado
    Empty,
    /// Sender dropado
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
    fn test_oneshot() {
        let (tx, rx) = channel();

        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_oneshot_drop_sender() {
        let (tx, rx) = channel::<i32>();
        drop(tx);

        assert_eq!(rx.recv(), Err(RecvError));
    }
}
