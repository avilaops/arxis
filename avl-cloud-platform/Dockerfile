# Avila Cloud Platform - Production Dockerfile

FROM rust:1.75-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy only avila-cloud project
COPY Cargo.toml ./
COPY src ./src

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create user
RUN useradd -m -u 1000 avila

# Copy binary
COPY --from=builder /build/target/release/avila-cloud /usr/local/bin/
COPY --from=builder /build/target/release/avila-cloud-cli /usr/local/bin/

# Create directories
RUN mkdir -p /var/lib/avila-cloud/{data,logs,config,ssl} && \
    chown -R avila:avila /var/lib/avila-cloud

USER avila
WORKDIR /var/lib/avila-cloud

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["avila-cloud"]
