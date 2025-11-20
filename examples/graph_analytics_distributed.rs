/// Análise avançada: Graph Analytics + Distributed Processing
/// Demonstra uso de petgraph e processamento paralelo massivo

use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::{dijkstra, connected_components, astar};
use petgraph::Directed;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    println!("🕸️  Graph Analytics & Distributed Processing - Arxis\n");

    // Pipeline 1: Análise de rede de eventos gravitacionais
    println!("📡 Pipeline 1: Rede de Eventos Gravitacionais");
    gravitational_wave_network();

    // Pipeline 2: Pathfinding em espaço-tempo
    println!("\n🌌 Pipeline 2: Pathfinding em Espaço-Tempo Curvo");
    spacetime_pathfinding();

    // Pipeline 3: Processamento paralelo massivo
    println!("\n⚡ Pipeline 3: Processamento Paralelo de Tensores");
    massive_parallel_processing();
}

/// Pipeline 1: Construir e analisar rede de eventos GW
fn gravitational_wave_network() {
    // 1. Criar grafo de eventos
    let mut graph = Graph::<EventNode, f64, Directed>::new();

    // Adicionar nós (eventos detectados)
    let mut nodes = Vec::new();
    for i in 0..100 {
        let event = EventNode {
            id: i,
            mass_total: 20.0 + rand::random::<f64>() * 60.0,
            distance: 100.0 + rand::random::<f64>() * 900.0,
            frequency: 50.0 + rand::random::<f64>() * 450.0,
        };
        nodes.push(graph.add_node(event));
    }

    // 2. Conectar eventos próximos (distância < threshold)
    let threshold = 200.0; // Mpc
    let mut edges_added = 0;

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let node_i = &graph[nodes[i]];
            let node_j = &graph[nodes[j]];

            // Calcular "distância" no espaço de features
            let dist = ((node_i.mass_total - node_j.mass_total).powi(2)
                + (node_i.distance - node_j.distance).powi(2) / 1000.0
                + (node_i.frequency - node_j.frequency).powi(2) / 10.0)
                .sqrt();

            if dist < threshold {
                graph.add_edge(nodes[i], nodes[j], dist);
                edges_added += 1;
            }
        }
    }

    println!("  ✓ Grafo criado:");
    println!("    Nós (eventos): {}", graph.node_count());
    println!("    Arestas (conexões): {}", edges_added);

    // 3. Analisar componentes conectados
    let components = connected_components(&graph);
    println!("  ✓ Componentes conectados: {}", components);

    // 4. Encontrar caminho mais curto entre dois eventos
    if graph.node_count() >= 2 {
        let start = nodes[0];
        let end = nodes[nodes.len() - 1];

        let distances = dijkstra(&graph, start, Some(end), |e| *e.weight());

        if let Some(&distance) = distances.get(&end) {
            println!("  ✓ Caminho mais curto (evento 0 → evento {}):", nodes.len() - 1);
            println!("    Distância total: {:.2}", distance);
        } else {
            println!("  ✓ Nenhum caminho encontrado entre eventos selecionados");
        }
    }

    // 5. Calcular centralidade (grau de cada nó)
    let mut centrality: HashMap<usize, usize> = HashMap::new();
    for node in nodes.iter() {
        let degree = graph.neighbors(*node).count();
        centrality.insert(node.index(), degree);
    }

    // Encontrar eventos mais "centrais"
    let mut sorted: Vec<_> = centrality.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("  ✓ Top 5 eventos mais centrais:");
    for (i, (node_idx, degree)) in sorted.iter().take(5).enumerate() {
        println!("    {}. Evento {}: {} conexões", i + 1, node_idx, degree);
    }
}

#[derive(Debug, Clone)]
struct EventNode {
    id: usize,
    mass_total: f64,
    distance: f64,
    frequency: f64,
}

/// Pipeline 2: Pathfinding em espaço-tempo curvo (geodésicas)
fn spacetime_pathfinding() {
    // 1. Criar grid 3D representando espaço-tempo
    let grid_size = 20;
    let mut graph = Graph::<SpaceTimePoint, f64, Directed>::new();
    let mut nodes = Vec::new();

    // Adicionar pontos do espaço-tempo
    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                let point = SpaceTimePoint {
                    x: x as f64,
                    y: y as f64,
                    z: z as f64,
                    t: 0.0,
                    curvature: calculate_curvature(x as f64, y as f64, z as f64),
                };
                nodes.push(graph.add_node(point));
            }
        }
    }

    println!("  ✓ Grid 3D criado: {} pontos", nodes.len());

    // 2. Conectar pontos vizinhos (6-conectividade)
    let mut edges_added = 0;
    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                let idx = x * grid_size * grid_size + y * grid_size + z;
                let current = nodes[idx];

                // Conectar com vizinhos
                let neighbors = [
                    (x + 1, y, z),
                    (x, y + 1, z),
                    (x, y, z + 1),
                ];

                for (nx, ny, nz) in neighbors {
                    if nx < grid_size && ny < grid_size && nz < grid_size {
                        let neighbor_idx = nx * grid_size * grid_size + ny * grid_size + nz;
                        let neighbor = nodes[neighbor_idx];

                        // Distância ponderada pela curvatura
                        let curvature_factor = 1.0 + graph[current].curvature.abs();
                        let distance = 1.0 * curvature_factor;

                        graph.add_edge(current, neighbor, distance);
                        edges_added += 1;
                    }
                }
            }
        }
    }

    println!("  ✓ Arestas criadas: {}", edges_added);

    // 3. Encontrar geodésica (caminho mais curto considerando curvatura)
    let start = nodes[0]; // Origem
    let end = nodes[nodes.len() - 1]; // Destino diagonal

    println!("  ✓ Calculando geodésica com A*...");

    let path = astar(
        &graph,
        start,
        |n| n == end,
        |e| *e.weight(),
        |n| {
            // Heurística: distância euclidiana
            let current = &graph[n];
            let target = &graph[end];
            ((current.x - target.x).powi(2)
                + (current.y - target.y).powi(2)
                + (current.z - target.z).powi(2))
            .sqrt()
        },
    );

    if let Some((distance, path_nodes)) = path {
        println!("  ✓ Geodésica encontrada!");
        println!("    Distância: {:.2}", distance);
        println!("    Passos: {}", path_nodes.len());

        // Mostrar primeiros e últimos pontos
        let start_point = &graph[path_nodes[0]];
        let end_point = &graph[*path_nodes.last().unwrap()];

        println!("    Início: ({:.0}, {:.0}, {:.0})", start_point.x, start_point.y, start_point.z);
        println!("    Fim: ({:.0}, {:.0}, {:.0})", end_point.x, end_point.y, end_point.z);
    } else {
        println!("  ✗ Nenhum caminho encontrado");
    }
}

#[derive(Debug, Clone)]
struct SpaceTimePoint {
    x: f64,
    y: f64,
    z: f64,
    t: f64,
    curvature: f64,
}

fn calculate_curvature(x: f64, y: f64, z: f64) -> f64 {
    // Simular curvatura com massa central em (10, 10, 10)
    let center_x = 10.0;
    let center_y = 10.0;
    let center_z = 10.0;

    let r = ((x - center_x).powi(2) + (y - center_y).powi(2) + (z - center_z).powi(2)).sqrt();

    // Curvatura ~ 1/r² (tipo Schwarzschild simplificado)
    if r > 1.0 {
        1.0 / r.powi(2)
    } else {
        1.0
    }
}

/// Pipeline 3: Processamento paralelo massivo com Rayon
fn massive_parallel_processing() {
    // 1. Gerar 1 milhão de tensores 4x4
    let n_tensors = 1_000_000;
    println!("  ✓ Gerando {} tensores 4x4...", n_tensors);

    let tensors: Vec<[[f64; 4]; 4]> = (0..n_tensors)
        .into_par_iter()
        .map(|i| {
            // Métrica de Minkowski perturbada
            let perturbation = (i as f64 * 0.00001).sin() * 0.01;
            [
                [-1.0 + perturbation, 0.0, 0.0, 0.0],
                [0.0, 1.0 + perturbation, 0.0, 0.0],
                [0.0, 0.0, 1.0 + perturbation, 0.0],
                [0.0, 0.0, 0.0, 1.0 + perturbation],
            ]
        })
        .collect();

    println!("  ✓ Tensores gerados!");

    // 2. Calcular determinantes em paralelo
    println!("  ✓ Calculando {} determinantes em paralelo...", n_tensors);

    let start = std::time::Instant::now();
    let determinants: Vec<f64> = tensors
        .par_iter()
        .map(|tensor| calculate_determinant_4x4(tensor))
        .collect();
    let elapsed = start.elapsed();

    println!("  ✓ Cálculo completo em {:.2}s", elapsed.as_secs_f64());
    println!("    Taxa: {:.0} tensores/segundo", n_tensors as f64 / elapsed.as_secs_f64());

    // 3. Estatísticas
    let mean: f64 = determinants.par_iter().sum::<f64>() / n_tensors as f64;
    let variance: f64 = determinants
        .par_iter()
        .map(|d| (d - mean).powi(2))
        .sum::<f64>()
        / n_tensors as f64;
    let std_dev = variance.sqrt();

    let min = determinants
        .par_iter()
        .cloned()
        .reduce(|| f64::INFINITY, f64::min);
    let max = determinants
        .par_iter()
        .cloned()
        .reduce(|| f64::NEG_INFINITY, f64::max);

    println!("  ✓ Estatísticas dos determinantes:");
    println!("    Média: {:.6}", mean);
    println!("    Desvio padrão: {:.6}", std_dev);
    println!("    Min: {:.6}", min);
    println!("    Max: {:.6}", max);

    // 4. Filtrar tensores "interessantes" (|det| próximo de 1)
    let interesting: Vec<_> = determinants
        .par_iter()
        .enumerate()
        .filter(|(_, &det)| (det.abs() - 1.0).abs() < 0.001)
        .map(|(i, &det)| (i, det))
        .collect();

    println!("  ✓ Tensores com |det| ≈ 1.0: {} ({:.2}%)",
        interesting.len(),
        (interesting.len() as f64 / n_tensors as f64) * 100.0
    );

    // 5. Processamento em chunks (simular MapReduce)
    println!("\n  ✓ Processamento MapReduce (chunks de 10k):");

    let chunk_size = 10_000;
    let chunk_results: Vec<ChunkResult> = tensors
        .par_chunks(chunk_size)
        .enumerate()
        .map(|(chunk_id, chunk)| {
            // Map: processar chunk
            let det_sum: f64 = chunk
                .iter()
                .map(|t| calculate_determinant_4x4(t))
                .sum();

            ChunkResult {
                chunk_id,
                count: chunk.len(),
                sum: det_sum,
                mean: det_sum / chunk.len() as f64,
            }
        })
        .collect();

    // Reduce: agregar resultados
    let total_sum: f64 = chunk_results.iter().map(|r| r.sum).sum();
    let global_mean = total_sum / n_tensors as f64;

    println!("    Chunks processados: {}", chunk_results.len());
    println!("    Média global (MapReduce): {:.6}", global_mean);
    println!("    Diferença da média direta: {:.2e}", (global_mean - mean).abs());
}

#[derive(Debug)]
struct ChunkResult {
    chunk_id: usize,
    count: usize,
    sum: f64,
    mean: f64,
}

fn calculate_determinant_4x4(matrix: &[[f64; 4]; 4]) -> f64 {
    // Determinante 4x4 simplificado (diagonal)
    matrix[0][0] * matrix[1][1] * matrix[2][2] * matrix[3][3]
}
