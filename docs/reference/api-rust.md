# Rust API Reference

The `kreuzcrawl` crate provides the core crawling engine and all public types. Add it as a dependency:

```toml
[dependencies]
kreuzcrawl = "0.1"
tokio = { version = "1", features = ["full"] }
```

## CrawlEngine

The main entry point. Built via a builder pattern with pluggable trait implementations.

### Construction

```rust
use kreuzcrawl::{CrawlEngine, CrawlConfig};

// Default configuration and default trait implementations
let engine = CrawlEngine::builder().build()?;

// Custom configuration
let config = CrawlConfig {
    max_depth: Some(3),
    max_pages: Some(100),
    stay_on_domain: true,
    respect_robots_txt: true,
    ..Default::default()
};
let engine = CrawlEngine::builder().config(config).build()?;
```

### CrawlEngineBuilder

| Method | Description |
|--------|-------------|
| `CrawlEngine::builder()` | Create a new builder |
| `.config(CrawlConfig)` | Set the crawl configuration |
| `.frontier(impl Frontier)` | Set the URL frontier (default: `InMemoryFrontier`) |
| `.rate_limiter(impl RateLimiter)` | Set the rate limiter (default: `PerDomainThrottle` at 200ms) |
| `.store(impl CrawlStore)` | Set the crawl store (default: `NoopStore`) |
| `.event_emitter(impl EventEmitter)` | Set the event emitter (default: `NoopEmitter`) |
| `.strategy(impl CrawlStrategy)` | Set the crawl strategy (default: `BfsStrategy`) |
| `.content_filter(impl ContentFilter)` | Set the content filter (default: `NoopFilter`) |
| `.cache(impl CrawlCache)` | Set the persistent cache (default: `NoopCache`) |
| `.build()` | Build the engine, validating config. Returns `Result<CrawlEngine, CrawlError>`. |

### Methods

#### scrape

```rust
pub async fn scrape(&self, url: &str) -> Result<ScrapeResult, CrawlError>
```

Scrape a single URL. Routes the request through the Tower service stack (rate limiting, UA rotation, caching) then runs the extraction pipeline.

#### crawl

```rust
pub async fn crawl(&self, url: &str) -> Result<CrawlResult, CrawlError>
```

Crawl a website starting from `url`, following links up to the configured depth and page limit.

#### map

```rust
pub async fn map(&self, url: &str) -> Result<MapResult, CrawlError>
```

Discover all pages on a website by following links and parsing sitemaps.

#### batch_scrape

```rust
pub async fn batch_scrape(&self, urls: &[&str]) -> Vec<(String, Result<ScrapeResult, CrawlError>)>
```

Scrape multiple URLs concurrently. Returns a vector of `(url, result)` pairs.

#### batch_crawl

```rust
pub async fn batch_crawl(&self, urls: &[&str]) -> Vec<(String, Result<CrawlResult, CrawlError>)>
```

Crawl multiple seed URLs concurrently.

## Example: Scrape

```rust
use kreuzcrawl::{CrawlEngine, CrawlError};

#[tokio::main]
async fn main() -> Result<(), CrawlError> {
    let engine = CrawlEngine::builder().build()?;
    let result = engine.scrape("https://example.com").await?;

    println!("Title: {:?}", result.metadata.title);
    println!("Links: {}", result.links.len());

    if let Some(ref md) = result.markdown {
        println!("Markdown:\n{}", md.content);
    }

    Ok(())
}
```

## Example: Crawl

```rust
use kreuzcrawl::{CrawlConfig, CrawlEngine, CrawlError};

#[tokio::main]
async fn main() -> Result<(), CrawlError> {
    let config = CrawlConfig {
        max_depth: Some(2),
        max_pages: Some(50),
        stay_on_domain: true,
        ..Default::default()
    };

    let engine = CrawlEngine::builder().config(config).build()?;
    let result = engine.crawl("https://example.com").await?;

    for page in &result.pages {
        println!("[{}] {} - {:?}", page.depth, page.url, page.metadata.title);
    }

    Ok(())
}
```

## Example: Custom Components

```rust
use kreuzcrawl::{CrawlEngine, PerDomainThrottle, DiskCache, Bm25Filter};
use std::time::Duration;

let engine = CrawlEngine::builder()
    .rate_limiter(PerDomainThrottle::new(Duration::from_millis(500)))
    .cache(DiskCache::new("/tmp/crawl-cache"))
    .content_filter(Bm25Filter::new())
    .build()?;
```

## Default Implementations

| Trait | Default | Alternatives |
|-------|---------|-------------|
| `Frontier` | `InMemoryFrontier` | -- |
| `RateLimiter` | `PerDomainThrottle` (200ms) | `NoopRateLimiter` |
| `CrawlStore` | `NoopStore` | -- |
| `EventEmitter` | `NoopEmitter` | -- |
| `CrawlStrategy` | `BfsStrategy` | `DfsStrategy`, `BestFirstStrategy`, `AdaptiveStrategy` |
| `ContentFilter` | `NoopFilter` | `Bm25Filter` |
| `CrawlCache` | `NoopCache` | `DiskCache` |

## REST API Server

```rust
use kreuzcrawl::{CrawlEngine, api};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), kreuzcrawl::CrawlError> {
    let engine = CrawlEngine::builder().build()?;
    api::serve("0.0.0.0", 3000, Arc::new(engine)).await
}
```

## MCP Server

```rust
use kreuzcrawl::mcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    mcp::start_mcp_server().await
}
```

## Bindings API

For polyglot bindings (Python, TypeScript, etc.), use the config-only construction path:

```rust
use kreuzcrawl::{create_engine, scrape, crawl, map_urls, batch_scrape, batch_crawl};
use kreuzcrawl::{CrawlConfig, CrawlEngineHandle};

let engine: CrawlEngineHandle = create_engine(None)?;
let result = scrape(&engine, "https://example.com").await?;
```

These functions use all default trait implementations internally and are the entry points that PyO3, NAPI-RS, and other binding layers call.

## Re-exports

All public types from `kreuzcrawl::types` and `kreuzcrawl::traits` are re-exported from the crate root. See [Types Reference](types.md) and [Errors Reference](errors.md) for complete type documentation.
