# avila-clustering

[![Crates.io](https://img.shields.io/crates/v/avila-clustering.svg)](https://crates.io/crates/avila-clustering)
[![Documentation](https://docs.rs/avila-clustering/badge.svg)](https://docs.rs/avila-clustering)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/avilaops/arxis/workflows/CI/badge.svg)](https://github.com/avilaops/arxis/actions)

**The most comprehensive clustering library in Rust - going beyond traditional clustering**

A high-performance, native Rust library providing 20+ clustering algorithms with advanced features for time series, graphs, text, streaming data, and scientific computing.

## 🎯 Why avila-clustering?

- 🚀 **Native Rust Performance**: Zero-copy operations, SIMD optimization, fearless concurrency
- 🧠 **Advanced Algorithms**: Time series, graphs, text, ensemble, online clustering
- 🔬 **Scientific Rigor**: Peer-reviewed implementations with proper references
- 📊 **Production Ready**: Comprehensive testing, benchmarks, and documentation
- 🎨 **Unique Features**: DTW clustering, community detection, concept drift, motif discovery
- 🌐 **Universal**: Works everywhere Rust runs - servers, edge, WASM

## 📦 Installation

```toml
[dependencies]
avila-clustering = "0.1"

# Optional GPU support
avila-clustering = { version = "0.1", features = ["gpu"] }
```

## 🚀 Quick Start

```rust
use avila_clustering::algorithms::kmeans::KMeansBuilder;
use ndarray::array;

fn main() -> avila_clustering::Result<()> {
    let data = array![
        [0.0, 0.0],
        [0.1, 0.1],
        [5.0, 5.0],
        [5.1, 5.1],
    ];

    let kmeans = KMeansBuilder::new(2)
        .max_iter(100)
        .fit(data.view())?;

    println!("Labels: {:?}", kmeans.labels());
    println!("Centers: {:?}", kmeans.cluster_centers());

    Ok(())
}
```

## 📚 Comprehensive Algorithm Coverage

### 🎯 Traditional Clustering (13 algorithms)

#### Partitional
- **K-Means**: Lloyd, Elkan, Mini-batch, K-means++ initialization
- **K-Medoids**: PAM, CLARA implementations
- **Fuzzy C-Means**: Soft clustering with membership probabilities
- **Mean Shift**: Bandwidth-based mode-seeking
- **Affinity Propagation**: Message-passing clustering

#### Density-Based
- **DBSCAN**: Classic density clustering with KD-tree optimization
- **HDBSCAN**: Hierarchical density-based with soft clustering
- **OPTICS**: Ordering points for cluster identification

#### Hierarchical
- **Agglomerative**: Ward, Single, Complete, Average linkage
- **BIRCH**: Balanced Iterative Reducing with CF-Tree

#### Model-Based
- **GMM**: Gaussian Mixture Models with EM algorithm
- **Spectral**: Normalized cuts, affinity-based

#### Streaming
- **Streaming K-Means**: Online clustering for data streams

### ⏱️ Time Series Clustering (NEW!)

```rust
use avila_clustering::algorithms::timeseries::{TimeSeriesKMeans, TimeSeriesMetric};

// Cluster heartbeat signals, stock prices, sensor data
let timeseries_data = array![...]; // shape: (n_series, time_length)

let result = TimeSeriesKMeans::new(3)
    .metric(TimeSeriesMetric::DTW)  // Dynamic Time Warping
    .fit(timeseries_data.view())?;

// Find recurring patterns
use avila_clustering::algorithms::timeseries::MotifDiscovery;

let motifs = MotifDiscovery::new(pattern_length, n_motifs)
    .threshold(0.5)
    .find_motifs(long_series.view())?;
```

**Features**:
- Dynamic Time Warping (DTW) distance
- Shape-Based Distance (SBD)
- Derivative DTW for trend analysis
- Motif discovery (recurring pattern detection)
- Works with variable-length sequences

**Use Cases**:
- Medical: ECG/EEG classification, patient monitoring
- Finance: Stock pattern recognition, trading signals
- IoT: Sensor anomaly detection, predictive maintenance
- Audio: Voice patterns, music genre classification

### 🕸️ Graph & Network Clustering (NEW!)

```rust
use avila_clustering::algorithms::graph::{Graph, LouvainClustering, LabelPropagation};

let mut graph = Graph::new(n_nodes);
graph.add_edge(0, 1, 1.0);
graph.add_edge(1, 2, 0.5);

// Community detection
let result = LouvainClustering::new()
    .resolution(1.0)
    .fit(&graph)?;

println!("Modularity: {}", result.modularity);
println!("Communities: {:?}", result.labels);
```

**Algorithms**:
- **Louvain**: Modularity optimization for community detection
- **Label Propagation**: Fast semi-supervised clustering
- **Connected Components**: Basic graph partitioning

**Use Cases**:
- Social networks: Friend groups, influence networks
- Biology: Protein interactions, gene clusters
- Citations: Research paper communities
- Recommendation systems: User/item clustering

### 📝 Text & Document Clustering (NEW!)

```rust
use avila_clustering::algorithms::text::{TfidfVectorizer, TopicModeling};

let documents = vec![
    "machine learning is great".to_string(),
    "deep learning models".to_string(),
    "natural language processing".to_string(),
];

// TF-IDF vectorization
let mut vectorizer = TfidfVectorizer::new()
    .max_features(1000)
    .min_df(2);

let tfidf_matrix = vectorizer.fit_transform(&documents)?;

// Topic modeling
let topics = TopicModeling::new(n_topics)
    .n_top_words(10)
    .fit(&documents)?;

for topic in topics.topics {
    println!("{}", topic.display());
}
```

**Features**:
- TF-IDF vectorization
- Topic modeling via clustering
- Cosine similarity for text
- Automatic vocabulary building

**Use Cases**:
- Document organization
- News categorization
- Customer feedback analysis
- Research paper grouping

### 🌊 Online & Streaming Clustering (NEW!)

```rust
use avila_clustering::algorithms::online::{OnlineKMeans, OnlineBIRCH, SlidingWindowClustering};

// Process streaming data
let mut online = OnlineKMeans::new(k)
    .batch_size(100)
    .learning_rate(0.1);

// Update with new points
for point in data_stream {
    let cluster = online.partial_fit(point.view())?;
    println!("Assigned to cluster: {}", cluster);
}

// Concept drift detection
let mut drift_detector = SlidingWindowClustering::new(k, window_size, overlap);

let info = drift_detector.update(&new_batch)?;
if info.drift_detected {
    println!("⚠️ Concept drift detected!");
}
```

**Algorithms**:
- **Online K-Means**: Mini-batch style updates
- **Online BIRCH**: Memory-efficient CF-Tree
- **Sliding Window**: Concept drift detection

**Use Cases**:
- Real-time monitoring: IoT sensors, network traffic
- Financial: Fraud detection, trading signals
- Manufacturing: Quality control, equipment health
- Web analytics: User behavior, A/B testing

### 🎭 Ensemble Clustering (NEW!)

```rust
use avila_clustering::algorithms::ensemble::{EnsembleClusteringBuilder, ConsensusMethod};

// Combine multiple clusterings for stability
let result = EnsembleClusteringBuilder::new(n_clusters)
    .n_iterations(20)
    .consensus_method(ConsensusMethod::CoAssociation)
    .subsample_ratio(0.8)
    .fit(data.view())?;

println!("Stability score: {}", result.stability_score());
```

**Benefits**:
- More stable results across runs
- Reduces initialization sensitivity
- Combines algorithm strengths
- Better handling of complex structures

## 📊 Validation Metrics

```rust
use avila_clustering::metrics::validation::{
    silhouette_score, davies_bouldin_score, calinski_harabasz_score
};

let silhouette = silhouette_score(&data, &labels, &Metric::Euclidean)?;
let davies_bouldin = davies_bouldin_score(&data, &labels)?;
let calinski = calinski_harabasz_score(&data, &labels)?;
```

**Available Metrics**:
- Silhouette Score (internal quality)
- Davies-Bouldin Index (cluster separation)
- Calinski-Harabasz Score (variance ratio)
- Modularity (for graphs)
- Stability scores (for ensemble)

## 🎯 Distance Metrics

**Standard**:
- Euclidean, Manhattan, Chebyshev, Minkowski
- Cosine, Correlation

**Advanced**:
- Mahalanobis (with covariance)
- Dynamic Time Warping (DTW)
- Shape-Based Distance (SBD)
- Haversine (geographic)

**Probabilistic**:
- Kullback-Leibler divergence
- Jensen-Shannon divergence
- Wasserstein distance
- Hellinger distance

## 💡 Examples

All examples are runnable:

```bash
# Traditional clustering
cargo run --example basic_clustering
cargo run --example customer_segmentation
cargo run --example image_segmentation
cargo run --example anomaly_detection

# Advanced features
cargo run --example timeseries_clustering
cargo run --example social_network
cargo run --example streaming_clustering
```

## 📖 Documentation

- [API Documentation](https://docs.rs/avila-clustering)
- [User Guide](https://docs.rs/avila-clustering)
- [Examples](./examples/)
- [Benchmarks](./BENCHMARKS.md)
- [Contributing Guide](./CONTRIBUTING.md)

## 🚀 Performance

Built for speed with:
- Zero-copy operations using ndarray views
- SIMD optimization for distance calculations
- Rayon for automatic parallelization
- KD-trees for spatial indexing
- Optional GPU acceleration (CUDA/ROCm)

See [BENCHMARKS.md](./BENCHMARKS.md) for detailed comparisons.

## 🌍 Use Cases

### 🏥 Healthcare
- Patient clustering for personalized medicine
- Disease outbreak detection
- ECG/EEG pattern recognition
- Medical image segmentation

### 💰 Finance
- Customer segmentation
- Fraud detection patterns
- Stock price pattern matching
- Risk assessment clustering

### 🏭 Manufacturing
- Quality control grouping
- Equipment failure patterns
- Sensor anomaly detection
- Supply chain optimization

### 🌐 Tech & Web
- User behavior segmentation
- Content recommendation
- Network traffic analysis
- Log pattern detection

### 🔬 Science
- Astronomical object classification
- Gene expression clustering
- Climate pattern analysis
- Protein structure grouping

### 📱 Social Media
- Community detection
- Influencer identification
- Content categorization
- Trend analysis

## 🛠️ Development

```bash
# Run tests
cargo test --all-features

# Run benchmarks
cargo bench

# Build documentation
cargo doc --no-deps --open

# Format code
cargo fmt

# Lint
cargo clippy --all-features
```

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

Dual-licensed under MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

## 🙏 Acknowledgments

Built on the shoulders of giants:
- scikit-learn for algorithm references
- HDBSCAN authors for hierarchical density clustering
- Research papers cited in individual algorithm docs

## 🗺️ Roadmap

- [ ] GPU acceleration (v0.2.0)
- [ ] Python bindings via PyO3 (v0.3.0)
- [ ] WASM support (v0.3.0)
- [ ] Distributed clustering (v0.4.0)
- [ ] Model persistence (v0.4.0)

See [STATUS.md](./STATUS.md) for detailed roadmap.

---

**Made with ❤️ for the Rust community**

[GitHub](https://github.com/avilaops/arxis) | [Crates.io](https://crates.io/crates/avila-clustering) | [Docs](https://docs.rs/avila-clustering)
