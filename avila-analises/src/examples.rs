/// Exemplos práticos de integração com AvilaDB
///
/// Este arquivo demonstra como usar o sistema de análise comportamental
/// em cenários reais de produção.

use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;

// ==================== EXEMPLO 1: E-commerce Completo ====================

pub async fn ecommerce_analytics_example() -> Result<()> {
    println!("=== E-commerce Analytics Example ===\n");

    // Simular integração com AvilaDB
    // let client = AvilaClient::connect("http://localhost:8000").await?;
    // let db = client.database("ecommerce").await?;
    // let events = db.collection("user_events").await?;

    // 1. Rastrear eventos do e-commerce
    let product_view = json!({
        "userId": "customer_123",
        "sessionId": "sess_abc",
        "timestamp": Utc::now(),
        "eventType": "product_view",
        "data": {
            "productId": "laptop_001",
            "category": "electronics",
            "price": 2500.0,
            "brand": "Dell"
        }
    });

    let add_to_cart = json!({
        "userId": "customer_123",
        "sessionId": "sess_abc",
        "timestamp": Utc::now(),
        "eventType": "add_to_cart",
        "data": {
            "productId": "laptop_001",
            "quantity": 1,
            "price": 2500.0
        }
    });

    // 2. Query de análise
    let query = r#"
        -- Análise de abandono de carrinho
        SELECT
            userId,
            COUNT(CASE WHEN eventType = 'add_to_cart' THEN 1 END) as carts,
            COUNT(CASE WHEN eventType = 'purchase' THEN 1 END) as purchases,
            (COUNT(CASE WHEN eventType = 'add_to_cart' THEN 1 END) -
             COUNT(CASE WHEN eventType = 'purchase' THEN 1 END)) as abandoned_carts
        FROM user_events
        WHERE timestamp > @start_date
        GROUP BY userId
        HAVING abandoned_carts > 0
    "#;

    println!("Query para identificar carrinhos abandonados:");
    println!("{}", query);

    Ok(())
}

// ==================== EXEMPLO 2: SaaS Analytics ====================

pub async fn saas_analytics_example() -> Result<()> {
    println!("\n=== SaaS Analytics Example ===\n");

    // Rastrear feature adoption
    let feature_usage = json!({
        "userId": "user_456",
        "timestamp": Utc::now(),
        "eventType": "feature_used",
        "data": {
            "featureName": "advanced_reporting",
            "duration_seconds": 120,
            "success": true
        }
    });

    // Query para análise de onboarding
    let onboarding_query = r#"
        -- Análise de sucesso do onboarding
        WITH user_cohorts AS (
            SELECT
                userId,
                MIN(timestamp) as signup_date,
                DATE_TRUNC('week', MIN(timestamp)) as cohort_week
            FROM user_events
            WHERE eventType = 'signup'
            GROUP BY userId
        ),
        activation_events AS (
            SELECT
                userId,
                MIN(timestamp) as activation_date
            FROM user_events
            WHERE eventType IN ('feature_used', 'project_created')
            GROUP BY userId
        )
        SELECT
            c.cohort_week,
            COUNT(DISTINCT c.userId) as signups,
            COUNT(DISTINCT a.userId) as activated,
            COUNT(DISTINCT a.userId) * 100.0 / COUNT(DISTINCT c.userId) as activation_rate,
            AVG(DATEDIFF(day, c.signup_date, a.activation_date)) as avg_days_to_activate
        FROM user_cohorts c
        LEFT JOIN activation_events a ON c.userId = a.userId
        GROUP BY c.cohort_week
        ORDER BY c.cohort_week DESC
    "#;

    println!("Query de análise de onboarding:");
    println!("{}", onboarding_query);

    Ok(())
}

// ==================== EXEMPLO 3: Gaming Analytics ====================

pub async fn gaming_analytics_example() -> Result<()> {
    println!("\n=== Gaming Analytics Example ===\n");

    // Rastrear sessões de jogo
    let game_session = json!({
        "userId": "player_789",
        "sessionId": "game_sess_xyz",
        "timestamp": Utc::now(),
        "eventType": "game_session",
        "data": {
            "gameMode": "multiplayer",
            "duration_minutes": 45,
            "score": 1250,
            "level_reached": 15,
            "deaths": 3,
            "kills": 12
        }
    });

    // Query para análise de retenção
    let retention_query = r#"
        -- Análise de retenção D1, D7, D30
        WITH first_play AS (
            SELECT
                userId,
                DATE(MIN(timestamp)) as install_date
            FROM game_events
            WHERE eventType = 'game_start'
            GROUP BY userId
        ),
        daily_active AS (
            SELECT DISTINCT
                userId,
                DATE(timestamp) as active_date
            FROM game_events
            WHERE eventType = 'game_start'
        )
        SELECT
            f.install_date,
            COUNT(DISTINCT f.userId) as installs,
            COUNT(DISTINCT CASE
                WHEN d.active_date = DATE_ADD(f.install_date, INTERVAL 1 DAY)
                THEN f.userId
            END) * 100.0 / COUNT(DISTINCT f.userId) as d1_retention,
            COUNT(DISTINCT CASE
                WHEN d.active_date = DATE_ADD(f.install_date, INTERVAL 7 DAY)
                THEN f.userId
            END) * 100.0 / COUNT(DISTINCT f.userId) as d7_retention,
            COUNT(DISTINCT CASE
                WHEN d.active_date = DATE_ADD(f.install_date, INTERVAL 30 DAY)
                THEN f.userId
            END) * 100.0 / COUNT(DISTINCT f.userId) as d30_retention
        FROM first_play f
        LEFT JOIN daily_active d ON f.userId = d.userId
        GROUP BY f.install_date
        ORDER BY f.install_date DESC
    "#;

    println!("Query de retenção para jogos:");
    println!("{}", retention_query);

    Ok(())
}

// ==================== EXEMPLO 4: Real-time Dashboard ====================

pub async fn realtime_dashboard_example() -> Result<()> {
    println!("\n=== Real-time Dashboard Example ===\n");

    // Query para métricas em tempo real
    let realtime_query = r#"
        -- Dashboard em tempo real (últimos 5 minutos)
        SELECT
            -- Usuários ativos
            COUNT(DISTINCT userId) as active_users,

            -- Eventos por segundo
            COUNT(*) / 300.0 as events_per_second,

            -- Taxa de conversão
            COUNT(CASE WHEN eventType = 'purchase' THEN 1 END) * 100.0 /
            COUNT(DISTINCT sessionId) as conversion_rate,

            -- Receita
            SUM(CASE WHEN eventType = 'purchase' THEN data.amount ELSE 0 END) as revenue,

            -- Top páginas
            ARRAY_AGG(
                DISTINCT data.url
                ORDER BY COUNT(*) DESC
                LIMIT 5
            ) as top_pages
        FROM user_events
        WHERE timestamp > DATETIME_SUB(NOW(), INTERVAL 5 MINUTE)
    "#;

    println!("Query para dashboard real-time:");
    println!("{}", realtime_query);

    Ok(())
}

// ==================== EXEMPLO 5: Machine Learning Integration ====================

pub async fn ml_integration_example() -> Result<()> {
    println!("\n=== ML Integration Example ===\n");

    // Preparar features para modelo de churn
    let feature_query = r#"
        -- Extrair features para predição de churn
        SELECT
            u.userId,

            -- Features de recência
            DATEDIFF(day, MAX(e.timestamp), NOW()) as days_since_last_activity,
            DATEDIFF(day, MIN(e.timestamp), NOW()) as account_age_days,

            -- Features de frequência
            COUNT(DISTINCT DATE(e.timestamp)) as active_days,
            COUNT(*) as total_events,
            COUNT(DISTINCT e.sessionId) as total_sessions,
            AVG(session_duration) as avg_session_duration,

            -- Features de monetary value
            COALESCE(SUM(CASE WHEN e.eventType = 'purchase' THEN e.data.amount END), 0) as total_spent,
            COALESCE(COUNT(CASE WHEN e.eventType = 'purchase' THEN 1 END), 0) as total_purchases,

            -- Features comportamentais
            COUNT(CASE WHEN e.eventType = 'search' THEN 1 END) as search_count,
            COUNT(CASE WHEN e.eventType = 'add_to_cart' THEN 1 END) as cart_adds,

            -- Target (churn)
            CASE
                WHEN DATEDIFF(day, MAX(e.timestamp), NOW()) > 30 THEN 1
                ELSE 0
            END as churned

        FROM users u
        LEFT JOIN user_events e ON u.userId = e.userId
        WHERE u.created_at < DATE_SUB(NOW(), INTERVAL 60 DAY)
        GROUP BY u.userId
    "#;

    println!("Query para preparar dataset de ML:");
    println!("{}", feature_query);

    // Usar modelo treinado
    println!("\nExemplo de uso do modelo:");
    println!("let churn_prob = predictor.predict_churn(&user_profile);");
    println!("if churn_prob > 0.7 {{");
    println!("    // Enviar campanha de retenção");
    println!("    send_retention_campaign(&user_profile);");
    println!("}}");

    Ok(())
}

// ==================== EXEMPLO 6: A/B Testing ====================

pub async fn ab_testing_example() -> Result<()> {
    println!("\n=== A/B Testing Example ===\n");

    // Rastrear variante
    let ab_event = json!({
        "userId": "user_999",
        "timestamp": Utc::now(),
        "eventType": "experiment_assigned",
        "data": {
            "experimentName": "checkout_redesign",
            "variant": "B",
            "assignmentTimestamp": Utc::now()
        }
    });

    // Query para análise de A/B test
    let ab_query = r#"
        -- Análise de teste A/B
        WITH experiment_users AS (
            SELECT
                userId,
                data.variant as variant,
                MIN(timestamp) as assignment_date
            FROM user_events
            WHERE eventType = 'experiment_assigned'
              AND data.experimentName = 'checkout_redesign'
            GROUP BY userId, data.variant
        ),
        conversions AS (
            SELECT
                userId,
                COUNT(*) as purchase_count,
                SUM(data.amount) as total_revenue
            FROM user_events
            WHERE eventType = 'purchase'
            GROUP BY userId
        )
        SELECT
            e.variant,
            COUNT(DISTINCT e.userId) as users_in_variant,
            COUNT(DISTINCT c.userId) as converted_users,
            COUNT(DISTINCT c.userId) * 100.0 / COUNT(DISTINCT e.userId) as conversion_rate,
            AVG(c.total_revenue) as avg_revenue_per_user,

            -- Estatística do teste
            STDDEV(CASE WHEN c.userId IS NOT NULL THEN 1 ELSE 0 END) as conversion_stddev
        FROM experiment_users e
        LEFT JOIN conversions c ON e.userId = c.userId
        GROUP BY e.variant
    "#;

    println!("Query para análise de A/B test:");
    println!("{}", ab_query);

    Ok(())
}

// ==================== Helper Functions ====================

pub fn print_integration_examples() {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║     EXEMPLOS DE INTEGRAÇÃO - AVILADB ANALYTICS        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Exemplos disponíveis:");
    println!("1. E-commerce Analytics");
    println!("2. SaaS Analytics");
    println!("3. Gaming Analytics");
    println!("4. Real-time Dashboard");
    println!("5. Machine Learning Integration");
    println!("6. A/B Testing");
    println!("\nExecute cada exemplo para ver queries e padrões de uso.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_examples() -> Result<()> {
        print_integration_examples();
        ecommerce_analytics_example().await?;
        saas_analytics_example().await?;
        gaming_analytics_example().await?;
        realtime_dashboard_example().await?;
        ml_integration_example().await?;
        ab_testing_example().await?;
        Ok(())
    }
}
