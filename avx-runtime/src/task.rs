//! # Task - Unidade de execução assíncrona

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Representa uma tarefa assíncrona no runtime AVX.
///
/// Encapsula um Future e gerencia seu ciclo de vida.
pub struct Task {
    id: usize,
}

impl Task {
    /// Cria nova Task a partir de um Future.
    ///
    /// # Argumentos
    ///
    /// * `future` - Future a ser executado
    pub fn new<F>(_future: F) -> Self
    where
        F: Future + Send + 'static,
    {
        // TODO: Pin and store future
        Task { id: 0 }
    }

    /// Retorna JoinHandle para aguardar conclusão desta task.
    pub fn handle<T>(&self) -> JoinHandle<T> {
        JoinHandle {
            task_id: self.id,
            _marker: std::marker::PhantomData,
        }
    }

    /// Executa a task até conclusão.
    pub fn run<F: Future>(&self) -> F::Output {
        // TODO: Implementar execução da task
        unimplemented!("task execution")
    }
}

/// Handle para aguardar conclusão de uma task assíncrona.
///
/// Similar a `std::thread::JoinHandle`, mas para tarefas assíncronas.
pub struct JoinHandle<T> {
    #[allow(dead_code)]
    task_id: usize,
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: Implement polling
        Poll::Pending
    }
}
