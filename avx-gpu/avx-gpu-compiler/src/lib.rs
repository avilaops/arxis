//! Kernel compiler for AVX-GPU
//!
//! Compiles GPU kernels from various source languages (WGSL, SPIR-V, etc.)
//! to target-specific formats.

pub mod wgsl;

/// Kernel source language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Wgsl,
    SpirV,
    Glsl,
}

/// Compilation target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Wgpu,
    Cuda,
    Metal,
    Vulkan,
}

/// Kernel compiler
pub struct Compiler {
    target: Target,
}

impl Compiler {
    pub fn new(target: Target) -> Self {
        Self { target }
    }

    pub fn compile(&self, source: &str, lang: SourceLanguage) -> anyhow::Result<Vec<u8>> {
        match (lang, self.target) {
            (SourceLanguage::Wgsl, Target::Wgpu) => {
                // WGSL is already the target format for wgpu
                Ok(source.as_bytes().to_vec())
            }
            _ => anyhow::bail!("Unsupported source/target combination: {:?} -> {:?}", lang, self.target),
        }
    }
}
