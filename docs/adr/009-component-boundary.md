# ADR-009: Component Boundary — Trait-Based Extension Architecture

**Status**: Accepted

**Date**: 2026-03-31 (updated 2026-04-01)

## Context

kreuzcrawl is a standalone, extensible crawling engine. It serves two roles simultaneously:

1. **A standalone open-source library** — released with polyglot bindings, useful for developers building crawl pipelines
2. **A crawling foundation for commercial platforms** — commercial implementations can inject distributed infrastructure via traits without modifying the engine itself

This dual role creates a boundary problem: the engine must be powerful enough standalone, yet extensible enough for production platforms to inject distributed infrastructure, proxy management, AI-powered features, and multi-tenant isolation.

### Design Principles

1. **Trait-based extension points**: Every infrastructure concern is a Rust trait with a robust default implementation. The defaults must work well for single-worker use — not stubs.
2. **The open-source version must stand on its own**: A developer can `cargo add kreuzcrawl` and have a fully functional crawler with rate limiting, multiple strategies, content filtering, markdown conversion, and optionally AI-powered extraction.
3. **No commercial logic in the engine**: If a feature requires multi-tenancy, billing, distributed coordination, or proprietary infrastructure, it belongs outside kreuzcrawl.
4. **Library first, CLI second**: kreuzcrawl is primarily a library. A CLI tool (`kreuzcrawl-cli`) ships for quick prototyping and one-off crawls. Consumers can also wrap the library in whatever service layer they need.

## Decision

### Seven Extension Traits

kreuzcrawl defines seven traits that compose via a `CrawlEngine` builder. Each has a default implementation that ships with the crate.

#### 1. `Frontier` — URL queue + deduplication

```rust
#[async_trait]
pub trait Frontier: Send + Sync {
    async fn push(&self, entry: FrontierEntry) -> Result<(), CrawlError>;
    async fn pop(&self) -> Result<Option<FrontierEntry>, CrawlError>;
    async fn pop_batch(&self, n: usize) -> Result<Vec<FrontierEntry>, CrawlError>;
    async fn len(&self) -> Result<usize, CrawlError>;
    async fn is_empty(&self) -> Result<bool, CrawlError>;
    async fn is_seen(&self, url: &str) -> Result<bool, CrawlError>;
    async fn mark_seen(&self, url: &str) -> Result<(), CrawlError>;
}
```

**Default**: `InMemoryFrontier` — `VecDeque` + `HashSet`. Handles site-scale crawls (< 100K URLs).

#### 2. `RateLimiter` — per-domain throttling

```rust
#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn acquire(&self, domain: &str) -> Result<(), CrawlError>;
    async fn record_response(&self, domain: &str, status: u16) -> Result<(), CrawlError>;
    async fn set_crawl_delay(&self, domain: &str, delay: Duration) -> Result<(), CrawlError>;
}
```

**Default**: `PerDomainThrottle` — per-domain delay enforcement with configurable default (200ms). Respects robots.txt `crawl-delay`. Backs off exponentially on 429 responses, capped at 60s.

#### 3. `CrawlStore` — result persistence

```rust
#[async_trait]
pub trait CrawlStore: Send + Sync {
    async fn store_page(&self, url: &str, result: &ScrapeResult) -> Result<(), CrawlError>;
    async fn store_crawl_page(&self, url: &str, result: &CrawlPageResult) -> Result<(), CrawlError>;
    async fn store_error(&self, url: &str, error: &CrawlError) -> Result<(), CrawlError>;
    async fn on_complete(&self, stats: &CrawlStats) -> Result<(), CrawlError>;
}
```

**Default**: `NoopStore` — results returned in-memory via `CrawlResult` or the streaming API. Nothing persisted to disk.

#### 4. `CrawlMiddleware` — request/response interceptors

```rust
#[async_trait]
pub trait CrawlMiddleware: Send + Sync {
    async fn before_request(&self, ctx: &mut RequestContext) -> Result<(), CrawlError>;
    async fn after_response(&self, ctx: &mut ResponseContext) -> Result<(), CrawlError>;
}
```

**Default implementations** (all shipped with the crate):

- `NoopMiddleware` — passes everything through unchanged.
- `CachingMiddleware` — in-memory HTTP response cache using ETag/Last-Modified for conditional requests. LRU-bounded.
- `UaRotationMiddleware` — rotates user-agent strings from a configured list.

#### 5. `EventEmitter` — crawl lifecycle events

```rust
#[async_trait]
pub trait EventEmitter: Send + Sync {
    async fn on_page(&self, event: &PageEvent);
    async fn on_error(&self, event: &ErrorEvent);
    async fn on_complete(&self, event: &CompleteEvent);
    async fn on_discovered(&self, url: &str, depth: usize);
}
```

**Default**: `NoopEmitter` — discards all events. The streaming API (`crawl_stream()`, `batch_crawl_stream()`) delivers events via `tokio::sync::mpsc` channels independently of the emitter.

#### 6. `CrawlStrategy` — crawl ordering and stopping

```rust
pub trait CrawlStrategy: Send + Sync {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize>;
    fn score_url(&self, url: &str, depth: usize) -> f64;
    fn should_continue(&self, stats: &CrawlStats) -> bool;
    fn on_page_processed(&self, page: &CrawlPageResult);
}
```

**Default implementations** (all shipped with the crate):

- `BfsStrategy` — breadth-first (FIFO). The default.
- `DfsStrategy` — depth-first (LIFO).
- `BestFirstStrategy` — priority queue ordered by `score_url`. Enables focused crawling.
- `AdaptiveStrategy` — stops crawling when content saturation is detected (new-term rate drops below a configurable threshold over a sliding window).

#### 7. `ContentFilter` — post-extraction filtering

```rust
#[async_trait]
pub trait ContentFilter: Send + Sync {
    async fn filter(&self, page: CrawlPageResult) -> Result<Option<CrawlPageResult>, CrawlError>;
}
```

**Default implementations** (all shipped with the crate):

- `NoopFilter` — pass everything through.
- `Bm25Filter` — keyword relevance scoring against a query string. Pure Rust, no external dependencies.
- `LlmExtractor` (feature-gated `ai`) — extracts structured data from crawled pages using an LLM via liter-llm. Supports JSON schema validation and custom extraction instructions.

### CrawlEngine Builder

All traits compose via a builder. Unset traits use their defaults. `build()` returns `Result` and validates the configuration before constructing the engine.

```rust
let engine = CrawlEngine::builder()
    .config(config)
    .frontier(InMemoryFrontier::new())
    .rate_limiter(PerDomainThrottle::new(Duration::from_millis(200)))
    .strategy(BfsStrategy)
    .content_filter(NoopFilter)
    .store(NoopStore)
    .middleware(CachingMiddleware::new(1000))
    .middleware(UaRotationMiddleware::new(user_agents))
    .event_emitter(NoopEmitter)
    .build()?;  // Returns Result — validates config

let result = engine.crawl("https://example.com").await?;
```

The top-level functions (`scrape()`, `crawl()`, `crawl_stream()`, `batch_scrape()`, `batch_crawl()`, `batch_crawl_stream()`, `map()`) continue to work as convenience wrappers that construct a `CrawlEngine` with all defaults internally. **Zero breaking changes.**

### What Is NOT a Trait

These remain concrete implementations — they are core differentiators, not extension points:

- **HTML extraction pipeline** — 40+ field extraction (metadata, links, images, feeds, JSON-LD)
- **Markdown conversion** — always-on HTML-to-markdown producing `MarkdownResult` with document structure, tables, and warnings
- **robots.txt parser** — RFC 9309 compliance
- **URL normalization** — deterministic algorithm
- **Sitemap parser** — standard XML/gzip format
- **Error classification** — WAF, DNS, SSL, timeout detection
- **Proxy configuration** — `ProxyConfig` for single-proxy passthrough

### Feature Flags

```toml
[features]
default = []
browser = ["dep:chromiumoxide"]   # Headless Chrome fallback + BrowserPool
ai = ["dep:liter-llm"]           # LlmExtractor for structured data extraction
```

Markdown conversion is always available (not feature-gated). It produces a `MarkdownResult` containing the converted content, document structure tree, extracted tables, and processing warnings.

### Feature Placement

| In kreuzcrawl (open source) | Rationale |
|------------------------------|-----------|
| BFS / DFS / Best-First / Adaptive strategies | Core engine algorithms |
| Per-domain rate limiting with backoff (`PerDomainThrottle`) | Polite crawling — every crawler needs this |
| HTML-to-markdown (always on, `MarkdownResult`) | Critical for AI/LLM pipelines |
| BM25 content filtering | Pure Rust, standard IR algorithm |
| LLM extraction (feature-gated `ai`) | Via liter-llm; enables structured data extraction |
| Adaptive crawling (content saturation detection) | Coverage/saturation scoring without LLM |
| HTTP conditional cache (`CachingMiddleware`) | Standard HTTP behavior, LRU-bounded |
| User-agent rotation (`UaRotationMiddleware`) | Basic anti-detection |
| Anti-bot detection (signal) | WAF/blocking detection already exists |
| Proxy support (`ProxyConfig`) | Single-proxy passthrough configuration |
| Browser fallback + BrowserPool (feature-gated `browser`) | SPA support |
| URL seeding via sitemaps + links | Part of existing `map()` API |
| Concurrent page fetching | Bounded by semaphore, pulls from frontier |
| `batch_crawl` and `batch_crawl_stream` | Multi-seed crawling with concurrency control |
| CLI tool (`kreuzcrawl-cli`) | Quick prototyping and one-off crawls |
| All 7 trait definitions | The extension API surface |

## Consequences

### Positive

- **Fully functional standalone**: Every trait has a working default. No "upgrade to unlock" experience.
- **Compile-time boundary enforcement**: Traits make the separation architectural, not just policy.
- **Ecosystem-friendly**: Third parties can implement traits for their own infrastructure (SQS frontier, Kafka emitter, etc.). Commercial implementations can be injected via traits.
- **Independently testable**: Each trait implementation is unit testable. Engine tests use defaults.
- **Config validation at build time**: `CrawlEngine::build()` returns `Result`, catching invalid configurations before the crawl starts.

### Negative

- **Trait design is load-bearing**: Getting signatures wrong means breaking changes. Must stabilize before 1.0.
- **Dynamic dispatch cost**: `dyn Trait` adds indirection. Negligible for I/O-bound crawling but noted.
- **Feature pressure**: Temptation to move commercial features into the engine for convenience.

### Neutral

- **Version coordination**: Consumers must track kreuzcrawl releases and test compatibility.
- **Trait evolution**: New traits and new methods with defaults are additive (minor version bumps).

## Alternatives Considered

### 1. No trait system — consumers wrap kreuzcrawl opaquely

**Rejected**: Would require reimplementing crawl orchestration to inject infrastructure concerns. Duplicates logic.

### 2. Plugin system with dynamic loading (dylib)

**Rejected**: Rust's `dylib` ABI instability makes this fragile. Compile-time traits are idiomatic and zero-cost.

### 3. Configuration-only extension (no traits, just config flags)

**Rejected**: A Redis frontier or NATS event emitter can't be expressed as a config flag. Traits are necessary.

### 4. Separate open-source and commercial engines

**Rejected**: Duplicates the core. Maintenance nightmare. Violates DRY.

## Implementation Notes

### Source Layout

```text
crates/kreuzcrawl/src/
  traits.rs                      # All 7 trait definitions + supporting types
  engine.rs                      # CrawlEngine struct + builder (build() returns Result)
  defaults/
    mod.rs                       # Re-exports
    frontier.rs                  # InMemoryFrontier
    rate_limiter.rs              # PerDomainThrottle, NoopRateLimiter
    store.rs                     # NoopStore
    middleware.rs                 # NoopMiddleware, CachingMiddleware, UaRotationMiddleware
    emitter.rs                   # NoopEmitter
    strategy.rs                  # BfsStrategy, DfsStrategy, BestFirstStrategy, AdaptiveStrategy
    filter.rs                    # NoopFilter, Bm25Filter
    llm_extractor.rs             # LlmExtractor (feature-gated `ai`)

crates/kreuzcrawl-cli/           # CLI tool for quick prototyping
```

### Trait Stability

Traits are **unstable until kreuzcrawl 1.0**. After 1.0:

- New methods with default implementations -> minor version
- Changed method signatures -> major version
- New traits -> minor version
