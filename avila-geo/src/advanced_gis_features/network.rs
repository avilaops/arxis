//! Network Analysis - Shortest Path, Dijkstra, A*
//!
//! Análise de redes: caminhos mais curtos, isócronas, áreas de serviço.
//! Similar ao ArcGIS Network Analyst.

use crate::coords::GeoCoord;
use crate::calc::haversine_distance;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// Grafo de rede geográfica
#[derive(Debug, Clone)]
pub struct NetworkGraph {
    nodes: Vec<NetworkNode>,
    edges: Vec<NetworkEdge>,
    adjacency: HashMap<usize, Vec<usize>>, // node_id -> edge_ids
}

#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub id: usize,
    pub coord: GeoCoord,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetworkEdge {
    pub id: usize,
    pub from: usize,
    pub to: usize,
    pub cost: f64, // em metros ou segundos
    pub speed_limit: Option<f64>, // km/h
    pub one_way: bool,
}

/// Resultado de rota
#[derive(Debug, Clone)]
pub struct Route {
    pub path: Vec<usize>,
    pub total_cost: f64,
    pub distance_meters: f64,
    pub duration_seconds: f64,
}

impl NetworkGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, coord: GeoCoord, name: Option<String>) -> usize {
        let id = self.nodes.len();
        self.nodes.push(NetworkNode { id, coord, name });
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize, speed_limit: Option<f64>, one_way: bool) {
        let from_coord = self.nodes[from].coord;
        let to_coord = self.nodes[to].coord;
        let distance = haversine_distance(&from_coord, &to_coord);

        let cost = if let Some(speed) = speed_limit {
            (distance / 1000.0) / speed * 3600.0 // seconds
        } else {
            distance // meters
        };

        let id = self.edges.len();
        self.edges.push(NetworkEdge {
            id,
            from,
            to,
            cost,
            speed_limit,
            one_way,
        });

        self.adjacency.entry(from).or_default().push(id);

        if !one_way {
            let reverse_id = self.edges.len();
            self.edges.push(NetworkEdge {
                id: reverse_id,
                from: to,
                to: from,
                cost,
                speed_limit,
                one_way: false,
            });
            self.adjacency.entry(to).or_default().push(reverse_id);
        }
    }

    /// Dijkstra: caminho mais curto
    pub fn shortest_path(&self, start: usize, end: usize) -> Option<Route> {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0.0);
        heap.push(State { node: start, cost: 0.0 });

        while let Some(State { node, cost }) = heap.pop() {
            if node == end {
                return Some(self.reconstruct_path(start, end, &previous, cost));
            }

            if cost > *distances.get(&node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            if let Some(edge_ids) = self.adjacency.get(&node) {
                for &edge_id in edge_ids {
                    let edge = &self.edges[edge_id];
                    let next_node = edge.to;
                    let next_cost = cost + edge.cost;

                    if next_cost < *distances.get(&next_node).unwrap_or(&f64::INFINITY) {
                        distances.insert(next_node, next_cost);
                        previous.insert(next_node, node);
                        heap.push(State { node: next_node, cost: next_cost });
                    }
                }
            }
        }

        None
    }

    /// A* pathfinding (heurística com distância geodésica)
    pub fn astar_path(&self, start: usize, end: usize) -> Option<Route> {
        let end_coord = self.nodes[end].coord;
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0.0);
        let h = haversine_distance(&self.nodes[start].coord, &end_coord);
        heap.push(State { node: start, cost: h });

        while let Some(State { node, cost: _ }) = heap.pop() {
            if node == end {
                let final_cost = *distances.get(&end).unwrap();
                return Some(self.reconstruct_path(start, end, &previous, final_cost));
            }

            let current_dist = *distances.get(&node).unwrap_or(&f64::INFINITY);

            if let Some(edge_ids) = self.adjacency.get(&node) {
                for &edge_id in edge_ids {
                    let edge = &self.edges[edge_id];
                    let next_node = edge.to;
                    let next_dist = current_dist + edge.cost;

                    if next_dist < *distances.get(&next_node).unwrap_or(&f64::INFINITY) {
                        distances.insert(next_node, next_dist);
                        previous.insert(next_node, node);

                        let h = haversine_distance(&self.nodes[next_node].coord, &end_coord);
                        heap.push(State {
                            node: next_node,
                            cost: next_dist + h,
                        });
                    }
                }
            }
        }

        None
    }

    /// Área de serviço (isócrona) - todos os nós alcançáveis em X minutos/metros
    pub fn service_area(&self, start: usize, max_cost: f64) -> Vec<usize> {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut reachable = Vec::new();

        distances.insert(start, 0.0);
        heap.push(State { node: start, cost: 0.0 });

        while let Some(State { node, cost }) = heap.pop() {
            if cost > max_cost {
                continue;
            }

            reachable.push(node);

            if let Some(edge_ids) = self.adjacency.get(&node) {
                for &edge_id in edge_ids {
                    let edge = &self.edges[edge_id];
                    let next_node = edge.to;
                    let next_cost = cost + edge.cost;

                    if next_cost <= max_cost && next_cost < *distances.get(&next_node).unwrap_or(&f64::INFINITY) {
                        distances.insert(next_node, next_cost);
                        heap.push(State { node: next_node, cost: next_cost });
                    }
                }
            }
        }

        reachable
    }

    fn reconstruct_path(&self, start: usize, end: usize, previous: &HashMap<usize, usize>, total_cost: f64) -> Route {
        let mut path = Vec::new();
        let mut current = end;

        while current != start {
            path.push(current);
            if let Some(&prev) = previous.get(&current) {
                current = prev;
            } else {
                break;
            }
        }
        path.push(start);
        path.reverse();

        let mut distance = 0.0;
        for window in path.windows(2) {
            let from_coord = self.nodes[window[0]].coord;
            let to_coord = self.nodes[window[1]].coord;
            distance += haversine_distance(&from_coord, &to_coord);
        }

        Route {
            path,
            total_cost,
            distance_meters: distance,
            duration_seconds: total_cost,
        }
    }
}

impl Default for NetworkGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
struct State {
    node: usize,
    cost: f64,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let mut graph = NetworkGraph::new();

        let n0 = graph.add_node(GeoCoord::new(0.0, 0.0), None);
        let n1 = graph.add_node(GeoCoord::new(0.1, 0.0), None);
        let n2 = graph.add_node(GeoCoord::new(0.1, 0.1), None);

        graph.add_edge(n0, n1, Some(50.0), false);
        graph.add_edge(n1, n2, Some(50.0), false);

        let route = graph.shortest_path(n0, n2).unwrap();
        assert_eq!(route.path.len(), 3);
        assert_eq!(route.path[0], n0);
        assert_eq!(route.path[2], n2);
    }

    #[test]
    fn test_service_area() {
        let mut graph = NetworkGraph::new();

        let n0 = graph.add_node(GeoCoord::new(0.0, 0.0), None);
        let n1 = graph.add_node(GeoCoord::new(0.01, 0.0), None);
        let n2 = graph.add_node(GeoCoord::new(0.02, 0.0), None);

        graph.add_edge(n0, n1, Some(50.0), false);
        graph.add_edge(n1, n2, Some(50.0), false);

        let area = graph.service_area(n0, 10000.0);
        assert!(area.contains(&n0));
        assert!(area.contains(&n1));
    }
}

