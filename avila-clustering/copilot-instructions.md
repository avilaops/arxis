# Avila Clustering - Copilot Instructions

**Projeto**: avila-clustering  
**Descrição**: State-of-the-Art Clustering Algorithms - Surpassing scikit-learn, HDBSCAN, RAPIDS cuML  
**Status**: v0.1.0 - Foundation Phase  
**Filosofia**: Scientific Excellence. Performance First. GPU-Accelerated Where It Matters.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Surpassing scikit-learn em Tudo
```rust
// ✅ CORRETO: Algoritmo superior ao sklearn
pub struct HDBSCAN {
    min_cluster_size: usize,
    min_samples: usize,
    cluster_selection_epsilon: f64,
    algorithm: Algorithm,
}

impl HDBSCAN {
    pub fn fit(&self, data: &Array2<f64>) -> Result<ClusteringResult> {
        // 1. Build minimum spanning tree (Prim's algorithm)
        // 2. Extend to single linkage hierarchy
        // 3. Condense tree (remove small clusters)
        // 4. Extract optimal clusters (stability-based)
        // 5. Soft clustering (prediction data)
        
        // Complexity: O(n log n) with Borůvka + KD-tree
        // sklearn HDBSCAN: O(n²) naive implementation
    }
}

// Target: 10x-50x mais rápido que sklearn em datasets >10K pontos
```

**Algoritmos a superar**:
- KMeans: Elkan algorithm (3x-5x faster than Lloyd)
- HDBSCAN: Borůvka + KD-tree (10x-50x faster)
- DBSCAN: Ball tree + vectorized (5x-10x faster)
- GMM: Vectorized EM (3x-5x faster)
- Spectral: Efficient eigensolver (2x-4x faster)

### 2. GPU Acceleration para Datasets Grandes
```rust
// ✅ CORRETO: CPU/GPU hybrid
impl KMeans {
    pub fn fit(&self, data: &Array2<f64>) -> Result<ClusteringResult> {
        // Auto-detect dataset size
        if data.nrows() > 100_000 {
            // GPU acceleration
            #[cfg(feature = "gpu")]
            return self.fit_gpu(data);
        }
        
        // CPU implementation
        self.fit_cpu(data)
    }
    
    #[cfg(feature = "gpu")]
    fn fit_gpu(&self, data: &Array2<f64>) -> Result<ClusteringResult> {
        use avx_gpu::*;
        
        let device = Device::auto()?;
        let data_gpu = device.buffer_from_slice(data.as_slice().unwrap())?;
        
        // GPU-accelerated k-means
        // - Distance computation: GEMM (matrix multiply)
        // - Centroid update: Reduce operations
        // - Assignment: ArgMin kernel
    }
}

// Target GPU speedup:
// - 10K points: 2x-5x
// - 100K points: 10x-20x
// - 1M+ points: 50x-100x
```

### 3. Métricas Científicas Avançadas
```rust
// ✅ CORRETO: Suporte a métricas especializadas
pub enum Metric {
    // Clássicas
    Euclidean,
    Manhattan,
    Cosine,
    
    // Científicas avançadas
    Mahalanobis(Array2<f64>),      // Com covariância
    Geodesic(Manifold),             // Em variedades curvas (Relatividade)
    SpectralAngle,                  // Astronomia (comparação de espectros)
    DynamicTimeWarping,             // Time series alignment
    Haversine,                      // Great-circle (Terra/esfera celeste)
    
    // Strings
    Levenshtein,
    JaroWinkler,
    
    Custom(Box<dyn Fn(&[f64], &[f64]) -> f64>),
}

// Exemplo: Clustering de eventos astronômicos com distância geodésica
let hdbscan = HDBSCAN::builder()
    .metric(Metric::Geodesic(Manifold::Schwarzschild))  // Spacetime curvature!
    .build();
```

---

**Avila Clustering** - Scientific Excellence in Rust 🚀
