//! Graph Clustering and Community Detection
//!
//! Algorithms for detecting communities and clusters in graph/network data.
//! Applications: social networks, biological networks, recommendation systems.

use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2};
use std::collections::{HashMap, HashSet, VecDeque};

/// Graph structure for community detection
pub struct Graph {
    pub n_nodes: usize,
    pub edges: Vec<(usize, usize, f64)>, // (from, to, weight)
    adj_list: HashMap<usize, Vec<(usize, f64)>>,
}

impl Graph {
    pub fn new(n_nodes: usize) -> Self {
        Self {
            n_nodes,
            edges: Vec::new(),
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.edges.push((from, to, weight));
        self.adj_list.entry(from).or_insert_with(Vec::new).push((to, weight));
        self.adj_list.entry(to).or_insert_with(Vec::new).push((from, weight));
    }

    pub fn from_adjacency_matrix(adj: &Array2<f64>) -> Self {
        let n = adj.nrows();
        let mut graph = Self::new(n);

        for i in 0..n {
            for j in (i + 1)..n {
                if adj[[i, j]] > 0.0 {
                    graph.add_edge(i, j, adj[[i, j]]);
                }
            }
        }

        graph
    }

    pub fn neighbors(&self, node: usize) -> &[(usize, f64)] {
        self.adj_list.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn degree(&self, node: usize) -> usize {
        self.neighbors(node).len()
    }

    pub fn total_weight(&self) -> f64 {
        self.edges.iter().map(|(_, _, w)| w).sum()
    }
}

/// Louvain method for community detection (modularity optimization)
pub struct LouvainClustering {
    resolution: f64,
    max_iterations: usize,
}

impl LouvainClustering {
    pub fn new() -> Self {
        Self {
            resolution: 1.0,
            max_iterations: 100,
        }
    }

    pub fn resolution(mut self, r: f64) -> Self {
        self.resolution = r;
        self
    }

    pub fn fit(&self, graph: &Graph) -> Result<CommunityResult> {
        let mut communities = (0..graph.n_nodes).collect::<Vec<_>>();
        let mut improved = true;
        let mut iteration = 0;

        let total_weight = graph.total_weight();

        while improved && iteration < self.max_iterations {
            improved = false;
            iteration += 1;

            for node in 0..graph.n_nodes {
                let current_community = communities[node];
                let mut best_community = current_community;
                let mut best_gain = 0.0;

                // Try moving node to each neighbor's community
                let mut tested_communities = HashSet::new();
                tested_communities.insert(current_community);

                for &(neighbor, _) in graph.neighbors(node) {
                    let neighbor_community = communities[neighbor];

                    if tested_communities.contains(&neighbor_community) {
                        continue;
                    }
                    tested_communities.insert(neighbor_community);

                    // Calculate modularity gain
                    let gain = self.modularity_gain(
                        node,
                        current_community,
                        neighbor_community,
                        &communities,
                        graph,
                        total_weight,
                    );

                    if gain > best_gain {
                        best_gain = gain;
                        best_community = neighbor_community;
                    }
                }

                // Move node if improvement found
                if best_community != current_community && best_gain > 1e-10 {
                    communities[node] = best_community;
                    improved = true;
                }
            }
        }

        // Renumber communities to be contiguous
        let unique_communities: HashSet<_> = communities.iter().cloned().collect();
        let community_map: HashMap<_, _> = unique_communities
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect();

        let labels: Vec<usize> = communities.iter().map(|&c| community_map[&c]).collect();

        let modularity = self.calculate_modularity(&labels, graph, total_weight);

        Ok(CommunityResult {
            labels,
            n_communities: unique_communities.len(),
            modularity,
        })
    }

    fn modularity_gain(
        &self,
        node: usize,
        from_community: usize,
        to_community: usize,
        communities: &[usize],
        graph: &Graph,
        total_weight: f64,
    ) -> f64 {
        let mut ki_in_to = 0.0;  // edges from node to 'to' community
        let mut ki_in_from = 0.0; // edges from node to 'from' community
        let ki = graph.neighbors(node).iter().map(|(_, w)| w).sum::<f64>();

        for &(neighbor, weight) in graph.neighbors(node) {
            if communities[neighbor] == to_community {
                ki_in_to += weight;
            }
            if communities[neighbor] == from_community {
                ki_in_from += weight;
            }
        }

        // Simplified modularity gain calculation
        let m2 = 2.0 * total_weight;
        let delta_q = (ki_in_to - ki_in_from) / m2 - self.resolution * ki * ki / (m2 * m2);

        delta_q
    }

    fn calculate_modularity(&self, labels: &[usize], graph: &Graph, total_weight: f64) -> f64 {
        if total_weight == 0.0 {
            return 0.0;
        }

        let mut modularity = 0.0;
        let m2 = 2.0 * total_weight;

        for i in 0..graph.n_nodes {
            for &(j, weight) in graph.neighbors(i) {
                if labels[i] == labels[j] {
                    let ki = graph.degree(i) as f64;
                    let kj = graph.degree(j) as f64;
                    modularity += weight - (ki * kj) / m2;
                }
            }
        }

        modularity / m2
    }
}

impl Default for LouvainClustering {
    fn default() -> Self {
        Self::new()
    }
}

/// Label Propagation for fast community detection
pub struct LabelPropagation {
    max_iterations: usize,
}

impl LabelPropagation {
    pub fn new() -> Self {
        Self {
            max_iterations: 100,
        }
    }

    pub fn fit(&self, graph: &Graph) -> Result<CommunityResult> {
        // Initialize: each node in its own community
        let mut labels = (0..graph.n_nodes).collect::<Vec<_>>();

        for _iter in 0..self.max_iterations {
            let old_labels = labels.clone();
            let mut changed = false;

            // Random node order
            let mut node_order: Vec<usize> = (0..graph.n_nodes).collect();
            use rand::seq::SliceRandom;
            node_order.shuffle(&mut rand::thread_rng());

            for &node in &node_order {
                // Count neighbor labels (weighted)
                let mut label_weights: HashMap<usize, f64> = HashMap::new();

                for &(neighbor, weight) in graph.neighbors(node) {
                    *label_weights.entry(labels[neighbor]).or_insert(0.0) += weight;
                }

                // Choose most frequent label
                if let Some((&best_label, _)) = label_weights.iter().max_by(|a, b| {
                    a.1.partial_cmp(b.1).unwrap()
                }) {
                    if labels[node] != best_label {
                        labels[node] = best_label;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        // Renumber communities
        let unique_labels: HashSet<_> = labels.iter().cloned().collect();
        let label_map: HashMap<_, _> = unique_labels
            .iter()
            .enumerate()
            .map(|(i, &l)| (l, i))
            .collect();

        let final_labels: Vec<usize> = labels.iter().map(|&l| label_map[&l]).collect();

        Ok(CommunityResult {
            labels: final_labels,
            n_communities: unique_labels.len(),
            modularity: 0.0, // Calculate if needed
        })
    }
}

impl Default for LabelPropagation {
    fn default() -> Self {
        Self::new()
    }
}

/// Connected components (simple but useful baseline)
pub fn connected_components(graph: &Graph) -> CommunityResult {
    let mut labels = vec![usize::MAX; graph.n_nodes];
    let mut component_id = 0;

    for start_node in 0..graph.n_nodes {
        if labels[start_node] != usize::MAX {
            continue;
        }

        // BFS to find connected component
        let mut queue = VecDeque::new();
        queue.push_back(start_node);
        labels[start_node] = component_id;

        while let Some(node) = queue.pop_front() {
            for &(neighbor, _) in graph.neighbors(node) {
                if labels[neighbor] == usize::MAX {
                    labels[neighbor] = component_id;
                    queue.push_back(neighbor);
                }
            }
        }

        component_id += 1;
    }

    CommunityResult {
        labels,
        n_communities: component_id,
        modularity: 0.0,
    }
}

pub struct CommunityResult {
    pub labels: Vec<usize>,
    pub n_communities: usize,
    pub modularity: f64,
}

impl CommunityResult {
    /// Get nodes in a specific community
    pub fn get_community(&self, community_id: usize) -> Vec<usize> {
        self.labels
            .iter()
            .enumerate()
            .filter_map(|(node, &label)| {
                if label == community_id {
                    Some(node)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get community sizes
    pub fn community_sizes(&self) -> Vec<usize> {
        let mut sizes = vec![0; self.n_communities];
        for &label in &self.labels {
            sizes[label] += 1;
        }
        sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(2, 3, 1.0);

        assert_eq!(graph.degree(1), 2);
        assert_eq!(graph.degree(0), 1);
    }

    #[test]
    fn test_label_propagation() {
        let mut graph = Graph::new(6);
        // Two clear communities: {0,1,2} and {3,4,5}
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(0, 2, 1.0);

        graph.add_edge(3, 4, 1.0);
        graph.add_edge(4, 5, 1.0);
        graph.add_edge(3, 5, 1.0);

        let result = LabelPropagation::new().fit(&graph).unwrap();

        assert!(result.n_communities >= 2);
    }
}
