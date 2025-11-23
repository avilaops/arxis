use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avl_auth::{AuthClient, Config, Credentials};
use uuid::Uuid;

fn generate_test_keys() -> (String, String) {
    // Pre-generated RSA-2048 key pair for testing
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

fn bench_password_hashing(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_test_client());

    c.bench_function("password_hash", |b| {
        b.iter(|| {
            client.password_manager()
                .hash_password(black_box("SecureP@ssw0rd123!"))
        })
    });
}

fn bench_password_verification(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_test_client());
    let password = "SecureP@ssw0rd123!";
    let hash = client.password_manager().hash_password(password).unwrap();

    c.bench_function("password_verify", |b| {
        b.iter(|| {
            client.password_manager()
                .verify_password(black_box(password), black_box(&hash))
        })
    });
}

fn bench_jwt_creation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_test_client());

    let claims = client.jwt_manager().create_claims(
        Uuid::new_v4(),
        "test@example.com".to_string(),
        vec!["user".to_string()],
        vec![],
        Uuid::new_v4(),
        vec!["*".to_string()],
        None,
    );

    c.bench_function("jwt_create", |b| {
        b.to_async(&rt).iter(|| async {
            client.jwt_manager()
                .create_token(black_box(&claims))
                .await
        })
    });
}

fn bench_jwt_verification(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_test_client());

    let claims = client.jwt_manager().create_claims(
        Uuid::new_v4(),
        "test@example.com".to_string(),
        vec!["user".to_string()],
        vec![],
        Uuid::new_v4(),
        vec!["*".to_string()],
        None,
    );

    let token = rt.block_on(client.jwt_manager().create_token(&claims)).unwrap();

    c.bench_function("jwt_verify", |b| {
        b.to_async(&rt).iter(|| async {
            client.jwt_manager()
                .verify_token(black_box(&token))
                .await
        })
    });
}

fn bench_full_login(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("full_login", |b| {
        b.to_async(&rt).iter_with_setup(
            || {
                let client = rt.block_on(create_test_client());
                let email = format!("user_{}@example.com", Uuid::new_v4());
                let password = "SecureP@ssw0rd123!";

                rt.block_on(async {
                    client.register(email.clone(), password.to_string()).await.unwrap();
                });

                (client, email, password.to_string())
            },
            |(client, email, password)| async move {
                let credentials = Credentials {
                    email,
                    password,
                    device_id: Some("test_device".to_string()),
                    ip_address: Some("127.0.0.1".parse().unwrap()),
                };

                client.login(black_box(credentials)).await
            },
        )
    });
}

fn bench_api_key_generation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_test_client());

    c.bench_function("api_key_generate", |b| {
        b.to_async(&rt).iter(|| async {
            client.api_key_manager()
                .generate_api_key(
                    black_box(Uuid::new_v4()),
                    black_box("Test Key".to_string()),
                    None,
                    vec!["read".to_string()],
                    Some(100),
                    None,
                )
                .await
        })
    });
}

fn bench_totp_generation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_test_client());
    let secret = client.mfa_manager().generate_totp_secret();

    c.bench_function("totp_verify", |b| {
        b.to_async(&rt).iter(|| async {
            // Generate a code to verify
            let code = "123456"; // Dummy code for benchmarking
            client.mfa_manager()
                .verify_totp(black_box(&secret), black_box(code), black_box(1))
        })
    });
}

fn bench_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("throughput");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.to_async(&rt).iter(|| async move {
                let client = create_test_client().await;

                let mut handles = vec![];
                for i in 0..size {
                    let email = format!("user{}@example.com", i);
                    let password = "SecureP@ss123!".to_string();

                    handles.push(tokio::spawn(async move {
                        let client = create_test_client().await;
                        client.register(email, password).await
                    }));
                }

                for handle in handles {
                    let _ = handle.await;
                }
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_password_hashing,
    bench_password_verification,
    bench_jwt_creation,
    bench_jwt_verification,
    bench_full_login,
    bench_api_key_generation,
    bench_totp_generation,
    bench_throughput
);

criterion_main!(benches);
