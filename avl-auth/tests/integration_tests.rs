use avl_auth::{AuthClient, Config, Credentials};

fn generate_test_keys() -> (String, String) {
    let private_key = r#"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC7VJTUt9Us8cKj
MzEfYyjiWA4R4/M2bS1+fWIcPm15A8UDXw01lbKOCOIhPUJPAydvSnPreWvRaINV
mBPMh8dXKDIXU+dK3Rk6NdKFLLTvDEu9e+iFW1MhxjcKPE8LmXGF4QnLxMC6ydCg
DcRxKW6sG0Ut0m2rKOREV3L2KqTX5BLjUpfHTjHDHyxZwBGH2wCTXzkCIZE9TH0r
SEvT1xvJDqPJnNThhpqZGjCpKcEJBGZ8uURhXKH2TwlZGE5K5nLxNkECJQAqPB1k
IfhJxw2wJCxPnhL4r5fgq5xvHfDwWQPiMzYA8VqCANQVr0FKa8PqZrCcKxMJp4qj
fZ2mXvPRAgMBAAECggEADL0+gIrQg4RsqQ2wLKE9kF3R/Ep3BoNx0JDRXjPQ8dxK
-----END PRIVATE KEY-----"#;

    let public_key = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAu1SU1LfVLPHCozMxH2Mo
4lgOEePzNm0tfn1iHD5teQPFA18NNZW yjgjiIT1CTwMnb0pz63lr0WiDVZgTzIfH
VygyF1PnSt0ZOjXShSy07wxLvXvohVtTIcY3CjxPC5lxheEJy8TAusnQoA3EcSlu
rBtFLdJtqyjkRFdy9iqk1+QS41KXx04xwx8sWcARh9sAk185AiGRPUx9K0hL09cb
yQ6jyZzU4YaamRowqSnBCQRmfLlEYVyh9k8JWRhOSuZy8TZBAiUAKjwdZCH4SccN
sCQsT54S+K+X4Kucbx3w8FkD4jM2APFaggDUFa9BSmvD6mawnCsTCaeKo32dpl7z
0QIDAQAB
-----END PUBLIC KEY-----"#;

    (private_key.to_string(), public_key.to_string())
}

async fn create_test_client() -> AuthClient {
    let (private_key, public_key) = generate_test_keys();

    let mut config = Config::default();
    config.jwt.private_key = private_key;
    config.jwt.public_key = public_key;
    config.jwt.algorithm = "RS256".to_string();

    AuthClient::new(config).await.unwrap()
}

#[tokio::test]
async fn test_user_registration() {
    let client = create_test_client().await;

    let result = client.register(
        "test@example.com".to_string(),
        "SecureP@ssw0rd123!".to_string(),
    ).await;

    assert!(result.is_ok());
    let user_id = result.unwrap();
    assert!(!user_id.is_nil());
}

#[tokio::test]
async fn test_duplicate_registration() {
    let client = create_test_client().await;
    let email = "duplicate@example.com".to_string();
    let password = "SecureP@ssw0rd123!".to_string();

    client.register(email.clone(), password.clone()).await.unwrap();
    let result = client.register(email, password).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_weak_password() {
    let client = create_test_client().await;

    let result = client.register(
        "test@example.com".to_string(),
        "weak".to_string(),
    ).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_successful_login() {
    let client = create_test_client().await;
    let email = "login@example.com".to_string();
    let password = "SecureP@ssw0rd123!".to_string();

    client.register(email.clone(), password.clone()).await.unwrap();

    let credentials = Credentials {
        email,
        password,
        device_id: Some("test_device".to_string()),
        ip_address: Some("127.0.0.1".parse().unwrap()),
    };

    let result = client.login(credentials).await;
    assert!(result.is_ok());

    let session = result.unwrap();
    assert!(!session.access_token.is_empty());
    assert!(!session.refresh_token.is_empty());
}

#[tokio::test]
async fn test_invalid_credentials() {
    let client = create_test_client().await;
    let email = "test@example.com".to_string();

    client.register(email.clone(), "SecureP@ssw0rd123!".to_string()).await.unwrap();

    let credentials = Credentials {
        email,
        password: "WrongPassword".to_string(),
        device_id: None,
        ip_address: None,
    };

    let result = client.login(credentials).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_jwt_token_flow() {
    let client = create_test_client().await;
    let email = "jwt@example.com".to_string();
    let password = "SecureP@ssw0rd123!".to_string();

    client.register(email.clone(), password.clone()).await.unwrap();

    let credentials = Credentials {
        email: email.clone(),
        password,
        device_id: None,
        ip_address: None,
    };

    let session = client.login(credentials).await.unwrap();

    // Verify token
    let claims = client.verify_token(&session.access_token).await.unwrap();
    assert_eq!(claims.email, email);
}

#[tokio::test]
async fn test_session_management() {
    let client = create_test_client().await;
    let email = "session@example.com".to_string();
    let password = "SecureP@ssw0rd123!".to_string();

    client.register(email.clone(), password.clone()).await.unwrap();

    let credentials = Credentials {
        email,
        password,
        device_id: Some("device_1".to_string()),
        ip_address: Some("127.0.0.1".parse().unwrap()),
    };

    let session = client.login(credentials).await.unwrap();

    // Validate session
    let validated = client.session_manager()
        .validate_session(&session.id, Some("127.0.0.1".parse().unwrap()), Some("device_1"))
        .await;

    assert!(validated.is_ok());

    // Logout
    client.logout(&session.id).await.unwrap();

    // Session should no longer exist
    let result = client.session_manager()
        .get_session(&session.id)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_password_strength() {
    let client = create_test_client().await;

    let weak = client.password_manager().calculate_strength("12345");
    let _medium = client.password_manager().calculate_strength("Password123");
    let strong = client.password_manager().calculate_strength("SecureP@ssw0rd123!");

    assert!(matches!(weak, avl_auth::password::PasswordStrength::Weak));
    assert!(matches!(strong, avl_auth::password::PasswordStrength::Strong | avl_auth::password::PasswordStrength::VeryStrong));
}

#[tokio::test]
async fn test_mfa_totp() {
    let client = create_test_client().await;

    let config = client.mfa_manager().generate_totp_config("test@example.com", None);
    assert!(!config.secret.is_empty());

    let uri = client.mfa_manager().generate_totp_uri(&config);
    assert!(uri.starts_with("otpauth://totp/"));
}

#[tokio::test]
async fn test_api_key_generation() {
    let client = create_test_client().await;

    let user_id = uuid::Uuid::new_v4();
    let result = client.api_key_manager()
        .generate_api_key(
            user_id,
            "Test API Key".to_string(),
            Some("For testing".to_string()),
            vec!["read".to_string(), "write".to_string()],
            Some(100),
            None,
        )
        .await;

    assert!(result.is_ok());
    let (key, api_key) = result.unwrap();

    assert!(key.starts_with("avl_"));
    assert_eq!(api_key.user_id, user_id);
    assert_eq!(api_key.scopes, vec!["read", "write"]);
}

#[tokio::test]
async fn test_api_key_verification() {
    let client = create_test_client().await;

    let user_id = uuid::Uuid::new_v4();
    let (key, _) = client.api_key_manager()
        .generate_api_key(
            user_id,
            "Test".to_string(),
            None,
            vec![],
            None,
            None,
        )
        .await
        .unwrap();

    let verified = client.api_key_manager()
        .verify_api_key(&key)
        .await;

    assert!(verified.is_ok());
    assert_eq!(verified.unwrap().user_id, user_id);
}

#[tokio::test]
async fn test_api_key_revocation() {
    let client = create_test_client().await;

    let (key, api_key) = client.api_key_manager()
        .generate_api_key(
            uuid::Uuid::new_v4(),
            "Test".to_string(),
            None,
            vec![],
            None,
            None,
        )
        .await
        .unwrap();

    // Revoke the key
    client.api_key_manager()
        .revoke_api_key(&api_key.id)
        .await
        .unwrap();

    // Verification should fail
    let result = client.api_key_manager()
        .verify_api_key(&key)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_backup_codes() {
    let client = create_test_client().await;

    let codes = client.mfa_manager().generate_backup_codes(10);
    assert_eq!(codes.len(), 10);

    for code in codes {
        assert_eq!(code.len(), 9); // Format: XXXX-XXXX
        assert!(code.contains('-'));
    }
}

#[tokio::test]
async fn test_crypto_operations() {
    let client = create_test_client().await;

    let token = client.crypto_manager().generate_token(32);
    assert!(!token.is_empty());

    let key = &[0u8; 32];
    let plaintext = b"secret data";

    let encrypted = client.crypto_manager().encrypt(plaintext, key).unwrap();
    let decrypted = client.crypto_manager().decrypt(&encrypted, key).unwrap();

    assert_eq!(plaintext, &decrypted[..]);
}
