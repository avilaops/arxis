//! Exemplo de análise de redes espaciais
//!
//! Demonstra:
//! - Criação de redes de transporte
//! - Algoritmo de Dijkstra para caminho mais curto
//! - A* com heurística geográfica
//! - Análise de centralidade
//! - Componentes conectados

use avila_geo::coords::GeoCoord;
use avila_geo::geoprocessing::network::{NetworkNode, NetworkEdge, SpatialNetwork};
use avila_geo::geoprocessing::analysis::haversine_distance;

fn main() {
    println!("🚗 Análise de Redes Espaciais - Avila Geo\n");

    // Criar rede de transporte urbano
    let network = create_city_network();

    println!("=== Rede de Transporte ===");
    println!("✓ Nós: {}", network.nodes.len());
    println!("✓ Arestas: {}", network.edges.len());
    println!();

    // 1. Caminho mais curto (Dijkstra)
    demo_shortest_path(&network);

    // 2. Caminho mais curto com A*
    demo_astar(&network);

    // 3. Análise de centralidade
    demo_centrality(&network);

    // 4. Componentes conectados
    demo_components(&network);

    // 5. Árvore geradora mínima
    demo_mst(&network);

    println!("\n✅ Análise de rede completa!");
}

fn create_city_network() -> SpatialNetwork {
    let mut network = SpatialNetwork::new();

    // Criar interseções (nós) de uma cidade fictícia
    let nodes = vec![
        (0, GeoCoord::new(-23.550, -46.633), "Centro"),
        (1, GeoCoord::new(-23.560, -46.643), "Zona Norte"),
        (2, GeoCoord::new(-23.540, -46.623), "Zona Sul"),
        (3, GeoCoord::new(-23.545, -46.638), "Zona Leste"),
        (4, GeoCoord::new(-23.555, -46.628), "Zona Oeste"),
        (5, GeoCoord::new(-23.565, -46.633), "Subúrbio Norte"),
        (6, GeoCoord::new(-23.535, -46.628), "Subúrbio Sul"),
    ];

    for (id, coord, name) in nodes {
        network.add_node(
            NetworkNode::new(id, coord)
                .with_property("name", name)
                .with_property("type", "intersection")
        );
    }

    // Criar vias (arestas) com pesos baseados em distância
    let edges = vec![
        (0, 0, 1), // Centro → Zona Norte
        (1, 0, 2), // Centro → Zona Sul
        (2, 0, 3), // Centro → Zona Leste
        (3, 0, 4), // Centro → Zona Oeste
        (4, 1, 3), // Zona Norte → Zona Leste
        (5, 1, 5), // Zona Norte → Subúrbio Norte
        (6, 2, 4), // Zona Sul → Zona Oeste
        (7, 2, 6), // Zona Sul → Subúrbio Sul
        (8, 3, 4), // Zona Leste → Zona Oeste
        (9, 4, 6), // Zona Oeste → Subúrbio Sul
    ];

    for (edge_id, from, to) in edges {
        let from_node = &network.nodes[from];
        let to_node = &network.nodes[to];

        // Peso baseado na distância real
        let weight = haversine_distance(&from_node.coord, &to_node.coord);

        let geometry = vec![from_node.coord, to_node.coord];

        network.add_edge(
            NetworkEdge::new(edge_id, from, to, weight)
                .with_geometry(geometry)
                .with_property("road_type", "arterial")
        );
    }

    network
}

fn demo_shortest_path(network: &SpatialNetwork) {
    println!("=== 1. Caminho Mais Curto (Dijkstra) ===");

    let start = 0; // Centro
    let end = 5;   // Subúrbio Norte

    if let Some(path) = network.shortest_path(start, end) {
        println!("✓ Caminho encontrado:");
        println!("  Nós: {:?}", path.nodes);
        println!("  Distância total: {:.2} metros", path.total_cost);
        println!("  Segmentos: {}", path.len());

        // Mostrar nomes dos nós
        print!("  Rota: ");
        for (i, &node_id) in path.nodes.iter().enumerate() {
            if let Some(node) = network.get_node(node_id) {
                if let Some(name) = node.properties.get("name") {
                    print!("{}", name);
                    if i < path.nodes.len() - 1 {
                        print!(" → ");
                    }
                }
            }
        }
        println!();
    } else {
        println!("✗ Caminho não encontrado");
    }
    println!();
}

fn demo_astar(network: &SpatialNetwork) {
    println!("=== 2. Caminho Mais Curto com A* (Heurística Geográfica) ===");

    let start = 0; // Centro
    let end = 6;   // Subúrbio Sul

    if let Some(path) = network.shortest_path_astar(start, end) {
        println!("✓ Caminho encontrado com A*:");
        println!("  Nós: {:?}", path.nodes);
        println!("  Distância total: {:.2} metros", path.total_cost);

        // Comparar com Dijkstra
        if let Some(dijkstra_path) = network.shortest_path(start, end) {
            println!("  Comparação:");
            println!("    A*: {:.2}m", path.total_cost);
            println!("    Dijkstra: {:.2}m", dijkstra_path.total_cost);

            let diff = (path.total_cost - dijkstra_path.total_cost).abs();
            if diff < 0.01 {
                println!("    ✓ Ambos encontraram o mesmo caminho ótimo");
            }
        }
    }
    println!();
}

fn demo_centrality(network: &SpatialNetwork) {
    println!("=== 3. Análise de Centralidade ===");

    // Centralidade de grau
    let degree = network.degree_centrality();
    println!("✓ Centralidade de Grau:");

    let mut sorted_degree: Vec<_> = degree.iter().collect();
    sorted_degree.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (node_id, centrality) in sorted_degree.iter().take(3) {
        if let Some(node) = network.get_node(**node_id) {
            if let Some(name) = node.properties.get("name") {
                println!("  {} (node {}): {:.3}", name, node_id, centrality);
            }
        }
    }

    // Centralidade de intermediação (betweenness)
    println!("\n✓ Calculando centralidade de intermediação...");
    let betweenness = network.betweenness_centrality();

    let mut sorted_betweenness: Vec<_> = betweenness.iter().collect();
    sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("✓ Centralidade de Intermediação (top 3):");
    for (node_id, centrality) in sorted_betweenness.iter().take(3) {
        if let Some(node) = network.get_node(**node_id) {
            if let Some(name) = node.properties.get("name") {
                println!("  {} (node {}): {:.3}", name, node_id, centrality);
            }
        }
    }
    println!();
}

fn demo_components(network: &SpatialNetwork) {
    println!("=== 4. Componentes Conectados ===");

    let components = network.connected_components();

    println!("✓ Número de componentes conectados: {}", components.len());

    for (i, component) in components.iter().enumerate() {
        println!("  Componente {}: {} nós", i + 1, component.len());

        // Mostrar nós do componente
        print!("    Nós: ");
        for (j, &node_id) in component.iter().enumerate() {
            if let Some(node) = network.get_node(node_id) {
                if let Some(name) = node.properties.get("name") {
                    print!("{}", name);
                    if j < component.len() - 1 {
                        print!(", ");
                    }
                }
            }
        }
        println!();
    }
    println!();
}

fn demo_mst(network: &SpatialNetwork) {
    println!("=== 5. Árvore Geradora Mínima (MST) ===");

    let mst_edges = network.minimum_spanning_tree();

    println!("✓ Arestas na MST: {}", mst_edges.len());

    let mut total_cost = 0.0;

    println!("✓ Conexões na MST:");
    for edge_id in &mst_edges {
        if let Some(edge) = network.edges.get(*edge_id) {
            let from_node = network.get_node(edge.from);
            let to_node = network.get_node(edge.to);

            if let (Some(from), Some(to)) = (from_node, to_node) {
                let from_name = from.properties.get("name").map(|s| s.as_str()).unwrap_or("?");
                let to_name = to.properties.get("name").map(|s| s.as_str()).unwrap_or("?");

                println!("  {} → {} ({:.2}m)", from_name, to_name, edge.weight);
                total_cost += edge.weight;
            }
        }
    }

    println!("✓ Custo total da MST: {:.2} metros", total_cost);
    println!();
}
