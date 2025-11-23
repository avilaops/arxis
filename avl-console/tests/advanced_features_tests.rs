//! Integration tests for advanced features

use avl_console::{Console, ConsoleConfig};

#[tokio::test]
async fn test_query_builder_module() {
    use avl_console::query_builder::*;

    // Test query execution simulation
    let (columns, rows) = simulate_query_execution("SELECT * FROM users").unwrap();
    assert_eq!(columns.len(), 3);
    assert!(columns.contains(&"id".to_string()));
    assert_eq!(rows.len(), 3);

    // Test products query
    let (columns, rows) = simulate_query_execution("SELECT * FROM products").unwrap();
    assert_eq!(columns.len(), 4);
    assert_eq!(rows.len(), 3);
}

#[tokio::test]
async fn test_monitoring_anomaly_detection() {
    use avl_console::monitoring::*;

    // Test normal values
    let historical = vec![100.0, 102.0, 98.0, 101.0, 99.0];
    assert!(!detect_anomaly(100.5, &historical, 2.0));
    assert!(!detect_anomaly(102.0, &historical, 2.0));

    // Test anomaly (spike)
    assert!(detect_anomaly(150.0, &historical, 2.0));
    assert!(detect_anomaly(50.0, &historical, 2.0));
    assert!(detect_anomaly(103.5, &historical, 2.0)); // Just over threshold

    // Test edge cases
    assert!(!detect_anomaly(100.0, &[], 2.0));
    assert!(!detect_anomaly(0.0, &[0.0], 2.0));
}

#[tokio::test]
async fn test_team_management_roles() {
    use avl_console::teams::*;

    // Test admin permissions
    let admin_perms = Role::Admin.default_permissions();
    assert!(admin_perms.contains(&Permission::ManageUsers));
    assert!(admin_perms.contains(&Permission::ManageTeams));
    assert!(admin_perms.contains(&Permission::ManageDatabase));
    assert_eq!(admin_perms.len(), 7);

    // Test developer permissions
    let dev_perms = Role::Developer.default_permissions();
    assert!(!dev_perms.contains(&Permission::ManageUsers));
    assert!(!dev_perms.contains(&Permission::ManageTeams));
    assert!(dev_perms.contains(&Permission::ManageDatabase));
    assert!(dev_perms.contains(&Permission::ManageStorage));
    assert_eq!(dev_perms.len(), 3);

    // Test viewer permissions
    let viewer_perms = Role::Viewer.default_permissions();
    assert!(!viewer_perms.contains(&Permission::ManageUsers));
    assert!(!viewer_perms.contains(&Permission::ManageDatabase));
    assert!(viewer_perms.contains(&Permission::ViewLogs));
    assert_eq!(viewer_perms.len(), 1);
}

#[tokio::test]
async fn test_user_permission_check() {
    use avl_console::teams::*;
    use std::collections::HashSet;

    let admin_user = User {
        id: "admin_1".to_string(),
        name: "Admin User".to_string(),
        email: "admin@test.com".to_string(),
        role: Role::Admin,
        teams: vec!["Engineering".to_string()],
        status: "active".to_string(),
        last_active: "now".to_string(),
        permissions: Role::Admin.default_permissions(),
    };

    assert!(has_permission(&admin_user, &Permission::ManageUsers));
    assert!(has_permission(&admin_user, &Permission::ManageDatabase));

    let viewer_user = User {
        id: "viewer_1".to_string(),
        name: "Viewer User".to_string(),
        email: "viewer@test.com".to_string(),
        role: Role::Viewer,
        teams: vec!["Marketing".to_string()],
        status: "active".to_string(),
        last_active: "now".to_string(),
        permissions: Role::Viewer.default_permissions(),
    };

    assert!(!has_permission(&viewer_user, &Permission::ManageUsers));
    assert!(!has_permission(&viewer_user, &Permission::ManageDatabase));
    assert!(has_permission(&viewer_user, &Permission::ViewLogs));
}

#[tokio::test]
async fn test_console_with_new_features() {
    let config = ConsoleConfig::default();
    let console = Console::new(config).await.unwrap();

    // Verify router can be built with new features
    let _router = console.router();

    // Test passes if no panics
}

#[tokio::test]
async fn test_query_template_structure() {
    use avl_console::query_builder::*;

    let template = QueryTemplate {
        id: "test_1".to_string(),
        name: "Test Query".to_string(),
        description: "Test description".to_string(),
        query: "SELECT * FROM test".to_string(),
        components: vec![],
        created_at: "2024-11-23T00:00:00Z".to_string(),
    };

    assert_eq!(template.id, "test_1");
    assert_eq!(template.name, "Test Query");
    assert!(template.components.is_empty());
}

#[tokio::test]
async fn test_metric_structure() {
    use avl_console::monitoring::*;

    let metric = Metric {
        name: "CPU Usage".to_string(),
        value: 75.5,
        unit: "%".to_string(),
        status: "normal".to_string(),
        change: 2.3,
        is_anomaly: false,
        threshold: 80.0,
    };

    assert_eq!(metric.name, "CPU Usage");
    assert!(!metric.is_anomaly);
    assert!(metric.value < metric.threshold);
}

#[tokio::test]
async fn test_alert_severity_levels() {
    use avl_console::monitoring::*;

    let critical_alert = Alert {
        id: "alert_1".to_string(),
        title: "Critical Error".to_string(),
        description: "System down".to_string(),
        severity: "critical".to_string(),
        icon: "🚨".to_string(),
        time: "now".to_string(),
        metric: "uptime".to_string(),
        value: 0.0,
    };

    assert_eq!(critical_alert.severity, "critical");

    let warning_alert = Alert {
        id: "alert_2".to_string(),
        title: "Warning".to_string(),
        description: "High load".to_string(),
        severity: "warning".to_string(),
        icon: "⚠️".to_string(),
        time: "now".to_string(),
        metric: "cpu".to_string(),
        value: 85.0,
    };

    assert_eq!(warning_alert.severity, "warning");
}

#[tokio::test]
async fn test_ml_insight_confidence() {
    use avl_console::monitoring::*;

    let insight = MLInsight {
        title: "Scaling Recommendation".to_string(),
        description: "Scale up recommended".to_string(),
        confidence: 0.95,
        recommendation: "Add 2 instances".to_string(),
    };

    assert!(insight.confidence > 0.9);
    assert!(insight.confidence <= 1.0);
}

#[tokio::test]
async fn test_audit_log_structure() {
    use avl_console::teams::*;

    let log = AuditLogEntry {
        id: "log_1".to_string(),
        action: "User Created".to_string(),
        details: "New user added to team".to_string(),
        icon: "👤".to_string(),
        user_id: "admin_1".to_string(),
        time: "2024-11-23T10:00:00Z".to_string(),
    };

    assert_eq!(log.action, "User Created");
    assert!(!log.user_id.is_empty());
}

#[tokio::test]
async fn test_ai_assistant_active_users_query() {
    use avl_console::ai_assistant::*;

    // Test AI processing for active users query
    let (response, sql, explanation, tips) =
        process_natural_language("quais são os 5 usuários mais ativos?");

    assert!(sql.is_some());
    let sql_query = sql.unwrap();
    assert!(sql_query.contains("SELECT"));
    assert!(sql_query.contains("users"));
    assert!(sql_query.contains("COUNT"));
    assert!(sql_query.contains("LIMIT 5"));
    assert!(response.contains("usuários mais ativos"));
    assert!(explanation.is_some());
    assert!(tips.is_some());
}

#[tokio::test]
async fn test_ai_assistant_sales_query() {
    use avl_console::ai_assistant::*;

    let (response, sql, explanation, tips) =
        process_natural_language("mostre o total de vendas por categoria");

    assert!(sql.is_some());
    let sql_query = sql.unwrap();
    assert!(sql_query.contains("categories"));
    assert!(sql_query.contains("SUM"));
    assert!(sql_query.contains("GROUP BY"));
    assert!(response.contains("vendas"));
    assert!(explanation.is_some());
    assert!(tips.is_some());
    assert!(tips.unwrap().len() > 0);
}

#[tokio::test]
async fn test_ai_assistant_pending_orders() {
    use avl_console::ai_assistant::*;

    let (response, sql, _, _) =
        process_natural_language("liste pedidos pendentes com valor acima de R$ 1000");

    assert!(sql.is_some());
    let sql_query = sql.unwrap();
    assert!(sql_query.contains("orders"));
    assert!(sql_query.contains("pending"));
    assert!(sql_query.contains("1000"));
    assert!(response.contains("pedidos"));
}

#[tokio::test]
async fn test_ai_assistant_optimization_tips() {
    use avl_console::ai_assistant::*;

    let (response, sql, explanation, tips) =
        process_natural_language("como posso otimizar minhas queries?");

    assert!(sql.is_none()); // Optimization request doesn't generate SQL
    assert!(response.contains("Otimização"));
    assert!(explanation.is_some());
    assert!(tips.is_some());

    let tips_vec = tips.unwrap();
    assert!(tips_vec.len() >= 2);
    assert!(tips_vec.iter().any(|t| t.contains("query log") || t.contains("profiling")));
}

#[tokio::test]
async fn test_ai_assistant_unknown_query() {
    use avl_console::ai_assistant::*;

    let (response, sql, _, _) =
        process_natural_language("xyz random nonsense text abc");

    assert!(sql.is_none());
    assert!(response.contains("detalhes") || response.contains("mais"));
}

#[tokio::test]
async fn test_ai_config_defaults() {
    use avl_console::ai_assistant::*;

    let config = AIConfig::default();
    assert_eq!(config.model, "gpt-4");
    assert_eq!(config.temperature, 0.7);
    assert_eq!(config.max_tokens, 1000);
}
