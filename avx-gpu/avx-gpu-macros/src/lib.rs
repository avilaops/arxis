//! Procedural macros for AVX-GPU
//!
//! Future home of #[gpu_kernel] macro for writing GPU kernels in Rust.

use proc_macro::TokenStream;

/// Mark a function as a GPU kernel (coming soon)
///
/// # Example
///
/// ```ignore
/// #[gpu_kernel]
/// fn vector_add(a: &[f32], b: &[f32], c: &mut [f32]) {
///     let idx = thread_id();
///     c[idx] = a[idx] + b[idx];
/// }
/// ```
#[proc_macro_attribute]
pub fn gpu_kernel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: Implement macro that converts Rust code to GPU kernel
    // For now, just pass through the input
    item
}
