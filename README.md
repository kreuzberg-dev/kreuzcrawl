# kreuzcrawl

A high-performance Rust web crawling engine for extracting structured data from websites.

## Quick Start

Add to `Cargo.toml`:

```toml
[dependencies]
kreuzcrawl = { version = "0.1", features = ["ai", "browser"] }
tokio = { version = "1", features = ["full"] }
```

Basic usage:

```rust
use kreuzcrawl::{CrawlConfig, CrawlEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CrawlConfig {
        max_depth: Some(2),
        max_pages: Some(100),
        ..Default::default()
    };

    let engine = CrawlEngine::builder()
        .config(config)
        .build()?;

    let result = engine.crawl("https://example.com").await?;

    for page in &result.pages {
        println!("{}: {}", page.url, page.metadata.title.as_deref().unwrap_or(""));
        if let Some(ref md) = page.markdown {
            println!("  Markdown: {} chars", md.content.len());
            if let Some(ref citations) = md.citations {
                println!("  Citations: {} references", citations.references.len());
            }
        }
    }

    Ok(())
}
```

## Features

### Core Crawling

- **CrawlEngine** — Builder pattern with validated configuration
- **Concurrent fetching** — JoinSet + Semaphore for parallel requests
- **Multiple strategies** — BFS, DFS, BestFirst, Adaptive traversal
- **Batch crawling** — Multi-seed `batch_crawl()` + `batch_crawl_stream()`
- **Streaming** — Real-time event streaming via `crawl_stream()`
- **URL discovery** — Sitemap parsing + link extraction

### Metadata Extraction

- **40+ metadata fields** — Open Graph, Twitter Card, Dublin Core, Article, JSON-LD
- **Link extraction** — 4 types: Internal, External, Anchor, Document
- **Images** — All sources (img tags, picture, og:image, srcset)
- **Feed discovery** — RSS, Atom, JSON Feed detection
- **Favicons** — Extraction and canonicalization
- **hreflang** — Language/region variant links
- **Headings** — H1-H6 extraction with hierarchy

### Markdown Conversion

- **Always-on HTML→Markdown** — Automatic conversion with document structure preservation
- **Markdown result** — MarkdownResult with content, tables, code blocks
- **Link-to-citations** — Numbered reference conversion
- **Fit markdown** — Content pruning and LLM-optimized output via BM25 relevance scoring

### AI & LLM (feature-gated: `ai`)

- **LlmExtractor** — Via litellm (142+ provider support)
- **JSON schema extraction** — Structured extraction with custom schemas
- **Cost tracking** — Estimated costs and token usage counters
- **ExtractionMeta** — Full metadata on LLM results

### Anti-Bot & Browser Automation

- **WAF detection** — 8 vendors: Cloudflare, Akamai, AWS WAF, Imperva, DataDome, PerimeterX, Sucuri, F5
- **Browser fallback** — Headless Chrome via chromiumoxide (feature-gated: `browser`)
- **BrowserPool** — Multi-browser management with health checks and crash recovery
- **JavaScript rendering** — Heuristic-based detection

### Network & Caching

- **Per-domain rate limiting** — PerDomainThrottle with configurable delays
- **Proxy support** — HTTP, HTTPS, SOCKS5
- **Proxy rotation** — Middleware-based rotation
- **User-Agent rotation** — Pluggable UA strategies
- **HTTP caching** — ETag/Last-Modified conditional requests (CachingMiddleware)
- **Disk cache** — blake3-hashed storage with TTL and automatic eviction

### Content Filtering & Relevance

- **BM25 scoring** — Adaptive relevance evaluation
- **Adaptive crawling** — Term saturation detection for early termination
- **Content pruning** — Intelligent truncation for LLM consumption

### Compliance & Standards

- **robots.txt** — RFC 9309 user-agent prefix matching
- **Sitemap parsing** — XML, gzip, and index files
- **Config validation** — serde with `deny_unknown_fields`
- **Redirect handling** — HTTP 3xx, Refresh header, meta refresh
- **Cookie tracking** — Deduplication and persistence
- **Authentication** — Basic, Bearer, custom headers
- **Charset detection** — Automatic encoding detection
- **Binary/PDF skipping** — Content-type aware filtering

### Extensibility

**8 pluggable traits** for deep customization:

- `Frontier` — Custom URL queue implementations
- `RateLimiter` — Custom rate limiting strategies
- `CrawlStore` — Custom storage backends
- `CrawlMiddleware` — Request/response interceptors
- `EventEmitter` — Custom event handling
- `CrawlStrategy` — Custom traversal algorithms
- `ContentFilter` — Custom content evaluation
- `CrawlCache` — Custom caching backends

### CLI

Command-line tools for common operations:

```bash
# Scrape single page with metadata
kreuzcrawl scrape https://example.com

# Crawl with depth limiting and markdown output
kreuzcrawl crawl https://example.com --depth 2 --max-pages 50 --format markdown

# Discover all URLs via sitemap + crawling
kreuzcrawl map https://example.com --respect-robots-txt
```

## Feature Comparison

| Feature | kreuzcrawl | spider-rs | firecrawl | crawl4ai |
|---------|-----------|-----------|-----------|----------|
| **Language** | Rust | Rust | TypeScript | Python |
| **Concurrent fetching** | ✅ JoinSet | ✅ JoinSet | ✅ Promise.race | ✅ asyncio |
| **Traversal strategies** | ✅ BFS, DFS, BestFirst, Adaptive | ✅ BFS only | ✅ BFS only | ✅ BFS, DFS, BestFirst |
| **Markdown (always-on)** | ✅ + structure, tables | ✅ Basic | ✅ Primary output | ✅ Basic |
| **Link-to-citations** | ✅ Numbered refs | — | — | ✅ |
| **Fit markdown (pruned for LLM)** | ✅ BM25 + adaptive | — | — | ✅ BM25/LLM-based |
| **LLM extraction** | ✅ 142 providers (litellm) | ✅ OpenAI, Gemini | ✅ OpenAI | ✅ litellm |
| **Cost tracking** | ✅ Estimated + tokens | — | ✅ | ✅ |
| **Metadata fields** | ✅ 40+ (OG, Twitter, DC, JSON-LD) | ✅ Basic | ✅ Basic | ✅ Basic |
| **WAF detection** | ✅ 8 vendors | ✅ 20+ vendors | Cloud only | ✅ 3-tier |
| **Proxy support** | ✅ HTTP/HTTPS/SOCKS5 | ✅ | ✅ | ✅ |
| **Proxy rotation** | ✅ Middleware | ✅ ClientRotator | Cloud only | ✅ |
| **User-Agent rotation** | ✅ Middleware | ✅ | — | ✅ |
| **Browser fallback** | ✅ chromiumoxide | ✅ chromey | ✅ Engines | ✅ Playwright |
| **Disk cache** | ✅ blake3 + TTL | ✅ SQLite | — | ✅ SQLite |
| **Rate limiting** | ✅ Per-domain | ✅ | Backend managed | ✅ |
| **robots.txt** | ✅ RFC 9309 | ✅ Partial | ✅ Partial | ✅ Basic |
| **Config validation** | ✅ serde strict | — | — | — |
| **Pluggable traits** | ✅ 8 traits | — | — | ✅ Partial |
| **CLI tools** | ✅ scrape/crawl/map | ✅ | — | ✅ |
| **Batch crawling** | ✅ `batch_crawl()` | — | ✅ API | — |
| **Streaming events** | ✅ Real-time | ✅ | ✅ Polling | ✅ |
| **Asset download + dedup** | ✅ SHA-256 | — | — | — |
| **Feed discovery** | ✅ RSS, Atom, JSON | — | — | — |
| **JSON-LD extraction** | ✅ Full | — | — | — |
| **Screenshot capture** | Stub | ✅ | ✅ | ✅ |
| **Page interaction** | — | — | ✅ | ✅ |
| **REST API** | — | — | ✅ | — |
| **Language SDKs** | — | — | ✅ 4 languages | ✅ Python |

## Architecture

kreuzcrawl uses a **trait-based engine** with pluggable components:

### Core Traits

1. **Frontier** — URL queue management (default: VecDeque + HashSet)
2. **RateLimiter** — Request throttling (default: per-domain with backoff)
3. **CrawlStore** — Result storage (default: in-memory Vec)
4. **CrawlMiddleware** — Request/response interceptors (proxy rotation, UA rotation, caching)
5. **EventEmitter** — Event callbacks (default: no-op)
6. **CrawlStrategy** — Traversal algorithm (BFS, DFS, BestFirst, Adaptive)
7. **ContentFilter** — Relevance evaluation (BM25 scoring, adaptive saturation)
8. **CrawlCache** — Response caching (CachingMiddleware, DiskCache)

### Data Flow

```
CrawlEngine::crawl()
    ↓
CrawlStrategy (BFS/DFS/BestFirst/Adaptive)
    ↓
Frontier (URL queue)
    ↓
RateLimiter (throttle per domain)
    ↓
CrawlMiddleware (proxy, UA, cache)
    ↓
HTTP fetch (reqwest + retry)
    ↓
HTML extraction (40+ fields, links, markdown)
    ↓
ContentFilter (BM25 relevance)
    ↓
CrawlStore (accumulate results)
    ↓
EventEmitter (stream events)
```

## Configuration

`CrawlConfig` provides fine-grained control:

```rust
pub struct CrawlConfig {
    pub max_depth: Option<usize>,              // Max traversal depth
    pub max_pages: Option<usize>,              // Max pages to fetch
    pub timeout: Duration,                     // Per-request timeout
    pub follow_redirects: bool,                // HTTP 3xx handling
    pub respect_robots_txt: bool,              // RFC 9309 enforcement
    pub allowed_domains: Option<Vec<String>>,  // Domain whitelist
    pub exclude_patterns: Option<Vec<String>>, // URL regex filters
    pub headers: Option<HashMap<String, String>>, // Custom headers
    pub cookies: Option<HashMap<String, String>>, // Persistent cookies
    pub proxy: Option<String>,                 // HTTP proxy URL
    pub auth: Option<Auth>,                    // Basic/Bearer auth
    pub user_agent: Option<String>,            // Custom UA
    pub cache_dir: Option<PathBuf>,            // Disk cache location
    pub cache_ttl: Duration,                   // Cache expiration
}
```

**All validation** is performed in `CrawlEngine::builder().build()` — invalid configs fail fast.

## CLI

### Commands

```bash
# Scrape a single page
kreuzcrawl scrape <URL>

# Crawl with traversal
kreuzcrawl crawl <URL> \
  --depth <N> \
  --max-pages <N> \
  --format <markdown|json|html> \
  --respect-robots-txt

# Discover URLs (sitemap + crawl)
kreuzcrawl map <URL> \
  --respect-robots-txt \
  --output <file>
```

### Output Formats

- **markdown** — MarkdownResult with citations and fit markdown
- **json** — Full CrawlResult with all metadata
- **html** — Original HTML + extracted links

## License

MIT License — see [LICENSE](LICENSE).
