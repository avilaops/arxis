//! Clustering algorithm benchmarks
//!
//! These benchmarks compare the performance of different clustering algorithms
//! on various dataset sizes and characteristics.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ndarray::{Array2, ArrayView2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

// Helper function to generate synthetic clustered data
fn generate_clustered_data(n_samples: usize, n_features: usize, n_clusters: usize) -> Array2<f64> {
    let mut data = Array2::zeros((n_samples, n_features));
    let samples_per_cluster = n_samples / n_clusters;

    for cluster_id in 0..n_clusters {
        let start_idx = cluster_id * samples_per_cluster;
        let end_idx = if cluster_id == n_clusters - 1 {
            n_samples
        } else {
            (cluster_id + 1) * samples_per_cluster
        };

        let center_offset = (cluster_id as f64) * 10.0;
        for i in start_idx..end_idx {
            for j in 0..n_features {
                data[[i, j]] = center_offset + (rand::random::<f64>() - 0.5) * 2.0;
            }
        }
    }

    data
}

fn kmeans_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::kmeans::KMeansBuilder;

    let mut group = c.benchmark_group("kmeans");

    for size in [100, 500, 1000, 5000].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = KMeansBuilder::new(5)
                        .max_iter(100)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn dbscan_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::dbscan::DBSCANBuilder;

    let mut group = c.benchmark_group("dbscan");

    for size in [100, 500, 1000, 2000].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = DBSCANBuilder::new(0.5, 5)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn hierarchical_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::hierarchical::{HierarchicalBuilder, Linkage};

    let mut group = c.benchmark_group("hierarchical");

    for size in [100, 250, 500].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = HierarchicalBuilder::new(5)
                        .linkage(Linkage::Ward)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn spectral_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::spectral::SpectralClusteringBuilder;

    let mut group = c.benchmark_group("spectral");

    for size in [100, 250, 500].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = SpectralClusteringBuilder::new(5)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn fuzzy_cmeans_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::fuzzy_cmeans::FuzzyCMeansBuilder;

    let mut group = c.benchmark_group("fuzzy_cmeans");

    for size in [100, 500, 1000, 5000].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = FuzzyCMeansBuilder::new(5)
                        .max_iter(100)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn mean_shift_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::mean_shift::MeanShiftBuilder;

    let mut group = c.benchmark_group("mean_shift");

    for size in [100, 250, 500].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = MeanShiftBuilder::new()
                        .bandwidth(2.0)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn affinity_propagation_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::affinity_propagation::AffinityPropagationBuilder;

    let mut group = c.benchmark_group("affinity_propagation");

    for size in [50, 100, 200].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = AffinityPropagationBuilder::new()
                        .max_iter(100)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

fn birch_benchmark(c: &mut Criterion) {
    use avila_clustering::algorithms::birch::BirchBuilder;

    let mut group = c.benchmark_group("birch");

    for size in [100, 500, 1000, 5000].iter() {
        let data = generate_clustered_data(*size, 10, 5);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let model = BirchBuilder::new()
                        .threshold(1.0).unwrap()
                        .n_clusters(5)
                        .fit(black_box(data.view()))
                        .unwrap();
                    black_box(model);
                });
            },
        );
    }

    group.finish();
}

// Comparison benchmark across all algorithms
fn algorithm_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithm_comparison");
    let data = generate_clustered_data(1000, 10, 5);

    use avila_clustering::algorithms::{
        kmeans::KMeansBuilder,
        dbscan::DBSCANBuilder,
        hierarchical::{HierarchicalBuilder, Linkage},
        fuzzy_cmeans::FuzzyCMeansBuilder,
        birch::BirchBuilder,
    };

    group.bench_function("kmeans", |b| {
        b.iter(|| {
            KMeansBuilder::new(5)
                .fit(black_box(data.view()))
                .unwrap()
        });
    });

    group.bench_function("dbscan", |b| {
        b.iter(|| {
            DBSCANBuilder::new(0.5, 5)
                .fit(black_box(data.view()))
                .unwrap()
        });
    });

    group.bench_function("hierarchical_ward", |b| {
        b.iter(|| {
            HierarchicalBuilder::new(5)
                .linkage(Linkage::Ward)
                .fit(black_box(data.view()))
                .unwrap()
        });
    });

    group.bench_function("fuzzy_cmeans", |b| {
        b.iter(|| {
            FuzzyCMeansBuilder::new(5)
                .fit(black_box(data.view()))
                .unwrap()
        });
    });

    group.bench_function("birch", |b| {
        b.iter(|| {
            BirchBuilder::new()
                .threshold(1.0).unwrap()
                .n_clusters(5)
                .fit(black_box(data.view()))
                .unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    kmeans_benchmark,
    dbscan_benchmark,
    hierarchical_benchmark,
    spectral_benchmark,
    fuzzy_cmeans_benchmark,
    mean_shift_benchmark,
    affinity_propagation_benchmark,
    birch_benchmark,
    algorithm_comparison
);

criterion_main!(benches);
