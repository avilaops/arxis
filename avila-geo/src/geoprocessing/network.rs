//! Análise de redes espaciais (Network Analysis)
//!
//! Este módulo implementa:
//! - Estruturas de rede espacial (nós e arestas)
//! - Algoritmos de roteamento (Dijkstra, A*)
//! - Análise de conectividade
//! - Análise de fluxo de rede

use crate::coords::GeoCoord;
use crate::geoprocessing::analysis::haversine_distance;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

/// Nó em uma rede espacial
#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub id: usize,
    pub coord: GeoCoord,
    pub properties: HashMap<String, String>,
}

impl NetworkNode {
    /// Cria um novo nó
    pub fn new(id: usize, coord: GeoCoord) -> Self {
        Self {
            id,
            coord,
            properties: HashMap::new(),
        }
    }

    /// Adiciona uma propriedade ao nó
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Aresta em uma rede espacial
#[derive(Debug, Clone)]
pub struct NetworkEdge {
    pub id: usize,
    pub from: usize,
    pub to: usize,
    pub weight: f64,
    pub geometry: Vec<GeoCoord>,
    pub properties: HashMap<String, String>,
}

impl NetworkEdge {
    /// Cria uma nova aresta
    pub fn new(id: usize, from: usize, to: usize, weight: f64) -> Self {
        Self {
            id,
            from,
            to,
            weight,
            geometry: Vec::new(),
            properties: HashMap::new(),
        }
    }

    /// Define a geometria da aresta
    pub fn with_geometry(mut self, geometry: Vec<GeoCoord>) -> Self {
        self.geometry = geometry;
        self
    }

    /// Adiciona uma propriedade à aresta
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Rede espacial para análise de roteamento e conectividade
#[derive(Debug, Clone)]
pub struct SpatialNetwork {
    pub nodes: Vec<NetworkNode>,
    pub edges: Vec<NetworkEdge>,
    pub adjacency: HashMap<usize, Vec<usize>>, // node_id -> edge_ids
    node_index: HashMap<usize, usize>,         // node_id -> index in nodes vec
}

impl SpatialNetwork {
    /// Cria uma nova rede espacial vazia
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
            node_index: HashMap::new(),
        }
    }

    /// Adiciona um nó à rede
    pub fn add_node(&mut self, node: NetworkNode) {
        let idx = self.nodes.len();
        self.node_index.insert(node.id, idx);
        self.nodes.push(node);
    }

    /// Adiciona uma aresta à rede
    pub fn add_edge(&mut self, edge: NetworkEdge) {
        let edge_id = self.edges.len();
        self.edges.push(edge.clone());

        self.adjacency
            .entry(edge.from)
            .or_insert_with(Vec::new)
            .push(edge_id);
    }

    /// Encontra o caminho mais curto usando Dijkstra
    pub fn shortest_path(&self, start: usize, end: usize) -> Option<Path> {
        if !self.node_index.contains_key(&start) || !self.node_index.contains_key(&end) {
            return None;
        }

        let mut distances: HashMap<usize, f64> = HashMap::new();
        let mut previous: HashMap<usize, (usize, usize)> = HashMap::new(); // node_id -> (prev_node, edge_id)
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0.0);
        heap.push(State {
            cost: 0.0,
            node: start,
        });

        while let Some(State { cost, node }) = heap.pop() {
            if node == end {
                break;
            }

            if cost > *distances.get(&node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            if let Some(edge_ids) = self.adjacency.get(&node) {
                for &edge_id in edge_ids {
                    let edge = &self.edges[edge_id];
                    let next_node = edge.to;
                    let next_cost = cost + edge.weight;

                    if next_cost < *distances.get(&next_node).unwrap_or(&f64::INFINITY) {
                        distances.insert(next_node, next_cost);
                        previous.insert(next_node, (node, edge_id));
                        heap.push(State {
                            cost: next_cost,
                            node: next_node,
                        });
                    }
                }
            }
        }

        // Reconstruir caminho
        let mut path_nodes = Vec::new();
        let mut path_edges = Vec::new();
        let mut current = end;

        while let Some((prev, edge_id)) = previous.get(&current) {
            path_nodes.push(current);
            path_edges.push(*edge_id);
            current = *prev;
        }
        path_nodes.push(start);

        path_nodes.reverse();
        path_edges.reverse();

        let total_cost = *distances.get(&end)?;

        Some(Path {
            nodes: path_nodes,
            edges: path_edges,
            total_cost,
        })
    }

    /// Encontra o caminho mais curto usando A* (com heurística)
    pub fn shortest_path_astar(&self, start: usize, end: usize) -> Option<Path> {
        let start_idx = *self.node_index.get(&start)?;
        let end_idx = *self.node_index.get(&end)?;
        let end_coord = &self.nodes[end_idx].coord;

        let mut distances: HashMap<usize, f64> = HashMap::new();
        let mut previous: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0.0);
        let start_h = haversine_distance(&self.nodes[start_idx].coord, end_coord);
        heap.push(State {
            cost: start_h,
            node: start,
        });

        while let Some(State { cost: _, node }) = heap.pop() {
            if node == end {
                break;
            }

            let current_dist = *distances.get(&node).unwrap_or(&f64::INFINITY);

            if let Some(edge_ids) = self.adjacency.get(&node) {
                for &edge_id in edge_ids {
                    let edge = &self.edges[edge_id];
                    let next_node = edge.to;
                    let next_dist = current_dist + edge.weight;

                    if next_dist < *distances.get(&next_node).unwrap_or(&f64::INFINITY) {
                        distances.insert(next_node, next_dist);
                        previous.insert(next_node, (node, edge_id));

                        let next_idx = *self.node_index.get(&next_node)?;
                        let h = haversine_distance(&self.nodes[next_idx].coord, end_coord);

                        heap.push(State {
                            cost: next_dist + h,
                            node: next_node,
                        });
                    }
                }
            }
        }

        // Reconstruir caminho
        let mut path_nodes = Vec::new();
        let mut path_edges = Vec::new();
        let mut current = end;

        while let Some((prev, edge_id)) = previous.get(&current) {
            path_nodes.push(current);
            path_edges.push(*edge_id);
            current = *prev;
        }
        path_nodes.push(start);

        path_nodes.reverse();
        path_edges.reverse();

        let total_cost = *distances.get(&end)?;

        Some(Path {
            nodes: path_nodes,
            edges: path_edges,
            total_cost,
        })
    }

    /// Encontra todos os nós dentro de um raio
    pub fn nodes_within_radius(&self, center: &GeoCoord, radius: f64) -> Vec<usize> {
        self.nodes
            .iter()
            .filter(|node| haversine_distance(&node.coord, center) <= radius)
            .map(|node| node.id)
            .collect()
    }

    /// Calcula a centralidade de grau (degree centrality)
    pub fn degree_centrality(&self) -> HashMap<usize, f64> {
        let mut centrality = HashMap::new();
        let n = self.nodes.len() as f64;

        if n <= 1.0 {
            return centrality;
        }

        for node in &self.nodes {
            let degree = self.adjacency.get(&node.id).map(|e| e.len()).unwrap_or(0) as f64;
            centrality.insert(node.id, degree / (n - 1.0));
        }

        centrality
    }

    /// Calcula a centralidade de intermediação (betweenness centrality) - simplificada
    pub fn betweenness_centrality(&self) -> HashMap<usize, f64> {
        let mut centrality: HashMap<usize, f64> = self.nodes.iter().map(|n| (n.id, 0.0)).collect();
        let n = self.nodes.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let start = self.nodes[i].id;
                let end = self.nodes[j].id;

                if let Some(path) = self.shortest_path(start, end) {
                    for &node_id in &path.nodes[1..path.nodes.len() - 1] {
                        *centrality.get_mut(&node_id).unwrap() += 1.0;
                    }
                }
            }
        }

        // Normalizar
        let normalizer = if n > 2 {
            ((n - 1) * (n - 2)) as f64 / 2.0
        } else {
            1.0
        };

        for value in centrality.values_mut() {
            *value /= normalizer;
        }

        centrality
    }

    /// Detecta componentes conectados
    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in &self.nodes {
            if !visited.contains(&node.id) {
                let mut component = Vec::new();
                let mut queue = VecDeque::new();
                queue.push_back(node.id);
                visited.insert(node.id);

                while let Some(current) = queue.pop_front() {
                    component.push(current);

                    if let Some(edge_ids) = self.adjacency.get(&current) {
                        for &edge_id in edge_ids {
                            let next = self.edges[edge_id].to;
                            if !visited.contains(&next) {
                                visited.insert(next);
                                queue.push_back(next);
                            }
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }

    /// Calcula a árvore geradora mínima (Minimum Spanning Tree) usando Prim
    pub fn minimum_spanning_tree(&self) -> Vec<usize> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let mut in_mst = HashSet::new();
        let mut mst_edges = Vec::new();
        let mut heap = BinaryHeap::new();

        // Começar do primeiro nó
        let start = self.nodes[0].id;
        in_mst.insert(start);

        if let Some(edge_ids) = self.adjacency.get(&start) {
            for &edge_id in edge_ids {
                let edge = &self.edges[edge_id];
                heap.push(MSTState {
                    cost: -edge.weight, // Negativo para min-heap
                    edge_id,
                });
            }
        }

        while let Some(MSTState { cost: _, edge_id }) = heap.pop() {
            let edge = &self.edges[edge_id];

            if in_mst.contains(&edge.to) {
                continue;
            }

            in_mst.insert(edge.to);
            mst_edges.push(edge_id);

            if let Some(next_edge_ids) = self.adjacency.get(&edge.to) {
                for &next_edge_id in next_edge_ids {
                    let next_edge = &self.edges[next_edge_id];
                    if !in_mst.contains(&next_edge.to) {
                        heap.push(MSTState {
                            cost: -next_edge.weight,
                            edge_id: next_edge_id,
                        });
                    }
                }
            }
        }

        mst_edges
    }

    /// Obtém nó por ID
    pub fn get_node(&self, id: usize) -> Option<&NetworkNode> {
        let idx = *self.node_index.get(&id)?;
        self.nodes.get(idx)
    }

    /// Obtém aresta por índice
    pub fn get_edge(&self, from: usize, to: usize) -> Option<&NetworkEdge> {
        let edge_ids = self.adjacency.get(&from)?;
        edge_ids
            .iter()
            .find_map(|&edge_id| {
                let edge = &self.edges[edge_id];
                if edge.to == to {
                    Some(edge)
                } else {
                    None
                }
            })
    }
}

impl Default for SpatialNetwork {
    fn default() -> Self {
        Self::new()
    }
}

/// Caminho em uma rede espacial
#[derive(Debug, Clone)]
pub struct Path {
    pub nodes: Vec<usize>,
    pub edges: Vec<usize>,
    pub total_cost: f64,
}

impl Path {
    /// Retorna o número de segmentos no caminho
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Verifica se o caminho está vazio
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Retorna a geometria completa do caminho
    pub fn geometry(&self, network: &SpatialNetwork) -> Vec<GeoCoord> {
        let mut coords = Vec::new();

        for &edge_id in &self.edges {
            if let Some(edge) = network.edges.get(edge_id) {
                if edge.geometry.is_empty() {
                    // Usar nós se não há geometria
                    if let Some(from_idx) = network.node_index.get(&edge.from) {
                        coords.push(network.nodes[*from_idx].coord);
                    }
                } else {
                    coords.extend(&edge.geometry);
                }
            }
        }

        // Adicionar último nó
        if let Some(&last_edge_id) = self.edges.last() {
            if let Some(edge) = network.edges.get(last_edge_id) {
                if let Some(to_idx) = network.node_index.get(&edge.to) {
                    coords.push(network.nodes[*to_idx].coord);
                }
            }
        }

        coords
    }
}

/// Estado para busca de caminho (Dijkstra/A*)
#[derive(Debug, Clone)]
struct State {
    cost: f64,
    node: usize,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Estado para MST
#[derive(Debug, Clone)]
struct MSTState {
    cost: f64,
    edge_id: usize,
}

impl Eq for MSTState {}

impl PartialEq for MSTState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for MSTState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for MSTState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let mut network = SpatialNetwork::new();

        // Criar rede simples
        network.add_node(NetworkNode::new(0, GeoCoord::new(0.0, 0.0)));
        network.add_node(NetworkNode::new(1, GeoCoord::new(1.0, 0.0)));
        network.add_node(NetworkNode::new(2, GeoCoord::new(2.0, 0.0)));

        network.add_edge(NetworkEdge::new(0, 0, 1, 1.0));
        network.add_edge(NetworkEdge::new(1, 1, 2, 1.0));
        network.add_edge(NetworkEdge::new(2, 0, 2, 3.0));

        let path = network.shortest_path(0, 2).unwrap();

        assert_eq!(path.total_cost, 2.0);
        assert_eq!(path.nodes, vec![0, 1, 2]);
    }

    #[test]
    fn test_connected_components() {
        let mut network = SpatialNetwork::new();

        network.add_node(NetworkNode::new(0, GeoCoord::new(0.0, 0.0)));
        network.add_node(NetworkNode::new(1, GeoCoord::new(1.0, 0.0)));
        network.add_node(NetworkNode::new(2, GeoCoord::new(2.0, 0.0)));

        network.add_edge(NetworkEdge::new(0, 0, 1, 1.0));

        let components = network.connected_components();
        assert_eq!(components.len(), 2);
    }
}
