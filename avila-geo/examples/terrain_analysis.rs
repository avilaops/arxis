//! Exemplo de análise de terreno com DEM
//!
//! Demonstra:
//! - Digital Elevation Model
//! - Cálculo de inclinação (slope) e orientação (aspect)
//! - Hillshade para visualização de relevo
//! - Análise de visibilidade (viewshed)
//! - Perfis de elevação

use avila_geo::coords::GeoCoord;
use avila_geo::geoprocessing::spatial::BoundingBox;
use avila_geo::geoprocessing::terrain::DigitalElevationModel;

fn main() {
    println!("⛰️  Análise de Terreno - Digital Elevation Model\n");

    // Criar DEM sintético de uma montanha
    let dem = create_synthetic_mountain();

    println!("=== Digital Elevation Model ===");
    println!("✓ Dimensões: {}x{} células", dem.rows, dem.cols);
    println!("✓ Resolução: {:.4} graus", dem.resolution);
    println!("✓ Área coberta: {:.2} x {:.2} graus",
        dem.bounds.max_x - dem.bounds.min_x,
        dem.bounds.max_y - dem.bounds.min_y
    );
    println!();

    // 1. Estatísticas de elevação
    demo_elevation_stats(&dem);

    // 2. Análise de inclinação
    demo_slope_analysis(&dem);

    // 3. Análise de orientação (aspect)
    demo_aspect_analysis(&dem);

    // 4. Hillshade (sombreamento)
    demo_hillshade(&dem);

    // 5. Perfil de elevação
    demo_elevation_profile(&dem);

    // 6. Análise de visibilidade
    demo_viewshed(&dem);

    // 7. Detecção de picos
    demo_peak_detection(&dem);

    println!("\n✅ Análise de terreno completa!");
}

fn create_synthetic_mountain() -> DigitalElevationModel {
    let bounds = BoundingBox::new(-1.0, -1.0, 1.0, 1.0);
    let mut dem = DigitalElevationModel::new(bounds, 0.02);

    // Criar elevação sintética (montanha no centro)
    let center_row = dem.rows / 2;
    let center_col = dem.cols / 2;

    for row in 0..dem.rows {
        for col in 0..dem.cols {
            let dx = (col as f64 - center_col as f64) / center_col as f64;
            let dy = (row as f64 - center_row as f64) / center_row as f64;

            // Função gaussiana para criar montanha
            let dist_sq = dx * dx + dy * dy;
            let elevation = 1000.0 * (-dist_sq * 3.0).exp(); // Pico de 1000m

            // Adicionar ruído para tornar mais realista
            let noise = ((row * 7 + col * 13) % 10) as f64 * 5.0;

            dem.set_elevation(row, col, elevation + noise);
        }
    }

    dem
}

fn demo_elevation_stats(dem: &DigitalElevationModel) {
    println!("=== 1. Estatísticas de Elevação ===");

    let mut min_elev = f64::INFINITY;
    let mut max_elev = f64::NEG_INFINITY;
    let mut sum_elev = 0.0;
    let mut count = 0;

    for row in &dem.data {
        for &elev in row {
            min_elev = min_elev.min(elev);
            max_elev = max_elev.max(elev);
            sum_elev += elev;
            count += 1;
        }
    }

    let mean_elev = sum_elev / count as f64;

    println!("✓ Elevação mínima: {:.2}m", min_elev);
    println!("✓ Elevação máxima: {:.2}m", max_elev);
    println!("✓ Elevação média: {:.2}m", mean_elev);
    println!("✓ Amplitude: {:.2}m", max_elev - min_elev);
    println!();
}

fn demo_slope_analysis(dem: &DigitalElevationModel) {
    println!("=== 2. Análise de Inclinação (Slope) ===");

    let slope_map = dem.slope_map();

    let mut min_slope = f64::INFINITY;
    let mut max_slope = f64::NEG_INFINITY;
    let mut sum_slope = 0.0;
    let mut count = 0;

    for row in &slope_map {
        for &slope in row {
            if slope > 0.0 {
                min_slope = min_slope.min(slope);
                max_slope = max_slope.max(slope);
                sum_slope += slope;
                count += 1;
            }
        }
    }

    let mean_slope = sum_slope / count as f64;

    println!("✓ Inclinação mínima: {:.2}°", min_slope);
    println!("✓ Inclinação máxima: {:.2}°", max_slope);
    println!("✓ Inclinação média: {:.2}°", mean_slope);

    // Classificação de inclinações
    let mut flat = 0;
    let mut gentle = 0;
    let mut moderate = 0;
    let mut steep = 0;

    for row in &slope_map {
        for &slope in row {
            if slope < 5.0 {
                flat += 1;
            } else if slope < 15.0 {
                gentle += 1;
            } else if slope < 30.0 {
                moderate += 1;
            } else {
                steep += 1;
            }
        }
    }

    let total = (flat + gentle + moderate + steep) as f64;
    println!("\n✓ Classificação do terreno:");
    println!("  Plano (< 5°): {:.1}%", flat as f64 / total * 100.0);
    println!("  Suave (5-15°): {:.1}%", gentle as f64 / total * 100.0);
    println!("  Moderado (15-30°): {:.1}%", moderate as f64 / total * 100.0);
    println!("  Íngreme (> 30°): {:.1}%", steep as f64 / total * 100.0);
    println!();
}

fn demo_aspect_analysis(dem: &DigitalElevationModel) {
    println!("=== 3. Análise de Orientação (Aspect) ===");

    let aspect_map = dem.aspect_map();

    // Contar orientações por direção cardeal
    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;
    let mut flat = 0;

    for row in &aspect_map {
        for &aspect in row {
            if aspect < 0.0 {
                flat += 1;
            } else if aspect < 45.0 || aspect >= 315.0 {
                north += 1;
            } else if aspect < 135.0 {
                east += 1;
            } else if aspect < 225.0 {
                south += 1;
            } else {
                west += 1;
            }
        }
    }

    let total = (north + south + east + west + flat) as f64;
    println!("✓ Distribuição de orientação:");
    println!("  Norte (0°): {:.1}%", north as f64 / total * 100.0);
    println!("  Leste (90°): {:.1}%", east as f64 / total * 100.0);
    println!("  Sul (180°): {:.1}%", south as f64 / total * 100.0);
    println!("  Oeste (270°): {:.1}%", west as f64 / total * 100.0);
    println!("  Plano: {:.1}%", flat as f64 / total * 100.0);
    println!();
}

fn demo_hillshade(dem: &DigitalElevationModel) {
    println!("=== 4. Hillshade (Sombreamento) ===");

    // Configurar iluminação: sol vindo do noroeste, altitude de 45°
    let azimuth = 315.0; // Noroeste
    let altitude = 45.0;

    let hillshade = dem.hillshade(azimuth, altitude);

    let mut min_shade = f64::INFINITY;
    let mut max_shade = f64::NEG_INFINITY;

    for row in &hillshade {
        for &shade in row {
            if shade > 0.0 {
                min_shade = min_shade.min(shade);
                max_shade = max_shade.max(shade);
            }
        }
    }

    println!("✓ Hillshade calculado");
    println!("  Azimute da luz: {:.0}° (Noroeste)", azimuth);
    println!("  Altitude da luz: {:.0}°", altitude);
    println!("  Valor mínimo: {:.1}", min_shade);
    println!("  Valor máximo: {:.1}", max_shade);
    println!("  (valores de 0-255 representam escuridão → iluminação)", );
    println!();
}

fn demo_elevation_profile(dem: &DigitalElevationModel) {
    println!("=== 5. Perfil de Elevação ===");

    // Criar perfil atravessando a montanha
    let start = GeoCoord::new(-0.8, -0.8);
    let end = GeoCoord::new(0.8, 0.8);

    let profile = dem.elevation_profile(&start, &end, 20);

    println!("✓ Perfil: ({:.2}, {:.2}) → ({:.2}, {:.2})",
        start.lat, start.lon, end.lat, end.lon);
    println!("✓ Pontos amostrados: {}", profile.len());
    println!("\n  Distância → Elevação:");

    for (i, (distance, elevation)) in profile.iter().enumerate() {
        let bar_length = (elevation / 1000.0 * 30.0) as usize;
        let bar = "█".repeat(bar_length);

        if i % 2 == 0 { // Mostrar a cada 2 pontos
            println!("  {:.3} → {:6.1}m {}", distance, elevation, bar);
        }
    }

    // Calcular ganho/perda de elevação
    let mut gain = 0.0;
    let mut loss = 0.0;

    for i in 1..profile.len() {
        let diff = profile[i].1 - profile[i - 1].1;
        if diff > 0.0 {
            gain += diff;
        } else {
            loss += diff.abs();
        }
    }

    println!("\n✓ Ganho de elevação: {:.1}m", gain);
    println!("✓ Perda de elevação: {:.1}m", loss);
    println!();
}

fn demo_viewshed(dem: &DigitalElevationModel) {
    println!("=== 6. Análise de Visibilidade (Viewshed) ===");

    // Colocar observador no topo da montanha
    let observer_row = dem.rows / 2;
    let observer_col = dem.cols / 2;
    let observer_height = 2.0; // 2m acima do terreno (pessoa)

    let observer_elev = dem.data[observer_row][observer_col];
    println!("✓ Observador em:");
    println!("  Posição: linha {}, coluna {}", observer_row, observer_col);
    println!("  Elevação do terreno: {:.1}m", observer_elev);
    println!("  Altura do observador: {:.1}m", observer_height);
    println!("  Elevação total: {:.1}m", observer_elev + observer_height);

    println!("\n✓ Calculando viewshed...");
    let viewshed = dem.viewshed(observer_row, observer_col, observer_height);

    // Contar células visíveis
    let mut visible_count = 0;
    let mut total_count = 0;

    for row in &viewshed {
        for &visible in row {
            if visible {
                visible_count += 1;
            }
            total_count += 1;
        }
    }

    let visibility_percent = visible_count as f64 / total_count as f64 * 100.0;

    println!("✓ Células visíveis: {} de {} ({:.1}%)",
        visible_count, total_count, visibility_percent);
    println!();
}

fn demo_peak_detection(dem: &DigitalElevationModel) {
    println!("=== 7. Detecção de Picos ===");

    let min_prominence = 50.0; // Mínimo de 50m de proeminência
    let peaks = dem.find_peaks(min_prominence);

    println!("✓ Picos encontrados (proeminência > {:.0}m): {}", min_prominence, peaks.len());

    // Ordenar por elevação
    let mut sorted_peaks = peaks.clone();
    sorted_peaks.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    println!("\n✓ Top 5 picos mais altos:");
    for (i, (row, col, elevation)) in sorted_peaks.iter().take(5).enumerate() {
        let lat = dem.bounds.min_y + (*row as f64 + 0.5) * dem.resolution;
        let lon = dem.bounds.min_x + (*col as f64 + 0.5) * dem.resolution;

        println!("  {}. Elevação: {:.1}m | Posição: ({:.3}, {:.3})",
            i + 1, elevation, lat, lon);
    }
    println!();
}
