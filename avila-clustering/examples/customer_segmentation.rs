//! Customer Segmentation Example
//!
//! This example demonstrates how to use various clustering algorithms
//! to segment customers based on their purchasing behavior.

use avila_clustering::algorithms::{
    kmeans::KMeansBuilder,
    dbscan::DBSCANBuilder,
    hierarchical::{HierarchicalBuilder, Linkage},
    fuzzy_cmeans::FuzzyCMeansBuilder,
};
use avila_clustering::metrics::validation::{silhouette_score, davies_bouldin_score, calinski_harabasz_score};
use avila_clustering::metrics::distance::Metric;
use ndarray::{Array2, array};

fn main() -> avila_clustering::Result<()> {
    println!("=== Customer Segmentation Example ===\n");

    // Simulated customer data: [Annual Income (k$), Spending Score (1-100)]
    let customers = array![
        [15.0, 39.0],  // Low income, low spending
        [16.0, 41.0],
        [17.0, 42.0],
        [19.0, 40.0],
        [40.0, 60.0],  // Medium income, medium spending
        [42.0, 62.0],
        [43.0, 61.0],
        [44.0, 59.0],
        [60.0, 81.0],  // High income, high spending
        [61.0, 82.0],
        [62.0, 80.0],
        [63.0, 79.0],
        [59.0, 28.0],  // High income, low spending (careful spenders)
        [60.0, 27.0],
        [61.0, 29.0],
        [20.0, 85.0],  // Low income, high spending (impulsive buyers)
        [21.0, 86.0],
        [19.0, 87.0],
    ];

    println!("Dataset: {} customers with 2 features (Income, Spending)\n", customers.nrows());

    // 1. K-Means Clustering
    println!("1. K-Means Clustering (k=4)");
    println!("{}", "=".repeat(50));

    let kmeans = KMeansBuilder::new(4)
        .max_iter(300)
        .n_init(10)
        .fit(customers.view())?;

    println!("Number of clusters: {}", kmeans.n_clusters());
    println!("Inertia: {:.2}", kmeans.inertia());
    println!("Labels: {:?}", kmeans.labels());

    // Calculate validation metrics
    let kmeans_labels = ndarray::Array1::from_vec(kmeans.labels().iter().map(|&x| x as usize).collect());
    let silhouette = silhouette_score(&customers.view(), &kmeans_labels.view(), &Metric::Euclidean)?;
    let davies_bouldin = davies_bouldin_score(&customers.view(), &kmeans_labels.view())?;
    let calinski = calinski_harabasz_score(&customers.view(), &kmeans_labels.view())?;

    println!("Silhouette Score: {:.4}", silhouette);
    println!("Davies-Bouldin Index: {:.4}", davies_bouldin);
    println!("Calinski-Harabasz Score: {:.2}\n", calinski);

    // 2. DBSCAN - Density-based clustering
    println!("2. DBSCAN (eps=5.0, min_samples=2)");
    println!("{}", "=".repeat(50));

    let dbscan = DBSCANBuilder::new(5.0, 2)
        .fit(customers.view())?;

    println!("Number of clusters: {}", dbscan.n_clusters());
    println!("Labels: {:?}", dbscan.labels());
    println!("Noise points: {}\n", dbscan.labels().iter().filter(|&&l| l == -1).count());

    // 3. Hierarchical Clustering with Ward linkage
    println!("3. Hierarchical Clustering (Ward linkage, k=4)");
    println!("{}", "=".repeat(50));

    let hierarchical = HierarchicalBuilder::new(4)
        .linkage(Linkage::Ward)
        .fit(customers.view())?;

    println!("Number of clusters: {}", hierarchical.n_clusters());
    println!("Labels: {:?}\n", hierarchical.labels());

    // 4. Fuzzy C-Means - Soft clustering
    println!("4. Fuzzy C-Means (k=4, fuzziness=2.0)");
    println!("{}", "=".repeat(50));

    let fcm = FuzzyCMeansBuilder::new(4)
        .fuzziness(2.0)
        .max_iter(150)
        .fit(customers.view())?;

    println!("Number of clusters: {}", fcm.n_clusters());

    // Show membership probabilities for first 5 customers
    println!("\nMembership probabilities (first 5 customers):");
    for i in 0..5 {
        print!("Customer {}: [", i);
        for j in 0..4 {
            print!("{:.3}", fcm.membership()[[i, j]]);
            if j < 3 {
                print!(", ");
            }
        }
        println!("]");
    }

    // 5. Compare all methods
    println!("\n5. Algorithm Comparison");
    println!("{}", "=".repeat(50));
    println!("{:<20} {:<15} {:<15}", "Algorithm", "Silhouette", "Davies-Bouldin");
    println!("{}", "-".repeat(50));
    println!("{:<20} {:<15.4} {:<15.4}", "K-Means", silhouette, davies_bouldin);

    // Best practices recommendation
    println!("\n=== Recommendations ===");
    println!("Based on validation metrics:");
    if silhouette > 0.5 {
        println!("✓ Excellent cluster separation (Silhouette > 0.5)");
    } else if silhouette > 0.3 {
        println!("✓ Good cluster separation (Silhouette > 0.3)");
    } else {
        println!("⚠ Weak cluster structure (Silhouette < 0.3)");
    }

    if davies_bouldin < 1.0 {
        println!("✓ Clusters are well-separated (DB Index < 1.0)");
    } else {
        println!("⚠ Some cluster overlap detected (DB Index > 1.0)");
    }

    println!("\n=== Customer Segments Identified ===");
    println!("1. Budget Conscious: Low income, low spending");
    println!("2. Average Customers: Medium income, medium spending");
    println!("3. High-Value: High income, high spending");
    println!("4. Careful Spenders: High income, low spending");
    println!("5. Impulsive Buyers: Low income, high spending");

    Ok(())
}
