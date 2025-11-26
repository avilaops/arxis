//! Demonstração de clustering espacial
//!
//! Este exemplo mostra como usar algoritmos de clustering para análise espacial:
//! - K-Means para agrupamento de pontos
//! - DBSCAN para detectar clusters de densidade
//! - Clustering hierárquico para análise em múltiplos níveis

use avila_geo::coords::Coord;
use avila_geo::geoprocessing::*;

fn main() {
    println!("=== Demonstração de Clustering Espacial ===\n");

    // Gerar dados sintéticos: cidades em 3 regiões
    let cities = generate_city_data();
    println!("✓ Gerados {} pontos de teste (3 regiões distintas)", cities.len());

    // 1. K-Means Clustering
    demo_kmeans(&cities);

    // 2. DBSCAN (Density-Based)
    demo_dbscan(&cities);

    // 3. Hierarchical Clustering
    demo_hierarchical(&cities);

    // 4. Comparação de métricas
    compare_clustering_metrics(&cities);
}

fn generate_city_data() -> Vec<Coord> {
    let mut cities = Vec::new();

    // Região 1: Sudeste (São Paulo, Rio)
    let southeast = vec![
        Coord::new(-23.5505, -46.6333), // São Paulo
        Coord::new(-23.6821, -46.8755), // Osasco
        Coord::new(-23.4205, -46.4893), // Guarulhos
        Coord::new(-22.9068, -43.1729), // Rio de Janeiro
        Coord::new(-22.8808, -43.3043), // Niterói
        Coord::new(-22.7633, -43.4489), // São Gonçalo
    ];

    // Região 2: Sul (Porto Alegre, Curitiba)
    let south = vec![
        Coord::new(-30.0346, -51.2177), // Porto Alegre
        Coord::new(-29.9155, -51.1965), // Canoas
        Coord::new(-29.7951, -51.1507), // Novo Hamburgo
        Coord::new(-25.4284, -49.2733), // Curitiba
        Coord::new(-25.5286, -49.1617), // São José dos Pinhais
        Coord::new(-25.4945, -49.2925), // Colombo
    ];

    // Região 3: Nordeste (Salvador, Recife)
    let northeast = vec![
        Coord::new(-12.9714, -38.5014), // Salvador
        Coord::new(-12.8672, -38.3675), // Lauro de Freitas
        Coord::new(-12.9058, -38.4193), // Camaçari
        Coord::new(-8.0476, -34.8770),  // Recife
        Coord::new(-8.0522, -34.9286),  // Olinda
        Coord::new(-8.1137, -34.8905),  // Jaboatão
    ];

    cities.extend(southeast);
    cities.extend(south);
    cities.extend(northeast);

    cities
}

fn demo_kmeans(cities: &[Coord]) {
    println!("\n--- K-Means Clustering ---");

    // Encontrar 3 clusters (uma para cada região)
    let clusters = KMeans::new(3)
        .max_iterations(100)
        .fit(cities);

    println!("✓ Encontrados {} clusters", clusters.len());

    for (i, cluster) in clusters.iter().enumerate() {
        println!("\nCluster {} (centro: {:.4}, {:.4}):",
            i + 1,
            cluster.center.x,
            cluster.center.y
        );
        println!("  - {} cidades", cluster.members.len());
        println!("  - Raio: {:.2} km", cluster.radius / 1000.0);

        // Mostrar algumas cidades do cluster
        for &idx in cluster.members.iter().take(3) {
            let city = &cities[idx];
            println!("    • ({:.4}, {:.4})", city.x, city.y);
        }
    }

    // Calcular métricas
    let silhouette = ClusterMetrics::silhouette_score(cities, &clusters);
    let davies_bouldin = ClusterMetrics::davies_bouldin_index(cities, &clusters);

    println!("\nMétricas de qualidade:");
    println!("  Silhouette Score: {:.4} (melhor: próximo de 1.0)", silhouette);
    println!("  Davies-Bouldin Index: {:.4} (melhor: próximo de 0.0)", davies_bouldin);
}

fn demo_dbscan(cities: &[Coord]) {
    println!("\n--- DBSCAN (Density-Based Clustering) ---");

    // DBSCAN com epsilon de 200km e mínimo de 2 pontos
    let epsilon = 200_000.0; // 200 km em metros
    let min_points = 2;

    let clusters = DBSCAN::new(epsilon, min_points).fit(cities);

    println!("✓ Encontrados {} clusters (densidade-based)", clusters.len());

    for (i, cluster) in clusters.iter().enumerate() {
        println!("\nCluster {} (centro: {:.4}, {:.4}):",
            i + 1,
            cluster.center.x,
            cluster.center.y
        );
        println!("  - {} pontos densos", cluster.members.len());
        println!("  - Densidade (raio {:.0}km): {:.2} pontos/cluster",
            epsilon / 1000.0,
            cluster.members.len() as f64
        );
    }

    // DBSCAN é bom para detectar outliers
    let total_clustered: usize = clusters.iter().map(|c| c.members.len()).sum();
    let outliers = cities.len() - total_clustered;

    if outliers > 0 {
        println!("\n⚠ {} pontos considerados outliers (não pertencem a clusters densos)", outliers);
    }
}

fn demo_hierarchical(cities: &[Coord]) {
    println!("\n--- Hierarchical Clustering ---");

    // Criar clustering hierárquico com linkage "average"
    let clusters = HierarchicalClustering::new(3)
        .linkage(LinkageType::Average)
        .fit(cities);

    println!("✓ Clusters hierárquicos criados (linkage: Average)");

    for (i, cluster) in clusters.iter().enumerate() {
        println!("\nCluster Hierárquico {}:", i + 1);
        println!("  - {} membros", cluster.members.len());
        println!("  - Centro: ({:.4}, {:.4})", cluster.center.x, cluster.center.y);
        println!("  - Raio máximo: {:.2} km", cluster.radius / 1000.0);
    }

    // Comparar diferentes tipos de linkage
    println!("\n--- Comparação de Linkage Methods ---");

    let linkages = vec![
        (LinkageType::Single, "Single (menor distância)"),
        (LinkageType::Complete, "Complete (maior distância)"),
        (LinkageType::Average, "Average (média das distâncias)"),
    ];

    for (linkage_type, description) in linkages {
        let clusters = HierarchicalClustering::new(3)
            .linkage(linkage_type)
            .fit(cities);

        let silhouette = ClusterMetrics::silhouette_score(cities, &clusters);
        println!("  {} - Silhouette: {:.4}", description, silhouette);
    }
}

fn compare_clustering_metrics(cities: &[Coord]) {
    println!("\n--- Comparação de Algoritmos ---");

    // K-Means
    let kmeans_clusters = KMeans::new(3).fit(cities);
    let kmeans_silhouette = ClusterMetrics::silhouette_score(cities, &kmeans_clusters);
    let kmeans_db = ClusterMetrics::davies_bouldin_index(cities, &kmeans_clusters);

    // DBSCAN
    let dbscan_clusters = DBSCAN::new(200_000.0, 2).fit(cities);
    let dbscan_silhouette = ClusterMetrics::silhouette_score(cities, &dbscan_clusters);
    let dbscan_db = ClusterMetrics::davies_bouldin_index(cities, &dbscan_clusters);

    // Hierarchical
    let hier_clusters = HierarchicalClustering::new(3)
        .linkage(LinkageType::Average)
        .fit(cities);
    let hier_silhouette = ClusterMetrics::silhouette_score(cities, &hier_clusters);
    let hier_db = ClusterMetrics::davies_bouldin_index(cities, &hier_clusters);

    println!("\n┌─────────────────┬───────────────┬────────────────────┐");
    println!("│ Algoritmo       │ Silhouette    │ Davies-Bouldin     │");
    println!("├─────────────────┼───────────────┼────────────────────┤");
    println!("│ K-Means         │ {:>13.4} │ {:>18.4} │", kmeans_silhouette, kmeans_db);
    println!("│ DBSCAN          │ {:>13.4} │ {:>18.4} │", dbscan_silhouette, dbscan_db);
    println!("│ Hierarchical    │ {:>13.4} │ {:>18.4} │", hier_silhouette, hier_db);
    println!("└─────────────────┴───────────────┴────────────────────┘");

    println!("\n📊 Interpretação:");
    println!("  • Silhouette Score: quanto mais próximo de 1.0, melhor a separação");
    println!("  • Davies-Bouldin: quanto mais próximo de 0.0, melhor a compactação");

    // Determinar melhor algoritmo
    let best_by_silhouette = if kmeans_silhouette >= dbscan_silhouette && kmeans_silhouette >= hier_silhouette {
        "K-Means"
    } else if dbscan_silhouette >= hier_silhouette {
        "DBSCAN"
    } else {
        "Hierarchical"
    };

    let best_by_db = if kmeans_db <= dbscan_db && kmeans_db <= hier_db {
        "K-Means"
    } else if dbscan_db <= hier_db {
        "DBSCAN"
    } else {
        "Hierarchical"
    };

    println!("\n🏆 Recomendação:");
    println!("  • Melhor separação (Silhouette): {}", best_by_silhouette);
    println!("  • Melhor compactação (Davies-Bouldin): {}", best_by_db);
}
