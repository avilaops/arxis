# Avila DataFrame - Copilot Instructions

**Projeto**: avila-dataframe
**Descrição**: DataFrame 100% Rust Nativo - Zero Overhead, Máxima Performance
**Status**: v0.2.0 - Production Ready
**Filosofia**: Essência antes de escala. Profundidade antes de velocidade. Zero deps científicas externas.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Zero Dependências Científicas Externas
```toml
# ✅ PERMITIDO
serde = "1"          # Serialization
rayon = "1.10"       # Paralelização
num-traits = "0.2"   # Traits matemáticos básicos
chrono = "0.4"       # Datas
csv = "1.3"          # I/O

# ❌ PROIBIDO
polars = "..."       # Implementar do zero!
arrow = "..."        # Nosso próprio formato
ndarray = "..."      # Própria álgebra linear
numpy = "..."        # Somos Rust puro!
pandas = "..."       # Replacement deles!
```

**Motivo**: Controle total da implementação, otimizações específicas para AVL Platform, zero bloat.

### 2. Implementar FFT do Zero (Cooley-Tukey)
```rust
// ✅ CORRETO: FFT próprio
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl Complex<f64> {
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }
}

pub fn fft_radix2(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    assert!(n.is_power_of_two(), "FFT requires power of 2 length");

    if n == 1 {
        return input.to_vec();
    }

    // Cooley-Tukey DIT (Decimation in Time)
    let even: Vec<_> = input.iter().step_by(2).copied().collect();
    let odd: Vec<_> = input.iter().skip(1).step_by(2).copied().collect();

    let fft_even = fft_radix2(&even);
    let fft_odd = fft_radix2(&odd);

    let mut result = vec![Complex::default(); n];
    for k in 0..(n/2) {
        let t = Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64) * fft_odd[k];
        result[k] = fft_even[k] + t;
        result[k + n/2] = fft_even[k] - t;
    }

    result
}

// ❌ ERRADO: Usar rustfft
use rustfft::FftPlanner; // PROIBIDO!
```

### 3. Formato Columnar Nativo (Inspirado em Arrow, mas próprio)
```rust
// ✅ CORRETO: Columnar storage próprio
pub enum ColumnData {
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    String(Vec<String>),
    DateTime(Vec<i64>),          // Unix timestamp
    Complex(Vec<Complex<f64>>),  // Para FFT
    SpaceTimeEvent(Vec<SpaceTimeEvent>), // Física

    // Nested types
    List(Vec<Vec<Value>>),
    Struct(Vec<HashMap<String, Value>>),
}

pub struct Column {
    name: String,
    data: ColumnData,
    null_bitmap: Option<Vec<bool>>, // Nullability
}
```

**Vantagens**:
- Cache-friendly (SIMD vectorization)
- Compression natural (run-length encoding)
- Zero-copy slicing

### 4. Performance > Polars/Arrow
```rust
// Target: 2x-5x mais rápido que Polars para operações em PT-BR
#[bench]
fn bench_groupby_sum_1m_rows(b: &mut Bencher) {
    let df = DataFrame::from_rows(generate_rows_1m()).unwrap();

    b.iter(|| {
        black_box(df.groupby(&["country"])
            .agg(&[("sales", AggFunc::Sum)])
            .unwrap())
    });
}

// Baseline: Polars = ~80ms
// Target: AvilaDF = ~20-40ms
```

**Otimizações obrigatórias**:
- Rayon para paralelização automática
- SIMD para operações vetoriais (AVX2/AVX-512)
- Zero-copy views onde possível
- Memory-mapped files para datasets grandes
- LRU cache para queries repetidas

---

## 📐 Arquitetura do Projeto

```
avila-dataframe/
├── src/
│   ├── lib.rs                 # Public API
│   ├── dataframe.rs           # Main DataFrame struct
│   ├── series.rs              # Column/Series
│   ├── row.rs                 # Row iterator
│   ├── types/
│   │   ├── mod.rs
│   │   ├── value.rs           # Value enum
│   │   ├── dtype.rs           # DataType enum
│   │   ├── complex.rs         # Complex<T> próprio
│   │   └── spacetime.rs       # SpaceTimeEvent para física
│   ├── ops/
│   │   ├── mod.rs
│   │   ├── select.rs          # Projection
│   │   ├── filter.rs          # WHERE clauses
│   │   ├── groupby.rs         # GROUP BY + agg
│   │   ├── join.rs            # INNER/LEFT/RIGHT/FULL
│   │   ├── sort.rs            # ORDER BY
│   │   ├── window.rs          # Window functions
│   │   └── pivot.rs           # Pivot tables
│   ├── agg/
│   │   ├── mod.rs
│   │   ├── sum.rs             # SUM
│   │   ├── mean.rs            # AVG
│   │   ├── std.rs             # STDDEV
│   │   ├── quantile.rs        # PERCENTILE
│   │   └── custom.rs          # User-defined agg
│   ├── scientific/
│   │   ├── mod.rs
│   │   ├── complex.rs         # Complex<T> impl
│   │   ├── fft_pure.rs        # FFT Cooley-Tukey
│   │   ├── spectrogram.rs     # STFT do zero
│   │   ├── windows.rs         # Hann, Hamming, etc.
│   │   ├── correlation.rs     # Cross-correlation
│   │   └── convolution.rs     # Convolution via FFT
│   ├── io/
│   │   ├── mod.rs
│   │   ├── csv.rs             # CSV reader/writer
│   │   ├── json.rs            # JSON (NDJSON)
│   │   ├── parquet.rs         # Parquet own impl
│   │   ├── aviladb.rs         # AvilaDB integration
│   │   └── mmap.rs            # Memory-mapped files
│   ├── expr/
│   │   ├── mod.rs
│   │   ├── ast.rs             # Expression AST
│   │   ├── eval.rs            # Evaluator
│   │   └── functions.rs       # Built-in functions
│   ├── lazy/
│   │   ├── mod.rs
│   │   ├── logical.rs         # Logical plan
│   │   ├── physical.rs        # Physical plan
│   │   └── optimizer.rs       # Query optimization
│   └── utils/
│       ├── mod.rs
│       ├── simd.rs            # SIMD helpers
│       ├── parallel.rs        # Rayon utils
│       └── memory.rs          # Memory pool
├── examples/
│   ├── quickstart_native.rs
│   ├── fft_native.rs
│   ├── groupby_agg.rs
│   ├── joins.rs
│   └── aviladb_integration.rs
├── benches/
│   ├── dataframe_bench.rs
│   ├── fft_bench.rs
│   └── vs_polars.rs
└── tests/
    ├── dataframe_tests.rs
    ├── fft_tests.rs
    ├── groupby_tests.rs
    └── join_tests.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: Core DataFrame (v0.2.0) ✅ COMPLETO
```rust
// ✅ Implementado
pub struct DataFrame {
    columns: Vec<Column>,
    height: usize,
}

impl DataFrame {
    pub fn new(columns: Vec<Column>) -> Result<Self>;
    pub fn height(&self) -> usize;
    pub fn width(&self) -> usize;
    pub fn schema(&self) -> Schema;

    pub fn head(&self, n: usize) -> Self;
    pub fn tail(&self, n: usize) -> Self;
    pub fn slice(&self, offset: usize, length: usize) -> Self;

    pub fn select(&self, columns: &[&str]) -> Result<Self>;
    pub fn filter(&self, predicate: Expr) -> Result<Self>;
    pub fn sort(&self, by: &[&str], descending: bool) -> Result<Self>;

    pub fn column(&self, name: &str) -> Result<&Series>;
    pub fn columns(&self) -> &[Column];
}

pub struct Series {
    name: String,
    data: ColumnData,
}

impl Series {
    pub fn len(&self) -> usize;
    pub fn dtype(&self) -> DataType;
    pub fn get(&self, index: usize) -> Option<Value>;

    pub fn sum(&self) -> Result<Value>;
    pub fn mean(&self) -> Result<f64>;
    pub fn std(&self) -> Result<f64>;
    pub fn min(&self) -> Result<Value>;
    pub fn max(&self) -> Result<Value>;
}
```

### Fase 2: FFT 100% Próprio (v0.3.0) - Semanas 1-3
```rust
// TODO: Implementar números complexos próprios
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn conj(&self) -> Self {
        Self::new(self.re, -self.im)
    }

    pub fn magnitude(&self) -> f64
    where
        T: Into<f64>,
    {
        let re: f64 = self.re.into();
        let im: f64 = self.im.into();
        (re * re + im * im).sqrt()
    }

    pub fn phase(&self) -> f64
    where
        T: Into<f64>,
    {
        let re: f64 = self.re.into();
        let im: f64 = self.im.into();
        im.atan2(re)
    }
}

// Arithmetic operators
impl Add for Complex<f64> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Mul for Complex<f64> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

// TODO: FFT Cooley-Tukey radix-2
pub fn fft_radix2(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();

    // Base case
    if n == 1 {
        return input.to_vec();
    }

    // Must be power of 2
    assert!(n.is_power_of_two());

    // Split into even and odd indices
    let even: Vec<_> = input.iter().step_by(2).copied().collect();
    let odd: Vec<_> = input.iter().skip(1).step_by(2).copied().collect();

    // Recursive calls
    let fft_even = fft_radix2(&even);
    let fft_odd = fft_radix2(&odd);

    // Combine results
    let mut result = vec![Complex::default(); n];
    for k in 0..(n/2) {
        // Twiddle factor: W_N^k = e^(-2πik/N)
        let angle = -2.0 * std::f64::consts::PI * k as f64 / n as f64;
        let twiddle = Complex::from_polar(1.0, angle);

        let t = twiddle * fft_odd[k];
        result[k] = fft_even[k] + t;
        result[k + n/2] = fft_even[k] - t;
    }

    result
}

// TODO: Versão iterativa (mais rápida)
pub fn fft_iterative(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    assert!(n.is_power_of_two());

    // Bit-reversal permutation
    let mut result = bit_reverse_copy(input);

    // Cooley-Tukey butterfly
    let mut size = 2;
    while size <= n {
        let half_size = size / 2;
        let angle = -2.0 * PI / size as f64;

        for i in (0..n).step_by(size) {
            for j in 0..half_size {
                let k = i + j;
                let twiddle = Complex::from_polar(1.0, angle * j as f64);
                let t = twiddle * result[k + half_size];

                result[k + half_size] = result[k] - t;
                result[k] = result[k] + t;
            }
        }

        size *= 2;
    }

    result
}

fn bit_reverse_copy(data: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = data.len();
    let log_n = (n as f64).log2() as usize;

    let mut result = vec![Complex::default(); n];
    for i in 0..n {
        let reversed = reverse_bits(i, log_n);
        result[reversed] = data[i];
    }
    result
}

fn reverse_bits(mut x: usize, num_bits: usize) -> usize {
    let mut result = 0;
    for _ in 0..num_bits {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}

// TODO: IFFT (inverse FFT)
pub fn ifft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();

    // Conjugate input
    let conj_input: Vec<_> = input.iter().map(|c| c.conj()).collect();

    // FFT
    let fft_result = fft_iterative(&conj_input);

    // Conjugate and normalize
    fft_result.into_iter()
        .map(|c| Complex::new(c.re / n as f64, -c.im / n as f64))
        .collect()
}

// TODO: Real FFT (otimizado)
pub fn rfft(signal: &[f64]) -> Vec<Complex<f64>> {
    let complex_input: Vec<_> = signal.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    let fft_result = fft_iterative(&complex_input);

    // Return only positive frequencies (N/2 + 1)
    fft_result[0..=(signal.len()/2)].to_vec()
}
```

### Fase 3: GroupBy & Aggregations (v0.4.0) - Semanas 4-6
```rust
// TODO: GroupBy implementation
pub struct GroupBy<'a> {
    df: &'a DataFrame,
    by: Vec<String>,
    groups: HashMap<Vec<Value>, Vec<usize>>, // group_key -> row indices
}

impl<'a> GroupBy<'a> {
    pub fn new(df: &'a DataFrame, by: &[&str]) -> Result<Self> {
        // 1. Create group keys for each row
        // 2. Hash into HashMap<key, indices>
        // 3. Parallel with rayon if >100K rows
    }

    pub fn agg(&self, aggs: &[(&str, AggFunc)]) -> Result<DataFrame> {
        // 1. For each group:
        //    - Apply each aggregation
        // 2. Construct result DataFrame
        // 3. Parallel across groups with rayon
    }

    pub fn sum(&self, columns: &[&str]) -> Result<DataFrame> {
        self.agg(&columns.iter().map(|c| (*c, AggFunc::Sum)).collect::<Vec<_>>())
    }

    pub fn mean(&self, columns: &[&str]) -> Result<DataFrame>;
    pub fn count(&self) -> Result<DataFrame>;
    pub fn first(&self, columns: &[&str]) -> Result<DataFrame>;
    pub fn last(&self, columns: &[&str]) -> Result<DataFrame>;
}

pub enum AggFunc {
    Sum,
    Mean,
    Std,
    Var,
    Min,
    Max,
    Count,
    CountDistinct,
    First,
    Last,
    Median,
    Quantile(f64),
    Custom(Box<dyn Fn(&Series) -> Value>),
}

// Example usage
let df = DataFrame::from_csv("sales.csv")?;
let result = df.groupby(&["country", "product"])
    .agg(&[
        ("sales", AggFunc::Sum),
        ("quantity", AggFunc::Mean),
        ("customer_id", AggFunc::CountDistinct),
    ])?;
```

### Fase 4: Joins (v0.5.0) - Semanas 7-9
```rust
// TODO: Hash join implementation
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

impl DataFrame {
    pub fn join(
        &self,
        other: &DataFrame,
        left_on: &[&str],
        right_on: &[&str],
        how: JoinType,
    ) -> Result<Self> {
        match how {
            JoinType::Inner => self.inner_join(other, left_on, right_on),
            JoinType::Left => self.left_join(other, left_on, right_on),
            JoinType::Right => other.left_join(self, right_on, left_on),
            JoinType::Full => self.full_join(other, left_on, right_on),
            JoinType::Cross => self.cross_join(other),
        }
    }

    fn inner_join(&self, other: &DataFrame, left_on: &[&str], right_on: &[&str])
        -> Result<Self> {
        // 1. Build hash table on smaller DataFrame
        // 2. Probe with larger DataFrame
        // 3. Parallel with rayon

        let (build_df, probe_df, build_on, probe_on) = if self.height() < other.height() {
            (self, other, left_on, right_on)
        } else {
            (other, self, right_on, left_on)
        };

        // Build phase
        let mut hash_table: HashMap<Vec<Value>, Vec<usize>> = HashMap::new();
        for (i, row) in build_df.iter_rows().enumerate() {
            let key: Vec<Value> = build_on.iter()
                .map(|col| row.get(col).unwrap())
                .collect();
            hash_table.entry(key).or_default().push(i);
        }

        // Probe phase (parallel)
        use rayon::prelude::*;
        let matches: Vec<_> = probe_df.iter_rows()
            .enumerate()
            .par_bridge()
            .flat_map(|(j, row)| {
                let key: Vec<Value> = probe_on.iter()
                    .map(|col| row.get(col).unwrap())
                    .collect();

                hash_table.get(&key)
                    .map(|indices| {
                        indices.iter().map(move |&i| (i, j))
                    })
                    .into_iter()
                    .flatten()
            })
            .collect();

        // Construct result
        // ...
    }
}
```

### Fase 5: Lazy Evaluation & Query Optimization (v0.6.0) - Semanas 10-12
```rust
// TODO: Lazy API
pub struct LazyFrame {
    logical_plan: LogicalPlan,
}

pub enum LogicalPlan {
    Scan { path: String, file_type: FileType },
    Select { input: Box<LogicalPlan>, columns: Vec<String> },
    Filter { input: Box<LogicalPlan>, predicate: Expr },
    GroupBy { input: Box<LogicalPlan>, by: Vec<String>, aggs: Vec<Agg> },
    Join { left: Box<LogicalPlan>, right: Box<LogicalPlan>, on: JoinKeys, how: JoinType },
}

impl LazyFrame {
    pub fn scan_csv(path: &str) -> Self;
    pub fn scan_parquet(path: &str) -> Self;
    pub fn scan_aviladb(query: &str) -> Self;

    pub fn select(&self, columns: &[&str]) -> Self;
    pub fn filter(&self, predicate: Expr) -> Self;
    pub fn groupby(&self, by: &[&str]) -> LazyGroupBy;

    pub fn collect(&self) -> Result<DataFrame> {
        // 1. Optimize logical plan
        // 2. Convert to physical plan
        // 3. Execute
        let optimized = self.optimize()?;
        let physical = optimized.to_physical()?;
        physical.execute()
    }

    fn optimize(&self) -> Result<LogicalPlan> {
        // Projection pushdown
        // Filter pushdown
        // Predicate reordering
        // Join reordering
        // Constant folding
    }
}
```

---

## 🧪 Testes Obrigatórios

### 1. FFT Correctness Tests
```rust
#[test]
fn test_fft_identity() {
    let signal: Vec<f64> = (0..128).map(|i| (i as f64 * 0.1).sin()).collect();

    let complex_signal: Vec<_> = signal.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    let fft_result = fft_iterative(&complex_signal);
    let recovered = ifft(&fft_result);

    for (original, recovered) in signal.iter().zip(recovered.iter()) {
        assert!((original - recovered.re).abs() < 1e-10);
        assert!(recovered.im.abs() < 1e-10);
    }
}

#[test]
fn test_fft_parseval() {
    // Parseval's theorem: energy conservation
    let signal: Vec<f64> = (0..256).map(|i| (i as f64 * 0.05).sin()).collect();

    let time_domain_energy: f64 = signal.iter().map(|x| x * x).sum();

    let spectrum = rfft(&signal);
    let freq_domain_energy: f64 = spectrum.iter()
        .map(|c| c.magnitude() * c.magnitude())
        .sum::<f64>() / signal.len() as f64;

    assert!((time_domain_energy - freq_domain_energy).abs() / time_domain_energy < 0.01);
}
```

### 2. GroupBy Performance Tests
```rust
#[bench]
fn bench_groupby_1m_rows(b: &mut Bencher) {
    let df = generate_sales_data(1_000_000);

    b.iter(|| {
        black_box(df.groupby(&["country", "product"])
            .agg(&[
                ("sales", AggFunc::Sum),
                ("quantity", AggFunc::Mean),
            ])
            .unwrap())
    });
}

// Target: <100ms for 1M rows, 10K groups
```

### 3. Join Tests
```rust
#[test]
fn test_inner_join_correctness() {
    let df1 = DataFrame::from_rows(vec![
        row!("id" => 1, "name" => "Alice"),
        row!("id" => 2, "name" => "Bob"),
        row!("id" => 3, "name" => "Charlie"),
    ]).unwrap();

    let df2 = DataFrame::from_rows(vec![
        row!("user_id" => 1, "city" => "NYC"),
        row!("user_id" => 2, "city" => "LA"),
        row!("user_id" => 4, "city" => "Chicago"),
    ]).unwrap();

    let result = df1.join(&df2, &["id"], &["user_id"], JoinType::Inner).unwrap();

    assert_eq!(result.height(), 2);
    assert_eq!(result.column("name").unwrap().get(0), Some(Value::String("Alice".into())));
    assert_eq!(result.column("city").unwrap().get(0), Some(Value::String("NYC".into())));
}
```

---

## 📊 API Pública Completa

### Main DataFrame API
```rust
pub struct DataFrame {
    columns: Vec<Column>,
    height: usize,
}

impl DataFrame {
    // Constructors
    pub fn new(columns: Vec<Column>) -> Result<Self>;
    pub fn from_rows(rows: Vec<Row>) -> Result<Self>;
    pub fn from_csv(path: &str) -> Result<Self>;
    pub fn from_json(path: &str) -> Result<Self>;
    pub fn from_aviladb(query: &str) -> Result<Self>;

    // Shape
    pub fn height(&self) -> usize;
    pub fn width(&self) -> usize;
    pub fn shape(&self) -> (usize, usize);
    pub fn is_empty(&self) -> bool;

    // Selection
    pub fn select(&self, columns: &[&str]) -> Result<Self>;
    pub fn column(&self, name: &str) -> Result<&Series>;
    pub fn columns(&self) -> &[Column];
    pub fn head(&self, n: usize) -> Self;
    pub fn tail(&self, n: usize) -> Self;
    pub fn slice(&self, offset: usize, length: usize) -> Self;

    // Filtering
    pub fn filter(&self, predicate: Expr) -> Result<Self>;
    pub fn filter_fn<F>(&self, f: F) -> Result<Self>
    where
        F: Fn(&Row) -> bool;

    // Sorting
    pub fn sort(&self, by: &[&str], descending: bool) -> Result<Self>;
    pub fn sort_by<F>(&self, f: F) -> Result<Self>
    where
        F: Fn(&Row, &Row) -> Ordering;

    // Aggregation
    pub fn groupby(&self, by: &[&str]) -> GroupBy;

    // Joins
    pub fn join(&self, other: &DataFrame, left_on: &[&str], right_on: &[&str], how: JoinType) -> Result<Self>;

    // I/O
    pub fn to_csv(&self, path: &str) -> Result<()>;
    pub fn to_json(&self, path: &str) -> Result<()>;
    pub fn to_aviladb(&self, collection: &str) -> Result<()>;

    // Iteration
    pub fn iter_rows(&self) -> RowIterator;
    pub fn iter_columns(&self) -> ColumnIterator;
}

pub struct Series {
    name: String,
    data: ColumnData,
}

impl Series {
    // Stats
    pub fn sum(&self) -> Result<Value>;
    pub fn mean(&self) -> Result<f64>;
    pub fn std(&self) -> Result<f64>;
    pub fn var(&self) -> Result<f64>;
    pub fn min(&self) -> Result<Value>;
    pub fn max(&self) -> Result<Value>;
    pub fn median(&self) -> Result<Value>;
    pub fn quantile(&self, q: f64) -> Result<Value>;

    // Scientific
    pub fn fft(&self) -> Result<Vec<Complex<f64>>>;
    pub fn ifft(&self, spectrum: &[Complex<f64>]) -> Result<Self>;
    pub fn power_spectrum(&self, sample_rate: f64) -> Result<Self>;
    pub fn spectrogram(&self, window_size: usize, hop_size: usize) -> Result<Spectrogram>;
}
```

---

## ⚠️ Erros Comuns a Evitar

### 1. Columnar vs Row-based
```rust
// ❌ ERRADO: Row-based iteration (cache misses)
for row in df.iter_rows() {
    sum += row.get("value").unwrap();
}

// ✅ CORRETO: Columnar iteration (vectorized)
let column = df.column("value")?;
let sum: f64 = column.as_f64()?.iter().sum();
```

### 2. Allocation em Loops
```rust
// ❌ ERRADO: Allocate em cada iteração
for group in groups {
    let result = Vec::new();  // Allocate!
    // ...
}

// ✅ CORRETO: Pre-allocate
let mut result = Vec::with_capacity(groups.len());
for group in groups {
    // Reuse result
}
```

### 3. Join Order
```rust
// ❌ ERRADO: Join large tables first
let result = large_df1.join(&large_df2, ...)
    .join(&small_df, ...);

// ✅ CORRETO: Join small tables first, filter early
let result = large_df1.filter(...)  // Reduce size first
    .join(&small_df, ...)
    .join(&large_df2, ...);
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR:

- [ ] **Zero Deps**: Nenhuma dep científica externa (polars, arrow, ndarray)
- [ ] **FFT Próprio**: Cooley-Tukey implementado e testado
- [ ] **Performance**: ≥2x Polars em operações comuns
- [ ] **Parallel**: Rayon integrado para >100K rows
- [ ] **SIMD**: AVX2/AVX-512 onde aplicável
- [ ] **Docs**: Cada função pública documentada
- [ ] **Tests**: Unit tests + integration tests
- [ ] **Benchmarks**: vs Polars/Arrow
- [ ] **Coverage**: ≥80% code coverage
- [ ] **AvilaDB**: Integração funcional

---

## 🚀 Como Começar

### Setup
```bash
cd arxis/avila-dataframe
cargo build
cargo test
cargo bench
```

### Exemplos
```bash
# Quickstart
cargo run --example quickstart_native

# FFT
cargo run --example fft_native --features scientific

# GroupBy
cargo run --example groupby_agg

# AvilaDB
cargo run --example aviladb_integration --features aviladb
```

### Benchmarks
```bash
# Benchmark internal
cargo bench

# vs Polars
cargo bench --bench vs_polars
```

---

**Lembre-se**: Essência antes de escala. Profundidade antes de velocidade. ZERO dependências científicas externas.

**Avila DataFrame** - DataFrame 100% Rust Nativo 🚀
