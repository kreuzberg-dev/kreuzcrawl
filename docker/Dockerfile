# =============================================================================
# Builder Stage - Build Rust binary with all dependencies
# =============================================================================
FROM rust:1.91-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        cmake \
        g++ \
        pkg-config \
        libssl-dev \
        && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace manifests for dependency caching
COPY Cargo.toml Cargo.lock ./

# Copy only the crates needed for the API server build
COPY crates/kreuzcrawl/ crates/kreuzcrawl/
COPY crates/kreuzcrawl-cli/ crates/kreuzcrawl-cli/

# Replace glob workspace members with only the crates we need.
# The original uses "crates/*" and "tools/*" which would pull in binding crates
# that require PyO3, NAPI-RS, ext-php-rs, wasm-bindgen, etc.
RUN sed -i 's|members = \["crates/\*", "tools/\*", "packages/elixir/native/kreuzcrawl_nif"\]|members = ["crates/kreuzcrawl", "crates/kreuzcrawl-cli"]|' Cargo.toml

# Build release binary with full server features (API, MCP, WARC, AI)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --package kreuzcrawl-cli \
        --features "kreuzcrawl/api,kreuzcrawl/mcp,kreuzcrawl/warc,kreuzcrawl/ai,kreuzcrawl/tracing" && \
    cp target/release/kreuzcrawl /build/kreuzcrawl && \
    strip /build/kreuzcrawl

# =============================================================================
# Runtime Stage - Minimal runtime environment
# =============================================================================
FROM debian:bookworm-slim

# OCI labels for container metadata
LABEL org.opencontainers.image.source="https://github.com/kreuzberg-dev/kreuzcrawl"
LABEL org.opencontainers.image.description="Kreuzcrawl web crawler and scraper - full API server"
LABEL org.opencontainers.image.licenses="Elastic-2.0"

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/kreuzcrawl /usr/local/bin/kreuzcrawl
RUN chmod +x /usr/local/bin/kreuzcrawl

# Create non-root user
RUN groupadd -r kreuzcrawl && \
    useradd -r -g kreuzcrawl -d /app -s /sbin/nologin kreuzcrawl && \
    chown -R kreuzcrawl:kreuzcrawl /app

# Environment configuration
ENV RUST_LOG=info

USER kreuzcrawl

EXPOSE 3000

# Health check against the API health endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["curl", "-f", "http://localhost:3000/health"]

# Default: start the API server
ENTRYPOINT ["kreuzcrawl"]
CMD ["serve", "--host", "0.0.0.0", "--port", "3000"]
