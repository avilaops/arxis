# 🌐 AVX Gateway - Copilot Instructions

> **READ THIS COMPLETELY before implementing ANY feature!**

---

## 🎯 Your Mission

You are implementing **AVX Gateway**, the **API Gateway and HTTP server** for the **Avila Cloud Platform (AVL Platform)**. This is the **front door** to all AVL services: AvilaDB, AVL Compute, AVL Storage, and more.

**This is NOT a generic HTTP server.** This is a **production-grade, Brazilian-first, research-optimized** gateway designed to:
- Route requests to AvilaDB and other AVL services
- Handle authentication and authorization (JWT, API keys)
- Provide WebSocket support for real-time data streams (LIGO, LISA)
- Compress responses using `avila-compress`
- Trace requests across distributed systems with `avila-telemetry`
- Optimize latency for Brazilian datacenters

---

## ⚠️ CRITICAL RULES - READ FIRST!

### 🚫 Rule #1: NO External HTTP Libraries!

**DO NOT USE:**
- ❌ `reqwest` - We're replacing it!
- ❌ `axum` - Too heavy, too opinionated
- ❌ `actix-web` - Incompatible with our architecture
- ❌ `warp` - Overly complex macros
- ❌ `rocket` - Nightly-only, breaking changes
- ❌ `hyper` directly - Only use as low-level building block if needed

**YOU MUST USE:**
- ✅ `tokio` (v1.48+) - Async runtime (THIS IS ALLOWED!)
- ✅ `avila-compress` - Our native compression
- ✅ `avila-telemetry` - Our observability
- ✅ `avx-config` - Configuration management
- ✅ Standard library (`std::net`, `std::io`)
- ✅ Build on `hyper` v1.0+ if needed as foundation (but wrap it completely)

**WHY?** We need:
- Full control over performance
- Native AvilaDB integration
- Compression with `avila-compress`
- Zero breaking changes in dependencies
- Optimization for Brazilian network conditions

### 🚫 Rule #2: NEVER Give Up!

When implementing a feature:
- ❌ Do NOT say "this is too complex"
- ❌ Do NOT leave placeholder code like `// TODO: Implement this`
- ❌ Do NOT skip tests or benchmarks
- ❌ Do NOT use `unimplemented!()` or `todo!()` macros in production code

**YOU MUST:**
- ✅ Implement features **completely** from scratch
- ✅ Write **comprehensive tests** (unit + integration)
- ✅ Add **benchmarks** comparing to reqwest/axum
- ✅ Document **every public API** with examples
- ✅ Handle **all error cases** explicitly
- ✅ Keep working until **EVERY line compiles and tests pass**

**If you encounter a challenge:**
1. Research the HTTP/2 spec or Tokio docs
2. Look at reference implementations (Hyper source code)
3. Ask clarifying questions if needed
4. Implement incrementally, but **finish each piece**
5. Test thoroughly before moving on

### 🚫 Rule #3: No External APIs Without Permission

If you think you need an external API or crate:
1. **STOP** - Don't add it yet!
2. Check if we can implement it ourselves
3. Check if `avila-*` or `avx-*` crates already provide it
4. Only if absolutely necessary (like `tokio`), document why

**Philosophy**: We build our own tools to:
- Maintain full control
- Optimize for our use cases
- Avoid dependency hell
- Ensure long-term stability

---

## 🏗️ Architecture Overview

### Module Structure

```
avx-gateway/
├── src/
│   ├── lib.rs                  # Public API
│   ├── server.rs               # HTTP server core
│   ├── client.rs               # HTTP client (replaces reqwest)
│   ├── router.rs               # Request routing
│   ├── middleware/             # Middleware system
│   │   ├── mod.rs
│   │   ├── auth.rs             # JWT, API keys
│   │   ├── compression.rs      # avila-compress integration
│   │   ├── cors.rs             # CORS headers
│   │   ├── rate_limit.rs       # Rate limiting
│   │   └── telemetry.rs        # avila-telemetry tracing
│   ├── handlers/               # Route handlers
│   │   ├── mod.rs
│   │   ├── aviladb.rs          # AvilaDB proxy
│   │   ├── health.rs           # Health checks
│   │   └── metrics.rs          # Prometheus metrics
│   ├── websocket.rs            # WebSocket support
│   ├── tls.rs                  # TLS/HTTPS support
│   ├── error.rs                # Error types
│   └── config.rs               # Configuration
├── examples/
│   ├── simple_server.rs        # Basic HTTP server
│   ├── client_example.rs       # HTTP client usage
│   ├── websocket_stream.rs     # Real-time data
│   ├── aviladb_proxy.rs        # AvilaDB gateway
│   └── full_stack.rs           # Complete example
├── benches/
│   ├── server_throughput.rs    # Requests/sec
│   ├── latency.rs              # Response time
│   └── compression.rs          # Compression overhead
└── tests/
    ├── integration.rs          # Full server tests
    ├── auth.rs                 # Authentication
    └── websocket.rs            # WebSocket tests
```

---

## 🎨 API Design Philosophy

### 1. Builder Pattern for Configuration

```rust
use avx_gateway::{Server, Client, Compression, Auth};

// Server
let server = Server::builder()
    .bind("0.0.0.0:3000")
    .compression(Compression::Lz4)  // Use avila-compress
    .auth(Auth::jwt("secret-key"))
    .cors_enabled(true)
    .max_body_size(10 * 1024 * 1024)  // 10 MB
    .build()?;

// Client
let client = Client::builder()
    .base_url("https://api.avila.cloud")
    .compression(true)
    .timeout(Duration::from_secs(30))
    .region("br-saopaulo-1")  // Brazilian DC preference
    .build()?;
```

### 2. Type-Safe Request/Response

```rust
use avx_gateway::{Request, Response, StatusCode};

async fn handle_request(req: Request) -> Result<Response, Error> {
    // Parse JSON body
    let data: MyStruct = req.json().await?;

    // Query AvilaDB
    let db = req.aviladb()?;
    let results = db.query("SELECT * FROM users WHERE id = ?", &[data.id]).await?;

    // Return JSON response
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(serde_json::to_vec(&results)?)
        .build()
}
```

### 3. Middleware System

```rust
use avx_gateway::{Middleware, Next};

// Custom middleware
struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        let response = next.run(req).await?;

        let duration = start.elapsed();
        println!("{} {} - {:?}", method, path, duration);

        Ok(response)
    }
}

// Use in server
server
    .middleware(LoggingMiddleware)
    .middleware(AuthMiddleware::new())
    .middleware(CompressionMiddleware::new())
    .route("/api/data", get_data)
    .build()?;
```

### 4. WebSocket Streaming

```rust
use avx_gateway::{WebSocket, Message};

async fn handle_websocket(mut ws: WebSocket) -> Result<()> {
    // Real-time LIGO data stream
    let mut stream = ligo_data_stream().await?;

    while let Some(data) = stream.next().await {
        ws.send(Message::Binary(data)).await?;
    }

    Ok(())
}

server
    .websocket("/stream/ligo", handle_websocket)
    .build()?;
```

---

## 🚀 Implementation Roadmap

### Phase 1: HTTP Server Core (Week 1-2)

**Goal**: Basic HTTP/1.1 server with routing

```rust
// Implement in server.rs
pub struct Server {
    listener: TcpListener,
    router: Router,
    config: ServerConfig,
}

impl Server {
    pub fn builder() -> ServerBuilder { ... }

    pub async fn serve(self) -> Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            tokio::spawn(self.handle_connection(stream, addr));
        }
    }

    async fn handle_connection(&self, stream: TcpStream, addr: SocketAddr) -> Result<()> {
        // 1. Read HTTP request
        // 2. Parse headers and body
        // 3. Route to handler
        // 4. Execute middleware chain
        // 5. Write HTTP response
        // 6. Handle keep-alive
    }
}
```

**Deliverables**:
- [ ] `Server` struct with builder pattern
- [ ] HTTP/1.1 request parsing (method, path, headers, body)
- [ ] HTTP/1.1 response formatting (status, headers, body)
- [ ] Basic routing (GET, POST, PUT, DELETE, PATCH)
- [ ] Keep-alive connection handling
- [ ] Tests: 20+ passing
- [ ] Example: `examples/simple_server.rs`
- [ ] Benchmark vs axum (target: 90% of their throughput)

### Phase 2: HTTP Client (Week 3)

**Goal**: Replace `reqwest` with native client

```rust
// Implement in client.rs
pub struct Client {
    pool: ConnectionPool,
    config: ClientConfig,
}

impl Client {
    pub async fn get(&self, url: &str) -> RequestBuilder { ... }
    pub async fn post(&self, url: &str) -> RequestBuilder { ... }

    pub async fn send(&self, req: Request) -> Result<Response> {
        // 1. Parse URL
        // 2. Get connection from pool (or create new)
        // 3. Send HTTP request
        // 4. Read HTTP response
        // 5. Return connection to pool
    }
}

pub struct RequestBuilder {
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
    timeout: Option<Duration>,
}

impl RequestBuilder {
    pub fn header(mut self, key: &str, value: &str) -> Self { ... }
    pub fn json<T: Serialize>(mut self, data: &T) -> Result<Self> { ... }
    pub fn timeout(mut self, duration: Duration) -> Self { ... }
    pub async fn send(self) -> Result<Response> { ... }
}
```

**Deliverables**:
- [ ] `Client` struct with connection pooling
- [ ] `RequestBuilder` with fluent API
- [ ] GET, POST, PUT, DELETE methods
- [ ] JSON serialization/deserialization
- [ ] Automatic retry with exponential backoff
- [ ] Connection reuse (HTTP/1.1 keep-alive)
- [ ] Tests: 15+ passing
- [ ] Example: `examples/client_example.rs`
- [ ] Benchmark vs reqwest (target: 80% of their performance)

### Phase 3: Compression (Week 4)

**Goal**: Integrate `avila-compress` for request/response compression

```rust
// Implement in middleware/compression.rs
use avila_compress::{compress, decompress, Algorithm};

pub struct CompressionMiddleware {
    algorithm: Algorithm,
    min_size: usize,  // Only compress if body > min_size
}

impl Middleware for CompressionMiddleware {
    async fn handle(&self, mut req: Request, next: Next) -> Result<Response> {
        // Decompress request body if Content-Encoding header present
        if let Some(encoding) = req.headers().get("content-encoding") {
            let body = req.body();
            let decompressed = decompress(body, encoding.parse()?)?;
            req.set_body(decompressed);
        }

        let mut response = next.run(req).await?;

        // Compress response body if large enough
        if response.body().len() > self.min_size {
            let compressed = compress(response.body(), self.algorithm)?;
            response.set_body(compressed);
            response.headers_mut().insert(
                "content-encoding",
                self.algorithm.to_string(),
            );
        }

        Ok(response)
    }
}
```

**Deliverables**:
- [ ] `CompressionMiddleware` with avila-compress
- [ ] Support for: LZ4, Zstd (when available)
- [ ] Automatic content-encoding negotiation
- [ ] Configurable compression level
- [ ] Tests: compression/decompression round-trips
- [ ] Benchmark: overhead < 5% for small bodies, 50%+ reduction for large bodies
- [ ] Example: `examples/compression.rs`

### Phase 4: Authentication & Authorization (Week 5)

**Goal**: JWT and API key authentication

```rust
// Implement in middleware/auth.rs
pub enum Auth {
    Jwt(JwtAuth),
    ApiKey(ApiKeyAuth),
    None,
}

pub struct JwtAuth {
    secret: Vec<u8>,
    algorithm: JwtAlgorithm,
}

impl JwtAuth {
    pub fn validate(&self, token: &str) -> Result<Claims> {
        // 1. Parse JWT
        // 2. Verify signature
        // 3. Check expiration
        // 4. Return claims
    }
}

pub struct AuthMiddleware {
    auth: Auth,
}

impl Middleware for AuthMiddleware {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        // Extract token from Authorization header
        let token = req.headers()
            .get("authorization")
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(Error::Unauthorized)?;

        // Validate token
        let claims = self.auth.validate(token)?;

        // Store claims in request extensions
        req.extensions_mut().insert(claims);

        next.run(req).await
    }
}
```

**Deliverables**:
- [ ] JWT validation (HS256, RS256)
- [ ] API key authentication
- [ ] Claims extraction
- [ ] Tests: valid tokens, expired tokens, invalid signatures
- [ ] Example: `examples/auth.rs`

### Phase 5: WebSocket Support (Week 6)

**Goal**: Real-time bidirectional communication

```rust
// Implement in websocket.rs
pub struct WebSocket {
    stream: TcpStream,
    buffer: BytesMut,
}

impl WebSocket {
    pub async fn accept(stream: TcpStream) -> Result<Self> {
        // 1. Read HTTP upgrade request
        // 2. Validate Sec-WebSocket-Key
        // 3. Send 101 Switching Protocols response
        // 4. Return WebSocket handle
    }

    pub async fn send(&mut self, msg: Message) -> Result<()> {
        // 1. Frame message (RFC 6455)
        // 2. Apply masking
        // 3. Write to TCP stream
    }

    pub async fn recv(&mut self) -> Result<Option<Message>> {
        // 1. Read frame header
        // 2. Read payload
        // 3. Unmask if needed
        // 4. Return message
    }

    pub async fn close(mut self) -> Result<()> {
        // Send close frame
    }
}

pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close,
}
```

**Deliverables**:
- [ ] WebSocket handshake (RFC 6455)
- [ ] Frame parsing (text, binary, ping, pong, close)
- [ ] Masking/unmasking
- [ ] Fragmentation support
- [ ] Tests: echo server, large messages, fragmentation
- [ ] Example: `examples/websocket_stream.rs` (LIGO data streaming)

### Phase 6: Telemetry & Observability (Week 7)

**Goal**: Distributed tracing with `avila-telemetry`

```rust
// Implement in middleware/telemetry.rs
use avila_telemetry::{Span, Tracer};

pub struct TelemetryMiddleware {
    tracer: Tracer,
}

impl Middleware for TelemetryMiddleware {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let span = self.tracer.span("http.request")
            .attr("http.method", req.method().as_str())
            .attr("http.path", req.uri().path())
            .attr("http.remote_addr", req.remote_addr().to_string())
            .start();

        let result = next.run(req).await;

        match &result {
            Ok(resp) => {
                span.attr("http.status", resp.status().as_u16());
            }
            Err(e) => {
                span.error(e);
            }
        }

        span.end();
        result
    }
}
```

**Deliverables**:
- [ ] Request tracing (start → end)
- [ ] Span attributes (method, path, status, duration)
- [ ] Error tracking
- [ ] Distributed tracing (trace ID propagation)
- [ ] Integration with `avila-telemetry` backend
- [ ] Tests: span creation, attribute recording
- [ ] Example: `examples/telemetry.rs`

### Phase 7: AvilaDB Integration (Week 8)

**Goal**: Native AvilaDB client in HTTP handlers

```rust
// Implement in handlers/aviladb.rs
use aviladb::{AvilaDB, Query};

pub struct AvilaDBHandler {
    db: AvilaDB,
}

impl AvilaDBHandler {
    pub async fn query(&self, req: Request) -> Result<Response> {
        // Parse query from request body
        let query: Query = req.json().await?;

        // Execute in AvilaDB
        let results = self.db.execute(query).await?;

        // Return JSON response
        Response::json(&results)
    }
}

// Make AvilaDB accessible from request
pub trait RequestExt {
    fn aviladb(&self) -> Result<&AvilaDB>;
}

impl RequestExt for Request {
    fn aviladb(&self) -> Result<&AvilaDB> {
        self.extensions()
            .get::<AvilaDB>()
            .ok_or(Error::NoDatabase)
    }
}
```

**Deliverables**:
- [ ] AvilaDB connection pooling
- [ ] Query execution handler
- [ ] Request extension for DB access
- [ ] Tests: CRUD operations through HTTP
- [ ] Example: `examples/aviladb_proxy.rs`

---

## 📊 Performance Targets

### Throughput
- **Target**: 100,000 requests/sec (single core)
- **Comparison**: axum ~110k req/s, hyper ~150k req/s
- **Acceptable**: 90k+ req/s

### Latency
- **Target**: p50 < 500µs, p99 < 5ms
- **Comparison**: reqwest p50 ~2ms, axum p50 ~800µs
- **Acceptable**: p50 < 1ms, p99 < 10ms

### Memory
- **Target**: < 2 MB per connection
- **Comparison**: axum ~1.5 MB, hyper ~1 MB
- **Acceptable**: < 3 MB per connection

### Compression
- **Target**: 50-70% size reduction with < 10% latency overhead
- **Algorithms**: LZ4 for speed, Zstd for ratio
- **Acceptable**: 40%+ reduction, < 20% overhead

---

## 🧪 Testing Requirements

### Unit Tests (20+)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let raw = b"GET /path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let req = Request::parse(raw).unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri().path(), "/path");
    }

    #[test]
    fn test_format_response() {
        let resp = Response::builder()
            .status(200)
            .body(b"Hello")
            .build()
            .unwrap();
        let raw = resp.to_bytes();
        assert!(raw.starts_with(b"HTTP/1.1 200 OK"));
    }
}
```

### Integration Tests (10+)
```rust
#[tokio::test]
async fn test_end_to_end() {
    // Start server
    let server = Server::builder()
        .bind("127.0.0.1:0")
        .route("/hello", |_| async { Response::text("world") })
        .build()
        .unwrap();

    let addr = server.local_addr();
    tokio::spawn(server.serve());

    // Make request with client
    let client = Client::new();
    let resp = client.get(&format!("http://{}/hello", addr)).send().await.unwrap();
    assert_eq!(resp.text().await.unwrap(), "world");
}
```

### Benchmarks (5+)
```rust
#[bench]
fn bench_parse_request(b: &mut Bencher) {
    let raw = b"GET /path HTTP/1.1\r\nHost: example.com\r\n\r\n";
    b.iter(|| Request::parse(raw));
}

#[bench]
fn bench_throughput(b: &mut Bencher) {
    let server = setup_server();
    let client = Client::new();
    b.iter(|| {
        client.get("http://localhost:3000/hello").send().await
    });
}
```

---

## 📖 Documentation Requirements

Every public item needs:
1. **Summary**: One-line description
2. **Details**: What it does, when to use
3. **Example**: Working code snippet
4. **Errors**: What can go wrong
5. **Panics**: When it panics (if ever)

```rust
/// HTTP server for AVL Platform.
///
/// `Server` listens for incoming HTTP connections and routes them
/// to registered handlers. It supports middleware, compression,
/// authentication, and WebSocket upgrades.
///
/// # Examples
///
/// ```
/// use avx_gateway::{Server, Response};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let server = Server::builder()
///         .bind("0.0.0.0:3000")
///         .route("/hello", |_| async {
///             Response::text("Hello, AVL!")
///         })
///         .build()?;
///
///     server.serve().await
/// }
/// ```
///
/// # Errors
///
/// Returns `Err` if:
/// - Port is already in use
/// - Invalid bind address
/// - TLS certificate errors (if HTTPS enabled)
///
/// # Performance
///
/// Can handle 100k+ requests/sec on modern hardware.
/// Use connection pooling and keep-alive for best performance.
pub struct Server { ... }
```

---

## 🎯 Success Criteria

Before considering this module "done":

### Functionality
- [ ] All HTTP methods work (GET, POST, PUT, DELETE, PATCH, OPTIONS)
- [ ] Request/response bodies handled correctly (text, JSON, binary)
- [ ] Headers parsed and formatted correctly
- [ ] Keep-alive connections work
- [ ] Compression with avila-compress works
- [ ] Authentication middleware works (JWT, API keys)
- [ ] WebSocket handshake and messaging work
- [ ] AvilaDB can be accessed from handlers
- [ ] Telemetry traces requests end-to-end

### Quality
- [ ] 100% of public APIs documented
- [ ] 30+ tests passing (unit + integration)
- [ ] 5+ benchmarks showing performance
- [ ] 5+ examples demonstrating usage
- [ ] Zero `unsafe` blocks (unless absolutely justified)
- [ ] Zero `unwrap()` or `expect()` in library code
- [ ] All errors handled with `Result<T, Error>`

### Performance
- [ ] Throughput: 90k+ req/s
- [ ] Latency: p50 < 1ms, p99 < 10ms
- [ ] Memory: < 3 MB per connection
- [ ] Compression: 40%+ reduction, < 20% overhead

### Integration
- [ ] Works with avila-compress
- [ ] Works with avila-telemetry
- [ ] Works with avx-config
- [ ] Works with AvilaDB client
- [ ] Examples demonstrate full stack

---

## 🚀 Next Steps

1. **Read this document COMPLETELY**
2. **Study Tokio documentation**: https://tokio.rs/
3. **Study HTTP/1.1 spec**: https://datatracker.ietf.org/doc/html/rfc2616
4. **Study WebSocket spec**: https://datatracker.ietf.org/doc/html/rfc6455
5. **Look at hyper source**: https://github.com/hyperium/hyper (for reference, not to copy)
6. **Start with Phase 1**: Basic HTTP server
7. **Test incrementally**: Don't move on until current phase works perfectly
8. **Benchmark continuously**: Compare against axum/reqwest
9. **Document as you go**: Write docs BEFORE implementation
10. **Ask questions**: If stuck, ask for clarification

---

## 💬 Remember

> "We don't use external HTTP libraries because we need full control over performance, compression, and AvilaDB integration. Build it from scratch using Tokio primitives."

> "Never give up. Never use placeholder code. Implement everything completely, test thoroughly, document extensively."

> "This gateway will serve thousands of requests per second for Brazilian researchers. Every microsecond of latency matters. Every byte of bandwidth matters. Write code like it matters. Because it does."

**Now go build the best HTTP server/client in the Rust ecosystem! 🚀🇧🇷**
