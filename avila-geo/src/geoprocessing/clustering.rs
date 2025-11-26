//! Clustering espacial e Machine Learning
//!
//! Implementa algoritmos de clustering e análise de padrões espaciais

use crate::coords::GeoCoord;
use crate::geoprocessing::analysis::haversine_distance;
use crate::geoprocessing::spatial::{BoundingBox, SpatialFeature};
use std::collections::{HashMap, HashSet};

/// Cluster de pontos
#[derive(Debug, Clone)]
pub struct Cluster {
    pub id: usize,
    pub center: GeoCoord,
    pub members: Vec<usize>,
    pub radius: f64,
}

impl Cluster {
    /// Cria um novo cluster
    pub fn new(id: usize, center: GeoCoord) -> Self {
        Self {
            id,
            center,
            members: Vec::new(),
            radius: 0.0,
        }
    }

    /// Adiciona um membro ao cluster
    pub fn add_member(&mut self, idx: usize) {
        self.members.push(idx);
    }

    /// Calcula o centro do cluster baseado em seus membros
    pub fn recalculate_center(&mut self, points: &[GeoCoord]) {
        if self.members.is_empty() {
            return;
        }

        let mut sum_lat = 0.0;
        let mut sum_lon = 0.0;

        for &idx in &self.members {
            sum_lat += points[idx].lat;
            sum_lon += points[idx].lon;
        }

        self.center = GeoCoord {
            lat: sum_lat / self.members.len() as f64,
            lon: sum_lon / self.members.len() as f64,
        };

        // Recalcular raio
        self.radius = self
            .members
            .iter()
            .map(|&idx| haversine_distance(&self.center, &points[idx]))
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
    }
}

/// K-Means Clustering espacial
pub struct KMeans {
    k: usize,
    max_iterations: usize,
    tolerance: f64,
}

impl KMeans {
    /// Cria um novo K-Means
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iterations: 100,
            tolerance: 1.0, // 1 metro
        }
    }

    /// Define número máximo de iterações
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Define tolerância de convergência
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Executa o clustering
    pub fn fit(&self, points: &[GeoCoord]) -> Vec<Cluster> {
        if points.len() < self.k {
            return Vec::new();
        }

        // Inicializar clusters com K-Means++
        let mut clusters = self.initialize_clusters_plusplus(points);

        for iteration in 0..self.max_iterations {
            // Atribuir pontos aos clusters mais próximos
            for cluster in &mut clusters {
                cluster.members.clear();
            }

            for (idx, point) in points.iter().enumerate() {
                let mut min_dist = f64::INFINITY;
                let mut closest_cluster = 0;

                for (i, cluster) in clusters.iter().enumerate() {
                    let dist = haversine_distance(point, &cluster.center);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_cluster = i;
                    }
                }

                clusters[closest_cluster].add_member(idx);
            }

            // Recalcular centros
            let mut max_movement = 0.0;
            for cluster in &mut clusters {
                let old_center = cluster.center;
                cluster.recalculate_center(points);
                let movement = haversine_distance(&old_center, &cluster.center);
                max_movement = max_movement.max(movement);
            }

            // Verificar convergência
            if max_movement < self.tolerance {
                println!("K-Means convergiu em {} iterações", iteration + 1);
                break;
            }
        }

        clusters
    }

    /// Inicialização K-Means++ (melhor que aleatória)
    fn initialize_clusters_plusplus(&self, points: &[GeoCoord]) -> Vec<Cluster> {
        let mut clusters = Vec::with_capacity(self.k);
        let mut used_indices = HashSet::new();

        // Primeiro centro: aleatório
        let first_idx = 0;
        used_indices.insert(first_idx);
        clusters.push(Cluster::new(0, points[first_idx]));

        // Demais centros: proporcional à distância do centro mais próximo
        for i in 1..self.k {
            let mut distances: Vec<(usize, f64)> = points
                .iter()
                .enumerate()
                .filter(|(idx, _)| !used_indices.contains(idx))
                .map(|(idx, point)| {
                    let min_dist = clusters
                        .iter()
                        .map(|c| haversine_distance(point, &c.center))
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(f64::INFINITY);
                    (idx, min_dist)
                })
                .collect();

            distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            if let Some((idx, _)) = distances.first() {
                used_indices.insert(*idx);
                clusters.push(Cluster::new(i, points[*idx]));
            }
        }

        clusters
    }
}

/// DBSCAN (Density-Based Spatial Clustering)
pub struct DBSCAN {
    epsilon: f64,     // Raio de vizinhança (metros)
    min_points: usize, // Mínimo de pontos para formar cluster
}

impl DBSCAN {
    /// Cria um novo DBSCAN
    pub fn new(epsilon: f64, min_points: usize) -> Self {
        Self {
            epsilon,
            min_points,
        }
    }

    /// Executa o clustering
    pub fn fit(&self, points: &[GeoCoord]) -> Vec<Cluster> {
        let mut labels = vec![-1_i32; points.len()]; // -1 = noise, 0+ = cluster id
        let mut cluster_id = 0;

        for i in 0..points.len() {
            if labels[i] != -1 {
                continue; // Já processado
            }

            let neighbors = self.range_query(points, i);

            if neighbors.len() < self.min_points {
                labels[i] = -2; // Noise
                continue;
            }

            // Iniciar novo cluster
            labels[i] = cluster_id;
            let mut seed_set = neighbors.clone();

            let mut j = 0;
            while j < seed_set.len() {
                let q = seed_set[j];
                j += 1;

                if labels[q] == -2 {
                    labels[q] = cluster_id; // Mudar de noise para border point
                }

                if labels[q] != -1 {
                    continue;
                }

                labels[q] = cluster_id;
                let neighbors_q = self.range_query(points, q);

                if neighbors_q.len() >= self.min_points {
                    seed_set.extend(neighbors_q);
                }
            }

            cluster_id += 1;
        }

        // Converter para Clusters
        let mut clusters_map: HashMap<i32, Cluster> = HashMap::new();

        for (idx, &label) in labels.iter().enumerate() {
            if label >= 0 {
                clusters_map
                    .entry(label)
                    .or_insert_with(|| Cluster::new(label as usize, points[idx]))
                    .add_member(idx);
            }
        }

        let mut clusters: Vec<Cluster> = clusters_map.into_values().collect();

        // Recalcular centros e raios
        for cluster in &mut clusters {
            cluster.recalculate_center(points);
        }

        clusters
    }

    /// Encontra vizinhos dentro do raio epsilon
    fn range_query(&self, points: &[GeoCoord], idx: usize) -> Vec<usize> {
        points
            .iter()
            .enumerate()
            .filter(|(i, point)| {
                *i != idx && haversine_distance(&points[idx], point) <= self.epsilon
            })
            .map(|(i, _)| i)
            .collect()
    }
}

/// Hierarquical Clustering (Agglomerative)
pub struct HierarchicalClustering {
    n_clusters: usize,
    linkage: Linkage,
}

#[derive(Debug, Clone, Copy)]
pub enum Linkage {
    Single,   // Distância mínima
    Complete, // Distância máxima
    Average,  // Distância média
}

impl HierarchicalClustering {
    /// Cria um novo clustering hierárquico
    pub fn new(n_clusters: usize, linkage: Linkage) -> Self {
        Self {
            n_clusters,
            linkage,
        }
    }

    /// Executa o clustering
    pub fn fit(&self, points: &[GeoCoord]) -> Vec<Cluster> {
        let n = points.len();
        if n < self.n_clusters {
            return Vec::new();
        }

        // Iniciar cada ponto como seu próprio cluster
        let mut clusters: Vec<Cluster> = (0..n)
            .map(|i| {
                let mut c = Cluster::new(i, points[i]);
                c.add_member(i);
                c
            })
            .collect();

        // Mesclar clusters até atingir n_clusters
        while clusters.len() > self.n_clusters {
            let (i, j) = self.find_closest_clusters(&clusters, points);

            // Mesclar clusters i e j
            let mut new_cluster = clusters[i].clone();
            new_cluster.members.extend(&clusters[j].members);
            new_cluster.recalculate_center(points);

            // Remover clusters antigos e adicionar novo
            let removed_j = clusters.remove(j);
            let removed_i = clusters.remove(i.min(j.saturating_sub(1)));

            new_cluster.id = clusters.len();
            clusters.push(new_cluster);
        }

        clusters
    }

    /// Encontra par de clusters mais próximo
    fn find_closest_clusters(&self, clusters: &[Cluster], points: &[GeoCoord]) -> (usize, usize) {
        let mut min_dist = f64::INFINITY;
        let mut closest_pair = (0, 1);

        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                let dist = self.cluster_distance(&clusters[i], &clusters[j], points);
                if dist < min_dist {
                    min_dist = dist;
                    closest_pair = (i, j);
                }
            }
        }

        closest_pair
    }

    /// Calcula distância entre clusters baseado na estratégia de linkage
    fn cluster_distance(&self, c1: &Cluster, c2: &Cluster, points: &[GeoCoord]) -> f64 {
        match self.linkage {
            Linkage::Single => {
                // Distância mínima entre qualquer par de pontos
                let mut min_dist = f64::INFINITY;
                for &i in &c1.members {
                    for &j in &c2.members {
                        let dist = haversine_distance(&points[i], &points[j]);
                        min_dist = min_dist.min(dist);
                    }
                }
                min_dist
            }
            Linkage::Complete => {
                // Distância máxima entre qualquer par de pontos
                let mut max_dist = 0.0;
                for &i in &c1.members {
                    for &j in &c2.members {
                        let dist = haversine_distance(&points[i], &points[j]);
                        max_dist = max_dist.max(dist);
                    }
                }
                max_dist
            }
            Linkage::Average => {
                // Distância média entre todos os pares
                let mut sum_dist = 0.0;
                let mut count = 0;
                for &i in &c1.members {
                    for &j in &c2.members {
                        sum_dist += haversine_distance(&points[i], &points[j]);
                        count += 1;
                    }
                }
                if count > 0 {
                    sum_dist / count as f64
                } else {
                    f64::INFINITY
                }
            }
        }
    }
}

/// Métricas de qualidade de clustering
pub struct ClusterMetrics;

impl ClusterMetrics {
    /// Silhouette Score (qualidade do clustering)
    /// Valores próximos de 1 = bom, próximos de 0 = neutro, próximos de -1 = ruim
    pub fn silhouette_score(points: &[GeoCoord], clusters: &[Cluster]) -> f64 {
        let mut scores = Vec::new();

        for cluster in clusters {
            for &i in &cluster.members {
                let a = Self::intra_cluster_distance(&points[i], cluster, points);
                let b = Self::nearest_cluster_distance(&points[i], cluster, clusters, points);

                let s = if a < b {
                    1.0 - a / b
                } else if a > b {
                    b / a - 1.0
                } else {
                    0.0
                };

                scores.push(s);
            }
        }

        if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f64>() / scores.len() as f64
        }
    }

    fn intra_cluster_distance(point: &GeoCoord, cluster: &Cluster, points: &[GeoCoord]) -> f64 {
        if cluster.members.len() <= 1 {
            return 0.0;
        }

        let sum: f64 = cluster
            .members
            .iter()
            .map(|&idx| haversine_distance(point, &points[idx]))
            .sum();

        sum / (cluster.members.len() - 1) as f64
    }

    fn nearest_cluster_distance(
        point: &GeoCoord,
        current_cluster: &Cluster,
        all_clusters: &[Cluster],
        points: &[GeoCoord],
    ) -> f64 {
        all_clusters
            .iter()
            .filter(|c| c.id != current_cluster.id)
            .map(|cluster| {
                let sum: f64 = cluster
                    .members
                    .iter()
                    .map(|&idx| haversine_distance(point, &points[idx]))
                    .sum();
                sum / cluster.members.len() as f64
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f64::INFINITY)
    }

    /// Davies-Bouldin Index (quanto menor, melhor)
    pub fn davies_bouldin_index(points: &[GeoCoord], clusters: &[Cluster]) -> f64 {
        let n = clusters.len();
        if n <= 1 {
            return 0.0;
        }

        let mut sum = 0.0;

        for i in 0..n {
            let mut max_ratio = 0.0;

            for j in 0..n {
                if i == j {
                    continue;
                }

                let s_i = Self::cluster_scatter(&clusters[i], points);
                let s_j = Self::cluster_scatter(&clusters[j], points);
                let d_ij = haversine_distance(&clusters[i].center, &clusters[j].center);

                let ratio = (s_i + s_j) / d_ij;
                max_ratio = max_ratio.max(ratio);
            }

            sum += max_ratio;
        }

        sum / n as f64
    }

    fn cluster_scatter(cluster: &Cluster, points: &[GeoCoord]) -> f64 {
        if cluster.members.is_empty() {
            return 0.0;
        }

        let sum: f64 = cluster
            .members
            .iter()
            .map(|&idx| haversine_distance(&points[idx], &cluster.center))
            .sum();

        sum / cluster.members.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans_clustering() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.01, 0.01),
            GeoCoord::new(1.0, 1.0),
            GeoCoord::new(1.01, 1.01),
        ];

        let kmeans = KMeans::new(2);
        let clusters = kmeans.fit(&points);

        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test_dbscan_clustering() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.001, 0.001),
            GeoCoord::new(0.002, 0.002),
            GeoCoord::new(1.0, 1.0),
        ];

        let dbscan = DBSCAN::new(500.0, 2); // 500m, min 2 pontos
        let clusters = dbscan.fit(&points);

        assert!(!clusters.is_empty());
    }
}
