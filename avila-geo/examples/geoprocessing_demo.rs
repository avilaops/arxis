//! Exemplo completo de geoprocessamento
//!
//! Demonstra:
//! - Análise espacial com QuadTree
//! - Interpolação IDW
//! - Kernel Density Estimation
//! - Análise estatística espacial (Moran's I)
//! - Operações geométricas

use avila_geo::coords::GeoCoord;
use avila_geo::geoprocessing::{
    analysis::{haversine_distance, idw_interpolation, kernel_density, morans_i, Grid, SpatialWeights},
    engine::{GeoprocessingEngine, Layer, OverlayOp},
    operations::{convex_hull, point_in_polygon, polygon_area},
    spatial::{BoundingBox, QuadTree, SpatialFeature},
};

fn main() {
    println!("🗺️  Sistema Completo de Geoprocessamento - Avila Geo\n");

    // 1. Estruturas de dados espaciais - QuadTree
    demo_quadtree();

    // 2. Análise espacial - Interpolação IDW
    demo_idw_interpolation();

    // 3. Kernel Density Estimation
    demo_kernel_density();

    // 4. Operações geométricas
    demo_geometric_operations();

    // 5. Engine de geoprocessamento
    demo_geoprocessing_engine();

    // 6. Análise estatística espacial
    demo_spatial_statistics();

    println!("\n✅ Demonstração completa!");
}

fn demo_quadtree() {
    println!("=== 1. QuadTree - Índice Espacial ===");

    let bounds = BoundingBox::new(-50.0, -30.0, -40.0, -20.0);
    let mut qtree = QuadTree::new(bounds, 4);

    // Inserir cidades brasileiras
    let cities = vec![
        (GeoCoord::new(-23.5505, -46.6333), "São Paulo"),
        (GeoCoord::new(-22.9068, -43.1729), "Rio de Janeiro"),
        (GeoCoord::new(-25.4284, -49.2733), "Curitiba"),
        (GeoCoord::new(-19.9167, -43.9345), "Belo Horizonte"),
        (GeoCoord::new(-30.0346, -51.2177), "Porto Alegre"),
    ];

    for (coord, name) in cities.iter() {
        qtree.insert(*coord, *name);
    }

    println!("✓ Inseridas {} cidades no QuadTree", qtree.size());

    // Query espacial
    let query_bounds = BoundingBox::new(-24.0, -47.0, -22.0, -43.0);
    let mut results = Vec::new();
    qtree.query(&query_bounds, &mut results);

    println!("✓ Cidades no sudeste: {}", results.len());
    for (coord, name) in results {
        println!("  - {} ({:.2}, {:.2})", name, coord.lat, coord.lon);
    }

    // Query por raio
    let center = GeoCoord::new(-23.0, -46.0);
    let mut nearby = Vec::new();
    qtree.query_radius(&center, 200000.0, &mut nearby); // 200km

    println!("✓ Cidades em 200km de ({:.2}, {:.2}): {}", center.lat, center.lon, nearby.len());
    println!();
}

fn demo_idw_interpolation() {
    println!("=== 2. Interpolação IDW (Inverse Distance Weighting) ===");

    // Estações meteorológicas com temperaturas
    let stations = vec![
        (GeoCoord::new(-23.5505, -46.6333), 22.0), // São Paulo - 22°C
        (GeoCoord::new(-22.9068, -43.1729), 28.0), // Rio - 28°C
        (GeoCoord::new(-25.4284, -49.2733), 18.0), // Curitiba - 18°C
        (GeoCoord::new(-19.9167, -43.9345), 24.0), // BH - 24°C
    ];

    // Estimar temperatura em pontos desconhecidos
    let targets = vec![
        GeoCoord::new(-23.0, -45.0),
        GeoCoord::new(-24.0, -46.0),
        GeoCoord::new(-22.0, -44.0),
    ];

    println!("✓ Estações: {} | Pontos de interpolação: {}", stations.len(), targets.len());

    for target in targets {
        let temp = idw_interpolation(&stations, &target, 2.0);
        println!("  Temperatura estimada em ({:.2}, {:.2}): {:.1}°C", target.lat, target.lon, temp);
    }
    println!();
}

fn demo_kernel_density() {
    println!("=== 3. Kernel Density Estimation ===");

    // Pontos de crime (exemplo fictício)
    let crime_points = vec![
        GeoCoord::new(-23.55, -46.63),
        GeoCoord::new(-23.56, -46.64),
        GeoCoord::new(-23.55, -46.64),
        GeoCoord::new(-23.54, -46.63),
        GeoCoord::new(-23.56, -46.62),
        GeoCoord::new(-23.57, -46.63),
    ];

    let bounds = BoundingBox::new(-23.6, -46.7, -23.5, -46.6);
    let grid = Grid::new(bounds, 10, 10);

    let density = kernel_density(&crime_points, &grid, 0.02);

    println!("✓ Grid de densidade: {}x{}", density.len(), density[0].len());
    println!("✓ Pontos analisados: {}", crime_points.len());

    // Encontrar hotspot
    let mut max_density = 0.0;
    let mut max_row = 0;
    let mut max_col = 0;

    for (row, row_data) in density.iter().enumerate() {
        for (col, &value) in row_data.iter().enumerate() {
            if value > max_density {
                max_density = value;
                max_row = row;
                max_col = col;
            }
        }
    }

    let hotspot = grid.cell_center(max_row, max_col);
    println!("✓ Hotspot detectado em: ({:.4}, {:.4}) | Densidade: {:.4}",
        hotspot.lat, hotspot.lon, max_density);
    println!();
}

fn demo_geometric_operations() {
    println!("=== 4. Operações Geométricas ===");

    // Criar polígono (exemplo: parque)
    let park = vec![
        GeoCoord::new(-23.55, -46.65),
        GeoCoord::new(-23.55, -46.63),
        GeoCoord::new(-23.53, -46.63),
        GeoCoord::new(-23.53, -46.65),
    ];

    // Calcular área
    let area = polygon_area(&park);
    println!("✓ Área do polígono: {:.6} graus²", area);

    // Point in polygon
    let test_points = vec![
        (GeoCoord::new(-23.54, -46.64), "Ponto A"),
        (GeoCoord::new(-23.52, -46.64), "Ponto B"),
        (GeoCoord::new(-23.54, -46.66), "Ponto C"),
    ];

    println!("✓ Teste de contenção:");
    for (point, name) in test_points {
        let inside = point_in_polygon(&point, &park);
        println!("  {} ({:.2}, {:.2}): {}",
            name, point.lat, point.lon,
            if inside { "dentro ✓" } else { "fora ✗" }
        );
    }

    // Convex Hull
    let scattered_points = vec![
        GeoCoord::new(0.0, 0.0),
        GeoCoord::new(5.0, 5.0),
        GeoCoord::new(10.0, 0.0),
        GeoCoord::new(5.0, 10.0),
        GeoCoord::new(5.0, 2.0), // Ponto interior
        GeoCoord::new(7.0, 3.0), // Ponto interior
    ];

    let hull = convex_hull(&scattered_points);
    println!("✓ Convex Hull: {} pontos de entrada → {} pontos no fecho",
        scattered_points.len(), hull.len());
    println!();
}

fn demo_geoprocessing_engine() {
    println!("=== 5. Engine de Geoprocessamento ===");

    let mut engine = GeoprocessingEngine::new();

    // Criar camada de pontos de interesse
    let mut poi_layer = Layer::new("POIs");
    poi_layer.add_feature(
        SpatialFeature::point(0, GeoCoord::new(-23.5505, -46.6333))
            .with_property("name", "Shopping Center")
            .with_property("type", "commercial")
    );
    poi_layer.add_feature(
        SpatialFeature::point(1, GeoCoord::new(-23.5605, -46.6433))
            .with_property("name", "Hospital")
            .with_property("type", "health")
    );
    poi_layer.add_feature(
        SpatialFeature::point(2, GeoCoord::new(-23.5405, -46.6233))
            .with_property("name", "School")
            .with_property("type", "education")
    );

    engine.add_layer(poi_layer);

    // Criar camada de áreas de risco
    let mut risk_layer = Layer::new("Risk Areas");
    let risk_zone = vec![
        GeoCoord::new(-23.56, -46.65),
        GeoCoord::new(-23.56, -46.63),
        GeoCoord::new(-23.54, -46.63),
        GeoCoord::new(-23.54, -46.65),
    ];
    risk_layer.add_feature(
        SpatialFeature::polygon(0, risk_zone)
            .with_property("risk_level", "high")
    );

    engine.add_layer(risk_layer);

    println!("✓ Camadas criadas: {}", engine.list_layers().len());

    // Query espacial
    let query_bounds = BoundingBox::new(-23.57, -46.66, -23.53, -46.62);
    let results = engine.query_spatial("POIs", &query_bounds);
    println!("✓ POIs na área de busca: {}", results.len());

    // Análise de proximidade
    let center = GeoCoord::new(-23.55, -46.64);
    let nearby = engine.proximity_analysis("POIs", &center, 5000.0); // 5km
    println!("✓ POIs em 5km do centro: {}", nearby.len());

    // Estatísticas da camada
    if let Some(stats) = engine.layer_statistics("POIs") {
        println!("✓ Estatísticas:");
        println!("  - Total de features: {}", stats.feature_count);
        println!("  - Pontos: {}", stats.point_count);
        println!("  - Linhas: {}", stats.line_count);
        println!("  - Polígonos: {}", stats.polygon_count);
    }

    // Buffer operation
    if let Some(buffered) = engine.buffer_layer("POIs", 0.01) {
        println!("✓ Buffer criado: {} features", buffered.count());
    }

    // Overlay operations
    if let Some(intersection) = engine.overlay("POIs", "Risk Areas", OverlayOp::Intersection) {
        println!("✓ Overlay (Intersection): {} features resultantes", intersection.count());
    }

    println!();
}

fn demo_spatial_statistics() {
    println!("=== 6. Análise Estatística Espacial ===");

    // Criar features com valores (ex: preço de imóveis)
    let features = vec![
        SpatialFeature::point(0, GeoCoord::new(-23.55, -46.63)),
        SpatialFeature::point(1, GeoCoord::new(-23.56, -46.64)),
        SpatialFeature::point(2, GeoCoord::new(-23.54, -46.62)),
        SpatialFeature::point(3, GeoCoord::new(-23.57, -46.65)),
    ];

    let values = vec![500000.0, 520000.0, 480000.0, 510000.0]; // Preços em reais

    // Matriz de pesos espaciais
    let weights = SpatialWeights::from_distance(&features, 0.05);

    // Calcular Moran's I (autocorrelação espacial)
    let morans = morans_i(&features, &values, &weights);

    println!("✓ Índice I de Moran: {:.4}", morans);
    if morans > 0.5 {
        println!("  → Clustering espacial positivo forte");
    } else if morans > 0.0 {
        println!("  → Clustering espacial positivo moderado");
    } else if morans < -0.5 {
        println!("  → Dispersão espacial forte");
    } else {
        println!("  → Padrão espacial aleatório");
    }

    // Estatísticas básicas
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let std_dev = variance.sqrt();

    println!("✓ Estatísticas dos valores:");
    println!("  - Média: R$ {:.2}", mean);
    println!("  - Desvio padrão: R$ {:.2}", std_dev);
    println!("  - Variância: {:.2}", variance);

    // Distâncias entre features
    println!("✓ Matriz de distâncias (km):");
    for i in 0..features.len() {
        for j in (i + 1)..features.len() {
            let bbox_i = features[i].bounding_box();
            let bbox_j = features[j].bounding_box();
            let c_i = bbox_i.center();
            let c_j = bbox_j.center();

            let coord_i = GeoCoord::new(c_i.1, c_i.0);
            let coord_j = GeoCoord::new(c_j.1, c_j.0);

            let dist = haversine_distance(&coord_i, &coord_j) / 1000.0;
            println!("  Feature {} → Feature {}: {:.2} km", i, j, dist);
        }
    }

    println!();
}
