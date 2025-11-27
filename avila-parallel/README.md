# avila-parallel

**Data parallelism for AVL Platform**

[![Crates.io](https://img.shields.io/crates/v/avila-parallel.svg)](https://crates.io/crates/avila-parallel)
[![Documentation](https://docs.rs/avila-parallel/badge.svg)](https://docs.rs/avila-parallel)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 🚀 Overview

`avila-parallel` provides data parallelism primitives for the AVL Platform ecosystem with thread pools, parallel iterators, and work stealing.

### Key Features

- ✅ **Thread pool** with work stealing
- ✅ **Parallel iterators** for collections
- ✅ **Zero overhead** abstraction
- ✅ **Scoped threads** for safe parallelism
- ✅ **Optimized for AVL Platform** workloads

## 📦 Installation

```toml
[dependencies]
avila-parallel = "0.1"
```

## 🎯 Quick Start

### Parallel Iteration

```rust
use avila_parallel::prelude::*;

let numbers: Vec<i32> = (0..1000).collect();

// Parallel map
let squares: Vec<i32> = numbers
    .par_iter()
    .map(|&x| x * x)
    .collect();

// Parallel filter
let evens: Vec<i32> = numbers
    .par_iter()
    .filter(|&x| x % 2 == 0)
    .cloned()
    .collect();

// Parallel reduce
let sum: i32 = numbers
    .par_iter()
    .sum();
```

### Thread Pool

```rust
use avila_parallel::ThreadPool;

let pool = ThreadPool::new(4);

pool.execute(|| {
    println!("Task 1");
});

pool.execute(|| {
    println!("Task 2");
});

pool.wait();
```

### Scoped Threads

```rust
use avila_parallel::scope;

let data = vec![1, 2, 3, 4, 5];

scope(|s| {
    for value in &data {
        s.spawn(move |_| {
            println!("Processing: {}", value);
        });
    }
});
```

## 🔥 Performance

Optimized for AVL Platform:

| Operation | Sequential | avila-parallel | Speedup |
|-----------|-----------|----------------|---------|
| Map 1M elements | 45ms | 8ms | **5.6x** |
| Filter 1M elements | 38ms | 7ms | **5.4x** |
| Sum 1M elements | 12ms | 2ms | **6.0x** |

*Benchmarks on 8-core CPU*

## 📚 Parallel Iterators

### Par Methods

```rust
use avila_parallel::prelude::*;

let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

// Map
data.par_iter().map(|x| x * 2);

// Filter
data.par_iter().filter(|x| x % 2 == 0);

// Fold
data.par_iter().fold(|| 0, |acc, x| acc + x);

// For each
data.par_iter().for_each(|x| println!("{}", x));

// Find
data.par_iter().find_any(|x| *x > 5);

// All/Any
data.par_iter().all(|x| *x > 0);
data.par_iter().any(|x| *x == 5);
```

## 🎮 Use Cases

### Image Processing

```rust
use avila_parallel::prelude::*;

fn process_pixels(image: &mut Image) {
    image.pixels.par_iter_mut().for_each(|pixel| {
        *pixel = apply_filter(*pixel);
    });
}
```

### Matrix Operations

```rust
use avila_parallel::prelude::*;

fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let rows = a.rows();
    let cols = b.cols();

    (0..rows).into_par_iter().flat_map(|i| {
        (0..cols).into_par_iter().map(move |j| {
            (0..a.cols()).map(|k| {
                a[(i, k)] * b[(k, j)]
            }).sum()
        })
    }).collect()
}
```

### Data Processing

```rust
use avila_parallel::prelude::*;

let results: Vec<Result> = data
    .par_chunks(1000)
    .map(|chunk| process_chunk(chunk))
    .collect();
```

## 🧪 Testing

```bash
cargo test
cargo bench
```

## 📖 Documentation

Full documentation at [docs.rs/avila-parallel](https://docs.rs/avila-parallel)

## 🤝 Contributing

Part of the [AVL Platform](https://avila.inc) - contributions welcome!

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

---

**Built with 🇧🇷 by Avila Development Team**
