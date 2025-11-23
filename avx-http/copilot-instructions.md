# AVX-HTTP - Copilot Instructions

**Projeto**: avx-http
**Descrição**: Native HTTP Client/Server Optimized for AVL Platform & Brazilian Infrastructure
**Status**: v0.1.0 - Foundation Complete
**Filosofia**: Brazil-First. Low Latency. Zero Overhead. Native Integration.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Brasil Tem Prioridade Máxima
```rust
// ✅ CORRETO: Regional routing automático
pub struct Client {
    preferred_region: Region,
    fallback_regions: Vec<Region>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
            .region(Region::BrSaoPaulo1)  // DEFAULT sempre Brasil!
            .fallback_region(Region::BrRioDeJaneiro1)
            .fallback_region(Region::UsEast1)  // Último recurso
    }
}

// ❌ ERRADO: US como default
impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
            .region(Region::UsEast1)  // PROIBIDO!
    }
}
```

**Latências esperadas**:
- São Paulo → SP DC: 5-10ms
- São Paulo → RJ DC: 15-25ms
- São Paulo → US East: 120-150ms
- São Paulo → EU West: 200-250ms

### 2. Sub-500µs Request Overhead
```rust
// Benchmark obrigatório em cada PR
#[bench]
fn bench_request_overhead(b: &mut Bencher) {
    let client = Client::new();

    b.iter(|| {
        // Apenas overhead do client (sem network)
        black_box(client.prepare_request("GET", "https://api.avila.cloud"))
    });
}

// Target: <500µs (nanoseconds idealmente)
// Baseline: reqwest = ~2-5ms overhead
// AVX-HTTP: <500µs (5-10x mais rápido)
```

**Otimizações obrigatórias**:
- Connection pooling (evitar handshakes)
- Zero-copy body parsing
- Pre-computed headers
- Memory pooling para buffers

### 3. AVL Platform Native Integration
```rust
// ✅ CORRETO: Built-in AVL auth
impl ClientBuilder {
    pub fn avl_auth(mut self, api_key: &str) -> Self {
        // Automatic JWT token refresh
        // Automatic cost tracking
        // Automatic telemetry
        self.auth = Auth::AvlApiKey(api_key.to_string());
        self
    }

    pub fn aviladb_connection(mut self, conn: AvilaDbConnection) -> Self {
        // Direct integration with AvilaDB
        self.aviladb = Some(conn);
        self
    }
}

// ❌ ERRADO: Manual header management
let response = client.get("https://api.avila.cloud")
    .header("Authorization", format!("Bearer {}", token))  // Manual!
    .send().await?;
```

### 4. Smart Retries para Brasil
```rust
// ✅ CORRETO: Retry com backoff exponencial
pub struct RetryPolicy {
    max_retries: usize,
    base_delay: Duration,
    max_delay: Duration,
    jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            jitter: true,  // Evitar thundering herd
        }
    }
}

// Retry apenas em erros retriáveis:
// - Connection timeout
// - DNS resolution failure
// - 5xx server errors
// - Network unreachable (comum em Brasil!)

// NÃO retry em:
// - 4xx client errors (bad request, etc.)
// - Authentication failures
// - Cancelled by user
```

---

## 📐 Arquitetura do Projeto

```
avx-http/
├── src/
│   ├── lib.rs               # Public API
│   ├── client/
│   │   ├── mod.rs
│   │   ├── builder.rs       # ClientBuilder
│   │   ├── request.rs       # Request builder
│   │   ├── response.rs      # Response wrapper
│   │   ├── pool.rs          # Connection pool
│   │   ├── retry.rs         # Retry logic
│   │   └── compression.rs   # avila-compress integration
│   ├── server/
│   │   ├── mod.rs
│   │   ├── server.rs        # Server struct
│   │   ├── router.rs        # Route matching
│   │   ├── handler.rs       # Handler trait
│   │   ├── middleware.rs    # Middleware chain
│   │   └── response.rs      # Response builder
│   ├── avl/
│   │   ├── mod.rs
│   │   ├── auth.rs          # AVL authentication
│   │   ├── telemetry.rs     # AVL telemetry integration
│   │   ├── region.rs        # Regional routing
│   │   └── cost.rs          # Cost tracking
│   ├── compression/
│   │   ├── mod.rs
│   │   ├── lz4.rs           # LZ4 (via avila-compress)
│   │   ├── zstd.rs          # Zstd (via avila-compress)
│   │   └── negotiate.rs     # Content-Encoding negotiation
│   ├── body/
│   │   ├── mod.rs
│   │   ├── stream.rs        # Streaming body
│   │   ├── bytes.rs         # Bytes body
│   │   └── json.rs          # JSON body (serde)
│   ├── error.rs             # Error types
│   └── utils/
│       ├── mod.rs
│       ├── headers.rs       # Header parsing
│       ├── uri.rs           # URI parsing
│       └── pool.rs          # Memory pool
├── benches/
│   ├── client_bench.rs
│   ├── server_bench.rs
│   ├── vs_reqwest.rs
│   └── vs_hyper.rs
├── examples/
│   ├── client_simple.rs
│   ├── client_aviladb.rs
│   ├── server_simple.rs
│   └── server_full.rs
└── tests/
    ├── client_tests.rs
    ├── server_tests.rs
    ├── retry_tests.rs
    └── compression_tests.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: HTTP Client Core (v0.1.0) ✅ COMPLETO
```rust
// ✅ Implementado
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    pool: ConnectionPool,
    config: ClientConfig,
}

impl Client {
    pub fn builder() -> ClientBuilder;

    pub fn get(&self, url: &str) -> RequestBuilder;
    pub fn post(&self, url: &str) -> RequestBuilder;
    pub fn put(&self, url: &str) -> RequestBuilder;
    pub fn delete(&self, url: &str) -> RequestBuilder;
    pub fn head(&self, url: &str) -> RequestBuilder;
}

pub struct RequestBuilder {
    client: Client,
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    body: Option<Body>,
    timeout: Option<Duration>,
    retry: Option<RetryPolicy>,
}

impl RequestBuilder {
    pub fn header(mut self, key: &str, value: &str) -> Self;
    pub fn body(mut self, body: impl Into<Body>) -> Self;
    pub fn json<T: Serialize>(mut self, json: &T) -> Result<Self>;
    pub fn timeout(mut self, duration: Duration) -> Self;
    pub fn retry(mut self, policy: RetryPolicy) -> Self;

    pub async fn send(self) -> Result<Response>;
}

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Body,
}

impl Response {
    pub fn status(&self) -> StatusCode;
    pub fn headers(&self) -> &HashMap<String, String>;
    pub async fn text(self) -> Result<String>;
    pub async fn json<T: DeserializeOwned>(self) -> Result<T>;
    pub async fn bytes(self) -> Result<Bytes>;
}
```

### Fase 2: Connection Pooling & Performance (v0.2.0) - Semanas 1-2
```rust
// TODO: Connection pool avançado
pub struct ConnectionPool {
    pools: HashMap<String, PerHostPool>, // host → pool
    config: PoolConfig,
}

struct PerHostPool {
    idle: VecDeque<Connection>,
    active: HashSet<ConnectionId>,
    waiters: VecDeque<Waiter>,
}

impl ConnectionPool {
    pub async fn get_connection(&self, host: &str) -> Result<Connection> {
        // 1. Check idle connections
        if let Some(conn) = self.pools.get(host)?.idle.pop_front() {
            if conn.is_alive() {
                return Ok(conn);
            }
        }

        // 2. Create new connection if under limit
        if self.can_create_connection(host) {
            return self.create_connection(host).await;
        }

        // 3. Wait for connection to become available
        let waiter = Waiter::new();
        self.pools.get_mut(host)?.waiters.push_back(waiter.clone());
        waiter.wait().await
    }

    pub fn return_connection(&self, conn: Connection) {
        let host = conn.host();

        // Return to idle pool or wake waiter
        if let Some(waiter) = self.pools.get_mut(host)?.waiters.pop_front() {
            waiter.wake(conn);
        } else {
            self.pools.get_mut(host)?.idle.push_back(conn);
        }
    }

    pub async fn prune_idle(&self) {
        // Remove connections idle > timeout
        for pool in self.pools.values_mut() {
            pool.idle.retain(|conn| {
                conn.idle_time() < self.config.idle_timeout
            });
        }
    }
}

pub struct PoolConfig {
    max_connections_per_host: usize,  // Default: 100
    max_idle_per_host: usize,         // Default: 10
    idle_timeout: Duration,           // Default: 60s
    connect_timeout: Duration,        // Default: 10s
}
```

### Fase 3: AVL Platform Integration (v0.3.0) - Semanas 3-4
```rust
// TODO: AVL authentication
pub enum Auth {
    None,
    AvlApiKey(String),
    AvlJwt(JwtToken),
    Bearer(String),
    Basic(String, String),
}

impl Client {
    async fn authenticate_request(&self, req: &mut Request) -> Result<()> {
        match &self.config.auth {
            Auth::AvlApiKey(key) => {
                // 1. Exchange API key for JWT
                let jwt = self.exchange_api_key(key).await?;

                // 2. Add to request
                req.headers.insert("Authorization", format!("Bearer {}", jwt.token));

                // 3. Cache JWT (expires after 1h)
                self.jwt_cache.insert(key.clone(), jwt);
            }
            Auth::AvlJwt(jwt) => {
                // Check expiration
                if jwt.is_expired() {
                    return Err(Error::AuthExpired);
                }

                req.headers.insert("Authorization", format!("Bearer {}", jwt.token));
            }
            // ... outros tipos
        }

        Ok(())
    }
}

// TODO: Regional routing
pub enum Region {
    BrSaoPaulo1,
    BrRioDeJaneiro1,
    BrBrasilia1,
    UsEast1,
    UsWest1,
    EuWest1,
    ApSoutheast1,
}

impl Client {
    async fn resolve_endpoint(&self, service: &str) -> String {
        // 1. Try preferred region
        let endpoint = format!("{}.{}.avila.cloud", service, self.config.region);
        if self.is_reachable(&endpoint).await {
            return endpoint;
        }

        // 2. Try fallback regions
        for region in &self.config.fallback_regions {
            let endpoint = format!("{}.{}.avila.cloud", service, region);
            if self.is_reachable(&endpoint).await {
                return endpoint;
            }
        }

        // 3. Last resort: global endpoint
        format!("{}.avila.cloud", service)
    }
}

// TODO: Cost tracking
pub struct CostTracker {
    requests: AtomicU64,
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,
}

impl CostTracker {
    pub fn record_request(&self, req: &Request, resp: &Response) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        self.bytes_sent.fetch_add(req.body.len() as u64, Ordering::Relaxed);
        self.bytes_received.fetch_add(resp.body.len() as u64, Ordering::Relaxed);

        // Send to AVL telemetry
        avx_telemetry::record_http_request(req, resp);
    }

    pub fn estimate_cost(&self) -> f64 {
        let requests = self.requests.load(Ordering::Relaxed);
        let bytes_out = self.bytes_sent.load(Ordering::Relaxed);
        let bytes_in = self.bytes_received.load(Ordering::Relaxed);

        // AVL pricing (exemplo)
        let request_cost = requests as f64 * 0.0001;  // R$ 0,0001 por request
        let bandwidth_cost = (bytes_out + bytes_in) as f64 / 1_000_000.0 * 0.05; // R$ 0,05 per MB

        request_cost + bandwidth_cost
    }
}
```

### Fase 4: HTTP Server (v0.4.0) - Semanas 5-7
```rust
// TODO: HTTP Server implementation
pub struct Server {
    router: Router,
    config: ServerConfig,
}

impl Server {
    pub fn bind(addr: impl ToSocketAddrs) -> ServerBuilder;

    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(&self.config.addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            let router = self.router.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, router).await {
                    eprintln!("Error handling connection from {}: {}", addr, e);
                }
            });
        }
    }
}

async fn handle_connection(stream: TcpStream, router: Router) -> Result<()> {
    let mut buffer = vec![0; 8192];

    loop {
        // Read request
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        // Parse HTTP request
        let request = parse_request(&buffer[..n])?;

        // Route to handler
        let response = router.handle(request).await;

        // Write response
        stream.write_all(&serialize_response(response)).await?;

        // Keep-alive or close
        if !request.headers.get("Connection").map(|v| v == "keep-alive").unwrap_or(false) {
            break;
        }
    }

    Ok(())
}

// TODO: Router implementation
pub struct Router {
    routes: Vec<Route>,
    middleware: Vec<Box<dyn Middleware>>,
}

impl Router {
    pub fn new() -> Self;

    pub fn get<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler + 'static,
    {
        self.routes.push(Route {
            method: Method::GET,
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    pub fn post<F>(mut self, path: &str, handler: F) -> Self;
    pub fn put<F>(mut self, path: &str, handler: F) -> Self;
    pub fn delete<F>(mut self, path: &str, handler: F) -> Self;

    pub fn middleware<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middleware.push(Box::new(middleware));
        self
    }

    async fn handle(&self, request: Request) -> Response {
        // Apply middleware
        let mut request = request;
        for mw in &self.middleware {
            request = mw.before(request).await;
        }

        // Find matching route
        let route = self.routes.iter()
            .find(|r| r.method == request.method && r.matches(&request.path))
            .ok_or(Error::NotFound)?;

        // Call handler
        let mut response = route.handler.call(request).await;

        // Apply middleware (reverse order)
        for mw in self.middleware.iter().rev() {
            response = mw.after(response).await;
        }

        response
    }
}

// TODO: Handler trait
#[async_trait]
pub trait Handler: Send + Sync {
    async fn call(&self, request: Request) -> Response;
}

// Blanket impl for functions
#[async_trait]
impl<F, Fut> Handler for F
where
    F: Fn(Request) -> Fut + Send + Sync,
    Fut: Future<Output = Response> + Send,
{
    async fn call(&self, request: Request) -> Response {
        self(request).await
    }
}
```

### Fase 5: Compression & Advanced Features (v0.5.0) - Semanas 8-10
```rust
// TODO: Automatic compression with avila-compress
impl RequestBuilder {
    pub fn compress(mut self, enable: bool) -> Self {
        self.compress = enable;
        self
    }

    pub async fn send(self) -> Result<Response> {
        let mut request = self.build()?;

        // Compress body if enabled and large enough
        if self.compress && request.body.len() > 1024 {
            use avila_compress::{Compressor, Algorithm};

            let compressor = Compressor::new(Algorithm::Zstd)?;
            let compressed = compressor.compress(&request.body)?;

            if compressed.len() < request.body.len() {
                request.body = compressed;
                request.headers.insert("Content-Encoding", "zstd");
            }
        }

        // Send request
        self.client.send(request).await
    }
}

// TODO: Streaming responses
impl Response {
    pub fn stream(self) -> impl Stream<Item = Result<Bytes>> {
        let body = self.body;

        async_stream::stream! {
            let mut reader = body.reader();
            let mut buffer = vec![0; 8192];

            loop {
                let n = reader.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }

                yield Ok(Bytes::copy_from_slice(&buffer[..n]));
            }
        }
    }
}

// TODO: Middleware system
#[async_trait]
pub trait Middleware: Send + Sync {
    async fn before(&self, request: Request) -> Request {
        request
    }

    async fn after(&self, response: Response) -> Response {
        response
    }
}

// Built-in middleware
pub struct LoggingMiddleware;
pub struct CompressionMiddleware;
pub struct CorsMiddleware;
pub struct RateLimitMiddleware;
pub struct AuthMiddleware;
```

---

## 🧪 Testes Obrigatórios

### 1. Request Overhead Benchmark
```rust
#[bench]
fn bench_request_overhead_vs_reqwest(b: &mut Bencher) {
    let avx_client = avx_http::Client::new();
    let reqwest_client = reqwest::Client::new();

    b.iter(|| {
        // AVX-HTTP
        let start = Instant::now();
        let _ = avx_client.prepare_request("GET", "https://api.avila.cloud");
        let avx_time = start.elapsed();

        // reqwest
        let start = Instant::now();
        let _ = reqwest_client.request(Method::GET, "https://api.avila.cloud");
        let reqwest_time = start.elapsed();

        assert!(avx_time < reqwest_time / 5); // 5x mais rápido
    });
}

// Target: AVX-HTTP <500µs, reqwest ~2-5ms
```

### 2. Regional Routing Tests
```rust
#[tokio::test]
async fn test_regional_routing_brazil() {
    let client = Client::builder()
        .region(Region::BrSaoPaulo1)
        .build()
        .unwrap();

    let response = client.get("https://api.avila.cloud/ping").send().await.unwrap();

    // Should route to São Paulo DC
    let server_region = response.headers().get("X-Avl-Region").unwrap();
    assert_eq!(server_region, "br-saopaulo-1");

    // Should be low latency
    let latency = response.headers().get("X-Avl-Latency-Ms").unwrap().parse::<u64>().unwrap();
    assert!(latency < 50); // <50ms from São Paulo
}
```

### 3. Retry Logic Tests
```rust
#[tokio::test]
async fn test_retry_on_network_error() {
    let client = Client::builder()
        .retry(RetryPolicy::default())
        .build()
        .unwrap();

    let mut attempts = 0;

    // Mock server that fails first 2 times
    let response = client
        .get("http://localhost:9999/flaky")
        .send()
        .await
        .unwrap();

    assert!(attempts == 3); // Initial + 2 retries
    assert_eq!(response.status(), 200);
}
```

---

## 📊 API Pública

### Client API
```rust
pub struct Client {
    inner: Arc<ClientInner>,
}

impl Client {
    pub fn builder() -> ClientBuilder;
    pub fn new() -> Self;

    pub fn get(&self, url: &str) -> RequestBuilder;
    pub fn post(&self, url: &str) -> RequestBuilder;
    pub fn put(&self, url: &str) -> RequestBuilder;
    pub fn delete(&self, url: &str) -> RequestBuilder;
    pub fn head(&self, url: &str) -> RequestBuilder;
    pub fn request(&self, method: Method, url: &str) -> RequestBuilder;
}

pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    pub fn region(mut self, region: Region) -> Self;
    pub fn fallback_region(mut self, region: Region) -> Self;
    pub fn avl_auth(mut self, api_key: &str) -> Self;
    pub fn timeout(mut self, duration: Duration) -> Self;
    pub fn pool_max_connections(mut self, max: usize) -> Self;
    pub fn compression(mut self, enable: bool) -> Self;
    pub fn telemetry(mut self, enable: bool) -> Self;

    pub fn build(self) -> Result<Client>;
}

pub struct RequestBuilder {
    client: Client,
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    body: Option<Body>,
    timeout: Option<Duration>,
    retry: Option<RetryPolicy>,
    compress: bool,
}

impl RequestBuilder {
    pub fn header(mut self, key: &str, value: &str) -> Self;
    pub fn headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>;

    pub fn body(mut self, body: impl Into<Body>) -> Self;
    pub fn json<T: Serialize>(mut self, json: &T) -> Result<Self>;
    pub fn form<T: Serialize>(mut self, form: &T) -> Result<Self>;

    pub fn timeout(mut self, duration: Duration) -> Self;
    pub fn retry(mut self, policy: RetryPolicy) -> Self;
    pub fn compress(mut self, enable: bool) -> Self;

    pub async fn send(self) -> Result<Response>;
}

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Body,
}

impl Response {
    pub fn status(&self) -> StatusCode;
    pub fn headers(&self) -> &HashMap<String, String>;
    pub fn header(&self, key: &str) -> Option<&str>;

    pub async fn text(self) -> Result<String>;
    pub async fn json<T: DeserializeOwned>(self) -> Result<T>;
    pub async fn bytes(self) -> Result<Bytes>;
    pub fn stream(self) -> impl Stream<Item = Result<Bytes>>;
}
```

### Server API
```rust
pub struct Server {
    router: Router,
    config: ServerConfig,
}

impl Server {
    pub fn bind(addr: impl ToSocketAddrs) -> ServerBuilder;
}

pub struct ServerBuilder {
    addr: SocketAddr,
    router: Option<Router>,
    config: ServerConfig,
}

impl ServerBuilder {
    pub fn router(mut self, router: Router) -> Self;
    pub fn compression(mut self, enable: bool) -> Self;
    pub fn telemetry(mut self, enable: bool) -> Self;
    pub fn cors(mut self, cors: CorsConfig) -> Self;

    pub async fn run(self) -> Result<()>;
}

pub struct Router {
    routes: Vec<Route>,
    middleware: Vec<Box<dyn Middleware>>,
}

impl Router {
    pub fn new() -> Self;

    pub fn get<F>(self, path: &str, handler: F) -> Self
    where
        F: Handler + 'static;

    pub fn post<F>(self, path: &str, handler: F) -> Self;
    pub fn put<F>(self, path: &str, handler: F) -> Self;
    pub fn delete<F>(self, path: &str, handler: F) -> Self;

    pub fn middleware<M: Middleware + 'static>(self, middleware: M) -> Self;
}
```

---

## ⚠️ Erros Comuns a Evitar

### 1. US-First Mindset
```rust
// ❌ ERRADO
let client = Client::builder()
    .region(Region::UsEast1)
    .build()?;

// ✅ CORRETO
let client = Client::builder()
    .region(Region::BrSaoPaulo1)
    .fallback_region(Region::UsEast1)
    .build()?;
```

### 2. Blocking em Async Context
```rust
// ❌ ERRADO
async fn handler() -> Response {
    let data = std::fs::read_to_string("file.txt").unwrap(); // Blocks!
    Response::text(data)
}

// ✅ CORRETO
async fn handler() -> Response {
    let data = tokio::fs::read_to_string("file.txt").await.unwrap();
    Response::text(data)
}
```

### 3. Connection Leaks
```rust
// ❌ ERRADO
loop {
    let client = Client::new(); // Creates new pool cada vez!
    let _ = client.get("...").send().await;
}

// ✅ CORRETO
let client = Client::new(); // Reuse!
loop {
    let _ = client.get("...").send().await;
}
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR:

- [ ] **Brasil-First**: Default region é Brasil
- [ ] **Low Overhead**: <500µs request overhead
- [ ] **AVL Integration**: Built-in AVL auth e telemetry
- [ ] **Smart Retries**: Retry logic para ISPs brasileiros
- [ ] **Compression**: avila-compress integrado
- [ ] **Docs**: Cada função pública documentada
- [ ] **Tests**: Unit + integration tests
- [ ] **Benchmarks**: vs reqwest/hyper
- [ ] **Examples**: Código funcional para usuários

---

## 🚀 Como Começar

### Setup
```bash
cd arxis/avx-http
cargo build
cargo test
```

### Exemplos
```bash
# Cliente simples
cargo run --example client_simple

# Cliente com AvilaDB
cargo run --example client_aviladb

# Servidor simples
cargo run --example server_simple

# Servidor completo
cargo run --example server_full
```

### Benchmarks
```bash
# Internal benchmarks
cargo bench

# vs reqwest
cargo bench --bench vs_reqwest

# vs hyper
cargo bench --bench vs_hyper
```

---

**Lembre-se**: Brasil tem prioridade. Sub-500µs overhead. Native AVL integration. Smart retries para ISPs brasileiros.

**AVX-HTTP** - HTTP para o Brasil 🇧🇷🚀
