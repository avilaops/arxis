//! Demonstração de processamento paralelo
//!
//! Este exemplo mostra ganhos de performance com paralelização:
//! - Kernel Density paralelo
//! - IDW interpolation em batch
//! - Rasterização paralela
//! - Comparação de performance serial vs paralelo

use avila_geo::coords::Coord;
use avila_geo::geoprocessing::*;
use std::time::Instant;

#[cfg(feature = "parallel")]
fn main() {
    println!("=== Demonstração de Processamento Paralelo ===\n");

    // Gerar dados de teste
    let points = generate_test_points(1000);
    let values: Vec<f64> = (0..points.len()).map(|i| (i as f64).sin() * 100.0).collect();

    println!("✓ Gerados {} pontos de teste\n", points.len());

    // 1. Kernel Density: Serial vs Paralelo
    demo_kernel_density_parallel(&points, &values);

    // 2. IDW Interpolation: Batch paralelo
    demo_idw_batch_parallel(&points, &values);

    // 3. Distance Matrix: Comparação
    demo_distance_matrix_parallel(&points);

    // 4. Point-in-Polygon: Batch paralelo
    demo_batch_point_in_polygon(&points);

    // 5. K-Nearest Neighbors paralelo
    demo_knn_parallel(&points);
}

#[cfg(feature = "parallel")]
fn generate_test_points(n: usize) -> Vec<Coord> {
    (0..n)
        .map(|i| {
            let angle = (i as f64) * 0.1;
            let radius = (i as f64) * 0.0001;
            Coord::new(
                -23.5505 + radius * angle.cos(),
                -46.6333 + radius * angle.sin(),
            )
        })
        .collect()
}

#[cfg(feature = "parallel")]
fn demo_kernel_density_parallel(points: &[Coord], values: &[f64]) {
    println!("--- Kernel Density Estimation ---");

    let bounds = BoundingBox::from_coords(points);
    let grid_width = 200;
    let grid_height = 200;
    let bandwidth = 0.01;

    // Versão serial
    let start = Instant::now();
    let _serial = kernel_density(points, values, &bounds, grid_width, grid_height, bandwidth);
    let serial_time = start.elapsed();

    // Versão paralela
    let start = Instant::now();
    let _parallel = kernel_density_parallel(points, values, &bounds, grid_width, grid_height, bandwidth);
    let parallel_time = start.elapsed();

    println!("  Grid: {}x{} = {} células", grid_width, grid_height, grid_width * grid_height);
    println!("  Serial:   {:?}", serial_time);
    println!("  Parallel: {:?}", parallel_time);
    println!("  Speedup:  {:.2}x", serial_time.as_secs_f64() / parallel_time.as_secs_f64());
    println!();
}

#[cfg(feature = "parallel")]
fn demo_idw_batch_parallel(points: &[Coord], values: &[f64]) {
    println!("--- IDW Interpolation (Batch) ---");

    // Criar pontos de consulta
    let query_points: Vec<Coord> = (0..500)
        .map(|i| {
            let offset = (i as f64) * 0.0002;
            Coord::new(-23.5505 + offset, -46.6333 + offset)
        })
        .collect();

    let start = Instant::now();
    let results = idw_interpolation_batch_parallel(points, values, &query_points, 2.0, 5);
    let elapsed = start.elapsed();

    println!("  Interpolação de {} pontos", query_points.len());
    println!("  Tempo: {:?}", elapsed);
    println!("  Throughput: {:.0} interpolações/segundo",
        query_points.len() as f64 / elapsed.as_secs_f64());

    // Mostrar algumas amostras
    println!("\n  Amostras:");
    for i in (0..results.len()).step_by(100) {
        println!("    Ponto {}: valor interpolado = {:.2}", i, results[i]);
    }
    println!();
}

#[cfg(feature = "parallel")]
fn demo_distance_matrix_parallel(points: &[Coord]) {
    println!("--- Distance Matrix ---");

    // Usar subset para não estourar memória
    let subset = &points[..100.min(points.len())];

    let start = Instant::now();
    let matrix = distance_matrix_parallel(subset);
    let elapsed = start.elapsed();

    println!("  Matriz {}x{} = {} distâncias calculadas",
        subset.len(), subset.len(), subset.len() * subset.len());
    println!("  Tempo: {:?}", elapsed);

    // Estatísticas da matriz
    let mut distances: Vec<f64> = matrix.iter()
        .flat_map(|row| row.iter())
        .copied()
        .filter(|&d| d > 0.0)
        .collect();

    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if !distances.is_empty() {
        let min = distances[0];
        let max = distances[distances.len() - 1];
        let median = distances[distances.len() / 2];

        println!("  Distâncias:");
        println!("    Mín:    {:.2}m", min);
        println!("    Mediana: {:.2}m", median);
        println!("    Máx:    {:.2}m", max);
    }
    println!();
}

#[cfg(feature = "parallel")]
fn demo_batch_point_in_polygon(points: &[Coord]) {
    println!("--- Batch Point-in-Polygon ---");

    // Criar um polígono grande ao redor de São Paulo
    let polygon = vec![
        Coord::new(-23.4, -46.8),
        Coord::new(-23.4, -46.4),
        Coord::new(-23.7, -46.4),
        Coord::new(-23.7, -46.8),
        Coord::new(-23.4, -46.8),
    ];

    let start = Instant::now();
    let results = batch_point_in_polygon_parallel(points, &polygon);
    let elapsed = start.elapsed();

    let inside_count = results.iter().filter(|&&x| x).count();

    println!("  Testados {} pontos", points.len());
    println!("  Dentro do polígono: {}", inside_count);
    println!("  Fora do polígono: {}", points.len() - inside_count);
    println!("  Tempo: {:?}", elapsed);
    println!("  Throughput: {:.0} testes/segundo",
        points.len() as f64 / elapsed.as_secs_f64());
    println!();
}

#[cfg(feature = "parallel")]
fn demo_knn_parallel(points: &[Coord]) {
    println!("--- K-Nearest Neighbors ---");

    let query_point = Coord::new(-23.5505, -46.6333); // Centro de São Paulo
    let k = 10;

    let start = Instant::now();
    let neighbors = k_nearest_neighbors_parallel(points, &query_point, k);
    let elapsed = start.elapsed();

    println!("  Buscando {} vizinhos mais próximos", k);
    println!("  Em {} pontos totais", points.len());
    println!("  Tempo: {:?}", elapsed);

    println!("\n  Vizinhos encontrados:");
    for (i, &(idx, dist)) in neighbors.iter().enumerate() {
        let point = &points[idx];
        println!("    {}. Ponto {} - Distância: {:.2}m",
            i + 1, idx, dist);
        println!("       Coordenadas: ({:.6}, {:.6})", point.x, point.y);
    }
    println!();
}

#[cfg(not(feature = "parallel"))]
fn main() {
    println!("❌ Este exemplo requer a feature 'parallel'");
    println!("   Compile com: cargo run --example parallel_demo --features parallel");
}
