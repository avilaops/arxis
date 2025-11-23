//! Cluster validation metrics

use crate::metrics::distance::Metric;
use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

/// Silhouette score (internal metric)
///
/// Score range: [-1, 1]
/// - 1: Sample is far from neighboring clusters
/// - 0: Sample is on or very close to decision boundary
/// - -1: Sample may have been assigned to wrong cluster
pub fn silhouette_score(
    data: &ArrayView2<f64>,
    labels: &ArrayView1<usize>,
    metric: &Metric,
) -> Result<f64> {
    let n_samples = data.nrows();

    if n_samples != labels.len() {
        return Err(ClusteringError::ShapeMismatch {
            expected: format!("data rows: {}", n_samples),
            actual: format!("labels: {}", labels.len()),
        });
    }

    // Find unique clusters (excluding noise if present)
    let unique_labels: Vec<usize> = labels.iter()
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    if unique_labels.len() < 2 {
        return Err(ClusteringError::InvalidParameter(
            "Need at least 2 clusters for silhouette score".to_string(),
        ));
    }

    let mut silhouette_values = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let label = labels[i];
        let sample = data.row(i);

        // Compute a: mean distance to same cluster
        let same_cluster: Vec<usize> = (0..n_samples)
            .filter(|&j| j != i && labels[j] == label)
            .collect();

        if same_cluster.is_empty() {
            silhouette_values.push(0.0);
            continue;
        }

        let a: f64 = same_cluster
            .iter()
            .map(|&j| metric.distance(&sample, &data.row(j)).unwrap_or(0.0))
            .sum::<f64>()
            / same_cluster.len() as f64;

        // Compute b: mean distance to nearest different cluster
        let mut b = f64::INFINITY;
        for &other_label in &unique_labels {
            if other_label == label {
                continue;
            }

            let other_cluster: Vec<usize> = (0..n_samples)
                .filter(|&j| labels[j] == other_label)
                .collect();

            if !other_cluster.is_empty() {
                let mean_dist: f64 = other_cluster
                    .iter()
                    .map(|&j| metric.distance(&sample, &data.row(j)).unwrap_or(0.0))
                    .sum::<f64>()
                    / other_cluster.len() as f64;

                b = b.min(mean_dist);
            }
        }

        let s = (b - a) / a.max(b);
        silhouette_values.push(s);
    }

    Ok(silhouette_values.iter().sum::<f64>() / n_samples as f64)
}

/// Davies-Bouldin index (internal metric, lower is better)
///
/// Average similarity between each cluster and its most similar cluster.
/// Lower values indicate better clustering (0 is best).
pub fn davies_bouldin_score(data: &ArrayView2<f64>, labels: &ArrayView1<usize>) -> Result<f64> {
    let n_samples = data.nrows();
    let n_features = data.ncols();

    if n_samples != labels.len() {
        return Err(ClusteringError::ShapeMismatch {
            expected: format!("data rows: {}", n_samples),
            actual: format!("labels: {}", labels.len()),
        });
    }

    let unique_labels: Vec<usize> = labels.iter()
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let n_clusters = unique_labels.len();

    if n_clusters < 2 {
        return Err(ClusteringError::InvalidParameter(
            "Need at least 2 clusters for Davies-Bouldin".to_string(),
        ));
    }

    // Compute centroids
    let mut centroids = Array2::zeros((n_clusters, n_features));
    for (cluster_idx, &label) in unique_labels.iter().enumerate() {
        let cluster_mask: Vec<usize> = (0..n_samples)
            .filter(|&i| labels[i] == label)
            .collect();

        let n_k = cluster_mask.len() as f64;
        if n_k == 0.0 {
            continue;
        }

        for &i in &cluster_mask {
            centroids.row_mut(cluster_idx).scaled_add(1.0, &data.row(i));
        }
        centroids.row_mut(cluster_idx).mapv_inplace(|x| x / n_k);
    }

    // Compute within-cluster scatter
    let mut scatter = vec![0.0; n_clusters];
    for (cluster_idx, &label) in unique_labels.iter().enumerate() {
        let cluster_mask: Vec<usize> = (0..n_samples)
            .filter(|&i| labels[i] == label)
            .collect();

        if cluster_mask.is_empty() {
            continue;
        }

        let centroid = centroids.row(cluster_idx);
        scatter[cluster_idx] = cluster_mask
            .iter()
            .map(|&i| {
                let diff = data.row(i).to_owned() - &centroid;
                diff.mapv(|x| x.powi(2)).sum().sqrt()
            })
            .sum::<f64>()
            / cluster_mask.len() as f64;
    }

    // Compute Davies-Bouldin index
    let mut db_sum = 0.0;
    for i in 0..n_clusters {
        let mut max_ratio: f64 = 0.0;
        for j in 0..n_clusters {
            if i == j {
                continue;
            }

            let diff = centroids.row(i).to_owned() - &centroids.row(j);
            let dist = diff.mapv(|x| x.powi(2)).sum().sqrt();

            if dist > 0.0 {
                let ratio = (scatter[i] + scatter[j]) / dist;
                max_ratio = max_ratio.max(ratio);
            }
        }
        db_sum += max_ratio;
    }

    Ok(db_sum / n_clusters as f64)
}

/// Calinski-Harabasz index (internal metric, higher is better)
///
/// Also known as Variance Ratio Criterion.
/// Higher values indicate better defined clusters.
pub fn calinski_harabasz_score(data: &ArrayView2<f64>, labels: &ArrayView1<usize>) -> Result<f64> {
    let n_samples = data.nrows();
    let n_features = data.ncols();

    if n_samples != labels.len() {
        return Err(ClusteringError::ShapeMismatch {
            expected: format!("data rows: {}", n_samples),
            actual: format!("labels: {}", labels.len()),
        });
    }

    let unique_labels: Vec<usize> = labels.iter()
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let n_clusters = unique_labels.len();

    if n_clusters < 2 || n_clusters >= n_samples {
        return Err(ClusteringError::InvalidParameter(
            "Need at least 2 clusters and n_clusters < n_samples".to_string(),
        ));
    }

    // Overall mean
    let mean = data.mean_axis(ndarray::Axis(0)).unwrap();

    // Between-cluster dispersion
    let mut between_dispersion = 0.0;
    for &label in &unique_labels {
        let cluster_mask: Vec<usize> = (0..n_samples)
            .filter(|&i| labels[i] == label)
            .collect();

        let n_k = cluster_mask.len() as f64;
        if n_k == 0.0 {
            continue;
        }

        // Cluster centroid
        let mut centroid = Array1::zeros(n_features);
        for &i in &cluster_mask {
            centroid = centroid + &data.row(i);
        }
        centroid = centroid / n_k;

        // Add to between dispersion
        let diff = &centroid - &mean;
        between_dispersion += n_k * diff.mapv(|x: f64| x.powi(2)).sum();
    }

    // Within-cluster dispersion
    let mut within_dispersion = 0.0;
    for &label in &unique_labels {
        let cluster_mask: Vec<usize> = (0..n_samples)
            .filter(|&i| labels[i] == label)
            .collect();

        if cluster_mask.is_empty() {
            continue;
        }

        // Cluster centroid
        let n_k = cluster_mask.len() as f64;
        let mut centroid: Array1<f64> = Array1::zeros(n_features);
        for &i in &cluster_mask {
            centroid = centroid + &data.row(i);
        }
        centroid = centroid / n_k;

        // Add to within dispersion
        for &i in &cluster_mask {
            let diff = data.row(i).to_owned() - &centroid;
            within_dispersion += diff.mapv(|x: f64| x.powi(2)).sum();
        }
    }

    if within_dispersion == 0.0 {
        return Ok(f64::INFINITY);
    }

    // Calinski-Harabasz score
    let score = (between_dispersion / within_dispersion)
        * ((n_samples - n_clusters) as f64 / (n_clusters - 1) as f64);

    Ok(score)
}

/// Dunn index (internal metric, higher is better)
pub fn dunn_index(
    data: &ArrayView2<f64>,
    labels: &ArrayView1<usize>,
    metric: &Metric,
) -> Result<f64> {
    // TODO: Implement Dunn index
    // Ratio of minimum inter-cluster distance to maximum intra-cluster distance
    unimplemented!("dunn_index")
}

/// Adjusted Rand Index (external metric)
///
/// Score range: [-1, 1]
/// - 1.0: Perfect agreement
/// - 0.0: Random labeling
/// - Negative: Worse than random
pub fn adjusted_rand_index(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    if true_labels.len() != pred_labels.len() {
        return Err(ClusteringError::ShapeMismatch {
            expected: format!("true_labels: {}", true_labels.len()),
            actual: format!("pred_labels: {}", pred_labels.len()),
        });
    }

    let n = true_labels.len();
    if n == 0 {
        return Ok(1.0);
    }

    // Build contingency table
    let true_unique: std::collections::HashSet<_> = true_labels.iter().copied().collect();
    let pred_unique: std::collections::HashSet<_> = pred_labels.iter().copied().collect();

    let mut contingency = std::collections::HashMap::new();
    for i in 0..n {
        *contingency.entry((true_labels[i], pred_labels[i])).or_insert(0) += 1;
    }

    // Compute sums
    let mut a_sum = 0i64;
    let mut b_sum = 0i64;

    // Sum over true clusters
    let mut true_class_counts = std::collections::HashMap::new();
    for &label in true_labels.iter() {
        *true_class_counts.entry(label).or_insert(0) += 1;
    }
    for &count in true_class_counts.values() {
        if count > 1 {
            a_sum += (count * (count - 1)) / 2;
        }
    }

    // Sum over predicted clusters
    let mut pred_class_counts = std::collections::HashMap::new();
    for &label in pred_labels.iter() {
        *pred_class_counts.entry(label).or_insert(0) += 1;
    }
    for &count in pred_class_counts.values() {
        if count > 1 {
            b_sum += (count * (count - 1)) / 2;
        }
    }

    // Sum over contingency table
    let mut n_ij_sum = 0i64;
    for &count in contingency.values() {
        if count > 1 {
            n_ij_sum += (count * (count - 1)) / 2;
        }
    }

    let expected_index = a_sum as f64 * b_sum as f64 / (n as f64 * (n as f64 - 1.0) / 2.0);
    let max_index = (a_sum as f64 + b_sum as f64) / 2.0;
    let index = n_ij_sum as f64;

    if max_index - expected_index == 0.0 {
        return Ok(1.0);
    }

    Ok((index - expected_index) / (max_index - expected_index))
}

/// Adjusted Mutual Information (external metric)
pub fn adjusted_mutual_info(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    // TODO: Implement AMI
    unimplemented!("adjusted_mutual_info")
}

/// Normalized Mutual Information (external metric)
pub fn normalized_mutual_info(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    // TODO: Implement NMI
    unimplemented!("normalized_mutual_info")
}

/// Fowlkes-Mallows score (external metric)
pub fn fowlkes_mallows_score(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    // TODO: Implement Fowlkes-Mallows score
    unimplemented!("fowlkes_mallows_score")
}

/// Homogeneity score (external metric)
pub fn homogeneity_score(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    // TODO: Implement homogeneity score
    unimplemented!("homogeneity_score")
}

/// Completeness score (external metric)
pub fn completeness_score(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    // TODO: Implement completeness score
    unimplemented!("completeness_score")
}

/// V-measure score (external metric)
pub fn v_measure_score(
    true_labels: &ArrayView1<usize>,
    pred_labels: &ArrayView1<usize>,
) -> Result<f64> {
    // TODO: Implement V-measure score
    // Harmonic mean of homogeneity and completeness
    unimplemented!("v_measure_score")
}

/// Clustering stability analysis
pub fn clustering_stability(
    data: &ArrayView2<f64>,
    algorithm: &dyn ClusteringAlgorithm,
    n_bootstraps: usize,
) -> Result<f64> {
    // TODO: Implement stability analysis using bootstrap sampling
    unimplemented!("clustering_stability")
}

/// Trait for clustering algorithms (for stability analysis)
pub trait ClusteringAlgorithm {
    fn fit_predict(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>>;
}
