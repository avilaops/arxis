mod models;
mod tracker;
mod funnel;
mod cohort;
mod segmentation;
mod prediction;
mod dashboard;
mod storage;
mod api;
mod websocket;
mod export;
mod industry40;

use chrono::{Duration, Utc};
use models::*;
use std::collections::HashMap;
use tracing_subscriber;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║   SISTEMA DE ANÁLISE COMPORTAMENTAL DIGITAL          ║");
    println!("║   Powered by AvilaDB - Rust Analytics Engine         ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // ========== 1. CONFIGURAR SISTEMA DE TRACKING ==========
    println!("🚀 Iniciando sistema de tracking...\n");

    let mut tracker = tracker::BehaviorTracker::new(30); // 30 min timeout

    // ========== 2. SIMULAR EVENTOS DE USUÁRIOS ==========
    println!("📊 Simulando eventos de comportamento digital...\n");

    simulate_user_behavior(&mut tracker).await?;

    // Aguardar para acumular dados
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // ========== 3. ANÁLISE DE FUNIL ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 ANÁLISE DE FUNIL DE CONVERSÃO");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let funnel_analyzer = funnel::FunnelAnalyzer::new();
    let ecommerce_funnel = funnel::FunnelAnalyzer::create_ecommerce_funnel();

    let all_events = tracker.get_event_store().get_all_events();
    let funnel_analysis = funnel_analyzer.analyze_funnel(&ecommerce_funnel, &all_events);

    funnel_analyzer.print_funnel_report(&funnel_analysis);

    // ========== 4. ANÁLISE DE PERFIS ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("👥 ANÁLISE DE PERFIS DE USUÁRIO");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let user_profiles = generate_user_profiles(&all_events);

    println!("Total de perfis criados: {}", user_profiles.len());

    // Exibir alguns perfis
    for (i, profile) in user_profiles.iter().take(3).enumerate() {
        println!("\n  Perfil #{}: {}", i + 1, profile.user_id);
        println!("    Engajamento: {:.2}", profile.engagement_score);
        println!("    Risco de Churn: {:.2}", profile.churn_risk);
        println!("    Prob. Conversão: {:.2}", profile.conversion_probability);
        println!("    Total gasto: R$ {:.2}", profile.behaviors.total_spent);
        println!("    Sessões: {}", profile.total_sessions);
    }

    // ========== 5. SEGMENTAÇÃO DE USUÁRIOS ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 SEGMENTAÇÃO RFM DE USUÁRIOS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let segmentation = segmentation::UserSegmentation::with_default_segments();
    segmentation.print_segmentation_report(&user_profiles);

    // ========== 6. ANÁLISE DE COHORT ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 ANÁLISE DE RETENÇÃO POR COHORT");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cohort_analyzer = cohort::CohortAnalyzer::new();
    let cohort_period = cohort::CohortBuilder::weekly().build();
    let cohorts = cohort_analyzer.create_cohorts(&user_profiles, cohort_period);

    let cohort_analysis = cohort_analyzer.analyze_cohorts(&cohorts, &all_events, 8);
    cohort_analyzer.print_cohort_report(&cohort_analysis);

    // ========== 7. PREDIÇÕES COM ML ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🤖 PREDIÇÕES COM MACHINE LEARNING");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut predictor = prediction::BehaviorPredictor::new();
    predictor.train_recommendation_model(&all_events);

    println!("Predições para usuários:");
    for profile in user_profiles.iter().take(5) {
        let churn_score = predictor.predict_churn(profile);
        let conversion_score = predictor.predict_conversion(profile);
        let recommendations = predictor.recommend_products(&profile.user_id, 3);

        println!("\n  Usuário: {}", profile.user_id);
        println!("    Risco de Churn: {:.2}%", churn_score * 100.0);
        println!("    Prob. Conversão: {:.2}%", conversion_score * 100.0);

        if !recommendations.is_empty() {
            println!("    Recomendações:");
            for (i, (product, score)) in recommendations.iter().enumerate() {
                println!("      {}. {} (score: {:.2})", i + 1, product, score);
            }
        }
    }

    // ========== 8. DASHBOARD EM TEMPO REAL ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 DASHBOARD EM TEMPO REAL");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let event_store = tracker.get_event_store().clone();
    let dashboard = dashboard::DashboardBuilder::new(event_store)
        .with_default_alerts()
        .build();

    // Iniciar monitoramento
    dashboard.start_monitoring().await;

    // Aguardar para coletar métricas
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Exibir dashboard
    dashboard.print_dashboard().await;

    // Obter snapshot de métricas
    let snapshot = dashboard.get_metrics_snapshot().await;
    println!("\n📸 Snapshot de Métricas:");
    println!("  Timestamp: {}", snapshot.timestamp.format("%Y-%m-%d %H:%M:%S"));
    println!("  Usuários Ativos: {}", snapshot.active_users_now);
    println!("  Eventos (última hora): {}", snapshot.events_last_hour);
    println!("  Eventos (hoje): {}", snapshot.events_today);
    println!("  Receita (hoje): R$ {:.2}", snapshot.revenue_today);
    println!("  Taxa de Conversão: {:.2}%", snapshot.conversion_rate_today * 100.0);
    println!("  Duração Média de Sessão: {:.0}s", snapshot.avg_session_duration);

    // ========== 9. ESTATÍSTICAS FINAIS ==========
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 ESTATÍSTICAS FINAIS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let (total_events, events_by_type) = tracker.get_real_time_stats().await;

    println!("Total de eventos rastreados: {}", total_events);
    println!("\nEventos por tipo:");

    let mut sorted_events: Vec<_> = events_by_type.iter().collect();
    sorted_events.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (event_type, count) in sorted_events {
        println!("  {}: {} eventos", event_type, count);
    }

    println!("\n✅ Sistema de análise comportamental executado com sucesso!");
    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║   PRONTO PARA INTEGRAÇÃO COM AVILADB!                ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    Ok(())
}

// ========== FUNÇÕES AUXILIARES ==========

/// Simular comportamento de usuários
async fn simulate_user_behavior(tracker: &mut tracker::BehaviorTracker) -> anyhow::Result<()> {
    let now = Utc::now();
    let users = vec!["user_001", "user_002", "user_003", "user_004", "user_005"];

    for user_id in &users {
        let session_id = Uuid::new_v4().to_string();

        // Simular jornada do usuário

        // 1. Visita à página inicial
        let event = create_event(
            user_id,
            &session_id,
            EventType::PageView {
                url: "/home".to_string(),
                title: "Home".to_string(),
                duration_ms: 5000,
            },
            now,
        );
        tracker.track_event(event).await?;

        // 2. Busca por produto
        let event = create_event(
            user_id,
            &session_id,
            EventType::Search {
                query: "laptop".to_string(),
                results_count: 15,
            },
            now + Duration::seconds(10),
        );
        tracker.track_event(event).await?;

        // 3. Visualizar produto
        let event = create_event(
            user_id,
            &session_id,
            EventType::PageView {
                url: "/product/laptop-abc".to_string(),
                title: "Laptop ABC".to_string(),
                duration_ms: 15000,
            },
            now + Duration::seconds(20),
        );
        tracker.track_event(event).await?;

        // 4. Alguns usuários adicionam ao carrinho
        if user_id == &"user_001" || user_id == &"user_003" || user_id == &"user_005" {
            let event = create_event(
                user_id,
                &session_id,
                EventType::AddToCart {
                    product_id: "laptop-abc".to_string(),
                    quantity: 1,
                },
                now + Duration::seconds(30),
            );
            tracker.track_event(event).await?;

            // 5. Checkout
            let event = create_event(
                user_id,
                &session_id,
                EventType::PageView {
                    url: "/checkout".to_string(),
                    title: "Checkout".to_string(),
                    duration_ms: 10000,
                },
                now + Duration::seconds(40),
            );
            tracker.track_event(event).await?;
        }

        // 6. Alguns concluem a compra
        if user_id == &"user_001" || user_id == &"user_005" {
            let event = create_event(
                user_id,
                &session_id,
                EventType::Purchase {
                    product_id: "laptop-abc".to_string(),
                    amount: 2500.0,
                    currency: "BRL".to_string(),
                },
                now + Duration::seconds(50),
            );
            tracker.track_event(event).await?;
        }
    }

    println!("✅ Simulados {} usuários com jornadas completas", users.len());

    Ok(())
}

/// Criar evento de teste
fn create_event(
    user_id: &str,
    session_id: &str,
    event_type: EventType,
    timestamp: chrono::DateTime<Utc>,
) -> BehaviorEvent {
    BehaviorEvent {
        event_id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        session_id: session_id.to_string(),
        timestamp,
        event_type,
        metadata: HashMap::new(),
        context: create_context(),
    }
}

/// Criar contexto de evento
fn create_context() -> EventContext {
    EventContext {
        device: DeviceInfo {
            device_type: DeviceType::Desktop,
            os: "Windows".to_string(),
            browser: "Chrome".to_string(),
            screen_resolution: (1920, 1080),
        },
        location: LocationInfo {
            country: "BR".to_string(),
            city: Some("São Paulo".to_string()),
            timezone: "America/Sao_Paulo".to_string(),
            ip_address: "192.168.1.100".to_string(),
        },
        referrer: Some("https://google.com".to_string()),
        user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string(),
        viewport: Viewport {
            width: 1920,
            height: 1080,
        },
    }
}

/// Gerar perfis de usuário a partir de eventos
fn generate_user_profiles(events: &[BehaviorEvent]) -> Vec<UserProfile> {
    let mut user_events: HashMap<String, Vec<&BehaviorEvent>> = HashMap::new();

    for event in events {
        user_events
            .entry(event.user_id.clone())
            .or_insert_with(Vec::new)
            .push(event);
    }

    user_events
        .iter()
        .map(|(user_id, user_events)| {
            let first_seen = user_events
                .iter()
                .map(|e| e.timestamp)
                .min()
                .unwrap_or_else(Utc::now);

            let last_seen = user_events
                .iter()
                .map(|e| e.timestamp)
                .max()
                .unwrap_or_else(Utc::now);

            let total_purchases = user_events
                .iter()
                .filter(|e| matches!(e.event_type, EventType::Purchase { .. }))
                .count();

            let total_spent: f64 = user_events
                .iter()
                .filter_map(|e| {
                    if let EventType::Purchase { amount, .. } = e.event_type {
                        Some(amount)
                    } else {
                        None
                    }
                })
                .sum();

            let days_since_last_purchase = if total_purchases > 0 {
                Some((Utc::now() - last_seen).num_days())
            } else {
                None
            };

            let engagement_score = calculate_engagement_score(user_events);
            let churn_risk = calculate_churn_risk(days_since_last_purchase, total_purchases);

            UserProfile {
                user_id: user_id.clone(),
                first_seen,
                last_seen,
                total_sessions: user_events
                    .iter()
                    .map(|e| &e.session_id)
                    .collect::<std::collections::HashSet<_>>()
                    .len(),
                total_events: user_events.len(),
                behaviors: UserBehaviors {
                    avg_session_duration_seconds: 120.0,
                    avg_pages_per_session: 3.5,
                    bounce_rate: 0.3,
                    conversion_rate: if total_purchases > 0 { 0.5 } else { 0.0 },
                    most_active_hours: vec![],
                    most_active_days: vec![],
                    total_purchases,
                    total_spent,
                    avg_order_value: if total_purchases > 0 {
                        total_spent / total_purchases as f64
                    } else {
                        0.0
                    },
                    days_since_last_purchase,
                    pages_viewed: std::collections::HashSet::new(),
                    search_queries: vec![],
                    clicked_products: vec![],
                },
                segments: vec![],
                engagement_score,
                loyalty_score: if total_purchases > 2 { 0.8 } else { 0.4 },
                conversion_probability: engagement_score * 0.7,
                churn_risk,
                interests: vec![],
                preferred_categories: HashMap::new(),
                browsing_patterns: BrowsingPatterns::default(),
            }
        })
        .collect()
}

fn calculate_engagement_score(events: &[&BehaviorEvent]) -> f64 {
    let page_views = events.iter().filter(|e| matches!(e.event_type, EventType::PageView { .. })).count();
    let interactions = events.len();

    (interactions as f64 / 10.0).min(1.0)
}

fn calculate_churn_risk(days_since_last_purchase: Option<i64>, total_purchases: usize) -> f64 {
    if total_purchases == 0 {
        return 0.5;
    }

    if let Some(days) = days_since_last_purchase {
        (days as f64 / 90.0).min(1.0)
    } else {
        0.5
    }
}
