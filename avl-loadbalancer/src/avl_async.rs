//! AVL Async Module
//!
//! Asynchronous runtime and utilities for AVL Cloud Platform.
//! Built on Tokio with AVL-specific extensions and monitoring.

pub use tokio::*;

use std::future::Future;
use std::time::Duration;

/// AVL runtime configuration
#[derive(Debug, Clone)]
pub struct AvlRuntimeConfig {
    /// Number of worker threads (None = CPU count)
    pub worker_threads: Option<usize>,
    /// Thread name prefix
    pub thread_name: String,
    /// Thread stack size
    pub thread_stack_size: Option<usize>,
    /// Enable I/O driver
    pub enable_io: bool,
    /// Enable time driver
    pub enable_time: bool,
}

impl Default for AvlRuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: None,
            thread_name: "avl-worker".to_string(),
            thread_stack_size: None,
            enable_io: true,
            enable_time: true,
        }
    }
}

/// AVL async runtime builder
pub struct AvlRuntime;

impl AvlRuntime {
    /// Create new multi-threaded runtime with default config
    pub fn new() -> tokio::runtime::Runtime {
        Self::builder(AvlRuntimeConfig::default()).build()
    }

    /// Create runtime builder with custom config
    pub fn builder(config: AvlRuntimeConfig) -> AvlRuntimeBuilder {
        AvlRuntimeBuilder::new(config)
    }

    /// Spawn a task on the current runtime
    pub fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        tokio::spawn(future)
    }

    /// Spawn a blocking task
    pub fn spawn_blocking<F, R>(f: F) -> tokio::task::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        tokio::task::spawn_blocking(f)
    }

    /// Sleep for duration
    pub async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await
    }

    /// Timeout a future
    pub async fn timeout<F>(duration: Duration, future: F) -> Result<F::Output, tokio::time::error::Elapsed>
    where
        F: Future,
    {
        tokio::time::timeout(duration, future).await
    }
}

impl Default for AvlRuntime {
    fn default() -> Self {
        Self
    }
}

/// AVL runtime builder
pub struct AvlRuntimeBuilder {
    config: AvlRuntimeConfig,
}

impl AvlRuntimeBuilder {
    /// Create new builder
    pub fn new(config: AvlRuntimeConfig) -> Self {
        Self { config }
    }

    /// Set worker threads
    pub fn worker_threads(mut self, threads: usize) -> Self {
        self.config.worker_threads = Some(threads);
        self
    }

    /// Set thread name prefix
    pub fn thread_name(mut self, name: impl Into<String>) -> Self {
        self.config.thread_name = name.into();
        self
    }

    /// Set thread stack size
    pub fn thread_stack_size(mut self, size: usize) -> Self {
        self.config.thread_stack_size = Some(size);
        self
    }

    /// Build the runtime
    pub fn build(self) -> tokio::runtime::Runtime {
        let mut builder = tokio::runtime::Builder::new_multi_thread();
        
        if let Some(threads) = self.config.worker_threads {
            builder.worker_threads(threads);
        }
        
        builder.thread_name(self.config.thread_name);
        
        if let Some(stack_size) = self.config.thread_stack_size {
            builder.thread_stack_size(stack_size);
        }
        
        if self.config.enable_io {
            builder.enable_io();
        }
        
        if self.config.enable_time {
            builder.enable_time();
        }
        
        builder.build().expect("Failed to build AVL runtime")
    }
}

/// AVL task utilities
pub struct AvlTask;

impl AvlTask {
    /// Spawn task with error logging
    pub fn spawn_logged<F>(name: &str, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let name = name.to_string();
        tokio::spawn(async move {
            let result = future.await;
            eprintln!("[AVL] Task '{}' completed", name);
            result
        })
    }

    /// Yield execution to scheduler
    pub async fn yield_now() {
        tokio::task::yield_now().await
    }
}

/// AVL channel utilities
pub struct AvlChannel;

impl AvlChannel {
    /// Create bounded MPSC channel
    pub fn bounded<T>(buffer: usize) -> (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>) {
        tokio::sync::mpsc::channel(buffer)
    }

    /// Create unbounded MPSC channel
    pub fn unbounded<T>() -> (tokio::sync::mpsc::UnboundedSender<T>, tokio::sync::mpsc::UnboundedReceiver<T>) {
        tokio::sync::mpsc::unbounded_channel()
    }

    /// Create oneshot channel
    pub fn oneshot<T>() -> (tokio::sync::oneshot::Sender<T>, tokio::sync::oneshot::Receiver<T>) {
        tokio::sync::oneshot::channel()
    }

    /// Create broadcast channel
    pub fn broadcast<T>(capacity: usize) -> (tokio::sync::broadcast::Sender<T>, tokio::sync::broadcast::Receiver<T>)
    where
        T: Clone,
    {
        tokio::sync::broadcast::channel(capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_task() {
        let handle = AvlRuntime::spawn(async {
            42
        });
        
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_spawn_blocking() {
        let handle = AvlRuntime::spawn_blocking(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            100
        });
        
        let result = handle.await.unwrap();
        assert_eq!(result, 100);
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let result = AvlRuntime::timeout(
            Duration::from_secs(1),
            async { 42 }
        ).await;
        
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_timeout_expired() {
        let result = AvlRuntime::timeout(
            Duration::from_millis(10),
            async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                42
            }
        ).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_channel_bounded() {
        let (tx, mut rx) = AvlChannel::bounded::<i32>(10);
        
        tx.send(42).await.unwrap();
        let received = rx.recv().await.unwrap();
        
        assert_eq!(received, 42);
    }

    #[tokio::test]
    async fn test_channel_oneshot() {
        let (tx, rx) = AvlChannel::oneshot();
        
        tx.send(100).unwrap();
        let received = rx.await.unwrap();
        
        assert_eq!(received, 100);
    }
}
