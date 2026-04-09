# Competitive Landscape

How kreuzcrawl compares to other web crawling and scraping tools.

---

title: Kreuzcrawl vs Firecrawl
description: Detailed comparison of Kreuzcrawl and Firecrawl web crawling tools
---

## Kreuzcrawl vs Firecrawl

[Firecrawl](https://github.com/mendableai/firecrawl) is a TypeScript-based web scraping platform built for SaaS-first delivery. It provides a hosted API, self-hosted Docker deployment, and SDKs for Python, JavaScript, Go, Java, Elixir, and Rust. Firecrawl focuses on making web data accessible through a managed service with built-in page interaction, LLM extraction, and stealth capabilities.

Kreuzcrawl is a Rust library and CLI designed to be embedded directly in your application. It provides a trait-based engine with a Tower middleware stack, giving developers fine-grained control over every stage of the crawl pipeline.

The fundamental difference: **Firecrawl is a service you call; Kreuzcrawl is a library you embed.**

---

## Feature Comparison

| Feature | Kreuzcrawl | Firecrawl |
|---|---|---|
| **Language** | Rust | TypeScript |
| **License** | Elastic-2.0 | AGPL-3.0 |
| **Distribution** | Library + CLI | SaaS + Self-hosted |
| **Headless browser** | chromiumoxide | Playwright |
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | BFS |
| **Concurrent fetching** | JoinSet + Semaphore | Bull queue workers |
| **Streaming events** | Real-time | SSE / polling |
| **Batch operations** | `batch_crawl()` | Async API |
| **Sitemap parsing** | XML, gzip, index | Yes |
| **robots.txt** | RFC 9309 | Yes |
| **Markdown conversion** | Always-on + structure preservation | Primary output |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | No |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | Basic |
| **JSON-LD extraction** | Full | No |
| **Feed discovery** | RSS, Atom, JSON Feed | No |
| **Link-to-citations** | Numbered refs | No |
| **LLM extraction** | Multi-provider (liter-llm) | 10+ providers |
| **Cost tracking** | USD + tokens | Yes |
| **PDF extraction** | No | Yes (FirePDF) |
| **WAF detection** | 8 vendors | Cloud only |
| **Stealth / anti-detect** | UA rotation | Stealth injection |
| **Proxy support** | HTTP/HTTPS/SOCKS5 | Rotating proxies |
| **Screenshot capture** | Stub | Yes |
| **Page interaction** | No | Click, scroll, type |
| **REST API server** | No | Yes (primary interface) |
| **MCP server** | No | Yes |
| **CLI** | scrape/crawl/map | No |
| **Language SDKs** | Rust only | Python, JS, Go, Java, Elixir, Rust |
| **Disk cache** | blake3 + TTL | Redis |
| **Per-domain rate limiting** | Tower layer | Global RPS |
| **HTTP caching (ETag)** | Yes | No |
| **Pluggable traits** | 7 traits | No |
| **Middleware stack** | Tower services | No |
| **Config validation** | serde strict | No |
| **BM25 relevance scoring** | Yes | No |
| **Adaptive crawling** | Term saturation | No |
| **Search integration** | No | Yes |

---

## Where Kreuzcrawl Wins

**Performance and resource efficiency.** Kreuzcrawl is compiled Rust with zero garbage collection overhead. For high-volume crawling workloads, this translates to lower memory usage and higher throughput per CPU core.

**Architectural extensibility.** Seven pluggable traits (`Frontier`, `RateLimiter`, `CrawlStore`, `EventEmitter`, `CrawlStrategy`, `ContentFilter`, `CrawlCache`) let you replace any component of the crawl pipeline. The Tower middleware stack allows composable request/response processing. Firecrawl has no equivalent extension mechanism.

**Crawl intelligence.** BM25 relevance scoring and adaptive crawling with term saturation detection allow Kreuzcrawl to prioritize high-value pages and terminate early when content becomes repetitive. Firecrawl offers only BFS traversal.

**Rich metadata extraction.** Kreuzcrawl extracts 40+ metadata fields including full JSON-LD, Dublin Core, feed discovery, and hreflang -- significantly more than Firecrawl's basic metadata output.

**Per-domain rate limiting.** Kreuzcrawl's Tower-based rate limiter operates per domain with backoff, while Firecrawl uses a global RPS limit. Per-domain limiting is more respectful to target sites and less likely to trigger blocks.

**No cloud dependency.** Kreuzcrawl runs entirely in your infrastructure with no external service calls. This matters for air-gapped environments, data sovereignty requirements, and cost predictability.

---

## Where Firecrawl Wins

**Managed infrastructure.** Firecrawl's cloud service handles browser management, proxy rotation, and scaling. You do not need to provision headless browsers or manage infrastructure.

**Page interaction.** Firecrawl supports clicking, scrolling, typing, and other browser interactions. This is essential for SPAs, infinite scroll pages, and sites that require form submission before revealing content. Kreuzcrawl does not currently support page interaction.

**Broad LLM provider support.** Firecrawl integrates with 10+ LLM providers out of the box. While Kreuzcrawl supports multiple providers through liter-llm, Firecrawl's ecosystem is more mature here.

**Language SDK breadth.** Firecrawl provides official SDKs for Python, JavaScript, Go, Java, Elixir, and Rust. Kreuzcrawl is currently Rust-only (polyglot bindings are in development).

**PDF extraction.** FirePDF handles PDF content extraction natively. Kreuzcrawl does not currently extract PDF content.

**REST API and MCP.** Firecrawl's primary interface is a REST API, making integration trivial for any language. It also provides an MCP server for AI agent workflows.

**Screenshot capture.** Firecrawl captures full-page screenshots. Kreuzcrawl has only a stub implementation.

---

## When to Use Which

**Choose Kreuzcrawl when:**

- You need to embed crawling directly in a Rust application
- You require custom crawl strategies, storage backends, or middleware
- You want per-domain rate limiting and HTTP caching (ETag/Last-Modified)
- You need rich metadata extraction (JSON-LD, Dublin Core, feeds)
- You want relevance-based crawling with BM25 scoring
- You are building a pipeline where you control the full stack
- Data sovereignty or air-gapped deployment is a requirement

**Choose Firecrawl when:**

- You want a managed service with minimal infrastructure management
- You need page interaction (clicking, scrolling, form filling)
- You need SDKs in Python, JavaScript, Go, or Java today
- You need PDF extraction
- You want a REST API that any language can call immediately
- You prefer SaaS pricing over self-managed infrastructure costs

---

## License Comparison

| | Kreuzcrawl | Firecrawl |
|---|---|---|
| **License** | Elastic-2.0 | AGPL-3.0 |
| **Commercial use** | Yes | Yes |
| **Modification** | Yes | Yes, but must open-source if hosting |
| **Hosting restriction** | Cannot provide as a managed crawling service | Must open-source all modifications when hosting |
| **Embedding in proprietary software** | Yes | Requires AGPL compliance (copyleft) |

Firecrawl's AGPL-3.0 license requires that any modifications to the software be open-sourced when the software is offered as a network service. This has significant implications for companies that want to self-host a modified version. Kreuzcrawl's Elastic-2.0 license allows proprietary modifications but prohibits offering Kreuzcrawl itself as a managed service.

---

title: Kreuzcrawl vs Spider
description: Detailed comparison of Kreuzcrawl and Spider web crawling libraries
---

## Kreuzcrawl vs Spider

[Spider](https://github.com/spider-rs/spider) is a mature Rust web crawling library (v2.48+) with a broad feature set including WARC output, search provider integration, AIMD-based concurrency control, and SDKs for Python and Node.js. Spider has been in development longer and has a wider deployment base.

Both Kreuzcrawl and Spider are Rust-native crawling libraries, making this the most direct architectural comparison in this series. The key differences lie in extensibility approach, content intelligence, and middleware design.

---

## Feature Comparison

| Feature | Kreuzcrawl | Spider |
|---|---|---|
| **Language** | Rust | Rust |
| **License** | Elastic-2.0 | MIT |
| **Distribution** | Library + CLI | Library + CLI + SaaS |
| **Headless browser** | chromiumoxide | chromey / WebDriver |
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | BFS |
| **Concurrent fetching** | JoinSet + Semaphore | Tokio multi-thread + AIMD |
| **Streaming events** | Real-time | Subscriber channels |
| **Batch operations** | `batch_crawl()` | No |
| **Sitemap parsing** | XML, gzip, index | Yes |
| **robots.txt** | RFC 9309 | Yes, with caching |
| **Markdown conversion** | Always-on + structure preservation | Yes |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | No |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | Basic |
| **JSON-LD extraction** | Full | No |
| **Feed discovery** | RSS, Atom, JSON Feed | No |
| **Link-to-citations** | Numbered refs | No |
| **LLM extraction** | Multi-provider (liter-llm) | OpenAI, Gemini |
| **Cost tracking** | USD + tokens | No |
| **PDF extraction** | No | No |
| **WAF detection** | 8 vendors | Smart mode (auto-escalate) |
| **Stealth / anti-detect** | UA rotation | ua_generator |
| **Proxy support** | HTTP/HTTPS/SOCKS5 | HTTP/HTTPS/SOCKS + Cloud |
| **User-Agent rotation** | Tower layer | Yes |
| **Screenshot capture** | Stub | Yes |
| **Page interaction** | No | Agent automation |
| **REST API server** | No | No |
| **MCP server** | No | Yes |
| **CLI** | scrape/crawl/map | Yes |
| **Language SDKs** | Rust only | Rust, Python, Node.js |
| **Disk cache** | blake3 + TTL | SQLite |
| **Per-domain rate limiting** | Tower layer | Token bucket + auto-throttle |
| **HTTP caching (ETag)** | Yes | Yes |
| **Pluggable traits** | 7 traits | No |
| **Middleware stack** | Tower services | No |
| **Config validation** | serde strict | No |
| **BM25 relevance scoring** | Yes | No |
| **Adaptive crawling** | Term saturation | No |
| **Asset download + dedup** | SHA-256 | No |
| **Search integration** | No | Serper, Brave, Bing, Tavily |
| **WARC output** | No | Yes |

---

## Where Kreuzcrawl Wins

**Pluggable architecture.** Kreuzcrawl defines 7 traits (`Frontier`, `RateLimiter`, `CrawlStore`, `EventEmitter`, `CrawlStrategy`, `ContentFilter`, `CrawlCache`) that let you replace any component. Spider does not expose equivalent extension points. If you need a custom URL queue, a custom storage backend, or a custom traversal algorithm, Kreuzcrawl's trait system makes this straightforward.

**Tower middleware stack.** Kreuzcrawl's request/response pipeline is built on Tower services -- the same abstraction used by Axum and Tonic. This means you can compose layers for tracing, caching, rate limiting, and UA rotation in a standard, well-understood pattern. Spider handles these concerns internally without a composable middleware abstraction.

**Crawl intelligence.** Four traversal strategies (BFS, DFS, BestFirst, Adaptive) versus Spider's BFS-only approach. BM25 relevance scoring lets Kreuzcrawl prioritize pages that match a query, and adaptive crawling terminates early when term saturation is detected.

**Metadata depth.** Kreuzcrawl extracts 40+ metadata fields including full JSON-LD, Dublin Core, RSS/Atom/JSON Feed discovery, and hreflang links. Spider provides basic metadata extraction.

**Fit markdown.** BM25-based content pruning produces LLM-optimized markdown that reduces token consumption. Spider does not offer content pruning.

**Strict configuration.** Kreuzcrawl uses serde with `deny_unknown_fields` for configuration validation, catching typos and invalid fields at build time rather than silently ignoring them.

---

## Where Spider Wins

**Maturity and stability.** Spider is at v2.48+ with a longer track record in production. It has had more time to handle edge cases across the wide variety of websites in the real world.

**WARC output.** Spider supports WARC (Web ARChive) format, which is the standard for web archival and replay. Kreuzcrawl does not currently produce WARC output.

**Search provider integration.** Spider integrates with Serper, Brave, Bing, and Tavily for search-driven crawling. This is useful for discovery workflows where you do not have a starting URL. Kreuzcrawl has no search integration.

**AIMD concurrency control.** Spider uses Additive Increase/Multiplicative Decrease (the algorithm behind TCP congestion control) to dynamically adjust concurrency based on server response. This is a sophisticated approach to avoiding rate limiting. Kreuzcrawl uses static semaphore-based concurrency with per-domain rate limiting.

**Page interaction.** Spider supports agent-based browser automation for interacting with page elements. Kreuzcrawl does not currently support page interaction.

**Language SDKs.** Spider provides official Python and Node.js bindings in addition to Rust. Kreuzcrawl is currently Rust-only.

**MCP server.** Spider provides an MCP server for AI agent integration. Kreuzcrawl does not.

**MIT license.** Spider's MIT license has no restrictions on commercial use, modification, or hosting. This is the most permissive option available.

---

## When to Use Which

**Choose Kreuzcrawl when:**

- You need to customize crawl components (frontier, strategy, storage, filtering)
- You want Tower middleware composition for request/response processing
- You need multiple traversal strategies (DFS, BestFirst, Adaptive)
- You want BM25 relevance scoring or adaptive early termination
- You need deep metadata extraction (JSON-LD, Dublin Core, feeds)
- You want LLM-optimized content pruning (fit markdown)
- You need strict configuration validation

**Choose Spider when:**

- You need WARC output for web archival
- You need search provider integration (Serper, Brave, Bing, Tavily)
- You need Python or Node.js bindings today
- You need page interaction / browser automation
- You want AIMD-based dynamic concurrency control
- You prefer MIT licensing with no restrictions
- You need MCP server support for AI agent workflows

---

## License Comparison

| | Kreuzcrawl | Spider |
|---|---|---|
| **License** | Elastic-2.0 | MIT |
| **Commercial use** | Yes | Yes |
| **Modification** | Yes | Yes |
| **Hosting restriction** | Cannot provide as a managed crawling service | None |
| **Embedding in proprietary software** | Yes | Yes |

Spider's MIT license is maximally permissive -- there are no restrictions on use, modification, or distribution. Kreuzcrawl's Elastic-2.0 license allows all commercial use and modification but prohibits offering Kreuzcrawl itself as a managed crawling service. For most use cases (embedding in applications, internal tooling, commercial products), both licenses work. The distinction matters only if you plan to offer a hosted crawling API as a product.

---

title: Kreuzcrawl vs Crawl4AI
description: Detailed comparison of Kreuzcrawl and Crawl4AI web crawling tools
---

## Kreuzcrawl vs Crawl4AI

[Crawl4AI](https://github.com/unclecode/crawl4ai) is a Python-based web crawling library optimized for AI and LLM workflows. With over 50,000 GitHub stars, it is one of the most popular open-source crawling tools. Crawl4AI provides a Python-native API, Playwright-based browser automation, BM25 content scoring, and a growing ecosystem of extraction strategies.

The core difference is the language runtime: **Kreuzcrawl is compiled Rust with zero GC overhead; Crawl4AI is Python with Playwright for browser automation.** This creates distinct trade-offs around performance, memory usage, and ecosystem integration.

---

## Feature Comparison

| Feature | Kreuzcrawl | Crawl4AI |
|---|---|---|
| **Language** | Rust | Python |
| **License** | Elastic-2.0 | Apache-2.0 |
| **Distribution** | Library + CLI | Library + CLI + API |
| **Headless browser** | chromiumoxide | Playwright |
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | BFS, DFS, BestFirst |
| **Concurrent fetching** | JoinSet + Semaphore | asyncio browser pool |
| **Streaming events** | Real-time | Yes |
| **Batch operations** | `batch_crawl()` | Deep crawl |
| **Sitemap parsing** | XML, gzip, index | No |
| **robots.txt** | RFC 9309 | Basic |
| **Markdown conversion** | Always-on + structure preservation | Yes |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | BM25/LLM-based |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | Basic |
| **JSON-LD extraction** | Full | No |
| **Feed discovery** | RSS, Atom, JSON Feed | No |
| **Link-to-citations** | Numbered refs | Yes |
| **LLM extraction** | Multi-provider (liter-llm) | litellm |
| **Cost tracking** | USD + tokens | Yes |
| **PDF extraction** | No | Yes |
| **WAF detection** | 8 vendors | 3-tier detection |
| **Stealth / anti-detect** | UA rotation | Playwright Stealth |
| **Proxy support** | HTTP/HTTPS/SOCKS5 | Yes, with escalation |
| **User-Agent rotation** | Tower layer | Yes |
| **Screenshot capture** | Stub | Yes |
| **Page interaction** | No | JS execution |
| **REST API server** | No | FastAPI |
| **MCP server** | No | Yes |
| **CLI** | scrape/crawl/map | `crwl` |
| **Language SDKs** | Rust only | Python |
| **Disk cache** | blake3 + TTL | SQLite |
| **Per-domain rate limiting** | Tower layer | Adaptive |
| **HTTP caching (ETag)** | Yes | No |
| **Pluggable traits** | 7 traits | Partial (strategies) |
| **Middleware stack** | Tower services | No |
| **Config validation** | serde strict | No |
| **BM25 relevance scoring** | Yes | Yes |
| **Adaptive crawling** | Term saturation | Pattern learning |
| **Search integration** | No | Google |

---

## Where Kreuzcrawl Wins

**Performance.** Rust compiles to native code with no garbage collector, no interpreter overhead, and no GIL. For CPU-bound extraction tasks -- HTML parsing, metadata extraction, markdown conversion -- Kreuzcrawl will be significantly faster per core. Memory usage is also lower and more predictable, which matters for large-scale crawls.

**Architectural extensibility.** Kreuzcrawl's 7 pluggable traits provide a formal extension contract. You can replace the URL frontier, rate limiter, storage backend, event system, traversal strategy, content filter, or cache without modifying the engine. Crawl4AI offers partial strategy customization but does not expose the same breadth of extension points.

**Tower middleware composition.** The Tower service stack allows you to compose request/response processing layers (tracing, caching, rate limiting, UA rotation) using a standard Rust ecosystem pattern. This is a fundamentally different approach from Crawl4AI's monolithic pipeline.

**Metadata depth.** Kreuzcrawl extracts 40+ fields including full JSON-LD, Dublin Core, feed discovery, and hreflang. Crawl4AI extracts basic metadata.

**Per-domain rate limiting.** Kreuzcrawl's Tower-based rate limiter operates per domain with backoff. This is more respectful to target sites than global rate limiting.

**HTTP caching.** ETag and Last-Modified conditional request support reduces unnecessary re-fetching. Crawl4AI does not implement HTTP-level caching.

**Sitemap parsing.** Kreuzcrawl parses XML sitemaps, gzip-compressed sitemaps, and sitemap index files for comprehensive URL discovery. Crawl4AI does not parse sitemaps.

---

## Where Crawl4AI Wins

**Community and ecosystem.** With 50,000+ GitHub stars and an active community, Crawl4AI has extensive documentation, tutorials, community support, and battle-tested real-world usage. Finding help, examples, and integrations is easier.

**Python-native.** If your stack is Python -- and many AI/ML pipelines are -- Crawl4AI integrates without any FFI boundary. You can use it directly in Jupyter notebooks, combine it with pandas, pass results to scikit-learn or PyTorch, and debug with standard Python tools.

**Page interaction.** Crawl4AI supports JavaScript execution through Playwright, enabling interaction with dynamic pages, SPAs, and content behind client-side rendering. Kreuzcrawl does not currently support page interaction.

**PDF extraction.** Crawl4AI handles PDF content extraction. Kreuzcrawl does not.

**Screenshot capture.** Full-page screenshot capture is supported. Kreuzcrawl has only a stub.

**REST API and MCP.** Crawl4AI provides a FastAPI-based REST API and an MCP server for AI agent workflows. Kreuzcrawl provides neither.

**Adaptive crawling with pattern learning.** Crawl4AI's adaptive crawling learns content patterns during a crawl and adjusts its strategy accordingly. Kreuzcrawl's adaptive strategy uses term saturation, which is effective but less dynamic.

**Search integration.** Crawl4AI integrates with Google search for discovery-driven crawling.

---

## When to Use Which

**Choose Kreuzcrawl when:**

- Performance is critical (high-volume crawling, tight latency requirements)
- Memory efficiency matters (large-scale crawls, constrained environments)
- You need deep architectural customization (custom frontier, storage, middleware)
- You are building a Rust application or polyglot system with Rust at the core
- You need comprehensive metadata extraction (JSON-LD, Dublin Core, feeds)
- You need sitemap-based URL discovery
- You want composable Tower middleware

**Choose Crawl4AI when:**

- Your stack is Python and you want zero-friction integration
- You need page interaction or JavaScript execution
- You need PDF extraction
- You want an active community with extensive examples and support
- You need a REST API or MCP server
- You are prototyping or working in Jupyter notebooks
- You need search-driven crawling via Google

---

## License Comparison

| | Kreuzcrawl | Crawl4AI |
|---|---|---|
| **License** | Elastic-2.0 | Apache-2.0 |
| **Commercial use** | Yes | Yes |
| **Modification** | Yes | Yes |
| **Hosting restriction** | Cannot provide as a managed crawling service | None |
| **Patent grant** | No | Yes (explicit patent grant) |
| **Embedding in proprietary software** | Yes | Yes |

Crawl4AI's Apache-2.0 license is highly permissive with an explicit patent grant, meaning contributors cannot later assert patent claims against users. Kreuzcrawl's Elastic-2.0 license allows all commercial use and modification but prohibits offering Kreuzcrawl itself as a managed crawling service. For embedding in applications or internal use, both licenses work well.

---

title: Kreuzcrawl vs Webclaw
description: Detailed comparison of Kreuzcrawl and Webclaw web crawling tools
---

## Kreuzcrawl vs Webclaw

[Webclaw](https://github.com/peterprototypes/webclaw) is a Rust web crawling tool that deliberately avoids headless browsers. Instead, Webclaw uses TLS fingerprinting and HTTP-level stealth to bypass anti-bot protections. This browserless approach yields extremely fast extraction times (reported 3.2ms per page) and zero browser overhead. Webclaw also includes brand extraction and MCP server support.

The core architectural difference: **Webclaw is browserless by design; Kreuzcrawl uses browser fallback when HTTP-only fetching is insufficient.** This creates distinct trade-offs around speed, compatibility, and resource usage.

---

## Feature Comparison

| Feature | Kreuzcrawl | Webclaw |
|---|---|---|
| **Language** | Rust | Rust |
| **License** | Elastic-2.0 | AGPL-3.0 |
| **Distribution** | Library + CLI | Library + CLI + MCP |
| **Headless browser** | chromiumoxide | None (TLS fingerprint) |
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | BFS |
| **Concurrent fetching** | JoinSet + Semaphore | Tokio |
| **Streaming events** | Real-time | No |
| **Batch operations** | `batch_crawl()` | Yes |
| **Sitemap parsing** | XML, gzip, index | Yes |
| **robots.txt** | RFC 9309 | Yes |
| **Markdown conversion** | Always-on + structure preservation | Yes |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | Token-optimized |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | Moderate |
| **JSON-LD extraction** | Full | Data islands |
| **Feed discovery** | RSS, Atom, JSON Feed | No |
| **Link-to-citations** | Numbered refs | No |
| **LLM extraction** | Multi-provider (liter-llm) | Ollama (local) |
| **Cost tracking** | USD + tokens | No |
| **PDF extraction** | No | Yes |
| **WAF detection** | 8 vendors | No |
| **Stealth / anti-detect** | UA rotation | TLS fingerprinting |
| **Proxy support** | HTTP/HTTPS/SOCKS5 | Yes |
| **User-Agent rotation** | Tower layer | No |
| **Screenshot capture** | Stub | No |
| **Page interaction** | No | No |
| **REST API server** | No | No |
| **MCP server** | No | Yes |
| **CLI** | scrape/crawl/map | Yes |
| **Language SDKs** | Rust only | Rust |
| **Disk cache** | blake3 + TTL | No |
| **Per-domain rate limiting** | Tower layer | No |
| **HTTP caching (ETag)** | Yes | No |
| **Pluggable traits** | 7 traits | No |
| **Middleware stack** | Tower services | No |
| **Config validation** | serde strict | No |
| **BM25 relevance scoring** | Yes | No |
| **Adaptive crawling** | Term saturation | No |
| **Asset download + dedup** | SHA-256 | No |
| **Search integration** | No | API key |

---

## Where Kreuzcrawl Wins

**Browser fallback.** Kreuzcrawl can fall back to headless Chrome (via chromiumoxide) when HTTP-only fetching fails. This handles JavaScript-rendered SPAs, Cloudflare challenges, and other scenarios where a real browser is required. Webclaw has no browser capability -- if TLS fingerprinting fails against a particular WAF, there is no fallback path.

**Pluggable architecture.** Seven traits and a Tower middleware stack provide deep customization. Webclaw is a more opinionated tool without equivalent extension points.

**Crawl intelligence.** Four traversal strategies, BM25 relevance scoring, and adaptive crawling with term saturation detection. Webclaw offers BFS only.

**Rich metadata.** 40+ metadata fields versus Webclaw's moderate extraction. Full JSON-LD parsing, Dublin Core, feed discovery, and hreflang links are available in Kreuzcrawl.

**WAF detection.** Kreuzcrawl detects 8 WAF vendors (Cloudflare, Akamai, AWS WAF, Imperva, DataDome, PerimeterX, Sucuri, F5) and can route to browser fallback. Webclaw does not detect WAFs -- it relies on TLS fingerprinting to avoid triggering them in the first place.

**Per-domain rate limiting and caching.** Tower-based per-domain rate limiting, ETag/Last-Modified HTTP caching, and blake3-hashed disk cache with TTL. Webclaw has none of these.

**Streaming events.** Real-time event streaming during crawls. Webclaw does not support streaming.

---

## Where Webclaw Wins

**Raw speed.** Without browser overhead, Webclaw achieves reported 3.2ms extraction times per page. This is an order of magnitude faster than any browser-based approach. For bulk extraction of static or server-rendered content, Webclaw's pure HTTP approach is extremely efficient.

**Zero browser dependencies.** No Chromium binary, no browser process management, no browser crashes to recover from. This simplifies deployment, reduces Docker image size, and eliminates an entire class of operational issues.

**TLS fingerprinting.** Webclaw's stealth approach operates at the TLS handshake level, which is harder for WAFs to detect than browser-level stealth injection. For many sites, this is sufficient to avoid blocks without the weight of a full browser.

**Brand extraction.** Webclaw includes built-in brand/company extraction from web pages -- a specialized feature for business intelligence use cases.

**PDF extraction.** Webclaw handles PDF content extraction. Kreuzcrawl does not.

**MCP server.** Webclaw provides an MCP server for AI agent integration. Kreuzcrawl does not.

**Lower resource footprint.** No browser means dramatically less memory and CPU per crawl worker. A single machine can run many more concurrent Webclaw workers than browser-based crawlers.

**Search integration.** Webclaw supports search-driven discovery via API key integration.

---

## When to Use Which

**Choose Kreuzcrawl when:**

- You need to handle JavaScript-rendered content or SPAs
- You need browser fallback for sites that block HTTP-only requests
- You want pluggable architecture with custom components
- You need multiple traversal strategies and relevance scoring
- You need comprehensive metadata extraction
- You need per-domain rate limiting and HTTP caching
- You want streaming events during crawls

**Choose Webclaw when:**

- Speed is the top priority and browser rendering is not required
- You are crawling server-rendered or static content at high volume
- You want minimal deployment dependencies (no Chromium)
- You need brand extraction for business intelligence
- You need PDF extraction
- You want MCP server support for AI agent workflows
- Resource efficiency (memory, CPU) is a primary constraint

---

## License Comparison

| | Kreuzcrawl | Webclaw |
|---|---|---|
| **License** | Elastic-2.0 | AGPL-3.0 |
| **Commercial use** | Yes | Yes |
| **Modification** | Yes | Yes, but must open-source if hosting |
| **Hosting restriction** | Cannot provide as a managed crawling service | Must open-source all modifications when hosting |
| **Embedding in proprietary software** | Yes | Requires AGPL compliance (copyleft) |

Webclaw's AGPL-3.0 license has strong copyleft requirements: if you modify Webclaw and offer it as a network service, you must release your modifications under AGPL. This also applies to software that links against Webclaw in some interpretations. Kreuzcrawl's Elastic-2.0 license allows proprietary modifications but prohibits offering Kreuzcrawl itself as a managed service. For embedding in a proprietary application, Elastic-2.0 is less restrictive than AGPL.

---

title: Kreuzcrawl vs CRW
description: Detailed comparison of Kreuzcrawl and CRW web crawling tools
---

## Kreuzcrawl vs CRW

[CRW](https://github.com/nicholasgasior/crw) is a Rust-based web crawling tool that emphasizes simplicity and Firecrawl API compatibility. CRW provides a Firecrawl-compatible REST API, making it a drop-in replacement for teams migrating from Firecrawl's hosted service. It supports LightPanda as an alternative lightweight browser and includes MCP server support.

The key distinction: **CRW focuses on being a simple, Firecrawl-compatible server; Kreuzcrawl focuses on being a deeply extensible crawling library.** CRW optimizes for ease of deployment and API compatibility, while Kreuzcrawl optimizes for architectural flexibility and content intelligence.

---

## Feature Comparison

| Feature | Kreuzcrawl | CRW |
|---|---|---|
| **Language** | Rust | Rust |
| **License** | Elastic-2.0 | AGPL-3.0 |
| **Distribution** | Library + CLI | CLI + MCP + API |
| **Headless browser** | chromiumoxide | LightPanda / Chrome |
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | BFS |
| **Concurrent fetching** | JoinSet + Semaphore | Tokio |
| **Streaming events** | Real-time | No |
| **Batch operations** | `batch_crawl()` | Yes |
| **Sitemap parsing** | XML, gzip, index | Yes |
| **robots.txt** | RFC 9309 | Yes |
| **Markdown conversion** | Always-on + structure preservation | Yes |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | No |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | Basic |
| **JSON-LD extraction** | Full | No |
| **Feed discovery** | RSS, Atom, JSON Feed | No |
| **Link-to-citations** | Numbered refs | No |
| **LLM extraction** | Multi-provider (liter-llm) | Claude, OpenAI |
| **Cost tracking** | USD + tokens | No |
| **PDF extraction** | No | Yes |
| **WAF detection** | 8 vendors | No |
| **Stealth / anti-detect** | UA rotation | Stealth injection |
| **Proxy support** | HTTP/HTTPS/SOCKS5 | HTTP/SOCKS5 |
| **User-Agent rotation** | Tower layer | Yes |
| **Screenshot capture** | Stub | No |
| **Page interaction** | No | No |
| **REST API server** | No | Yes (Firecrawl-compatible) |
| **MCP server** | No | Yes |
| **CLI** | scrape/crawl/map | Yes |
| **Language SDKs** | Rust only | Rust |
| **Disk cache** | blake3 + TTL | No |
| **Per-domain rate limiting** | Tower layer | Global RPS |
| **HTTP caching (ETag)** | Yes | No |
| **Pluggable traits** | 7 traits | No |
| **Middleware stack** | Tower services | No |
| **Config validation** | serde strict | No |
| **BM25 relevance scoring** | Yes | No |
| **Adaptive crawling** | Term saturation | No |
| **Asset download + dedup** | SHA-256 | No |
| **Search integration** | No | API key |

---

## Where Kreuzcrawl Wins

**Deep extensibility.** Seven pluggable traits and a Tower middleware stack allow you to customize every aspect of the crawl pipeline. CRW is designed as a turnkey server -- you configure it, but you do not extend it at the architectural level.

**Crawl intelligence.** BFS, DFS, BestFirst, and Adaptive traversal strategies with BM25 relevance scoring and term saturation-based early termination. CRW supports BFS only.

**Content quality.** Fit markdown with BM25-based content pruning, 40+ metadata fields, full JSON-LD extraction, feed discovery, and link-to-citations conversion. CRW produces basic metadata and standard markdown.

**WAF detection.** Kreuzcrawl detects 8 WAF vendors and can route to browser fallback. CRW does not detect WAFs.

**Caching infrastructure.** blake3-hashed disk cache with TTL eviction, ETag/Last-Modified HTTP caching, and per-domain rate limiting via Tower layers. CRW has no disk cache, no HTTP caching, and only global RPS limiting.

**Streaming events.** Real-time event streaming during crawls for monitoring and progress tracking. CRW does not support streaming.

**Cost tracking.** Kreuzcrawl tracks LLM extraction costs in USD and tokens. CRW does not track costs.

---

## Where CRW Wins

**Firecrawl API compatibility.** CRW implements a Firecrawl-compatible REST API, making it a drop-in replacement for existing Firecrawl integrations. If you have applications that call the Firecrawl API, you can point them at CRW without code changes. Kreuzcrawl does not provide a REST API.

**LightPanda browser.** CRW supports LightPanda as an alternative to Chromium. LightPanda is a lightweight browser engine that uses significantly less memory than full Chromium, making it viable for resource-constrained deployments.

**Simple deployment.** CRW is designed as a server you deploy and call via HTTP. For teams that want a crawling service without writing Rust code, CRW is more accessible. Kreuzcrawl requires Rust integration (until polyglot bindings are available).

**MCP server.** CRW provides an MCP server for AI agent workflows. Kreuzcrawl does not.

**PDF extraction.** CRW handles PDF content extraction. Kreuzcrawl does not.

**Search integration.** CRW supports search-driven discovery via API key integration.

---

## When to Use Which

**Choose Kreuzcrawl when:**

- You are building a Rust application that needs embedded crawling
- You need custom crawl components (frontier, strategy, storage, middleware)
- You need relevance-based crawling with BM25 scoring
- You want deep metadata extraction and LLM-optimized content
- You need per-domain rate limiting and HTTP caching
- You need streaming events for real-time monitoring
- You want cost tracking for LLM extraction

**Choose CRW when:**

- You want a Firecrawl-compatible API server with no code changes
- You are migrating from Firecrawl and want API compatibility
- You want a simple deployment model (server + HTTP API)
- You need a lightweight browser option (LightPanda)
- You need PDF extraction
- You need MCP server support for AI agents
- You do not need custom crawl logic or extensibility

---

## License Comparison

| | Kreuzcrawl | CRW |
|---|---|---|
| **License** | Elastic-2.0 | AGPL-3.0 |
| **Commercial use** | Yes | Yes |
| **Modification** | Yes | Yes, but must open-source if hosting |
| **Hosting restriction** | Cannot provide as a managed crawling service | Must open-source all modifications when hosting |
| **Embedding in proprietary software** | Yes | Requires AGPL compliance (copyleft) |

CRW's AGPL-3.0 license requires that modifications be open-sourced when the software is offered as a network service. Since CRW is primarily a server, this means any customizations to a hosted CRW instance must be released under AGPL. Kreuzcrawl's Elastic-2.0 license allows proprietary modifications but prohibits offering Kreuzcrawl itself as a managed crawling service. For embedding in proprietary applications, Elastic-2.0 is less restrictive.

---

title: Kreuzcrawl vs ScrapeGraphAI
description: Detailed comparison of Kreuzcrawl and ScrapeGraphAI web crawling tools
---

## Kreuzcrawl vs ScrapeGraphAI

[ScrapeGraphAI](https://github.com/ScrapeGraphAI/Scrapegraph-ai) is a Python library that puts LLMs at the center of the extraction pipeline. Instead of writing CSS selectors or XPath queries, you describe what you want in natural language and ScrapeGraphAI uses LLM-driven graph execution to extract it. It integrates deeply with LangChain and supports multiple LLM providers.

The fundamental difference: **ScrapeGraphAI is LLM-native -- the LLM is the extraction engine. Kreuzcrawl is extraction-native -- structured parsing runs first, with LLM as an optional enhancement.** This creates very different cost profiles, latency characteristics, and reliability models.

---

## Feature Comparison

| Feature | Kreuzcrawl | ScrapeGraphAI |
|---|---|---|
| **Language** | Rust | Python |
| **License** | Elastic-2.0 | MIT |
| **Distribution** | Library + CLI | Library + SaaS API |
| **Headless browser** | chromiumoxide | Playwright |
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | LLM-driven graph |
| **Concurrent fetching** | JoinSet + Semaphore | asyncio |
| **Streaming events** | Real-time | No |
| **Batch operations** | `batch_crawl()` | No |
| **Sitemap parsing** | XML, gzip, index | No |
| **robots.txt** | RFC 9309 | No |
| **Markdown conversion** | Always-on + structure preservation | Yes |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | No |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | No |
| **JSON-LD extraction** | Full | No |
| **Feed discovery** | RSS, Atom, JSON Feed | No |
| **Link-to-citations** | Numbered refs | No |
| **LLM extraction** | Multi-provider (liter-llm) | LangChain (core) |
| **Cost tracking** | USD + tokens | Token counting |
| **PDF extraction** | No | Yes |
| **WAF detection** | 8 vendors | No |
| **Stealth / anti-detect** | UA rotation | Undetected Playwright |
| **Proxy support** | HTTP/HTTPS/SOCKS5 | Via Playwright |
| **User-Agent rotation** | Tower layer | No |
| **Screenshot capture** | Stub | Yes |
| **Page interaction** | No | No |
| **REST API server** | No | SaaS API |
| **MCP server** | No | Yes (via Toolhouse) |
| **CLI** | scrape/crawl/map | No |
| **Language SDKs** | Rust only | Python, Node.js |
| **Disk cache** | blake3 + TTL | No |
| **Per-domain rate limiting** | Tower layer | No |
| **HTTP caching (ETag)** | Yes | No |
| **Pluggable traits** | 7 traits | Graph nodes |
| **Middleware stack** | Tower services | No |
| **Config validation** | serde strict | No |
| **BM25 relevance scoring** | Yes | No |
| **Adaptive crawling** | Term saturation | No |
| **Asset download + dedup** | SHA-256 | No |
| **Search integration** | No | DuckDuckGo |

---

## Where Kreuzcrawl Wins

**No LLM required for basic extraction.** Kreuzcrawl extracts metadata, converts to markdown, discovers links, and parses sitemaps without calling any LLM. This means zero API costs, zero LLM latency, and deterministic output for standard extraction tasks. ScrapeGraphAI requires an LLM for virtually every operation.

**Cost efficiency at scale.** Crawling 10,000 pages with Kreuzcrawl costs nothing beyond compute. The same workload with ScrapeGraphAI would incur significant LLM API costs -- potentially hundreds of dollars depending on the provider and model. Kreuzcrawl's LLM extraction is opt-in and feature-gated, so you pay only when you choose to use it.

**Performance.** Compiled Rust versus interpreted Python, with no LLM round-trip on the critical path. Kreuzcrawl's extraction pipeline runs in microseconds per field; ScrapeGraphAI's LLM calls add seconds per page at minimum.

**Deterministic output.** Kreuzcrawl's HTML parsing produces the same output for the same input every time. LLM-based extraction is inherently non-deterministic -- the same page may produce different results across runs. For pipelines that require reproducibility, this matters.

**Crawling infrastructure.** Kreuzcrawl is a full crawling engine with 4 traversal strategies, per-domain rate limiting, sitemap parsing, robots.txt compliance, HTTP caching, and streaming events. ScrapeGraphAI is primarily an extraction tool with basic fetching; it does not provide crawl orchestration, rate limiting, or compliance features.

**Rich metadata extraction.** 40+ metadata fields including full JSON-LD, Dublin Core, Open Graph, Twitter Card, feed discovery, and hreflang. ScrapeGraphAI does not extract structured metadata -- it relies on the LLM to identify relevant information.

**WAF detection and browser fallback.** Kreuzcrawl detects 8 WAF vendors and can route to headless Chrome. ScrapeGraphAI relies on Undetected Playwright but does not detect or classify WAFs.

---

## Where ScrapeGraphAI Wins

**Natural language extraction.** Describe what you want in plain English and ScrapeGraphAI extracts it. No CSS selectors, no XPath, no schema definitions needed for ad-hoc extraction tasks. This dramatically lowers the barrier to entry for non-technical users and exploratory workflows.

**Schema-free extraction.** ScrapeGraphAI can extract structured data from pages without predefined schemas. The LLM infers the structure from context. Kreuzcrawl's LLM extraction requires you to define a JSON schema for the output you want.

**LangChain integration.** Deep integration with the LangChain ecosystem means ScrapeGraphAI fits naturally into existing LLM application pipelines. If you are already using LangChain for other parts of your application, ScrapeGraphAI slots in seamlessly.

**Graph-based execution.** ScrapeGraphAI's graph execution model allows complex multi-step extraction workflows where the output of one node feeds into another. This is powerful for extraction tasks that require reasoning across multiple page elements.

**PDF extraction.** ScrapeGraphAI can extract content from PDFs. Kreuzcrawl cannot.

**Screenshot capture.** Full screenshot support for visual extraction workflows.

**MIT license.** ScrapeGraphAI's MIT license has no restrictions on use, modification, or hosting.

**Node.js SDK.** Available in both Python and Node.js.

---

## When to Use Which

**Choose Kreuzcrawl when:**

- You need to crawl at scale without LLM costs
- Deterministic, reproducible output is required
- You need a full crawling engine (rate limiting, robots.txt, sitemaps)
- You want structured metadata extraction without LLM dependency
- Performance and resource efficiency are priorities
- You need per-domain rate limiting and HTTP caching
- You want LLM extraction as an optional enhancement, not a requirement

**Choose ScrapeGraphAI when:**

- You need ad-hoc extraction described in natural language
- You do not know the page structure in advance
- You are already using LangChain in your pipeline
- You need multi-step reasoning across page elements
- LLM costs are acceptable for your volume
- You need PDF extraction
- You want the lowest possible barrier to entry for extraction tasks
- You prefer MIT licensing

---

## License Comparison

| | Kreuzcrawl | ScrapeGraphAI |
|---|---|---|
| **License** | Elastic-2.0 | MIT |
| **Commercial use** | Yes | Yes |
| **Modification** | Yes | Yes |
| **Hosting restriction** | Cannot provide as a managed crawling service | None |
| **Embedding in proprietary software** | Yes | Yes |

ScrapeGraphAI's MIT license is maximally permissive with no restrictions. Kreuzcrawl's Elastic-2.0 license allows all commercial use and modification but prohibits offering Kreuzcrawl itself as a managed crawling service. For most use cases, both licenses are compatible with commercial development.

---

## Cost Model Comparison

The cost difference between these tools deserves special attention because their architectures create fundamentally different cost profiles:

| Scenario | Kreuzcrawl | ScrapeGraphAI |
|---|---|---|
| **10,000 pages, metadata only** | Compute only (~$0) | ~$50-200 in LLM API costs |
| **10,000 pages, structured extraction** | Compute + LLM costs (opt-in) | ~$50-200 in LLM API costs |
| **100,000 pages, basic crawl** | Compute only (~$0) | ~$500-2,000 in LLM API costs |
| **Latency per page** | Milliseconds (no LLM) | Seconds (LLM round-trip) |

These are rough estimates and vary significantly by LLM provider, model, and page complexity. The key point is that Kreuzcrawl's cost scales with compute only for standard extraction, while ScrapeGraphAI's cost scales linearly with page count due to per-page LLM calls.

---

