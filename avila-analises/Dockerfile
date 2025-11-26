# Multi-stage build para otimizar tamanho da imagem
FROM rust:1.75-slim as builder

WORKDIR /app

# Instalar dependências do sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar manifests
COPY Cargo.toml Cargo.lock ./

# Copiar código fonte
COPY src ./src
COPY benches ./benches
COPY avila-location ./avila-location

# Build release
RUN cargo build --release --bin server

# Stage final - imagem mínima
FROM debian:bookworm-slim

WORKDIR /app

# Instalar dependências runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copiar binário do builder
COPY --from=builder /app/target/release/server /app/server

# Criar usuário não-root
RUN useradd -m -u 1000 appuser && chown -R appuser:appuser /app
USER appuser

# Expor porta
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Comando para iniciar
CMD ["/app/server"]
