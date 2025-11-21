//! GPU buffer management

use crate::backend::Backend;
use crate::error::Result;
use parking_lot::RwLock;
use std::marker::PhantomData;
use std::sync::Arc;

/// Buffer usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferUsage {
    pub storage: bool,
    pub uniform: bool,
    pub copy_src: bool,
    pub copy_dst: bool,
}

impl Default for BufferUsage {
    fn default() -> Self {
        Self {
            storage: true,
            copy_src: true,
            copy_dst: true,
            uniform: false,
        }
    }
}

/// Opaque handle to GPU buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferHandle(pub u64);

/// Typed GPU buffer
pub struct Buffer<T: bytemuck::Pod> {
    handle: BufferHandle,
    count: usize,
    backend: Arc<RwLock<Box<dyn Backend>>>,
    _phantom: PhantomData<T>,
}

impl<T: bytemuck::Pod> Buffer<T> {
    pub(crate) fn new(handle: BufferHandle, count: usize, backend: Arc<RwLock<Box<dyn Backend>>>) -> Self {
        Self {
            handle,
            count,
            backend,
            _phantom: PhantomData,
        }
    }

    /// Get buffer handle
    pub fn handle(&self) -> BufferHandle {
        self.handle
    }

    /// Get element count
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get buffer size in bytes
    pub fn size_bytes(&self) -> usize {
        self.count * std::mem::size_of::<T>()
    }

    /// Write data to buffer
    pub fn write(&mut self, data: &[T]) -> Result<()> {
        if data.len() > self.count {
            return Err(crate::error::Error::BufferSizeMismatch {
                expected: self.count,
                actual: data.len(),
            });
        }
        let bytes = bytemuck::cast_slice(data);
        self.backend.write().write_buffer(self.handle, bytes)
    }

    /// Read data from buffer
    pub fn read(&self) -> Result<Vec<T>> {
        let mut data = vec![T::zeroed(); self.count];
        let bytes = bytemuck::cast_slice_mut(&mut data);
        self.backend.write().read_buffer(self.handle, bytes)?;
        Ok(data)
    }

    /// Copy data from another buffer
    pub fn copy_from(&mut self, src: &Buffer<T>) -> Result<()> {
        if src.count != self.count {
            return Err(crate::error::Error::BufferSizeMismatch {
                expected: self.count,
                actual: src.count,
            });
        }
        self.backend
            .write()
            .copy_buffer(src.handle, self.handle, self.size_bytes())
    }
}

impl<T: bytemuck::Pod> Drop for Buffer<T> {
    fn drop(&mut self) {
        let _ = self.backend.write().free_buffer(self.handle);
    }
}

impl<T: bytemuck::Pod> std::fmt::Debug for Buffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("handle", &self.handle)
            .field("count", &self.count)
            .field("type", &std::any::type_name::<T>())
            .finish()
    }
}
