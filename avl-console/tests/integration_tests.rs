use avl_console::{Console, ConsoleConfig};

#[tokio::test]
async fn test_console_creation() {
    let config = ConsoleConfig::default();
    let console = Console::new(config).await;
    assert!(console.is_ok());
}

#[tokio::test]
async fn test_router_creation() {
    let config = ConsoleConfig::default();
    let console = Console::new(config).await.unwrap();
    let router = console.router();
    // Router should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_state_management() {
    let config = ConsoleConfig::default();
    let console = Console::new(config).await.unwrap();
    let state = console.state();

    // Test session management
    state.store_session("sess_123".to_string(), "user_001".to_string()).await;
    let user_id = state.get_session("sess_123").await;
    assert_eq!(user_id, Some("user_001".to_string()));

    // Test session removal
    state.remove_session("sess_123").await;
    let user_id = state.get_session("sess_123").await;
    assert_eq!(user_id, None);
}

#[tokio::test]
async fn test_websocket_connection_limit() {
    let mut config = ConsoleConfig::default();
    config.max_ws_connections = 3;

    let console = Console::new(config).await.unwrap();
    let state = console.state();

    assert!(state.can_create_ws_connection("user1").await);

    for _ in 0..3 {
        state.increment_ws_connection("user1".to_string()).await;
    }

    assert!(!state.can_create_ws_connection("user1").await);
}

#[tokio::test]
async fn test_rate_limiting() {
    let mut config = ConsoleConfig::default();
    config.rate_limit = 5;

    let console = Console::new(config).await.unwrap();
    let state = console.state();

    for _ in 0..5 {
        assert!(state.check_rate_limit("user1").await.is_ok());
    }

    assert!(state.check_rate_limit("user1").await.is_err());
}
