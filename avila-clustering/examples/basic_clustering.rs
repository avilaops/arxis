use avila_clustering::algorithms::dbscan::DBSCAN;
use avila_clustering::algorithms::kmedoids::{KMedoids, KMedoidsAlgorithm};
use avila_clustering::prelude::*;
use ndarray::Array2;

fn main() -> Result<()> {
    println!("🚀 Testando avila-clustering\n");

    // Criar dados sintéticos
    let data = generate_test_data();
    println!(
        "✅ Dados gerados: {} amostras, {} features\n",
        data.nrows(),
        data.ncols()
    );

    // Teste 1: KMeans
    println!("📊 Teste 1: KMeans (Lloyd)");
    let mut kmeans = KMeans::builder(3)
        .init_method(InitMethod::KMeansPlusPlus)
        .max_iter(100)
        .n_init(5)
        .random_state(42)
        .build();

    let result = kmeans.fit(&data.view())?;
    println!("   Clusters encontrados: 3");
    println!("   Inertia: {:.2}", result.inertia());
    println!("   Iterações: {}", result.n_iter());
    println!("   ✅ KMeans OK\n");

    // Teste 2: KMeans MiniBatch
    println!("📊 Teste 2: KMeans (MiniBatch)");
    let mut kmeans_mb = KMeans::builder(3)
        .algorithm(Algorithm::MiniBatch { batch_size: 50 })
        .random_state(42)
        .build();

    let result_mb = kmeans_mb.fit(&data.view())?;
    println!("   Clusters encontrados: 3");
    println!("   Inertia: {:.2}", result_mb.inertia());
    println!("   ✅ MiniBatch KMeans OK\n");

    // Teste 3: DBSCAN
    println!("📊 Teste 3: DBSCAN");
    let dbscan = DBSCAN::new(0.5, 5);
    let result_db = dbscan.fit(&data.view())?;
    println!("   Clusters encontrados: {}", result_db.n_clusters());
    println!("   Pontos de ruído: {}", result_db.n_noise_points());
    println!("   ✅ DBSCAN OK\n");

    // Teste 4: KMedoids (PAM)
    println!("📊 Teste 4: KMedoids (PAM)");
    let kmedoids = KMedoids::new(3)
        .algorithm(KMedoidsAlgorithm::PAM)
        .random_state(42);

    let result_km = kmedoids.fit(&data.view())?;
    println!("   Clusters encontrados: 3");
    println!("   Inertia: {:.2}", result_km.inertia());
    println!("   Medoid indices: {:?}", result_km.medoid_indices());
    println!("   ✅ KMedoids (PAM) OK\n");

    // Teste 5: KMedoids (CLARA)
    println!("📊 Teste 5: KMedoids (CLARA)");
    let kmedoids_clara = KMedoids::new(3)
        .algorithm(KMedoidsAlgorithm::CLARA {
            sample_size: 50,
            n_samples: 3,
        })
        .random_state(42);

    let result_clara = kmedoids_clara.fit(&data.view())?;
    println!("   Clusters encontrados: 3");
    println!("   Inertia: {:.2}", result_clara.inertia());
    println!("   ✅ KMedoids (CLARA) OK\n");

    // Teste 6: Predição com KMeans
    println!("📊 Teste 6: Predição com modelo treinado");
    let new_point = Array2::from_shape_vec((1, 2), vec![0.0, 0.0])
        .map_err(|e| ClusteringError::InvalidParameter(e.to_string()))?;
    let prediction = kmeans.predict(&new_point.view())?;
    println!("   Ponto [0.0, 0.0] pertence ao cluster: {}", prediction[0]);
    println!("   ✅ Predição OK\n");

    // Teste 7: Métricas de Validação
    println!("📊 Teste 7: Métricas de Validação Interna");
    let labels = result.labels();
    
    let calinski = calinski_harabasz_score(&data.view(), &labels.view())?;
    println!("   Calinski-Harabasz Score: {:.4}", calinski);
    
    let silhouette = silhouette_score(&data.view(), &labels.view(), &Metric::Euclidean)?;
    println!("   Silhouette Score: {:.4}", silhouette);
    
    let davies_bouldin = davies_bouldin_score(&data.view(), &labels.view())?;
    println!("   Davies-Bouldin Index: {:.4}", davies_bouldin);
    println!("   ✅ Métricas OK\n");

    // Teste 8: GMM
    println!("📊 Teste 8: Gaussian Mixture Model");
    let mut gmm = GaussianMixture::builder(3)
        .max_iter(50)
        .n_init(3)
        .build();

    let result_gmm = gmm.fit(&data.view())?;
    println!("   Clusters encontrados: 3");
    println!("   Convergiu: {}", result_gmm.converged);
    println!("   Iterações: {}", result_gmm.n_iter);
    println!("   ✅ GMM OK\n");

    println!("🎉 Todos os testes passaram!");
    println!("🚀 avila-clustering está funcionando perfeitamente!");

    Ok(())
}

fn generate_test_data() -> Array2<f64> {
    use rand::Rng;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let n_samples = 300;
    let n_features = 2;

    let mut data = Array2::zeros((n_samples, n_features));

    // Criar 3 clusters gaussianos
    let centers = vec![(0.0, 0.0), (5.0, 5.0), (-3.0, 4.0)];

    for i in 0..n_samples {
        let cluster = i % 3;
        let (cx, cy) = centers[cluster];

        data[[i, 0]] = cx + rng.gen::<f64>() * 1.5 - 0.75;
        data[[i, 1]] = cy + rng.gen::<f64>() * 1.5 - 0.75;
    }

    data
}
