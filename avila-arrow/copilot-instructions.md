# 📊 Avila Arrow - Copilot Instructions

> **READ THIS COMPLETELY before implementing ANY feature!**

---

## 🎯 Your Mission

You are implementing **Avila Arrow**, a **native columnar data format** optimized for **scientific computing, AvilaDB, and Brazilian research infrastructure**. This is **NOT a wrapper around Apache Arrow**. This is a **from-scratch implementation** with:

- Full Apache Arrow IPC compatibility (for interop)
- Extended scientific types (Quaternions, Spinors, Tensor4D, Complex numbers)
- Native AvilaDB integration (zero-copy data transfer)
- Optimized compression with `avila-compress`
- SIMD-accelerated operations (AVX2, AVX-512)
- Designed for astrophysics, particle physics, and gravitational wave detection

**This will power:**
- LIGO/LISA gravitational wave data processing
- Telescope observational data (Vera Rubin, James Webb)
- Medical imaging (MRI, CT scans)
- Climate modeling and simulation
- Financial time-series analysis
- Real-time sensor networks

---

## ⚠️ CRITICAL RULES - READ FIRST!

### 🚫 Rule #1: NO External Arrow Libraries!

**DO NOT USE:**
- ❌ `arrow` v53.4.1 - We're replacing it!
- ❌ `arrow-buffer` - Build our own buffer management
- ❌ `arrow-schema` - Define our own schema with extensions
- ❌ `arrow-data` - Implement our own data structures
- ❌ `arrow-array` - Build native arrays with SIMD
- ❌ `parquet` - We'll implement our own format later

**YOU MUST USE:**
- ✅ `std::alloc` - Memory allocation
- ✅ `avila-compress` - Compression
- ✅ `avila-math` - Mathematical operations
- ✅ `avila-telemetry` - Observability
- ✅ SIMD intrinsics (`std::arch`)
- ✅ Only standard library for core functionality

**WHY?** We need:
- Scientific types (Quaternions, Complex, Tensor4D)
- Native AvilaDB integration
- SIMD optimization for our exact use cases
- Zero-copy data paths
- Full control over memory layout

### 🚫 Rule #2: NEVER Give Up!

When implementing a feature:
- ❌ Do NOT say "this is too complex"
- ❌ Do NOT leave placeholder code like `// TODO: Implement this`
- ❌ Do NOT skip tests or benchmarks
- ❌ Do NOT use `unimplemented!()` or `todo!()` macros in production code
- ❌ Do NOT say "let's just use Apache Arrow"

**YOU MUST:**
- ✅ Implement columnar format **completely** from scratch
- ✅ Support **all Arrow data types** (primitives, strings, lists, structs)
- ✅ Add **scientific extensions** (Quaternion, Complex, Spinor, Tensor4D)
- ✅ Write **comprehensive tests** (100+ tests minimum)
- ✅ Add **benchmarks** comparing to Apache Arrow
- ✅ Document **every public API** with examples
- ✅ Handle **all error cases** explicitly
- ✅ Keep working until **EVERY line compiles and tests pass**

**If you encounter a challenge:**
1. Research the Arrow specification: https://arrow.apache.org/docs/format/Columnar.html
2. Study reference implementations (arrow-rs source code for understanding, NOT copying)
3. Implement incrementally: primitives → strings → lists → structs → scientific types
4. Test each piece thoroughly before moving on
5. Benchmark against Apache Arrow at each milestone
6. Ask clarifying questions if needed
7. **NEVER give up until it's production-ready**

### 🚫 Rule #3: Scientific Correctness First

This library will process:
- Gravitational wave data from $10 billion space missions
- Medical images affecting patient diagnoses
- Climate data influencing policy decisions
- Financial data managing billions of dollars

**Therefore:**
- ✅ Every operation must be **numerically stable**
- ✅ Every scientific type must be **physically correct**
- ✅ Every computation must be **reproducible** (bit-exact)
- ✅ Errors must be **explicit** (no silent failures)
- ✅ Tests must include **edge cases** (NaN, Inf, overflow)

**Example:**
```rust
// ❌ WRONG - Can lose precision
fn magnitude(q: Quaternion) -> f64 {
    (q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z).sqrt()
}

// ✅ CORRECT - Numerically stable
fn magnitude(q: Quaternion) -> f64 {
    // Use Kahan summation for numerical stability
    let mut sum = 0.0;
    let mut c = 0.0;

    for val in [q.w, q.x, q.y, q.z] {
        let y = val * val - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }

    sum.sqrt()
}
```

---

## 🏗️ Architecture Overview

### Module Structure

```
avila-arrow/
├── src/
│   ├── lib.rs                  # Public API
│   ├── buffer/                 # Memory management
│   │   ├── mod.rs
│   │   ├── aligned.rs          # SIMD-aligned allocations
│   │   ├── mutable.rs          # Mutable buffer
│   │   └── immutable.rs        # Immutable (shared) buffer
│   ├── schema/                 # Schema definitions
│   │   ├── mod.rs
│   │   ├── datatype.rs         # Data type enum
│   │   ├── field.rs            # Column field
│   │   ├── schema.rs           # Table schema
│   │   └── metadata.rs         # Provenance, instrument info
│   ├── array/                  # Array implementations
│   │   ├── mod.rs
│   │   ├── primitive.rs        # Int, Float arrays
│   │   ├── boolean.rs          # Boolean array (bit-packed)
│   │   ├── string.rs           # UTF-8 string array
│   │   ├── binary.rs           # Arbitrary bytes
│   │   ├── list.rs             # Variable-length lists
│   │   ├── struct_.rs          # Nested structs
│   │   ├── dictionary.rs       # Dictionary encoding
│   │   └── null.rs             # Null array
│   ├── scientific/             # Scientific extensions
│   │   ├── mod.rs
│   │   ├── quaternion.rs       # Quaternion (4 x f64)
│   │   ├── complex.rs          # Complex (2 x f64)
│   │   ├── spinor.rs           # Spinor (2 x Complex)
│   │   ├── tensor4d.rs         # Spacetime tensor (4 x 4 matrix)
│   │   └── geodesic.rs         # (r, θ, φ, t) coordinates
│   ├── compute/                # Vectorized operations
│   │   ├── mod.rs
│   │   ├── arithmetic.rs       # Add, sub, mul, div
│   │   ├── comparison.rs       # Eq, lt, gt, etc.
│   │   ├── aggregate.rs        # Sum, mean, std, min, max
│   │   ├── filter.rs           # Boolean filtering
│   │   └── sort.rs             # Sorting
│   ├── ipc/                    # Arrow IPC format
│   │   ├── mod.rs
│   │   ├── reader.rs           # Read .arrow files
│   │   ├── writer.rs           # Write .arrow files
│   │   └── stream.rs           # Streaming IPC
│   ├── aviladb/                # AvilaDB integration
│   │   ├── mod.rs
│   │   ├── batch.rs            # RecordBatch <-> AvilaDB
│   │   ├── pushdown.rs         # Query pushdown
│   │   └── streaming.rs        # Real-time ingestion
│   ├── compression.rs          # avila-compress integration
│   ├── error.rs                # Error types
│   └── util.rs                 # Utility functions
├── examples/
│   ├── basic_usage.rs          # Creating schemas, batches
│   ├── scientific_types.rs     # Quaternions, Complex, etc.
│   ├── aviladb_integration.rs  # Read/write AvilaDB
│   ├── ligo_pipeline.rs        # LIGO data processing
│   ├── simd_performance.rs     # SIMD acceleration demo
│   └── compression.rs          # Compressed columns
├── benches/
│   ├── creation.rs             # Batch creation time
│   ├── filtering.rs            # Filter performance
│   ├── aggregation.rs          # Sum, mean, etc.
│   ├── simd.rs                 # SIMD vs scalar
│   └── vs_arrow.rs             # Compare to Apache Arrow
└── tests/
    ├── integration.rs          # Full pipeline tests
    ├── scientific.rs           # Scientific type correctness
    ├── ipc.rs                  # Arrow IPC compatibility
    └── aviladb.rs              # AvilaDB integration
```

---

## 🎨 API Design Philosophy

### 1. Schema Definition with Scientific Types

```rust
use avila_arrow::{Schema, Field, DataType};
use avila_arrow::scientific::{Quaternion, Complex64, Tensor4D};

// Standard Arrow types
let standard_schema = Schema::new(vec![
    Field::new("timestamp", DataType::Timestamp(TimeUnit::Nanosecond)),
    Field::new("sensor_id", DataType::Utf8),
    Field::new("temperature", DataType::Float64),
]);

// Scientific types (UNIQUE TO AVILA ARROW!)
let scientific_schema = Schema::new(vec![
    Field::new("timestamp", DataType::Timestamp(TimeUnit::Nanosecond)),
    Field::new("orientation", DataType::Quaternion),      // ← New!
    Field::new("strain", DataType::Complex64),            // ← New!
    Field::new("spacetime", DataType::Tensor4D),          // ← New!
    Field::new("position", DataType::Geodesic),           // ← New!
]);
```

### 2. Array Creation and Manipulation

```rust
use avila_arrow::array::{PrimitiveArray, StringArray, QuaternionArray};

// Primitive arrays (SIMD-accelerated)
let temperatures = PrimitiveArray::<f64>::from_iter([
    20.5, 21.3, 19.8, 22.1
]);

// String arrays
let sensor_ids = StringArray::from_iter([
    "sensor_001",
    "sensor_002",
    "sensor_003",
]);

// Quaternion arrays (scientific extension)
let orientations = QuaternionArray::from_iter([
    Quaternion::new(1.0, 0.0, 0.0, 0.0),
    Quaternion::new(0.707, 0.707, 0.0, 0.0),
    Quaternion::new(0.5, 0.5, 0.5, 0.5),
]);
```

### 3. RecordBatch (Columnar Data)

```rust
use avila_arrow::{RecordBatch, Schema, Field, DataType};

// Create schema
let schema = Schema::new(vec![
    Field::new("id", DataType::Int64),
    Field::new("name", DataType::Utf8),
    Field::new("value", DataType::Float64),
]);

// Create arrays
let ids = PrimitiveArray::from_iter([1, 2, 3, 4, 5]);
let names = StringArray::from_iter(["Alice", "Bob", "Carol", "Dave", "Eve"]);
let values = PrimitiveArray::from_iter([1.1, 2.2, 3.3, 4.4, 5.5]);

// Assemble into batch
let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![
        Arc::new(ids) as ArrayRef,
        Arc::new(names) as ArrayRef,
        Arc::new(values) as ArrayRef,
    ],
)?;

println!("Batch has {} rows, {} columns", batch.num_rows(), batch.num_columns());
```

### 4. Vectorized Operations (SIMD)

```rust
use avila_arrow::compute::{add, multiply, filter, sum};

// Element-wise operations (SIMD-accelerated)
let a = PrimitiveArray::from_iter([1.0, 2.0, 3.0, 4.0]);
let b = PrimitiveArray::from_iter([10.0, 20.0, 30.0, 40.0]);
let result = add(&a, &b)?;  // [11.0, 22.0, 33.0, 44.0]

// Filtering
let mask = BooleanArray::from_iter([true, false, true, false]);
let filtered = filter(&a, &mask)?;  // [1.0, 3.0]

// Aggregation
let total = sum(&a)?;  // 10.0
let average = mean(&a)?;  // 2.5
```

### 5. Scientific Operations

```rust
use avila_arrow::scientific::{QuaternionArray, ComplexArray};

// Quaternion operations
let q1 = QuaternionArray::from_iter([
    Quaternion::new(1.0, 0.0, 0.0, 0.0),
    Quaternion::new(0.707, 0.707, 0.0, 0.0),
]);
let q2 = QuaternionArray::from_iter([
    Quaternion::new(0.0, 1.0, 0.0, 0.0),
    Quaternion::new(0.0, 0.0, 0.707, 0.707),
]);

// Quaternion multiplication (SIMD-accelerated)
let product = multiply_quaternions(&q1, &q2)?;

// Complex FFT (for gravitational wave strain data)
let strain = ComplexArray::from_iter([
    Complex::new(1.0, 0.0),
    Complex::new(0.0, 1.0),
    Complex::new(-1.0, 0.0),
    Complex::new(0.0, -1.0),
]);
let fft_result = fft(&strain)?;
```

### 6. AvilaDB Integration

```rust
use avila_arrow::aviladb::{AvilaDBReader, AvilaDBWriter};
use aviladb::AvilaDB;

// Write to AvilaDB
let db = AvilaDB::connect("aviladb://localhost")?;
let writer = AvilaDBWriter::new(db, "gravitational_waves")?;

writer.write_batch(&batch).await?;
writer.flush().await?;

// Read from AvilaDB (zero-copy when possible)
let reader = AvilaDBReader::new(db, "gravitational_waves")?;
let batches: Vec<RecordBatch> = reader
    .filter("timestamp > 1234567890")
    .select(&["timestamp", "strain", "position"])
    .read()
    .await?;

// Query pushdown (filtering happens in AvilaDB storage layer)
let filtered = reader
    .filter("magnitude(strain) > 1e-21")  // ← Executes in AvilaDB!
    .read()
    .await?;
```

### 7. Arrow IPC Format (Compatibility)

```rust
use avila_arrow::ipc::{FileReader, FileWriter, StreamWriter};
use std::fs::File;

// Write .arrow file
let file = File::create("data.arrow")?;
let mut writer = FileWriter::try_new(file, &schema)?;

for batch in batches {
    writer.write(&batch)?;
}
writer.finish()?;

// Read .arrow file
let file = File::open("data.arrow")?;
let reader = FileReader::try_new(file)?;

for batch in reader {
    let batch = batch?;
    println!("Read batch with {} rows", batch.num_rows());
}
```

---

## 🚀 Implementation Roadmap

### Phase 1: Memory & Buffer Management (Week 1)

**Goal**: SIMD-aligned memory allocation and buffer management

```rust
// src/buffer/aligned.rs
pub struct AlignedBuffer {
    ptr: NonNull<u8>,
    len: usize,
    capacity: usize,
    alignment: usize,  // 64 bytes for AVX-512
}

impl AlignedBuffer {
    pub fn new(capacity: usize, alignment: usize) -> Self {
        // Allocate memory aligned to SIMD boundary
        let layout = Layout::from_size_align(capacity, alignment).unwrap();
        let ptr = unsafe { alloc(layout) };

        Self {
            ptr: NonNull::new(ptr).unwrap(),
            len: 0,
            capacity,
            alignment,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

// src/buffer/mutable.rs
pub struct MutableBuffer {
    buffer: AlignedBuffer,
}

impl MutableBuffer {
    pub fn push<T: Copy>(&mut self, value: T) {
        // Ensure capacity
        // Write value
        // Update length
    }

    pub fn extend_from_slice<T: Copy>(&mut self, values: &[T]) {
        // Bulk append (optimized for large arrays)
    }
}
```

**Deliverables**:
- [ ] `AlignedBuffer` with 64-byte alignment (AVX-512)
- [ ] `MutableBuffer` for building arrays
- [ ] `Buffer` (immutable) for sharing data
- [ ] SIMD-aligned allocation/deallocation
- [ ] Zero-copy slicing
- [ ] Tests: allocation, deallocation, alignment checks
- [ ] Benchmarks: allocation speed vs Vec<u8>

### Phase 2: Schema & Data Types (Week 2)

**Goal**: Define all Arrow data types + scientific extensions

```rust
// src/schema/datatype.rs
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    // Primitive types
    Null,
    Boolean,
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float16, Float32, Float64,

    // Temporal types
    Timestamp(TimeUnit, Option<String>),  // timezone
    Date32, Date64,
    Time32(TimeUnit), Time64(TimeUnit),
    Duration(TimeUnit),
    Interval(IntervalUnit),

    // Variable-length types
    Utf8, LargeUtf8,
    Binary, LargeBinary,

    // Nested types
    List(Box<Field>),
    LargeList(Box<Field>),
    FixedSizeList(Box<Field>, i32),  // size
    Struct(Vec<Field>),
    Map(Box<Field>, bool),  // sorted
    Union(Vec<Field>, UnionMode),

    // Dictionary encoding
    Dictionary(Box<DataType>, Box<DataType>),

    // SCIENTIFIC EXTENSIONS (UNIQUE TO AVILA!)
    Quaternion,        // 4 x f64 (w, x, y, z)
    Complex32,         // 2 x f32 (real, imag)
    Complex64,         // 2 x f64 (real, imag)
    Spinor,            // 2 x Complex64
    Tensor4D,          // 4x4 matrix (spacetime)
    Geodesic,          // (r, θ, φ, t) + metric
}

// src/schema/field.rs
pub struct Field {
    name: String,
    data_type: DataType,
    nullable: bool,
    metadata: HashMap<String, String>,  // Instrument, observatory, etc.
}

// src/schema/schema.rs
pub struct Schema {
    fields: Vec<Field>,
    metadata: HashMap<String, String>,  // Provenance, version, etc.
}
```

**Deliverables**:
- [ ] All standard Arrow data types
- [ ] Scientific extensions (Quaternion, Complex, Spinor, Tensor4D, Geodesic)
- [ ] Field with metadata
- [ ] Schema with provenance
- [ ] Tests: schema creation, equality, metadata
- [ ] Documentation: when to use each type

### Phase 3: Primitive Arrays (Week 3)

**Goal**: Int, Float, Boolean arrays with SIMD operations

```rust
// src/array/primitive.rs
pub struct PrimitiveArray<T: ArrowPrimitiveType> {
    data_type: DataType,
    values: Buffer,      // Raw values
    nulls: Option<Bitmap>,  // Null bitmap
}

impl<T: ArrowPrimitiveType> PrimitiveArray<T> {
    pub fn from_iter<I: IntoIterator<Item = T::Native>>(iter: I) -> Self {
        let values: Vec<T::Native> = iter.collect();
        let buffer = Buffer::from_vec(values);

        Self {
            data_type: T::DATA_TYPE,
            values: buffer,
            nulls: None,
        }
    }

    pub fn value(&self, index: usize) -> T::Native {
        // SAFETY: Assume index is in bounds (checked by caller)
        unsafe { self.values.typed_data::<T::Native>()[index] }
    }

    pub fn values(&self) -> &[T::Native] {
        unsafe { self.values.typed_data::<T::Native>() }
    }
}

// SIMD operations
impl PrimitiveArray<Float64Type> {
    #[cfg(target_feature = "avx2")]
    pub fn simd_add(&self, other: &Self) -> Result<Self> {
        use std::arch::x86_64::*;

        let a = self.values();
        let b = other.values();
        let mut result = Vec::with_capacity(a.len());

        unsafe {
            let chunks = a.len() / 4;  // 4 x f64 per AVX2 register

            for i in 0..chunks {
                let va = _mm256_loadu_pd(a.as_ptr().add(i * 4));
                let vb = _mm256_loadu_pd(b.as_ptr().add(i * 4));
                let vr = _mm256_add_pd(va, vb);

                let mut tmp = [0.0; 4];
                _mm256_storeu_pd(tmp.as_mut_ptr(), vr);
                result.extend_from_slice(&tmp);
            }

            // Handle remainder
            for i in (chunks * 4)..a.len() {
                result.push(a[i] + b[i]);
            }
        }

        Ok(Self::from_vec(result))
    }
}
```

**Deliverables**:
- [ ] PrimitiveArray for all numeric types
- [ ] BooleanArray (bit-packed)
- [ ] Null bitmap support
- [ ] SIMD operations: add, sub, mul, div (AVX2)
- [ ] Scalar operations
- [ ] Tests: creation, access, nulls, SIMD correctness
- [ ] Benchmarks: SIMD vs scalar (target: 4x speedup)

### Phase 4: String & Binary Arrays (Week 4)

**Goal**: Variable-length data (strings, byte arrays)

```rust
// src/array/string.rs
pub struct StringArray {
    offsets: Buffer,    // i32 offsets into data buffer
    data: Buffer,       // UTF-8 bytes
    nulls: Option<Bitmap>,
}

impl StringArray {
    pub fn from_iter<I, S>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut offsets = vec![0i32];
        let mut data = Vec::new();

        for s in iter {
            let bytes = s.as_ref().as_bytes();
            data.extend_from_slice(bytes);
            offsets.push(data.len() as i32);
        }

        Self {
            offsets: Buffer::from_vec(offsets),
            data: Buffer::from_vec(data),
            nulls: None,
        }
    }

    pub fn value(&self, index: usize) -> &str {
        let offsets = unsafe { self.offsets.typed_data::<i32>() };
        let start = offsets[index] as usize;
        let end = offsets[index + 1] as usize;

        let bytes = &self.data.as_slice()[start..end];
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}
```

**Deliverables**:
- [ ] StringArray (UTF-8)
- [ ] LargeStringArray (i64 offsets)
- [ ] BinaryArray (arbitrary bytes)
- [ ] LargeBinaryArray
- [ ] Tests: creation, access, UTF-8 validation
- [ ] Benchmarks: creation speed vs Vec<String>

### Phase 5: Scientific Arrays (Week 5-6)

**Goal**: Quaternion, Complex, Spinor, Tensor4D arrays

```rust
// src/scientific/quaternion.rs
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f64,  // scalar part
    pub x: f64,  // i component
    pub y: f64,  // j component
    pub z: f64,  // k component
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.w, -self.x, -self.y, -self.z)
    }

    pub fn magnitude(&self) -> f64 {
        // Numerically stable computation
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self::new(self.w / mag, self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        // Hamilton product (non-commutative!)
        Self::new(
            self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        )
    }
}

pub struct QuaternionArray {
    buffer: Buffer,  // Interleaved: [w0, x0, y0, z0, w1, x1, y1, z1, ...]
    nulls: Option<Bitmap>,
}

impl QuaternionArray {
    pub fn from_iter<I: IntoIterator<Item = Quaternion>>(iter: I) -> Self {
        let quaternions: Vec<Quaternion> = iter.collect();
        let len = quaternions.len() * 4;  // 4 components per quaternion

        let mut buffer = MutableBuffer::new(len * size_of::<f64>());
        for q in quaternions {
            buffer.push(q.w);
            buffer.push(q.x);
            buffer.push(q.y);
            buffer.push(q.z);
        }

        Self {
            buffer: buffer.into(),
            nulls: None,
        }
    }

    pub fn value(&self, index: usize) -> Quaternion {
        let data = unsafe { self.buffer.typed_data::<f64>() };
        let base = index * 4;

        Quaternion::new(
            data[base],
            data[base + 1],
            data[base + 2],
            data[base + 3],
        )
    }

    #[cfg(target_feature = "avx2")]
    pub fn simd_multiply(&self, other: &Self) -> Result<Self> {
        // SIMD quaternion multiplication (complex but doable!)
        // Process 2 quaternions at a time with AVX2 (8 x f64 = 512 bits)
        unimplemented!("Implement SIMD quaternion multiplication")
    }
}

// src/scientific/complex.rs
#[derive(Debug, Clone, Copy)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

pub struct ComplexArray {
    buffer: Buffer,  // Interleaved: [re0, im0, re1, im1, ...]
    nulls: Option<Bitmap>,
}

// Similar implementation to QuaternionArray...
```

**Deliverables**:
- [ ] Quaternion type with Hamilton product
- [ ] QuaternionArray with SIMD operations
- [ ] Complex32/Complex64 types
- [ ] ComplexArray with FFT support
- [ ] Spinor type (2 x Complex64)
- [ ] SpinorArray
- [ ] Tensor4D type (4x4 spacetime metric)
- [ ] Tensor4DArray
- [ ] Geodesic type (r, θ, φ, t)
- [ ] GeodesicArray
- [ ] Tests: 50+ tests for numerical correctness
- [ ] Benchmarks: vs generic Vec<Quaternion>

### Phase 6: RecordBatch & Compute (Week 7)

**Goal**: Columnar batches and vectorized operations

```rust
// src/lib.rs
pub struct RecordBatch {
    schema: Arc<Schema>,
    columns: Vec<ArrayRef>,
    row_count: usize,
}

impl RecordBatch {
    pub fn try_new(schema: Arc<Schema>, columns: Vec<ArrayRef>) -> Result<Self> {
        // Validate:
        // 1. Column count matches schema
        // 2. Column types match schema
        // 3. All columns have same length

        let row_count = columns.first().map(|c| c.len()).unwrap_or(0);

        Ok(Self {
            schema,
            columns,
            row_count,
        })
    }

    pub fn num_rows(&self) -> usize {
        self.row_count
    }

    pub fn num_columns(&self) -> usize {
        self.columns.len()
    }

    pub fn column(&self, index: usize) -> &ArrayRef {
        &self.columns[index]
    }

    pub fn filter(&self, mask: &BooleanArray) -> Result<Self> {
        let filtered_columns: Result<Vec<_>> = self.columns
            .iter()
            .map(|col| compute::filter(col.as_ref(), mask))
            .collect();

        Self::try_new(self.schema.clone(), filtered_columns?)
    }
}

// src/compute/arithmetic.rs
pub fn add(left: &dyn Array, right: &dyn Array) -> Result<ArrayRef> {
    match (left.data_type(), right.data_type()) {
        (DataType::Float64, DataType::Float64) => {
            let left = left.as_any().downcast_ref::<Float64Array>().unwrap();
            let right = right.as_any().downcast_ref::<Float64Array>().unwrap();

            #[cfg(target_feature = "avx2")]
            return Ok(Arc::new(left.simd_add(right)?));

            #[cfg(not(target_feature = "avx2"))]
            return Ok(Arc::new(left.scalar_add(right)?));
        }
        _ => Err(Error::InvalidOperation),
    }
}
```

**Deliverables**:
- [ ] RecordBatch with schema validation
- [ ] Compute operations: add, sub, mul, div
- [ ] Comparison operations: eq, neq, lt, gt, lte, gte
- [ ] Aggregations: sum, mean, std, min, max
- [ ] Filter with boolean mask
- [ ] Sort by column
- [ ] Tests: 30+ compute tests
- [ ] Benchmarks: vs Apache Arrow

### Phase 7: Arrow IPC Format (Week 8)

**Goal**: Read/write .arrow files for interop

```rust
// src/ipc/writer.rs
pub struct FileWriter<W: Write> {
    writer: W,
    schema: Arc<Schema>,
    written_batches: usize,
}

impl<W: Write> FileWriter<W> {
    pub fn try_new(mut writer: W, schema: &Schema) -> Result<Self> {
        // Write Arrow file header
        // Write schema as Flatbuffers

        Ok(Self {
            writer,
            schema: Arc::new(schema.clone()),
            written_batches: 0,
        })
    }

    pub fn write(&mut self, batch: &RecordBatch) -> Result<()> {
        // Validate batch schema matches
        // Write batch as Flatbuffers + raw data
        // Update footer

        self.written_batches += 1;
        Ok(())
    }

    pub fn finish(mut self) -> Result<()> {
        // Write file footer
        Ok(())
    }
}

// src/ipc/reader.rs
pub struct FileReader<R: Read + Seek> {
    reader: R,
    schema: Arc<Schema>,
    batches: Vec<Block>,  // Metadata about each batch
}

impl<R: Read + Seek> FileReader<R> {
    pub fn try_new(mut reader: R) -> Result<Self> {
        // Read Arrow file header
        // Parse schema from Flatbuffers
        // Read footer for batch locations

        Ok(Self {
            reader,
            schema,
            batches,
        })
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn read_batch(&mut self, index: usize) -> Result<RecordBatch> {
        // Seek to batch location
        // Parse Flatbuffers message
        // Read raw data buffers
        // Reconstruct RecordBatch

        unimplemented!()
    }
}
```

**Deliverables**:
- [ ] FileWriter for .arrow files
- [ ] FileReader for .arrow files
- [ ] StreamWriter/StreamReader for IPC streams
- [ ] Flatbuffers schema encoding/decoding
- [ ] Tests: write and read round-trip
- [ ] Compatibility tests with Apache Arrow files

### Phase 8: AvilaDB Integration (Week 9-10)

**Goal**: Zero-copy data transfer with AvilaDB

```rust
// src/aviladb/batch.rs
pub trait AvilaDBExt {
    async fn write_batch(&self, table: &str, batch: &RecordBatch) -> Result<()>;
    async fn read_batches(&self, table: &str, filter: Option<&str>) -> Result<Vec<RecordBatch>>;
}

impl AvilaDBExt for AvilaDB {
    async fn write_batch(&self, table: &str, batch: &RecordBatch) -> Result<()> {
        // Convert RecordBatch to AvilaDB format
        // Use avila-compress for compression
        // Write to storage

        unimplemented!()
    }

    async fn read_batches(&self, table: &str, filter: Option<&str>) -> Result<Vec<RecordBatch>> {
        // Query AvilaDB
        // Decompress with avila-compress
        // Zero-copy conversion to RecordBatch

        unimplemented!()
    }
}

// src/aviladb/pushdown.rs
pub struct QueryBuilder {
    table: String,
    filters: Vec<String>,
    projections: Vec<String>,
}

impl QueryBuilder {
    pub fn filter(mut self, expr: &str) -> Self {
        self.filters.push(expr.to_string());
        self
    }

    pub fn select(mut self, columns: &[&str]) -> Self {
        self.projections.extend(columns.iter().map(|s| s.to_string()));
        self
    }

    pub async fn execute(&self, db: &AvilaDB) -> Result<Vec<RecordBatch>> {
        // Build AvilaDB query with pushdown filters
        // Execute in database (avoid loading unnecessary data)
        // Return batches

        unimplemented!()
    }
}
```

**Deliverables**:
- [ ] Write RecordBatch to AvilaDB
- [ ] Read RecordBatch from AvilaDB
- [ ] Query pushdown (filter in database)
- [ ] Zero-copy data paths where possible
- [ ] Compression with avila-compress
- [ ] Tests: write/read round-trip
- [ ] Example: LIGO data pipeline

---

## 📊 Performance Targets

### Array Operations
- **SIMD speedup**: 4x for AVX2, 8x for AVX-512 (vs scalar)
- **Memory efficiency**: < 10% overhead vs raw Vec<T>
- **Cache efficiency**: SIMD-aligned allocations

### RecordBatch Creation
- **Target**: < 1ms for 1M rows (primitive types)
- **Comparison**: Apache Arrow ~1.5ms
- **Acceptable**: < 2ms

### Filtering
- **Target**: 500 MB/s (SIMD-accelerated)
- **Comparison**: Apache Arrow ~400 MB/s
- **Acceptable**: 300 MB/s

### Scientific Types
- **Quaternion multiply**: < 10ns per operation (SIMD)
- **Complex FFT**: Competitive with FFTW
- **Tensor4D operations**: < 50ns per element

### AvilaDB Integration
- **Zero-copy reads**: 95%+ of cases
- **Write throughput**: 1 GB/s
- **Query latency**: < 10ms for 1M rows

---

## 🧪 Testing Requirements

### Unit Tests (100+)
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_array_creation() { ... }

    #[test]
    fn test_simd_add_correctness() { ... }

    #[test]
    fn test_quaternion_multiplication() { ... }

    #[test]
    fn test_string_array_utf8() { ... }
}
```

### Integration Tests (20+)
```rust
#[tokio::test]
async fn test_aviladb_round_trip() {
    let db = AvilaDB::connect_test().await;
    let batch = create_test_batch();

    db.write_batch("test_table", &batch).await.unwrap();
    let read = db.read_batches("test_table", None).await.unwrap();

    assert_eq!(batch, read[0]);
}
```

### Scientific Correctness Tests (30+)
```rust
#[test]
fn test_quaternion_rotation() {
    // Test quaternion represents rotation correctly
    let q = Quaternion::from_axis_angle(Vector3::z_axis(), PI / 2.0);
    let p = Vector3::new(1.0, 0.0, 0.0);
    let rotated = q.rotate_vector(&p);

    assert_float_eq!(rotated.x, 0.0, 1e-10);
    assert_float_eq!(rotated.y, 1.0, 1e-10);
    assert_float_eq!(rotated.z, 0.0, 1e-10);
}

#[test]
fn test_complex_fft_parseval() {
    // Parseval's theorem: energy in time domain = energy in frequency domain
    let signal = ComplexArray::from_iter(...);
    let fft = fft(&signal).unwrap();

    let time_energy: f64 = signal.iter().map(|c| c.norm_sqr()).sum();
    let freq_energy: f64 = fft.iter().map(|c| c.norm_sqr()).sum() / signal.len() as f64;

    assert_float_eq!(time_energy, freq_energy, 1e-8);
}
```

### Benchmarks (10+)
```rust
#[bench]
fn bench_simd_add(b: &mut Bencher) {
    let a = Float64Array::from_iter(0..1000);
    let b = Float64Array::from_iter(0..1000);

    b.iter(|| a.simd_add(&b).unwrap());
}

#[bench]
fn bench_quaternion_multiply(b: &mut Bencher) {
    let qa = QuaternionArray::from_iter(...);
    let qb = QuaternionArray::from_iter(...);

    b.iter(|| qa.simd_multiply(&qb).unwrap());
}
```

---

## 📖 Documentation Requirements

Every public item needs:
1. **Summary**: One-line description
2. **Mathematical definition** (for scientific types)
3. **Example**: Working code snippet
4. **Performance notes**: SIMD, cache behavior
5. **References**: Papers, specs, standards

```rust
/// Quaternion: 4-dimensional number for representing 3D rotations.
///
/// Quaternions are an extension of complex numbers to 4 dimensions,
/// used extensively in computer graphics, robotics, and spacecraft
/// attitude control. They avoid gimbal lock and interpolate smoothly.
///
/// # Mathematical Definition
///
/// q = w + xi + yj + zk
///
/// where i² = j² = k² = ijk = -1
///
/// # Examples
///
/// ```
/// use avila_arrow::scientific::Quaternion;
/// use std::f64::consts::PI;
///
/// // 90-degree rotation around Z-axis
/// let q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);
///
/// // Rotate vector
/// let v = [1.0, 0.0, 0.0];
/// let rotated = q.rotate_vector(v);
/// // rotated ≈ [0.0, 1.0, 0.0]
/// ```
///
/// # Performance
///
/// - Multiplication: ~10ns with AVX2
/// - Normalization: ~5ns with SIMD
/// - Memory: 32 bytes (4 x f64)
///
/// # References
///
/// - Hamilton, W. R. (1843). "On Quaternions"
/// - Shoemake, K. (1985). "Animating Rotation with Quaternion Curves"
/// - NASA Technical Memorandum 78696 (1977)
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

---

## 🎯 Success Criteria

Before considering this module "done":

### Functionality
- [ ] All Arrow primitive types work
- [ ] String and binary arrays work
- [ ] Nested types (List, Struct) work
- [ ] Scientific types (Quaternion, Complex, Spinor, Tensor4D) work
- [ ] SIMD operations are correct and fast
- [ ] Arrow IPC read/write compatible
- [ ] AvilaDB integration works
- [ ] Zero-copy paths verified

### Quality
- [ ] 100% of public APIs documented
- [ ] 100+ tests passing
- [ ] 10+ benchmarks comparing to Apache Arrow
- [ ] 5+ examples demonstrating usage
- [ ] Scientific correctness validated (physics, math)
- [ ] Zero `unsafe` blocks (except in SIMD intrinsics)
- [ ] All errors handled with `Result<T, Error>`

### Performance
- [ ] SIMD speedup: 4x+ vs scalar
- [ ] Batch creation: < 2ms for 1M rows
- [ ] Filtering: 300+ MB/s
- [ ] Quaternion ops: < 10ns each
- [ ] AvilaDB writes: 1+ GB/s

### Integration
- [ ] Works with avila-compress
- [ ] Works with avila-telemetry
- [ ] Works with avila-math
- [ ] Works with AvilaDB
- [ ] Examples demonstrate LIGO/LISA pipeline

---

## 🚀 Next Steps

1. **Read this document COMPLETELY**
2. **Study Arrow specification**: https://arrow.apache.org/docs/format/Columnar.html
3. **Study SIMD**: Intel Intrinsics Guide: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/
4. **Study quaternion math**: Shoemake 1985 paper
5. **Start with Phase 1**: Memory & buffer management
6. **Test incrementally**: Verify each piece before moving on
7. **Benchmark continuously**: Compare to Apache Arrow
8. **Document as you go**: Write docs BEFORE implementation
9. **Ask questions**: If stuck, ask for clarification
10. **NEVER give up**: Implement completely, no placeholders!

---

## 💬 Remember

> "Avila Arrow is NOT Apache Arrow. It's a scientific columnar format with Quaternions, Complex numbers, Spinors, and Tensor4D types. It's SIMD-accelerated, AvilaDB-native, and designed for gravitational wave detection, astrophysics, and Brazilian research infrastructure."

> "Never use placeholder code. Implement EVERY feature completely. Test EVERY operation. Benchmark EVERY optimization. Document EVERY API. This library will process data from $10 billion space missions. It must be perfect."

> "When you implement Quaternion multiplication, it must be mathematically correct AND SIMD-accelerated. When you implement FFT, it must match FFTW's accuracy AND beat its performance. When you integrate with AvilaDB, it must be zero-copy. No compromises."

**Now go build the best scientific columnar format in the Rust ecosystem! 🚀🔬🇧🇷**
