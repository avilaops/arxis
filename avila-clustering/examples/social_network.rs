//! Social Network Analysis Example
//!
//! Demonstrates community detection in social networks using graph clustering.

use avila_clustering::algorithms::graph::{Graph, LouvainClustering, LabelPropagation, connected_components};
use ndarray::array;

fn main() -> avila_clustering::Result<()> {
    println!("=== Social Network Analysis Example ===\n");

    // Create a social network graph
    // Nodes: 0-11 (12 people)
    // Edges represent friendships
    let mut network = Graph::new(12);

    // Community 1: Tech enthusiasts (0-3)
    network.add_edge(0, 1, 1.0);
    network.add_edge(0, 2, 1.0);
    network.add_edge(1, 2, 1.0);
    network.add_edge(1, 3, 1.0);
    network.add_edge(2, 3, 1.0);

    // Community 2: Sports fans (4-7)
    network.add_edge(4, 5, 1.0);
    network.add_edge(4, 6, 1.0);
    network.add_edge(5, 6, 1.0);
    network.add_edge(5, 7, 1.0);
    network.add_edge(6, 7, 1.0);

    // Community 3: Artists (8-11)
    network.add_edge(8, 9, 1.0);
    network.add_edge(8, 10, 1.0);
    network.add_edge(9, 10, 1.0);
    network.add_edge(9, 11, 1.0);
    network.add_edge(10, 11, 1.0);

    // Bridge connections (weaker ties)
    network.add_edge(3, 4, 0.5); // Tech to Sports
    network.add_edge(7, 8, 0.5); // Sports to Arts

    println!("Network: {} people, {} connections\n",
             network.n_nodes, network.edges.len());

    // 1. Louvain Method (modularity optimization)
    println!("1. Louvain Community Detection");
    println!("{}", "=".repeat(50));

    let louvain_result = LouvainClustering::new()
        .resolution(1.0)
        .fit(&network)?;

    println!("Communities found: {}", louvain_result.n_communities);
    println!("Modularity: {:.4}", louvain_result.modularity);
    println!("Assignments: {:?}\n", louvain_result.labels);

    // Display communities
    for c in 0..louvain_result.n_communities {
        let members = louvain_result.get_community(c);
        println!("Community {}: {:?}", c, members);
    }

    // 2. Label Propagation (fast alternative)
    println!("\n2. Label Propagation");
    println!("{}", "=".repeat(50));

    let lp_result = LabelPropagation::new().fit(&network)?;

    println!("Communities found: {}", lp_result.n_communities);
    println!("Assignments: {:?}\n", lp_result.labels);

    for c in 0..lp_result.n_communities {
        let members = lp_result.get_community(c);
        if !members.is_empty() {
            println!("Community {}: {:?}", c, members);
        }
    }

    // 3. Connected Components (baseline)
    println!("\n3. Connected Components");
    println!("{}", "=".repeat(50));

    let cc_result = connected_components(&network);

    println!("Components found: {}", cc_result.n_communities);
    println!("Assignments: {:?}", cc_result.labels);

    // 4. Community analysis
    println!("\n=== Community Analysis ===");
    let sizes = louvain_result.community_sizes();
    println!("Community sizes: {:?}", sizes);

    // Find most connected person in each community
    for c in 0..louvain_result.n_communities {
        let members = louvain_result.get_community(c);
        if !members.is_empty() {
            let mut max_degree = 0;
            let mut hub = members[0];

            for &person in &members {
                let degree = network.degree(person);
                if degree > max_degree {
                    max_degree = degree;
                    hub = person;
                }
            }

            println!("Community {} hub: Person {} (degree: {})", c, hub, max_degree);
        }
    }

    // 5. Bridge detection (people connecting communities)
    println!("\n=== Bridge Persons ===");
    for person in 0..network.n_nodes {
        let mut neighbor_communities = std::collections::HashSet::new();

        for &(neighbor, _) in network.neighbors(person) {
            neighbor_communities.insert(louvain_result.labels[neighbor]);
        }

        if neighbor_communities.len() > 1 {
            println!("Person {} bridges {} communities", person, neighbor_communities.len());
        }
    }

    println!("\n=== Use Cases ===");
    println!("• Social Media: Friend groups, influence networks");
    println!("• Biology: Protein interaction networks, gene clusters");
    println!("• Business: Customer segments, supply chains");
    println!("• Transportation: Traffic zones, route optimization");
    println!("• Citations: Research paper clusters, field identification");
    println!("• Recommendation: User preferences, content grouping");

    println!("\n=== Algorithm Comparison ===");
    println!("Louvain: High quality, modularity optimization, slower");
    println!("Label Propagation: Fast, simple, less stable");
    println!("Connected Components: Fastest, finds disconnected groups");

    Ok(())
}
