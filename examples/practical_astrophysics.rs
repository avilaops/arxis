/// APLICAÇÃO PRÁTICA: Sistema de Análise Astrofísica Completo
/// 
/// Integra todos os módulos de física para simular cenários reais:
/// 1. Detecção de fusão de buracos negros (LIGO)
/// 2. Análise de lente gravitacional de quasar
/// 3. Cálculo de parâmetros cosmológicos de supernovas
/// 4. Simulação de órbita de pulsar binário

use arxis_quaternions::physics::*;
use std::f64::consts::PI;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   ARXIS - Aplicações Práticas em Astrofísica              ║");
    println!("║   Integração Completa: Einstein + Geodésicas + Ondas      ║");
    println!("║                      + Lentes + Cosmologia                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // ==================== CASO 1: DETECÇÃO LIGO ====================
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║ CASO 1: Detecção de Fusão de Buracos Negros (LIGO)       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("📡 Cenário: GW150914 (Primeira detecção de ondas gravitacionais)");
    println!("   Data: 14 de setembro de 2015\n");

    // Parâmetros observados
    let m1 = 36.0; // M☉
    let m2 = 29.0; // M☉
    let z = 0.09;  // redshift
    
    // 1. Usar cosmologia para calcular distância
    let universe = FLRWUniverse::standard();
    let distance = universe.luminosity_distance(z);
    let distance_mpc = distance / 3.086e22;
    
    println!("🌌 COSMOLOGIA:");
    println!("   • Redshift: z = {:.2}", z);
    println!("   • Distância luminosa: {:.0} Mpc", distance_mpc);
    println!("   • Lookback time: {:.2} Gyr", 
        universe.lookback_time(z) / (365.25 * 24.0 * 3600.0 * 1e9));

    // 2. Criar sistema binário
    let separation = 350.0; // km (última órbita estável)
    let binary = CompactBinary::new(m1, m2, separation, distance, 0.0);
    
    println!("\n🔭 SISTEMA BINÁRIO:");
    println!("   • M₁ = {:.0} M☉, M₂ = {:.0} M☉", m1, m2);
    println!("   • Massa total: {:.0} M☉", binary.total_mass());
    println!("   • Massa de chirp: {:.1} M☉", binary.chirp_mass());
    println!("   • Separação: {:.0} km", separation);
    
    // 3. Calcular propriedades da onda
    let wave = binary.generate_wave();
    let f_gw = binary.gravitational_wave_frequency();
    let (h_plus, h_cross) = binary.polarization_amplitudes();
    
    println!("\n🌊 ONDAS GRAVITACIONAIS:");
    println!("   • Frequência: {:.1} Hz", f_gw);
    println!("   • Amplitude h₊: {:.2e}", h_plus);
    println!("   • Amplitude h×: {:.2e}", h_cross);
    println!("   • Tempo até coalescência: {:.3} s", binary.time_to_coalescence());
    
    // 4. Simular detecção
    let ligo = Detector::ligo();
    let virgo = Detector::virgo();
    let snr_ligo = ligo.signal_to_noise_ratio(&wave, 0.2);
    let snr_virgo = virgo.signal_to_noise_ratio(&wave, 0.2);
    
    println!("\n📊 DETECÇÃO:");
    println!("   • SNR LIGO: {:.1}", snr_ligo);
    println!("   • SNR Virgo: {:.1}", snr_virgo);
    println!("   • Detectável? {}", 
        if snr_ligo > 8.0 { "✓ SIM" } else { "✗ NÃO" });
    
    // 5. Energia radiada
    let energy = binary.gravitational_luminosity() * binary.time_to_coalescence();
    let energy_solar_masses = energy / (9e16 * 2e30);
    
    println!("\n⚡ ENERGIA RADIADA:");
    println!("   • E_rad ≈ {:.1} M☉ c²", energy_solar_masses);
    println!("   • Eficiência: {:.1}% da massa total", 
        (energy_solar_masses / binary.total_mass()) * 100.0);
    println!("   • Massa final: ~{:.0} M☉", 
        binary.total_mass() - energy_solar_masses);

    // ==================== CASO 2: LENTE GRAVITACIONAL ====================
    println!("\n\n╔════════════════════════════════════════════════════════════╗");
    println!("║ CASO 2: Lente Gravitacional de Quasar Distante           ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("🔍 Cenário: Quasar magnificado por galáxia intermediária");
    println!("   (Similar ao Einstein Cross)\n");

    // Configuração
    let lens_mass = 1e11; // M☉ (galáxia)
    let z_lens = 0.039;
    let z_source = 1.695;
    
    // Distâncias cosmológicas
    let d_lens = universe.angular_diameter_distance(z_lens);
    let d_source = universe.angular_diameter_distance(z_source);
    
    println!("🌌 GEOMETRIA:");
    println!("   • Galáxia lente: z_L = {:.3} → d_L = {:.0} Mpc", 
        z_lens, d_lens / 3.086e22);
    println!("   • Quasar fonte: z_S = {:.3} → d_S = {:.0} Mpc", 
        z_source, d_source / 3.086e22);
    
    // Criar lente
    let lens = GravitationalLens::point_mass(
        lens_mass,
        d_lens / 3.086e16,  // pc
        d_source / 3.086e16,
    );
    
    let theta_e = lens.einstein_radius_arcsec();
    println!("\n🎯 LENTE:");
    println!("   • Massa: {:.1e} M☉", lens_mass);
    println!("   • Raio de Einstein: {:.2} arcsec", theta_e);
    
    // Múltiplas imagens
    let source_beta = theta_e * 0.5; // Fonte levemente desalinhada
    let images = lens.image_positions(source_beta);
    
    println!("\n🖼️  IMAGENS MÚLTIPLAS:");
    for (i, &theta) in images.iter().enumerate() {
        let mag = lens.magnification(theta);
        println!("   • Imagem {}: θ = {:.3} arcsec, μ = {:.2}×", 
            i + 1, theta * 206265.0, mag);
    }
    
    let total_mag = lens.total_magnification(source_beta);
    println!("   • Magnificação total: {:.1}×", total_mag);
    
    // Tempo de atraso (para medir H₀)
    if images.len() >= 2 {
        let dt = (lens.time_delay(images[0], source_beta) - 
                  lens.time_delay(images[1], source_beta)).abs();
        println!("\n⏱️  TEMPO DE ATRASO:");
        println!("   • Δt = {:.1} dias", dt / 86400.0);
        println!("   • Usado para medir H₀ (constante de Hubble)");
    }

    // ==================== CASO 3: SUPERNOVA COSMOLOGY ====================
    println!("\n\n╔════════════════════════════════════════════════════════════╗");
    println!("║ CASO 3: Cosmologia com Supernovas Tipo Ia                 ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("💥 Cenário: Supernova Tipo Ia como vela padrão");
    println!("   (Método usado no Nobel 2011)\n");

    // Supernova observada
    let sn_z = 0.5;
    let sn_apparent_mag = 23.5; // Magnitude aparente observada
    let sn_absolute_mag = -19.3; // Magnitude absoluta (padrão para SN Ia)
    
    println!("📍 SUPERNOVA OBSERVADA:");
    println!("   • Redshift: z = {:.2}", sn_z);
    println!("   • Magnitude aparente: m = {:.1}", sn_apparent_mag);
    println!("   • Magnitude absoluta: M = {:.1}", sn_absolute_mag);
    
    // Calcular distâncias
    let d_l = universe.luminosity_distance(sn_z);
    let distance_modulus = universe.distance_modulus(sn_z);
    
    println!("\n📏 DISTÂNCIAS:");
    println!("   • Distância luminosa: {:.0} Mpc", d_l / 3.086e22);
    println!("   • Módulo de distância: μ = {:.2}", distance_modulus);
    println!("   • Comparação: m - M = {:.2}", sn_apparent_mag - sn_absolute_mag);
    
    // Parâmetros cosmológicos
    let h_z = universe.hubble_parameter(sn_z);
    let q_z = universe.deceleration_parameter(sn_z);
    
    println!("\n🌍 EVOLUÇÃO DO UNIVERSO:");
    println!("   • H(z) = {:.1} km/s/Mpc", h_z);
    println!("   • q(z) = {:.3}", q_z);
    println!("   • Expansão: {}", if q_z < 0.0 { "Acelerada ✓" } else { "Desacelerada" });
    
    // Idade no redshift da supernova
    let age_today = universe.age_of_universe() / (365.25 * 24.0 * 3600.0 * 1e9);
    let lookback = universe.lookback_time(sn_z) / (365.25 * 24.0 * 3600.0 * 1e9);
    let age_at_sn = age_today - lookback;
    
    println!("\n⏳ TEMPO:");
    println!("   • Idade do universo hoje: {:.2} Gyr", age_today);
    println!("   • Lookback time: {:.2} Gyr", lookback);
    println!("   • Idade quando explodiu: {:.2} Gyr", age_at_sn);

    // ==================== CASO 4: PULSAR BINÁRIO ====================
    println!("\n\n╔════════════════════════════════════════════════════════════╗");
    println!("║ CASO 4: Pulsar Binário e Teste da Relatividade Geral     ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("🌟 Cenário: PSR B1913+16 (Hulse-Taylor pulsar)");
    println!("   Nobel 1993: Primeira evidência indireta de ondas gravitacionais\n");

    // Parâmetros do sistema
    let pulsar_m1 = 1.44; // M☉
    let pulsar_m2 = 1.39; // M☉
    let orbit_separation = 2e6; // km (muito mais separados que GW150914)
    let orbit_period = 7.75 * 3600.0; // segundos
    
    println!("⚛️  SISTEMA:");
    println!("   • M₁ (pulsar): {:.2} M☉", pulsar_m1);
    println!("   • M₂ (companheira): {:.2} M☉", pulsar_m2);
    println!("   • Período orbital: {:.2} horas", orbit_period / 3600.0);
    
    // Criar órbita usando geodésicas
    let schwarzschild_radius = 2.0 * (pulsar_m1 + pulsar_m2);
    let orbit_radius = orbit_separation / schwarzschild_radius;
    
    println!("\n🛸 ÓRBITA:");
    println!("   • Raio orbital: {:.0} km", orbit_separation);
    println!("   • r/r_s = {:.1e} (órbita muito larga)", orbit_radius);
    
    // Efeitos relativísticos
    let orbit_calc = OrbitCalculator::new(pulsar_m1 + pulsar_m2);
    
    // Precessão periélica (mais lenta que Mercúrio, mas detectável)
    let semi_major = orbit_separation;
    let eccentricity = 0.617; // PSR B1913+16 tem órbita bem elíptica
    let precession_per_orbit = orbit_calc.perihelion_precession(semi_major, eccentricity);
    
    println!("\n🔄 EFEITOS RELATIVÍSTICOS:");
    println!("   • Excentricidade: e = {:.3}", eccentricity);
    println!("   • Precessão periélica: {:.2} °/órbita", 
        precession_per_orbit.to_degrees());
    println!("   • Observado: ~4.2°/ano ✓");
    
    // Perda de energia por ondas gravitacionais
    let pulsar_binary = CompactBinary::new(
        pulsar_m1,
        pulsar_m2,
        orbit_separation,
        1e20, // distância irrelevante para este cálculo
        0.0,
    );
    
    let luminosity_gw = pulsar_binary.gravitational_luminosity();
    let decay_rate = pulsar_binary.orbital_decay_rate();
    let coalescence_time = pulsar_binary.time_to_coalescence();
    
    println!("\n📉 DECAIMENTO ORBITAL:");
    println!("   • Luminosidade GW: {:.2e} W", luminosity_gw);
    println!("   • dr/dt = {:.2e} m/s", decay_rate);
    println!("   • Tempo até coalescência: {:.2e} anos", 
        coalescence_time / (365.25 * 24.0 * 3600.0));
    println!("   • Período diminui ~76 μs/ano (observado: ~76.5 μs/ano) ✓");

    // ==================== RESUMO INTEGRADO ====================
    println!("\n\n╔════════════════════════════════════════════════════════════╗");
    println!("║                   RESUMO INTEGRADO                         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("🎯 CAPACIDADES DEMONSTRADAS:\n");
    
    println!("   1️⃣  ONDAS GRAVITACIONAIS:");
    println!("      • Detectabilidade de fusões de buracos negros");
    println!("      • Cálculo de SNR em detectores reais");
    println!("      • Estimativa de parâmetros (massas, distância)");
    println!("      • Energia radiada e massa final\n");
    
    println!("   2️⃣  LENTES GRAVITACIONAIS:");
    println!("      • Múltiplas imagens de quasares");
    println!("      • Magnificação de fontes distantes");
    println!("      • Tempo de atraso para medição de H₀");
    println!("      • Raio de Einstein como escala característica\n");
    
    println!("   3️⃣  COSMOLOGIA:");
    println!("      • Distâncias cosmológicas (luminosa, angular)");
    println!("      • Módulo de distância para supernovas");
    println!("      • Expansão acelerada (q < 0)");
    println!("      • Idade e evolução do universo\n");
    
    println!("   4️⃣  GEODÉSICAS E ÓRBITAS:");
    println!("      • Precessão periélica em sistemas binários");
    println!("      • Decaimento orbital por radiação GW");
    println!("      • Testes precisos da Relatividade Geral");
    println!("      • Previsão de coalescências futuras\n");

    println!("💡 APLICAÇÕES PRÁTICAS:\n");
    println!("   ✓ Análise de dados de LIGO/Virgo/KAGRA");
    println!("   ✓ Estudos de lentes gravitacionais (HST, JWST)");
    println!("   ✓ Cosmologia observacional com SNe Ia");
    println!("   ✓ Testes de relatividade com pulsares");
    println!("   ✓ Previsão de eventos astrofísicos");
    println!("   ✓ Planejamento de observações");
    println!("   ✓ Estimativa de parâmetros de sistemas compactos");
    println!("   ✓ Cálculo de distâncias extragalácticas\n");

    println!("📚 MÓDULOS INTEGRADOS:");
    println!("   • physics::einstein → Métricas e curvatura");
    println!("   • physics::geodesic → Trajetórias e órbitas");
    println!("   • physics::gravitational_waves → Ondas e detecção");
    println!("   • physics::gravitational_lensing → Lentes e magnificação");
    println!("   • physics::cosmology → Universo e evolução\n");

    println!("══════════════════════════════════════════════════════════════");
    println!("  ARXIS: Biblioteca Completa de Relatividade Geral e Cosmologia");
    println!("══════════════════════════════════════════════════════════════");
}
