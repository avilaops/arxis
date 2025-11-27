//! # Runtime - Core async runtime
//!
//! Gerencia workers, scheduler, reactor e execução de tarefas.

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use crate::scheduler::Scheduler;
use crate::reactor::Reactor;
use crate::task::JoinHandle;

/// Configuração do runtime
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Número de worker threads (default: num_cpus)
    pub worker_threads: usize,

    /// Enable work-stealing (default: true)
    pub work_stealing: bool,

    /// Thread stack size in bytes (default: 2MB)
    pub thread_stack_size: usize,

    /// Budget per task before yield (default: 128 iterations)
    pub task_budget: usize,

    /// Enable metrics collection (default: false)
    pub enable_metrics: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        let worker_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        Self {
            worker_threads,
            work_stealing: true,
            thread_stack_size: 2 * 1024 * 1024, // 2MB
            task_budget: 128,
            enable_metrics: false,
        }
    }
}

/// AVX Runtime - Async executor
pub struct Runtime {
    scheduler: Arc<Scheduler>,
    #[allow(dead_code)]
    reactor: Arc<Reactor>,
    handle: Arc<Handle>,
}

/// Handle to the runtime
pub struct Handle {
    scheduler: Arc<Scheduler>,
    #[allow(dead_code)]
    reactor: Arc<Reactor>,
}

impl Runtime {
    /// Cria novo runtime com configuração padrão
    pub fn new() -> std::io::Result<Self> {
        Self::with_config(RuntimeConfig::default())
    }

    /// Cria novo runtime com configuração customizada
    pub fn with_config(config: RuntimeConfig) -> std::io::Result<Self> {
        let reactor = Arc::new(Reactor::new()?);
        let scheduler = Arc::new(Scheduler::new(config, Arc::clone(&reactor)));

        let handle = Arc::new(Handle {
            scheduler: Arc::clone(&scheduler),
            reactor: Arc::clone(&reactor),
        });

        Ok(Self {
            scheduler,
            reactor,
            handle,
        })
    }

    /// Retorna handle para o runtime
    pub fn handle(&self) -> Arc<Handle> {
        Arc::clone(&self.handle)
    }

    /// Bloqueia a thread atual e executa uma Future até completar
    pub fn block_on<F>(&self, _future: F) -> F::Output
    where
        F: Future + Send,
    {
        // Entra no contexto do runtime
        let _enter = self.enter();

        // TODO: Implementar execução bloqueante real
        // Por agora, apenas unimplemented
        unimplemented!("block_on - TODO: implementar execução de futures")
    }

    /// Spawna uma nova task no runtime
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.scheduler.spawn(future)
    }

    /// Desliga o runtime gracefully
    pub fn shutdown_timeout(self, timeout: Duration) {
        self.scheduler.shutdown(timeout);
    }

    /// Entra no contexto do runtime
    fn enter(&self) -> EnterGuard {
        EnterGuard::new(Arc::clone(&self.handle))
    }
}

/// Guard para contexto do runtime
struct EnterGuard {
    _handle: Arc<Handle>,
}

impl EnterGuard {
    fn new(handle: Arc<Handle>) -> Self {
        // TODO: Set thread-local runtime context
        Self { _handle: handle }
    }
}

impl Drop for EnterGuard {
    fn drop(&mut self) {
        // TODO: Clear thread-local runtime context
    }
}

impl Handle {
    /// Spawna uma task a partir de qualquer thread
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.scheduler.spawn(future)
    }

    /// Retorna referência ao scheduler
    #[allow(dead_code)]
    pub(crate) fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    /// Retorna referência ao reactor
    #[allow(dead_code)]
    pub(crate) fn reactor(&self) -> &Reactor {
        &self.reactor
    }
}

thread_local! {
    static RUNTIME_CONTEXT: std::cell::RefCell<Option<Arc<Handle>>> = const { std::cell::RefCell::new(None) };
}

/// Retorna handle do runtime atual
pub fn runtime_handle() -> Option<Arc<Handle>> {
    RUNTIME_CONTEXT.with(|ctx| ctx.borrow().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new().unwrap();
        assert!(runtime.handle().is_some());
    }

    #[test]
    fn test_runtime_config() {
        let config = RuntimeConfig {
            worker_threads: 4,
            work_stealing: true,
            thread_stack_size: 4 * 1024 * 1024,
            task_budget: 256,
            enable_metrics: true,
        };

        let runtime = Runtime::with_config(config).unwrap();
        assert!(runtime.handle().is_some());
    }

    #[test]
    fn test_block_on() {
        let runtime = Runtime::new().unwrap();

        let result = runtime.block_on(async {
            42
        });

        assert_eq!(result, 42);
    }
}
