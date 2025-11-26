//! Network analysis and routing algorithms

use crate::error::{GeoError, Result};
use geo::Coord;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Graph edge with weight (distance or time)
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

/// Network graph for routing
#[derive(Debug, Clone)]
pub struct Network {
    pub nodes: Vec<Coord<f64>>,
    edges: Vec<Vec<(usize, f64)>>, // adjacency list
}

#[derive(Debug, Clone)]
struct DijkstraState {
    node: usize,
    cost: f64,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for DijkstraState {}
impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && (self.cost - other.cost).abs() < 1e-9
    }
}

impl Network {
    pub fn new(nodes: Vec<Coord<f64>>) -> Self {
        let n = nodes.len();
        Self {
            nodes,
            edges: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.edges[from].push((to, weight));
    }

    pub fn add_bidirectional_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.add_edge(from, to, weight);
        self.add_edge(to, from, weight);
    }

    /// Dijkstra's shortest path algorithm
    pub fn shortest_path(&self, start: usize, end: usize) -> Result<(Vec<usize>, f64)> {
        if start >= self.nodes.len() || end >= self.nodes.len() {
            return Err(GeoError::InvalidParameter("Invalid node index".to_string()));
        }

        let mut dist = vec![f64::INFINITY; self.nodes.len()];
        let mut prev = vec![None; self.nodes.len()];
        let mut heap = BinaryHeap::new();

        dist[start] = 0.0;
        heap.push(DijkstraState { node: start, cost: 0.0 });

        while let Some(DijkstraState { node, cost }) = heap.pop() {
            if node == end {
                break;
            }

            if cost > dist[node] {
                continue;
            }

            for &(neighbor, edge_cost) in &self.edges[node] {
                let new_cost = cost + edge_cost;
                if new_cost < dist[neighbor] {
                    dist[neighbor] = new_cost;
                    prev[neighbor] = Some(node);
                    heap.push(DijkstraState { node: neighbor, cost: new_cost });
                }
            }
        }

        if dist[end] == f64::INFINITY {
            return Err(GeoError::InvalidNetwork("No path found".to_string()));
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = end;
        while let Some(p) = prev[current] {
            path.push(current);
            current = p;
        }
        path.push(start);
        path.reverse();

        Ok((path, dist[end]))
    }
}

// Placeholder modules for future implementation
pub mod terrain {}
pub mod clustering {}
