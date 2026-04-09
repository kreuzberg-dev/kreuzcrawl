# =============================================================================
# Builder Stage - Build Rust binary with all features
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

# Copy only the crates needed for the build
COPY crates/kreuzcrawl/ crates/kreuzcrawl/
COPY crates/kreuzcrawl-cli/ crates/kreuzcrawl-cli/

# Replace glob workspace members with only the crates we need
RUN sed -i 's|^members = .*|members = ["crates/kreuzcrawl", "crates/kreuzcrawl-cli"]|' Cargo.toml && \
    sed -i '/^exclude/,/^\]/d' Cargo.toml

# Build release binary with all features
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --package kreuzcrawl-cli \
        --features "kreuzcrawl-cli/api,kreuzcrawl-cli/mcp,kreuzcrawl/api,kreuzcrawl/mcp,kreuzcrawl/warc,kreuzcrawl/ai,kreuzcrawl/tracing,kreuzcrawl/browser,kreuzcrawl/interact" && \
    cp target/release/kreuzcrawl /build/kreuzcrawl && \
    strip /build/kreuzcrawl

# =============================================================================
# Runtime Stage - Chainguard base with Chromium for browser automation
# =============================================================================
FROM cgr.dev/chainguard/wolfi-base:latest

LABEL org.opencontainers.image.source="https://github.com/kreuzberg-dev/kreuzcrawl"
LABEL org.opencontainers.image.description="Kreuzcrawl — high-performance web crawler with browser automation, API server, and MCP"
LABEL org.opencontainers.image.licenses="Elastic-2.0"

WORKDIR /app

# Install runtime deps: Chromium + fonts + curl for healthcheck
RUN apk add --no-cache \
    ca-certificates \
    curl \
    chromium \
    font-liberation \
    font-noto-emoji \
    glib \
    nss

# Copy binary from builder
COPY --from=builder /build/kreuzcrawl /usr/local/bin/kreuzcrawl

# Chromium flags: set KREUZCRAWL_DISABLE_GPU=false to enable GPU acceleration
# (useful when running with --gpus or in GPU-enabled environments)
ARG DISABLE_GPU=true
ENV RUST_LOG=info \
    CHROME_BIN=/usr/bin/chromium-browser \
    KREUZCRAWL_DISABLE_GPU=${DISABLE_GPU} \
    CHROMIUM_FLAGS="--no-sandbox --disable-dev-shm-usage"

# Run as non-root (Chainguard images default to nonroot user)
USER nonroot

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["curl", "-f", "http://localhost:3000/health"]

# Entrypoint: kreuzcrawl binary. Override CMD for CLI usage.
# API server: docker run kreuzcrawl (default)
# CLI:        docker run kreuzcrawl scrape https://example.com
# MCP:        docker run kreuzcrawl mcp
ENTRYPOINT ["kreuzcrawl"]
CMD ["serve", "--host", "0.0.0.0", "--port", "3000"]
