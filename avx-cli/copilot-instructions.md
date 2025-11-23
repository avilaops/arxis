# AVX CLI - Copilot Instructions

**Projeto**: avx-cli
**Descrição**: Command-line tools for Avila Experience Fabric - Deploy, Monitor, Manage
**Status**: Desenvolvimento inicial (estrutura criada)
**Filosofia**: Developer Experience > Feature Completeness

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. UX Primeiro - CLI Deve Ser Intuitiva
```bash
# ❌ ERRADO: Comandos obscuros
avx cfg val env=prod svc=gw

# ✅ CORRETO: Auto-explicativo
avx config validate --env production --service gateway

# ✅ MELHOR AINDA: Defaults inteligentes
avx config validate  # Valida tudo por default
```

**Princípio**: Se você precisa consultar docs para um comando básico, o comando está errado.

### 2. Feedback Imediato e Claro
```rust
// ❌ ERRADO: Output confuso
println!("Error: 0x8004");

// ✅ CORRETO: Contexto completo
eprintln!("❌ Failed to deploy service 'api-gateway'");
eprintln!("   Reason: Connection timeout to prod.avila.cloud:8443");
eprintln!("   Suggestion: Check network connectivity or try --retry 3");

// ✅ MELHOR: Progress bars e spinners
use indicatif::ProgressBar;
let pb = ProgressBar::new(100);
for i in 0..100 {
    pb.set_message(format!("Deploying service {}/10", i / 10));
    pb.inc(1);
}
pb.finish_with_message("✅ Deployment complete!");
```

### 3. Zero Dependências Externas Pesadas
```toml
# ✅ PERMITIDO (CLI fundamentals)
clap = "4"              # Arg parsing
tabled = "0.15"         # Tables
colored_json = "4"      # JSON pretty-print
indicatif = "0.17"      # Progress bars
dialoguer = "0.11"      # Prompts interativos
tokio = "1"             # Async runtime

# ❌ PROIBIDO (deps externas)
- kubectl = "..."       # Implementar K8s client próprio
- terraform = "..."     # Implementar IaC próprio
- docker-api = "..."    # Usar hyper direto
```

**Motivo**: CLI deve ser fast to compile (<10s) e lightweight (<5MB binary).

### 4. Configuration-as-Code
```toml
# avx.toml - Tudo configurável via arquivo
[project]
name = "my-avx-app"
version = "0.1.0"

[dev]
port = 8080
hot_reload = true
log_level = "debug"

[gateway]
routes = [
    { path = "/api/users", upstream = "http://user-service:8001" },
    { path = "/api/products", upstream = "http://product-service:8002" },
]

[deployment.production]
host = "prod.avila.cloud"
region = "br-saopaulo-1"
replicas = 3
health_check = "/health"
```

**Regra**: CLI flags > Env vars > Config file > Defaults (prioridade decrescente)

---

## 📐 Arquitetura do Projeto

```
avx-cli/
├── src/
│   ├── main.rs                # Entry point + Clap setup
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs            # avx init
│   │   ├── config.rs          # avx config
│   │   ├── dev.rs             # avx dev (hot reload)
│   │   ├── deploy.rs          # avx deploy
│   │   ├── logs.rs            # avx logs
│   │   ├── metrics.rs         # avx metrics
│   │   ├── gateway.rs         # avx gateway
│   │   ├── telemetry.rs       # avx telemetry
│   │   └── trace.rs           # avx trace
│   ├── config/
│   │   ├── mod.rs
│   │   ├── loader.rs          # Load avx.toml
│   │   ├── validator.rs       # Validate config
│   │   └── templates.rs       # Project templates
│   ├── output/
│   │   ├── mod.rs
│   │   ├── table.rs           # Tabular output
│   │   ├── json.rs            # JSON output
│   │   ├── spinner.rs         # Progress indicators
│   │   └── colors.rs          # Terminal colors
│   ├── client/
│   │   ├── mod.rs
│   │   ├── gateway.rs         # HTTP client for gateway
│   │   ├── telemetry.rs       # Telemetry API client
│   │   └── aviladb.rs         # AvilaDB client
│   └── utils/
│       ├── mod.rs
│       ├── git.rs             # Git operations
│       ├── docker.rs          # Docker API
│       └── k8s.rs             # Kubernetes API
├── templates/
│   ├── gateway/               # avx init --template gateway
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── avx.toml
│   ├── service/
│   └── full-stack/
├── examples/
│   └── custom_command.rs      # Plugin example
└── tests/
    ├── cli_test.rs            # E2E CLI tests
    └── config_test.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: Core CLI (Semanas 1-2)
```rust
// TODO: Implementar estrutura Clap
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "avx")]
#[command(about = "Avila Experience Fabric CLI")]
#[command(version)]
struct Cli {
    /// Global log level
    #[arg(long, global = true, default_value = "info")]
    log_level: String,

    /// Config file path
    #[arg(long, global = true, default_value = "avx.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize new AVX project
    Init {
        /// Project name
        name: String,

        /// Project template
        #[arg(long, default_value = "service")]
        template: String,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Start development server
    Dev {
        /// Port to bind
        #[arg(long, short, default_value = "8080")]
        port: u16,

        /// Enable hot reload
        #[arg(long, default_value = "true")]
        hot_reload: bool,
    },

    /// Deploy to environment
    Deploy {
        /// Environment name
        env: String,

        /// Specific service to deploy
        #[arg(long)]
        service: Option<String>,

        /// Image tag
        #[arg(long)]
        tag: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Validate configuration
    Validate {
        #[arg(long)]
        env: Option<String>,
    },

    /// Show configuration
    Show {
        #[arg(long)]
        env: Option<String>,
    },

    /// Set configuration value
    Set {
        /// Key path (e.g., "server.port")
        key: String,

        /// Value
        value: String,
    },
}
```

**Deliverables**:
- [x] Estrutura Clap completa
- [ ] Config loading (avx.toml + env vars)
- [ ] Pretty output (tables, JSON, colors)
- [ ] Error handling com contexto

### Fase 2: Init & Templates (Semanas 3-4)
```rust
// TODO: Implementar avx init
pub async fn init_project(name: &str, template: &str) -> Result<()> {
    println!("🚀 Initializing AVX project '{}'...", name);

    // 1. Criar diretório
    fs::create_dir_all(name)?;

    // 2. Copiar template
    let template_path = format!("templates/{}", template);
    copy_dir_recursive(&template_path, name)?;

    // 3. Substituir placeholders
    replace_in_files(name, &[
        ("{{PROJECT_NAME}}", name),
        ("{{YEAR}}", &chrono::Utc::now().year().to_string()),
    ])?;

    // 4. Inicializar Git
    Command::new("git")
        .args(&["init"])
        .current_dir(name)
        .output()?;

    // 5. Install dependencies
    println!("📦 Installing dependencies...");
    Command::new("cargo")
        .args(&["build"])
        .current_dir(name)
        .status()?;

    println!("✅ Project '{}' created successfully!", name);
    println!("\n📝 Next steps:");
    println!("   cd {}", name);
    println!("   avx dev");

    Ok(())
}

// Templates disponíveis
pub enum ProjectTemplate {
    Gateway,      // API Gateway setup
    Service,      // Microservice template
    FullStack,    // Frontend + Backend + DB
    QuantumRender,// Quantum renderer project
}
```

**Templates a criar**:
1. **gateway**: avx-gateway preconfigured
2. **service**: Generic microservice (tokio + axum + avx-telemetry)
3. **full-stack**: Leptos (frontend) + Axum (backend) + AvilaDB
4. **quantum-render**: avx-quantum-render project

### Fase 3: Config Management (Semanas 5-6)
```rust
// TODO: Config loader robusto
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct AvxConfig {
    pub project: ProjectConfig,
    pub dev: DevConfig,
    pub gateway: Option<GatewayConfig>,
    pub telemetry: TelemetryConfig,
    pub deployment: HashMap<String, DeploymentConfig>,
}

impl AvxConfig {
    /// Load config com fallback hierarchy:
    /// 1. ./avx.toml (project)
    /// 2. ~/.avx/config.toml (user)
    /// 3. /etc/avx/config.toml (system)
    /// 4. Defaults
    pub fn load() -> Result<Self> {
        let paths = vec![
            Path::new("./avx.toml"),
            &home_dir().join(".avx/config.toml"),
            Path::new("/etc/avx/config.toml"),
        ];

        for path in paths {
            if path.exists() {
                let contents = fs::read_to_string(path)?;
                return toml::from_str(&contents)
                    .with_context(|| format!("Failed to parse {}", path.display()));
            }
        }

        // Defaults
        Ok(Self::default())
    }

    /// Validate config
    pub fn validate(&self) -> Result<Vec<ValidationError>> {
        let mut errors = vec![];

        // Validar portas
        if self.dev.port == 0 || self.dev.port > 65535 {
            errors.push(ValidationError::InvalidPort(self.dev.port));
        }

        // Validar routes (gateway)
        if let Some(gw) = &self.gateway {
            for route in &gw.routes {
                if !route.path.starts_with('/') {
                    errors.push(ValidationError::InvalidRoutePath(route.path.clone()));
                }

                if let Err(e) = Url::parse(&route.upstream) {
                    errors.push(ValidationError::InvalidUpstream(route.upstream.clone(), e));
                }
            }
        }

        Ok(errors)
    }

    /// Merge com env vars (AVX_* prefix)
    pub fn merge_env_vars(&mut self) {
        if let Ok(port) = env::var("AVX_PORT") {
            if let Ok(p) = port.parse() {
                self.dev.port = p;
            }
        }

        if let Ok(level) = env::var("AVX_LOG_LEVEL") {
            self.telemetry.level = level;
        }
    }
}
```

**Comandos a implementar**:
- `avx config validate` - Validar sintaxe e semântica
- `avx config show` - Mostrar config merged (file + env)
- `avx config set key value` - Atualizar key no arquivo
- `avx config get key` - Ler valor específico

### Fase 4: Dev Server (Semanas 7-8)
```rust
// TODO: Hot reload com file watching
use notify::{Watcher, RecursiveMode};

pub async fn dev_server(config: &AvxConfig) -> Result<()> {
    println!("🔥 Starting development server...");
    println!("   Port: {}", config.dev.port);
    println!("   Hot reload: {}", config.dev.hot_reload);
    println!();

    // 1. Build inicial
    build_project()?;

    // 2. Start server process
    let mut child = spawn_server(config.dev.port)?;

    // 3. Watch filesystem
    if config.dev.hot_reload {
        let (tx, rx) = channel();
        let mut watcher = notify::watcher(tx, Duration::from_secs(1))?;
        watcher.watch("./src", RecursiveMode::Recursive)?;

        println!("👀 Watching for file changes...");

        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Write(_)) | Ok(DebouncedEvent::Create(_)) => {
                    println!("🔄 File changed, rebuilding...");

                    // Kill old process
                    child.kill()?;

                    // Rebuild
                    if let Err(e) = build_project() {
                        eprintln!("❌ Build failed: {}", e);
                        continue;
                    }

                    // Restart
                    child = spawn_server(config.dev.port)?;
                    println!("✅ Server restarted");
                }
                Err(e) => eprintln!("Watch error: {}", e),
                _ => {}
            }
        }
    } else {
        // Just wait for Ctrl+C
        child.wait()?;
    }

    Ok(())
}

fn build_project() -> Result<()> {
    let status = Command::new("cargo")
        .args(&["build"])
        .status()?;

    if !status.success() {
        bail!("cargo build failed");
    }

    Ok(())
}

fn spawn_server(port: u16) -> Result<Child> {
    Command::new("./target/debug/my-service")
        .env("PORT", port.to_string())
        .spawn()
        .context("Failed to spawn server")
}
```

**Features**:
- Auto-rebuild on file change (Rust, TOML)
- Restart server process
- Pretty error messages
- Browser auto-refresh (via WebSocket)

### Fase 5: Deployment (Semanas 9-10)
```rust
// TODO: Deploy para K8s/Docker/AVL Cloud
pub async fn deploy(env: &str, service: Option<&str>, tag: Option<&str>)
    -> Result<()> {
    let config = AvxConfig::load()?;
    let deploy_config = config.deployment.get(env)
        .ok_or_else(|| anyhow!("Environment '{}' not found", env))?;

    println!("🚀 Deploying to {} ({})...", env, deploy_config.host);

    // 1. Build Docker image
    println!("📦 Building Docker image...");
    let image_tag = tag.unwrap_or("latest");
    let image_name = format!("{}/{}:{}", deploy_config.registry, config.project.name, image_tag);

    docker_build(&image_name)?;

    // 2. Push to registry
    println!("☁️  Pushing to registry...");
    docker_push(&image_name)?;

    // 3. Deploy to K8s/Cloud
    match deploy_config.platform.as_str() {
        "kubernetes" => k8s_deploy(&image_name, deploy_config).await?,
        "avl-cloud" => avl_deploy(&image_name, deploy_config).await?,
        "docker" => docker_deploy(&image_name, deploy_config).await?,
        _ => bail!("Unknown platform: {}", deploy_config.platform),
    }

    // 4. Health check
    println!("🏥 Running health check...");
    wait_for_healthy(deploy_config).await?;

    println!("✅ Deployment successful!");
    println!("   URL: https://{}", deploy_config.host);

    Ok(())
}

// Kubernetes deployment
async fn k8s_deploy(image: &str, config: &DeploymentConfig) -> Result<()> {
    // Generate K8s manifest
    let manifest = generate_k8s_manifest(image, config)?;

    // Apply via kubectl
    let status = Command::new("kubectl")
        .args(&["apply", "-f", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
        .stdin.as_mut()
        .unwrap()
        .write_all(manifest.as_bytes())?;

    Ok(())
}

// AVL Cloud deployment (native)
async fn avl_deploy(image: &str, config: &DeploymentConfig) -> Result<()> {
    let client = AvlCloudClient::new(&config.host, &config.token);

    client.deploy(DeployRequest {
        image: image.to_string(),
        replicas: config.replicas,
        region: config.region.clone(),
        health_check: config.health_check.clone(),
    }).await?;

    Ok(())
}
```

**Plataformas suportadas**:
1. **Kubernetes**: Via kubectl + manifests
2. **AVL Cloud**: API nativa (HTTP client)
3. **Docker**: docker-compose up

### Fase 6: Observability (Semanas 11-12)
```rust
// TODO: Logs, metrics, traces
use avx_telemetry::TelemetryClient;

// avx logs
pub async fn logs(service: Option<&str>, follow: bool, level: Option<&str>)
    -> Result<()> {
    let client = TelemetryClient::connect("telemetry.avila.cloud:9090").await?;

    let query = LogQuery {
        service: service.map(String::from),
        level: level.map(String::from),
        since: Some(Duration::from_secs(3600)), // Last hour
    };

    if follow {
        let mut stream = client.logs_stream(query).await?;
        while let Some(log) = stream.next().await {
            println!("{}", format_log(&log?));
        }
    } else {
        let logs = client.logs(query).await?;
        for log in logs {
            println!("{}", format_log(&log));
        }
    }

    Ok(())
}

fn format_log(log: &LogEntry) -> String {
    let level_color = match log.level.as_str() {
        "error" => "red",
        "warn" => "yellow",
        "info" => "green",
        "debug" => "blue",
        _ => "white",
    };

    format!(
        "{} {} {} {}",
        log.timestamp.format("%Y-%m-%d %H:%M:%S"),
        format!("[{}]", log.level).color(level_color),
        log.service,
        log.message
    )
}

// avx metrics
pub async fn metrics(service: Option<&str>) -> Result<()> {
    let client = TelemetryClient::connect("telemetry.avila.cloud:9090").await?;

    let metrics = client.metrics(service).await?;

    // Tabela formatada
    let mut table = Table::new(&metrics);
    table.with(Style::modern());
    println!("{}", table);

    Ok(())
}

// avx trace <request-id>
pub async fn trace(request_id: &str) -> Result<()> {
    let client = TelemetryClient::connect("telemetry.avila.cloud:9090").await?;

    let trace = client.get_trace(request_id).await?;

    // Waterfall view
    println!("🔍 Trace: {}", request_id);
    println!();

    for span in trace.spans {
        let indent = "  ".repeat(span.depth);
        let duration_ms = span.duration.as_millis();

        println!(
            "{}{} ({} ms)",
            indent,
            span.name,
            duration_ms
        );
    }

    println!();
    println!("Total duration: {} ms", trace.duration.as_millis());

    Ok(())
}
```

---

## 🧪 Testes Obrigatórios

### 1. CLI Integration Tests
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_avx_version() {
    let mut cmd = Command::cargo_bin("avx").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("avx"));
}

#[test]
fn test_avx_init_creates_project() {
    let temp = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("avx").unwrap();
    cmd.arg("init")
        .arg("test-project")
        .arg("--template")
        .arg("service")
        .current_dir(&temp);

    cmd.assert().success();

    // Verificar arquivos criados
    assert!(temp.path().join("test-project/Cargo.toml").exists());
    assert!(temp.path().join("test-project/avx.toml").exists());
    assert!(temp.path().join("test-project/src/main.rs").exists());
}

#[test]
fn test_config_validate_invalid() {
    let mut cmd = Command::cargo_bin("avx").unwrap();
    cmd.arg("config")
        .arg("validate")
        .arg("--config")
        .arg("tests/fixtures/invalid-config.toml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid port"));
}
```

### 2. Config Validation Tests
```rust
#[test]
fn test_config_merge_env_vars() {
    temp_env::with_vars(
        vec![
            ("AVX_PORT", Some("9000")),
            ("AVX_LOG_LEVEL", Some("debug")),
        ],
        || {
            let mut config = AvxConfig::default();
            config.merge_env_vars();

            assert_eq!(config.dev.port, 9000);
            assert_eq!(config.telemetry.level, "debug");
        },
    );
}

#[test]
fn test_config_validation_errors() {
    let config = AvxConfig {
        dev: DevConfig { port: 99999 },  // Invalid
        gateway: Some(GatewayConfig {
            routes: vec![
                Route {
                    path: "invalid-path".to_string(),  // Missing /
                    upstream: "not-a-url".to_string(),  // Invalid URL
                },
            ],
        }),
        ..Default::default()
    };

    let errors = config.validate().unwrap();
    assert_eq!(errors.len(), 3);
}
```

---

## 📊 Integração com AVX Ecosystem

### 1. Gateway Management
```bash
# Add route
avx gateway add-route /api/users http://user-service:8001

# List routes
avx gateway routes

# Test route
avx gateway test /api/users --method POST --body '{"name":"João"}'

# Reload gateway
avx gateway reload
```

### 2. Telemetry Queries
```bash
# Logs
avx logs --service gateway --follow --level error

# Metrics
avx metrics --service gateway --from "1 hour ago"

# Traces
avx trace ABC123-REQUEST-ID

# Anomaly detection
avx telemetry anomalies --metric response_time --threshold 3.0
```

### 3. AvilaDB Queries
```bash
# Connect
avx db connect --account my-account

# Query
avx db query "SELECT * FROM users WHERE active = true"

# Insert
avx db insert users '{"name":"João","email":"joao@example.com"}'
```

---

## 🎓 Recursos de Aprendizado

### CLI Best Practices
- **12 Factor App**: https://12factor.net/
- **Command Line Interface Guidelines**: https://clig.dev/
- **Rust CLI Book**: https://rust-cli.github.io/book/

### Tools a estudar
- **clap**: Parsing de args
- **tabled**: Tabelas formatadas
- **indicatif**: Progress bars
- **dialoguer**: Prompts interativos
- **colored**: Terminal colors

---

## ⚠️ Erros Comuns a Evitar

### 1. Não validar input do usuário
```rust
// ❌ ERRADO: Panic em input inválido
let port: u16 = args.port.parse().unwrap();

// ✅ CORRETO: Validar e dar erro descritivo
let port: u16 = args.port.parse()
    .with_context(|| format!("Invalid port '{}', must be 1-65535", args.port))?;

if port == 0 {
    bail!("Port cannot be 0");
}
```

### 2. Output sem cores/formatação
```rust
// ❌ ERRADO: Plain text
println!("Error: Failed to deploy");

// ✅ CORRETO: Cores e ícones
use console::style;
eprintln!("{} {}",
    style("❌").red().bold(),
    style("Failed to deploy service").red()
);
```

### 3. Comandos blocking sem feedback
```rust
// ❌ ERRADO: User não sabe o que está acontecendo
docker_build(&image)?;

// ✅ CORRETO: Spinner durante operação longa
let spinner = ProgressBar::new_spinner();
spinner.set_message("Building Docker image...");
spinner.enable_steady_tick(Duration::from_millis(100));

docker_build(&image)?;

spinner.finish_with_message("✅ Image built successfully");
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR:

- [ ] **UX**: Comandos intuitivos e auto-explicativos
- [ ] **Feedback**: Progress bars para operações longas
- [ ] **Errors**: Mensagens de erro com contexto e sugestões
- [ ] **Colors**: Output colorido e formatado
- [ ] **Config**: Suporte a avx.toml + env vars
- [ ] **Tests**: CLI integration tests passando
- [ ] **Docs**: Help text completo (`--help`)
- [ ] **Binary Size**: <10MB release build
- [ ] **Compile Time**: <30s clean build

---

## 🚀 Como Começar

### Setup
```bash
cd arxis/avx-cli
cargo build
cargo test
```

### Testar localmente
```bash
cargo run -- --help
cargo run -- init test-project
cargo run -- config validate
```

### Install para uso global
```bash
cargo install --path .
avx --version
```

---

**Lembre-se**: A CLI é a primeira impressão do desenvolvedor com AVX. UX ruim = desenvolvedor abandona o projeto.

**AVX CLI** - Developer Experience Matters 🚀
