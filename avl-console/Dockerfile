# Multi-stage build para otimizar tamanho da imagem

# Stage 1: Builder
FROM rust:1.75-slim-bookworm AS builder

# Instalar dependências de build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /build

# Copiar workspace Cargo.toml primeiro para cache de dependências
COPY Cargo.toml Cargo.lock ./
COPY avl-console/ ./avl-console/
COPY aviladb/ ./aviladb/
COPY avl-auth/ ./avl-auth/
COPY avl-storage/ ./avl-storage/
COPY avx-gateway/ ./avx-gateway/
COPY avx-telemetry/ ./avx-telemetry/
COPY avila-telemetry/ ./avila-telemetry/

# Build em release mode com features de produção
WORKDIR /build/avl-console
RUN cargo build --release --features production

# Stage 2: Runtime
FROM debian:bookworm-slim

# Instalar runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -m -u 1000 avlconsole

# Criar diretórios necessários
RUN mkdir -p /app/data /app/logs && \
    chown -R avlconsole:avlconsole /app

# Copiar binário compilado
COPY --from=builder /build/target/release/avl-console /app/avl-console

# Copiar assets estáticos (se houver)
COPY --from=builder /build/avl-console/templates /app/templates

# Mudar para usuário não-root
USER avlconsole
WORKDIR /app

# Expor portas
EXPOSE 3000 9090

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Variáveis de ambiente padrão
ENV RUST_LOG=info
ENV AVL_CONSOLE_PORT=3000
ENV AVL_CONSOLE_HOST=0.0.0.0
ENV AVL_METRICS_PORT=9090

# Comando de inicialização
CMD ["/app/avl-console"]
