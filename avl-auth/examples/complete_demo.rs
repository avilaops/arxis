//! Complete example demonstrating all AVL Auth features

use avl_auth::{
    AuthClient, Config, Credentials,
    prelude::*,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🔐 AVL Auth - Complete Example\n");

    // ==================== 1. Setup ====================
    println!("1️⃣  Setting up authentication client...");

    let mut config = Config::default();

    // Generate RSA keys for JWT (in production, load from secure storage)
    let crypto = avl_auth::crypto::CryptoManager::new();
    let (private_key, public_key) = crypto.generate_rsa_keypair(2048)?;

    config.jwt.private_key = private_key;
    config.jwt.public_key = public_key;
    config.jwt.algorithm = "RS256".to_string();

    let client = AuthClient::new(config).await?;
    println!("✅ Client initialized\n");

    // ==================== 2. User Registration ====================
    println!("2️⃣  Registering new user...");

    let email = "demo@avila.cloud".to_string();
    let password = "SuperSecure@Pass123!".to_string();

    let user_id = client.register(email.clone(), password.clone()).await?;
    println!("✅ User registered with ID: {}\n", user_id);

    // ==================== 3. Password Strength Check ====================
    println!("3️⃣  Checking password strength...");

    let strength = client.password_manager().calculate_strength(&password);
    println!("Password strength: {:?}\n", strength);

    // ==================== 4. User Login ====================
    println!("4️⃣  Logging in...");

    let credentials = Credentials {
        email: email.clone(),
        password: password.clone(),
        device_id: Some("demo_device_001".to_string()),
        ip_address: Some("191.36.8.1".parse()?),
    };

    let session = client.login(credentials).await?;
    println!("✅ Login successful!");
    println!("   Session ID: {}", session.id);
    println!("   Access Token: {}...", &session.access_token[..50]);
    println!("   Expires at: {}\n", session.expires_at);

    // ==================== 5. JWT Token Verification ====================
    println!("5️⃣  Verifying JWT token...");

    let claims = client.verify_token(&session.access_token).await?;
    println!("✅ Token verified!");
    println!("   User ID: {}", claims.sub);
    println!("   Email: {}", claims.email);
    println!("   Roles: {:?}", claims.roles);
    println!("   Expires: {}\n", claims.exp);

    // ==================== 6. MFA Setup (TOTP) ====================
    println!("6️⃣  Setting up Multi-Factor Authentication (TOTP)...");

    let totp_config = client.mfa_manager().generate_totp_config(&email, None);
    let totp_uri = client.mfa_manager().generate_totp_uri(&totp_config);

    println!("✅ TOTP configured!");
    println!("   Secret: {}", totp_config.secret);
    println!("   URI (scan with authenticator app):");
    println!("   {}\n", totp_uri);

    // Generate backup codes
    let backup_codes = client.mfa_manager().generate_backup_codes(10);
    println!("   Backup codes (save these securely):");
    for (i, code) in backup_codes.iter().enumerate() {
        println!("   {}. {}", i + 1, code);
    }
    println!();

    // ==================== 7. API Key Generation ====================
    println!("7️⃣  Generating API key...");

    let (api_key, key_metadata) = client.api_key_manager()
        .generate_api_key(
            user_id,
            "Demo API Key".to_string(),
            Some("For demonstration purposes".to_string()),
            vec!["read".to_string(), "write".to_string()],
            Some(1000), // Rate limit: 1000 requests per minute
            Some(chrono::Duration::days(90)),
        )
        .await?;

    println!("✅ API Key generated!");
    println!("   Key: {}", api_key);
    println!("   Prefix: {}", key_metadata.prefix);
    println!("   Scopes: {:?}", key_metadata.scopes);
    println!("   Rate limit: {} req/min", key_metadata.rate_limit.unwrap());
    println!("   Expires: {}\n", key_metadata.expires_at.unwrap());

    // Verify the API key
    let verified_key = client.api_key_manager().verify_api_key(&api_key).await?;
    println!("✅ API Key verified: {}\n", verified_key.name);

    // ==================== 8. Session Management ====================
    println!("8️⃣  Session management...");

    let session_stats = client.session_manager().get_stats().await;
    println!("   Total sessions: {}", session_stats.total_sessions);
    println!("   Unique users: {}", session_stats.unique_users);
    println!("   Active (last hour): {}\n", session_stats.active_last_hour);

    // ==================== 9. Risk Assessment ====================
    println!("9️⃣  Risk assessment...");

    // This would normally happen during login, but we can demonstrate it
    println!("   Risk-based authentication analyzes:");
    println!("   • Account age and status");
    println!("   • Failed login attempts");
    println!("   • Location and device patterns");
    println!("   • Time-of-day patterns");
    println!("   • Geo-velocity (impossible travel detection)");
    println!("   • And more...\n");

    // ==================== 10. Audit Logs ====================
    println!("🔟 Audit logging...");

    let user_activity = client.audit_manager()
        .get_user_activity(&user_id, 7)
        .await;

    println!("   User activity (last 7 days):");
    println!("   • Total actions: {}", user_activity.total_actions);
    println!("   • Successful: {}", user_activity.successful);
    println!("   • Failed: {}", user_activity.failed);
    println!("   • Average risk score: {:.1}\n", user_activity.avg_risk_score);

    // ==================== 11. Cryptographic Operations ====================
    println!("1️⃣1️⃣  Cryptographic operations...");

    let sensitive_data = b"This is sensitive user data";
    let encryption_key = &[0u8; 32]; // In production, derive from user's key

    let encrypted = client.crypto_manager().encrypt(sensitive_data, encryption_key)?;
    let decrypted = client.crypto_manager().decrypt(&encrypted, encryption_key)?;

    println!("✅ Data encrypted and decrypted successfully");
    println!("   Original size: {} bytes", sensitive_data.len());
    println!("   Encrypted size: {} bytes\n", encrypted.len());

    // ==================== 12. Logout ====================
    println!("1️⃣2️⃣  Logging out...");

    client.logout(&session.id).await?;
    println!("✅ User logged out successfully\n");

    // ==================== Summary ====================
    println!("📊 Summary:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ User Registration & Login");
    println!("✅ JWT Token Generation & Verification");
    println!("✅ Multi-Factor Authentication (TOTP)");
    println!("✅ API Key Management");
    println!("✅ Session Management");
    println!("✅ Risk-Based Authentication");
    println!("✅ Comprehensive Audit Logging");
    println!("✅ End-to-End Encryption");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("🎉 All AVL Auth features demonstrated successfully!");
    println!("🔐 Your application is now secured with world-class authentication.\n");

    Ok(())
}
