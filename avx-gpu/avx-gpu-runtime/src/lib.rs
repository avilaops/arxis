//! Runtime and scheduling for AVX-GPU

pub mod scheduler;
pub mod executor;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub max_concurrent_kernels: usize,
    pub enable_profiling: bool,
    pub enable_kernel_cache: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_concurrent_kernels: 4,
            enable_profiling: false,
            enable_kernel_cache: true,
        }
    }
}

/// GPU runtime
pub struct Runtime {
    config: RuntimeConfig,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}
