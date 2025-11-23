# AVX-GPU - Copilot Instructions

**Projeto**: avx-gpu
**Descrição**: Cross-Platform GPU Compute Framework - Surpassing CUDA in Developer Experience
**Status**: v0.1.0 - Foundation Complete, Multi-Backend Expansion
**Filosofia**: Performance + Portability. Rust-first. Zero C++ footprint.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Cross-Platform é Não-Negociável
```rust
// ✅ CORRETO: Backend abstraction
pub trait GpuBackend {
    fn device_info(&self) -> DeviceInfo;
    fn compile_kernel(&self, source: &str, entry: &str) -> Result<Kernel>;
    fn execute_kernel(&self, kernel: &Kernel, args: &[&Buffer]) -> Result<()>;
}

// Implementações:
impl GpuBackend for WgpuBackend { ... }  // ✅ Windows/Linux/macOS/Web
impl GpuBackend for CudaBackend { ... }  // ✅ NVIDIA only
impl GpuBackend for MetalBackend { ... } // ✅ Apple only
impl GpuBackend for RocmBackend { ... }  // ✅ AMD only

// ❌ ERRADO: Hardcoded CUDA
use cudarc::driver::*; // PROIBIDO sem abstraction!
```

**Motivo**: AVL Platform opera globalmente. Brasil tem mix NVIDIA/AMD. Apple users existem.

### 2. Target 90-110% CUDA Performance
```rust
// Benchmark obrigatório em cada PR
#[bench]
fn bench_matmul_1024_vs_cuda(b: &mut Bencher) {
    let device = Device::auto().unwrap();
    let a = device.buffer::<f32>(1024 * 1024).unwrap();
    let b = device.buffer::<f32>(1024 * 1024).unwrap();

    b.iter(|| {
        black_box(device.matmul(&a, &b).unwrap())
    });
}

// Target:
// - AVX-GPU (wgpu): 45ms @ RTX 4090
// - CUDA cuBLAS: 40ms @ RTX 4090
// - Ratio: 112% (acceptable!)
// - AVX-GPU (CUDA backend): 38ms (target 95% cuBLAS)
```

**Performance targets por operação**:
- Vector add: >95% CUDA
- Matrix multiply (GEMM): 85-100% cuBLAS
- FFT: 80-95% cuFFT
- Convolution: 85-100% cuDNN

### 3. Type-Safe GPU Memory
```rust
// ✅ CORRETO: Type-safe buffers
pub struct Buffer<T> {
    inner: Arc<dyn BufferImpl>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: GpuType> Buffer<T> {
    pub fn read(&self) -> Result<Vec<T>>;
    pub fn write(&mut self, data: &[T]) -> Result<()>;
    pub fn len(&self) -> usize;
}

// Compile-time type checking
let buf_f32: Buffer<f32> = device.buffer(1024)?;
let buf_i32: Buffer<i32> = device.buffer(1024)?;

device.execute_kernel(&kernel, &[&buf_f32, &buf_i32])?; // ✅ Types checked

// ❌ ERRADO: Type-erased buffers
let buf: Buffer = device.buffer(1024, "f32")?; // Runtime type!
```

### 4. WGSL Como Lingua Franca
```rust
// ✅ CORRETO: WGSL shader (cross-platform)
const VECTOR_ADD: &str = r#"
@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> c: array<f32>;

@compute @workgroup_size(256)
fn vector_add(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    if (idx < arrayLength(&a)) {
        c[idx] = a[idx] + b[idx];
    }
}
"#;

// Futuro: Rust → WGSL compiler
#[gpu_kernel]
fn vector_add(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

// ❌ ERRADO: CUDA-specific code sem abstraction
const CUDA_KERNEL: &str = "__global__ void kernel() { ... }"; // PROIBIDO!
```

---

## 📐 Arquitetura do Projeto

```
avx-gpu/
├── avx-gpu-core/
│   ├── src/
│   │   ├── lib.rs             # Public API
│   │   ├── device.rs          # Device abstraction
│   │   ├── buffer.rs          # Type-safe buffers
│   │   ├── kernel.rs          # Kernel abstraction
│   │   ├── backend.rs         # Backend trait
│   │   ├── error.rs           # Error types
│   │   └── types.rs           # GpuType trait
│   └── Cargo.toml
├── avx-gpu-backends/
│   ├── wgpu/
│   │   ├── src/
│   │   │   ├── lib.rs         # WgpuBackend impl
│   │   │   ├── device.rs
│   │   │   ├── buffer.rs
│   │   │   ├── kernel.rs
│   │   │   └── compiler.rs    # WGSL → SPIR-V
│   │   └── Cargo.toml
│   ├── cuda/
│   │   ├── src/
│   │   │   ├── lib.rs         # CudaBackend impl
│   │   │   ├── device.rs
│   │   │   ├── buffer.rs
│   │   │   ├── kernel.rs
│   │   │   └── compiler.rs    # WGSL → PTX
│   │   └── Cargo.toml
│   ├── metal/
│   │   ├── src/
│   │   │   ├── lib.rs         # MetalBackend impl
│   │   │   ├── device.rs
│   │   │   ├── buffer.rs
│   │   │   ├── kernel.rs
│   │   │   └── compiler.rs    # WGSL → Metal
│   │   └── Cargo.toml
│   ├── rocm/
│   │   ├── src/
│   │   │   ├── lib.rs         # RocmBackend impl
│   │   │   ├── device.rs
│   │   │   ├── buffer.rs
│   │   │   ├── kernel.rs
│   │   │   └── compiler.rs    # WGSL → AMDGPU
│   │   └── Cargo.toml
│   └── vulkan/
│       ├── src/
│       │   ├── lib.rs         # VulkanBackend (via ash)
│       │   └── ...
│       └── Cargo.toml
├── avx-gpu-compiler/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── parser.rs          # Rust AST → IR
│   │   ├── optimizer.rs       # IR optimizations
│   │   ├── codegen/
│   │   │   ├── wgsl.rs        # IR → WGSL
│   │   │   ├── spirv.rs       # IR → SPIR-V
│   │   │   ├── ptx.rs         # IR → PTX (CUDA)
│   │   │   └── metal.rs       # IR → Metal
│   │   └── analysis.rs        # Data flow analysis
│   └── Cargo.toml
├── avx-gpu-runtime/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── scheduler.rs       # Multi-GPU scheduling
│   │   ├── memory_pool.rs     # Pooled allocator
│   │   ├── stream.rs          # Async execution
│   │   └── profiler.rs        # Performance profiling
│   └── Cargo.toml
├── avx-gpu-std/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── linalg/
│   │   │   ├── vector.rs      # BLAS Level 1
│   │   │   ├── matrix.rs      # BLAS Level 2/3
│   │   │   ├── gemm.rs        # Optimized GEMM
│   │   │   └── svd.rs         # SVD decomposition
│   │   ├── signal/
│   │   │   ├── fft.rs         # FFT (Cooley-Tukey)
│   │   │   ├── convolution.rs
│   │   │   └── filter.rs
│   │   ├── image/
│   │   │   ├── resize.rs
│   │   │   ├── convolution.rs
│   │   │   └── transform.rs
│   │   └── nn/
│   │       ├── conv2d.rs      # Convolution layers
│   │       ├── linear.rs      # Fully connected
│   │       └── activation.rs  # ReLU, etc.
│   └── Cargo.toml
├── avx-gpu-macros/
│   ├── src/
│   │   ├── lib.rs
│   │   └── gpu_kernel.rs      # #[gpu_kernel] macro
│   └── Cargo.toml
└── examples/
    ├── vector_add.rs
    ├── matrix_multiply.rs
    ├── fft.rs
    └── image_filter.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: Foundation (v0.1.0) ✅ COMPLETO
```rust
// ✅ Core API
pub struct Device {
    backend: Arc<dyn GpuBackend>,
}

impl Device {
    pub fn auto() -> Result<Self> {
        // Try backends in order:
        // 1. CUDA (if NVIDIA GPU)
        // 2. Metal (if Apple Silicon)
        // 3. ROCm (if AMD GPU)
        // 4. wgpu (fallback, works everywhere)
    }

    pub fn from_backend(backend: impl GpuBackend + 'static) -> Self;

    pub fn buffer<T: GpuType>(&self, len: usize) -> Result<Buffer<T>>;
    pub fn buffer_from_slice<T: GpuType>(&self, data: &[T]) -> Result<Buffer<T>>;

    pub fn compile_kernel(&self, source: &str, entry: &str) -> Result<Kernel>;
    pub fn execute_kernel(&self, kernel: &Kernel, args: &[&dyn BufferTrait]) -> Result<()>;
}

// ✅ wgpu backend
pub struct WgpuBackend {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}
```

**Deliverables**:
- [x] Core abstractions (Device, Buffer, Kernel)
- [x] wgpu backend (cross-platform)
- [x] Type-safe buffer API
- [x] WGSL kernel compilation
- [x] Examples (vector_add, matmul)
- [x] Benchmarks vs CPU

### Fase 2: Multi-Backend (v0.2.0) - Semanas 1-4
```rust
// TODO: CUDA backend
pub struct CudaBackend {
    context: CudaContext,
    device: CudaDevice,
    stream: CudaStream,
}

impl GpuBackend for CudaBackend {
    fn compile_kernel(&self, source: &str, entry: &str) -> Result<Kernel> {
        // 1. Parse WGSL
        // 2. Convert to CUDA PTX
        // 3. Load with cuModuleLoadData
        // 4. Get kernel function handle

        let module = naga::front::wgsl::parse_str(source)?;
        let ptx = wgsl_to_ptx(&module)?;

        let cuda_module = self.context.load_module(&ptx)?;
        let function = cuda_module.get_function(entry)?;

        Ok(Kernel {
            backend_kernel: Box::new(CudaKernel { function }),
        })
    }

    fn execute_kernel(&self, kernel: &Kernel, args: &[&Buffer]) -> Result<()> {
        // Launch kernel with grid/block config
        let grid_size = (args[0].len() + 255) / 256;
        let block_size = 256;

        unsafe {
            kernel.launch(
                grid_size,
                block_size,
                args.iter().map(|b| b.device_ptr()).collect(),
            )?;
        }

        self.stream.synchronize()?;
        Ok(())
    }
}

// TODO: Metal backend
pub struct MetalBackend {
    device: metal::Device,
    command_queue: metal::CommandQueue,
}

impl GpuBackend for MetalBackend {
    fn compile_kernel(&self, source: &str, entry: &str) -> Result<Kernel> {
        // WGSL → Metal Shading Language
        let module = naga::front::wgsl::parse_str(source)?;
        let msl = wgsl_to_metal(&module)?;

        let library = self.device.new_library_with_source(&msl, &metal::CompileOptions::new())?;
        let function = library.get_function(entry, None)?;

        Ok(Kernel {
            backend_kernel: Box::new(MetalKernel { function }),
        })
    }
}

// TODO: ROCm backend (HIP)
pub struct RocmBackend {
    device: hip_sys::Device,
    context: hip_sys::Context,
}

impl GpuBackend for RocmBackend {
    fn compile_kernel(&self, source: &str, entry: &str) -> Result<Kernel> {
        // WGSL → AMDGPU assembly
        let module = naga::front::wgsl::parse_str(source)?;
        let asm = wgsl_to_amdgpu(&module)?;

        // Compile with ROCm compiler
        let code_object = hip_compile(&asm)?;

        Ok(Kernel {
            backend_kernel: Box::new(RocmKernel { code_object }),
        })
    }
}
```

**Backends a implementar**:
1. CUDA (NVIDIA) ✅ Priority 1
2. Metal (Apple) ✅ Priority 2
3. ROCm (AMD) ✅ Priority 3
4. Vulkan (via ash, fallback) ⏳ Priority 4

### Fase 3: Kernel Compiler (v0.3.0) - Semanas 5-8
```rust
// TODO: #[gpu_kernel] macro
#[gpu_kernel]
fn vector_add(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

// Expands to:
mod __gpu_vector_add {
    pub const WGSL_SOURCE: &str = r#"
        @group(0) @binding(0) var<storage, read> a: array<f32>;
        @group(0) @binding(1) var<storage, read> b: array<f32>;
        @group(0) @binding(2) var<storage, read_write> result: array<f32>;

        @compute @workgroup_size(256)
        fn vector_add(@builtin(global_invocation_id) id: vec3<u32>) {
            let idx = id.x;
            if (idx < arrayLength(&a)) {
                result[idx] = a[idx] + b[idx];
            }
        }
    "#;

    pub fn run(device: &Device, a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
        let buf_a = device.buffer_from_slice(a)?;
        let buf_b = device.buffer_from_slice(b)?;
        let mut buf_result = device.buffer::<f32>(a.len())?;

        let kernel = device.compile_kernel(WGSL_SOURCE, "vector_add")?;
        device.execute_kernel(&kernel, &[&buf_a, &buf_b, &buf_result])?;

        buf_result.read()
    }
}

pub use __gpu_vector_add::run as vector_add;

// Compiler pipeline:
// 1. Parse Rust AST (syn)
// 2. Convert to intermediate IR
// 3. Optimize IR (dead code, constant folding)
// 4. Generate WGSL
// 5. Embed in Rust code
```

### Fase 4: GPU Standard Library (v0.4.0) - Semanas 9-12
```rust
// TODO: GPU BLAS (Basic Linear Algebra Subprograms)
pub mod linalg {
    // Level 1: Vector operations
    pub fn dot(a: &Buffer<f32>, b: &Buffer<f32>) -> Result<f32>;
    pub fn axpy(alpha: f32, x: &Buffer<f32>, y: &mut Buffer<f32>) -> Result<()>; // y = alpha*x + y
    pub fn norm(x: &Buffer<f32>) -> Result<f32>;

    // Level 2: Matrix-vector
    pub fn gemv(
        alpha: f32,
        a: &Buffer<f32>,  // m x n matrix
        x: &Buffer<f32>,  // n vector
        beta: f32,
        y: &mut Buffer<f32>, // m vector
    ) -> Result<()>; // y = alpha*A*x + beta*y

    // Level 3: Matrix-matrix
    pub fn gemm(
        alpha: f32,
        a: &Buffer<f32>,  // m x k
        b: &Buffer<f32>,  // k x n
        beta: f32,
        c: &mut Buffer<f32>, // m x n
    ) -> Result<()>; // C = alpha*A*B + beta*C

    // Advanced
    pub fn svd(a: &Buffer<f32>) -> Result<SVDResult>;
    pub fn qr(a: &Buffer<f32>) -> Result<QRResult>;
}

// TODO: GPU Signal Processing
pub mod signal {
    pub fn fft(signal: &Buffer<Complex<f32>>) -> Result<Buffer<Complex<f32>>>;
    pub fn ifft(spectrum: &Buffer<Complex<f32>>) -> Result<Buffer<Complex<f32>>>;
    pub fn convolve(a: &Buffer<f32>, b: &Buffer<f32>) -> Result<Buffer<f32>>;
    pub fn correlate(a: &Buffer<f32>, b: &Buffer<f32>) -> Result<Buffer<f32>>;
}

// TODO: GPU Image Processing
pub mod image {
    pub fn resize(
        input: &Buffer<u8>,
        input_size: (usize, usize),
        output_size: (usize, usize),
    ) -> Result<Buffer<u8>>;

    pub fn gaussian_blur(
        input: &Buffer<u8>,
        size: (usize, usize),
        sigma: f32,
    ) -> Result<Buffer<u8>>;

    pub fn sobel_filter(
        input: &Buffer<u8>,
        size: (usize, usize),
    ) -> Result<Buffer<u8>>;
}
```

### Fase 5: Advanced Features (v0.5.0) - Semanas 13-16
```rust
// TODO: Multi-GPU support
pub struct MultiGpuDevice {
    devices: Vec<Device>,
    scheduler: Scheduler,
}

impl MultiGpuDevice {
    pub fn all_gpus() -> Result<Self>;

    pub fn split_buffer<T: GpuType>(&self, data: &[T]) -> Result<Vec<Buffer<T>>> {
        // Distribute data across GPUs
        // Use NCCL/RCCL for inter-GPU communication
    }

    pub fn parallel_execute<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&Device) -> Result<()> + Send + Sync;
}

// TODO: Async execution
pub struct Stream {
    backend_stream: Box<dyn StreamImpl>,
}

impl Stream {
    pub fn execute_async(&self, kernel: &Kernel, args: &[&Buffer]) -> Result<Event>;
    pub fn synchronize(&self) -> Result<()>;
}

pub struct Event {
    backend_event: Box<dyn EventImpl>,
}

impl Event {
    pub fn wait(&self) -> Result<()>;
    pub fn is_complete(&self) -> bool;
}

// TODO: Memory pool
pub struct MemoryPool {
    free_blocks: Vec<(usize, *mut u8)>,
    used_blocks: HashMap<*mut u8, usize>,
}

impl MemoryPool {
    pub fn allocate(&mut self, size: usize) -> Result<*mut u8>;
    pub fn deallocate(&mut self, ptr: *mut u8);
    pub fn reset(&mut self); // Free all without deallocation
}

// TODO: Auto-tuning
pub struct AutoTuner {
    cache: HashMap<String, TuneParams>,
}

impl AutoTuner {
    pub fn tune_kernel(&mut self, kernel: &Kernel) -> Result<TuneParams> {
        // Try different block sizes, shared memory configs
        // Cache best configuration
    }
}
```

---

## 🧪 Testes Obrigatórios

### 1. Cross-Backend Compatibility
```rust
#[test]
fn test_vector_add_all_backends() {
    let backends = vec![
        Device::wgpu(),
        Device::cuda().ok(),
        Device::metal().ok(),
        Device::rocm().ok(),
    ];

    for backend in backends.into_iter().flatten() {
        let a = backend.buffer_from_slice(&[1.0f32, 2.0, 3.0, 4.0]).unwrap();
        let b = backend.buffer_from_slice(&[5.0f32, 6.0, 7.0, 8.0]).unwrap();
        let mut c = backend.buffer::<f32>(4).unwrap();

        let kernel = backend.compile_kernel(VECTOR_ADD, "vector_add").unwrap();
        backend.execute_kernel(&kernel, &[&a, &b, &c]).unwrap();

        let result = c.read().unwrap();
        assert_eq!(result, vec![6.0, 8.0, 10.0, 12.0]);
    }
}
```

### 2. Performance vs CUDA
```rust
#[bench]
fn bench_matmul_1024_avx_gpu(b: &mut Bencher) {
    let device = Device::auto().unwrap();
    let a = device.buffer::<f32>(1024 * 1024).unwrap();
    let b = device.buffer::<f32>(1024 * 1024).unwrap();

    b.iter(|| {
        black_box(linalg::gemm(1.0, &a, &b, 0.0, &mut c).unwrap())
    });
}

#[bench]
fn bench_matmul_1024_cublas(b: &mut Bencher) {
    // Compare against cuBLAS
    let handle = cublas::CublasHandle::new().unwrap();
    // ...
}

// Target: AVX-GPU >= 90% cuBLAS performance
```

### 3. Memory Safety
```rust
#[test]
fn test_buffer_type_safety() {
    let device = Device::auto().unwrap();

    let buf_f32: Buffer<f32> = device.buffer(1024).unwrap();
    let buf_i32: Buffer<i32> = device.buffer(1024).unwrap();

    // This should compile
    let _: Vec<f32> = buf_f32.read().unwrap();

    // This should NOT compile (type mismatch)
    // let _: Vec<i32> = buf_f32.read().unwrap(); // ❌
}
```

---

## 📊 API Pública

### Core API
```rust
pub struct Device {
    backend: Arc<dyn GpuBackend>,
}

impl Device {
    // Device selection
    pub fn auto() -> Result<Self>;
    pub fn wgpu() -> Result<Self>;
    pub fn cuda() -> Result<Self>;
    pub fn metal() -> Result<Self>;
    pub fn rocm() -> Result<Self>;
    pub fn from_backend(backend: impl GpuBackend + 'static) -> Self;

    // Device info
    pub fn name(&self) -> &str;
    pub fn vendor(&self) -> Vendor;
    pub fn compute_units(&self) -> usize;
    pub fn memory_size(&self) -> usize;

    // Memory management
    pub fn buffer<T: GpuType>(&self, len: usize) -> Result<Buffer<T>>;
    pub fn buffer_from_slice<T: GpuType>(&self, data: &[T]) -> Result<Buffer<T>>;

    // Kernel execution
    pub fn compile_kernel(&self, source: &str, entry: &str) -> Result<Kernel>;
    pub fn execute_kernel(&self, kernel: &Kernel, args: &[&dyn BufferTrait]) -> Result<()>;
}

pub struct Buffer<T> {
    inner: Arc<dyn BufferImpl>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: GpuType> Buffer<T> {
    pub fn len(&self) -> usize;
    pub fn read(&self) -> Result<Vec<T>>;
    pub fn write(&mut self, data: &[T]) -> Result<()>;
    pub fn copy_from(&mut self, src: &Buffer<T>) -> Result<()>;
}

pub struct Kernel {
    backend_kernel: Box<dyn KernelImpl>,
}

pub trait GpuType: Copy + Send + Sync + 'static {
    fn wgsl_type() -> &'static str;
}

impl GpuType for f32 {
    fn wgsl_type() -> &'static str { "f32" }
}
impl GpuType for i32 {
    fn wgsl_type() -> &'static str { "i32" }
}
// ... outros tipos
```

---

## ⚠️ Erros Comuns a Evitar

### 1. Backend-Specific Code
```rust
// ❌ ERRADO: Expor detalhes do backend
pub fn execute_cuda_kernel(ptx: &str) { ... }

// ✅ CORRETO: Backend-agnostic API
pub fn execute_kernel(kernel: &Kernel) { ... }
```

### 2. Synchronous API Blocking
```rust
// ❌ ERRADO: Block main thread
let result = buffer.read()?; // Blocks!

// ✅ CORRETO: Async where possible
let result = buffer.read_async().await?;
```

### 3. Memory Leaks
```rust
// ❌ ERRADO: No cleanup
fn process() {
    let buf = device.buffer::<f32>(1_000_000)?; // Leak!
    // Esqueceu de dropar
}

// ✅ CORRETO: RAII pattern
fn process() {
    let buf = device.buffer::<f32>(1_000_000)?;
    // ... use buf
    // Automatic drop at end of scope
}
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR:

- [ ] **Cross-Platform**: Funciona em wgpu, CUDA, Metal, ROCm
- [ ] **Performance**: ≥90% CUDA para operação testada
- [ ] **Type Safety**: Buffers type-safe em compile-time
- [ ] **Zero Unsafe**: Minimizar unsafe Rust
- [ ] **Docs**: Cada função pública documentada
- [ ] **Tests**: Unit tests + cross-backend tests
- [ ] **Benchmarks**: vs CUDA/cuBLAS/cuDNN
- [ ] **Examples**: Código funcional para usuários

---

## 🚀 Como Começar

### Setup
```bash
cd arxis/avx-gpu
cargo build --all
cargo test --all
```

### Exemplos
```bash
# Vector add (cross-platform)
cargo run --example vector_add

# Matrix multiply (optimized)
cargo run --example matrix_multiply

# FFT
cargo run --example fft
```

### Benchmarks
```bash
# Internal benchmarks
cargo bench --workspace

# vs CUDA (se disponível)
cargo bench --bench vs_cuda
```

---

**Lembre-se**: Cross-platform é não-negociável. Performance > 90% CUDA. Type-safety em compile-time.

**AVX-GPU** - GPU Computing for Everyone 🚀
