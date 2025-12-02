//! Network mapper
//!
//! Map hidden service network, analyze connections, graph topology

use std::collections::{BTreeMap, BTreeSet};

/// Network graph of hidden services
#[derive(Debug)]
pub struct NetworkGraph {
    pub nodes: BTreeMap<String, Node>,     // onion -> node
    pub edges: Vec<Edge>,                  // connections
}

#[derive(Debug, Clone)]
pub struct Node {
    pub onion_address: String,
    pub node_type: NodeType,
    pub connections: usize,                // Number of edges
    pub pagerank: f64,                     // PageRank score
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Marketplace,
    Forum,
    Wiki,
    Blog,
    Service,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: f64,                       // Link weight
}

impl NetworkGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: Vec::new(),
        }
    }

    /// Add node to graph
    pub fn add_node(&mut self, onion: String, node_type: NodeType) {
        let node = Node {
            onion_address: onion.clone(),
            node_type,
            connections: 0,
            pagerank: 1.0,
        };

        self.nodes.insert(onion, node);
    }

    /// Add edge (link) between nodes
    pub fn add_edge(&mut self, from: String, to: String) {
        // Ensure nodes exist
        if !self.nodes.contains_key(&from) {
            self.add_node(from.clone(), NodeType::Unknown);
        }
        if !self.nodes.contains_key(&to) {
            self.add_node(to.clone(), NodeType::Unknown);
        }

        // Add edge
        self.edges.push(Edge {
            from: from.clone(),
            to: to.clone(),
            weight: 1.0,
        });

        // Update connection counts
        if let Some(node) = self.nodes.get_mut(&from) {
            node.connections += 1;
        }
        if let Some(node) = self.nodes.get_mut(&to) {
            node.connections += 1;
        }
    }

    /// Calculate PageRank (simplified)
    pub fn calculate_pagerank(&mut self, iterations: usize) {
        let damping = 0.85;
        let n = self.nodes.len() as f64;

        if n == 0.0 {
            return;
        }

        // Initialize PageRank
        for node in self.nodes.values_mut() {
            node.pagerank = 1.0 / n;
        }

        // Iterate
        for _ in 0..iterations {
            let mut new_ranks = BTreeMap::new();

            // Calculate new ranks
            for (addr, node) in &self.nodes {
                let mut rank = (1.0 - damping) / n;

                // Sum contributions from incoming links
                for edge in &self.edges {
                    if edge.to == *addr {
                        if let Some(from_node) = self.nodes.get(&edge.from) {
                            let outbound = self.edges.iter()
                                .filter(|e| e.from == edge.from)
                                .count() as f64;

                            if outbound > 0.0 {
                                rank += damping * (from_node.pagerank / outbound);
                            }
                        }
                    }
                }

                new_ranks.insert(addr.clone(), rank);
            }

            // Update ranks
            for (addr, rank) in new_ranks {
                if let Some(node) = self.nodes.get_mut(&addr) {
                    node.pagerank = rank;
                }
            }
        }
    }

    /// Get top nodes by PageRank
    pub fn top_nodes(&self, n: usize) -> Vec<Node> {
        let mut nodes: Vec<Node> = self.nodes.values().cloned().collect();
        nodes.sort_by(|a, b| b.pagerank.partial_cmp(&a.pagerank).unwrap());
        nodes.truncate(n);
        nodes
    }

    /// Find communities (simplified clustering)
    pub fn find_communities(&self) -> Vec<Community> {
        let mut communities = Vec::new();
        let mut visited = BTreeSet::new();

        for addr in self.nodes.keys() {
            if visited.contains(addr) {
                continue;
            }

            // BFS to find connected component
            let mut community = Vec::new();
            let mut queue = vec![addr.clone()];

            while let Some(current) = queue.pop() {
                if visited.contains(&current) {
                    continue;
                }

                visited.insert(current.clone());
                community.push(current.clone());

                // Add neighbors
                for edge in &self.edges {
                    if edge.from == current && !visited.contains(&edge.to) {
                        queue.push(edge.to.clone());
                    }
                    if edge.to == current && !visited.contains(&edge.from) {
                        queue.push(edge.from.clone());
                    }
                }
            }

            if !community.is_empty() {
                communities.push(Community {
                    members: community,
                    size: communities.len(),
                });
            }
        }

        communities
    }

    /// Network statistics
    pub fn stats(&self) -> NetworkStats {
        let total_edges = self.edges.len();
        let total_nodes = self.nodes.len();

        let avg_connections = if total_nodes > 0 {
            total_edges as f64 / total_nodes as f64
        } else {
            0.0
        };

        let density = if total_nodes > 1 {
            total_edges as f64 / (total_nodes * (total_nodes - 1) / 2) as f64
        } else {
            0.0
        };

        NetworkStats {
            nodes: total_nodes,
            edges: total_edges,
            avg_connections,
            density,
        }
    }
}

#[derive(Debug)]
pub struct Community {
    pub members: Vec<String>,
    pub size: usize,
}

#[derive(Debug)]
pub struct NetworkStats {
    pub nodes: usize,
    pub edges: usize,
    pub avg_connections: f64,
    pub density: f64,
}

/// Deep web map
#[derive(Debug)]
pub struct DeepWebMap {
    pub graph: NetworkGraph,
    pub regions: BTreeMap<String, Region>,
}

#[derive(Debug)]
pub struct Region {
    pub name: String,
    pub category: String,
    pub services: Vec<String>,
}

impl DeepWebMap {
    pub fn new() -> Self {
        Self {
            graph: NetworkGraph::new(),
            regions: BTreeMap::new(),
        }
    }

    /// Build map from crawler data
    pub fn build_from_services(&mut self, services: Vec<(String, Vec<String>, NodeType)>) {
        for (onion, links, node_type) in services {
            self.graph.add_node(onion.clone(), node_type);

            for link in links {
                self.graph.add_edge(onion.clone(), link);
            }
        }

        // Calculate PageRank
        self.graph.calculate_pagerank(10);
    }

    /// Add region
    pub fn add_region(&mut self, name: String, category: String, services: Vec<String>) {
        self.regions.insert(
            name.clone(),
            Region {
                name,
                category,
                services,
            },
        );
    }

    /// Get most important services
    pub fn get_hubs(&self, n: usize) -> Vec<Node> {
        self.graph.top_nodes(n)
    }

    /// Export to GraphViz DOT format
    pub fn export_dot(&self) -> String {
        let mut dot = String::from("digraph DeepWeb {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Nodes
        for (addr, node) in &self.graph.nodes {
            let color = match node.node_type {
                NodeType::Marketplace => "red",
                NodeType::Forum => "blue",
                NodeType::Wiki => "green",
                NodeType::Blog => "yellow",
                NodeType::Service => "purple",
                NodeType::Unknown => "gray",
            };

            dot.push_str(&format!(
                "  \"{}\" [color={}, label=\"{}\\nPR: {:.3}\"];\n",
                addr,
                color,
                addr.split('.').next().unwrap_or(addr),
                node.pagerank
            ));
        }

        dot.push_str("\n");

        // Edges
        for edge in &self.graph.edges {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", edge.from, edge.to));
        }

        dot.push_str("}\n");
        dot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_graph() {
        let mut graph = NetworkGraph::new();

        graph.add_node("a.onion".to_string(), NodeType::Marketplace);
        graph.add_node("b.onion".to_string(), NodeType::Forum);

        graph.add_edge("a.onion".to_string(), "b.onion".to_string());

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_pagerank() {
        let mut graph = NetworkGraph::new();

        graph.add_edge("a.onion".to_string(), "b.onion".to_string());
        graph.add_edge("a.onion".to_string(), "c.onion".to_string());
        graph.add_edge("b.onion".to_string(), "c.onion".to_string());

        graph.calculate_pagerank(10);

        // c.onion should have highest PageRank (2 incoming links)
        let c_rank = graph.nodes.get("c.onion").unwrap().pagerank;
        let a_rank = graph.nodes.get("a.onion").unwrap().pagerank;

        assert!(c_rank > a_rank);
    }

    #[test]
    fn test_communities() {
        let mut graph = NetworkGraph::new();

        // Community 1
        graph.add_edge("a.onion".to_string(), "b.onion".to_string());
        graph.add_edge("b.onion".to_string(), "c.onion".to_string());

        // Community 2 (isolated)
        graph.add_edge("x.onion".to_string(), "y.onion".to_string());

        let communities = graph.find_communities();
        assert_eq!(communities.len(), 2);
    }

    #[test]
    fn test_export_dot() {
        let mut map = DeepWebMap::new();
        map.graph.add_edge("a.onion".to_string(), "b.onion".to_string());

        let dot = map.export_dot();
        assert!(dot.contains("digraph DeepWeb"));
        assert!(dot.contains("a.onion"));
    }
}
