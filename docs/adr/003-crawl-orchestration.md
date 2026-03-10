# ADR-003: Crawl Orchestration — Async BFS with URL Deduplication

**Status**: Accepted (updated 2026-03-10)

**Date**: 2026-03-09

## Context

A crawling engine needs to efficiently traverse websites by following links, avoiding duplicate work, and honoring robots.txt. The orchestration layer must handle:

- URL deduplication across seen URLs
- Depth-bounded traversal (BFS preferred over DFS for breadth coverage)
- robots.txt compliance
- Sitemap-driven URL discovery as an alternative to link following
- Domain scoping and URL filtering
- Redirect chain handling (HTTP 3xx, Refresh header, meta refresh)

## Decision

### URL Deduplication

The crawler tracks visited URLs using a `HashSet<String>` of dedup-normalized URLs:

- **Seen set**: `HashSet<String>` storing dedup-normalized URLs. URLs are normalized by stripping fragments, lowercasing scheme/host, removing default ports, sorting query parameters, and stripping trailing slashes.
- **Queue**: `VecDeque<(String, usize)>` for BFS ordering with depth tracking.
- **Two normalization functions**: `normalize_url()` for display (preserves path casing) and `normalize_url_for_dedup()` for deduplication (additional normalization for comparison).

### robots.txt

- **Custom parser** in `robots.rs`: Parses robots.txt with user-agent group matching, Allow/Disallow rules with wildcard support (`*` and `$`), and crawl-delay extraction.
- **RFC 9309 prefix matching**: User-agent matching uses prefix comparison per the RFC — `"googlebot"` matches `"googlebot-mobile"` but not `"mygooglebot"`.
- Fetched once per domain on first visit.
- If robots.txt returns 4xx or fetch fails, assume all paths allowed (graceful degradation).

### Sitemap Parsing

- **quick-xml crate**: Streaming XML parsing for sitemap.xml and sitemap index files.
- Supports: `<urlset>` (plain sitemaps), `<sitemapindex>` (sitemap of sitemaps), gzip-compressed sitemaps (via flate2).
- Sitemap URLs fed into map results.
- Used by the `map()` function; crawl uses link following.

### BFS Crawl Loop

```
1. Resolve redirects for seed URL (HTTP 3xx, Refresh header, meta refresh)
2. Pre-compile include/exclude regex patterns
3. Fetch robots.txt if configured
4. Seed BFS queue with final URL at depth 0
5. While queue has URLs AND max_pages not reached:
   a. Pop URL from queue
   b. Check: excluded by regex? robots.txt disallowed?
   c. Fetch URL via http_fetch()
   d. Apply max_body_size truncation
   e. Extract page data (metadata, links, images, feeds, JSON-LD)
   f. For each internal/document link at depth < max_depth:
      - Strip fragment, check domain scope, dedup-normalize
      - If not visited, add to queue at depth+1
   g. Emit CrawlEvent::Page if streaming
   h. Push CrawlPageResult
6. Deduplicate cookies by (name, domain, path)
7. Return CrawlResult
```

### Redirect Handling

The initial URL goes through a redirect resolution loop that handles three redirect types:
- **HTTP 3xx redirects**: Location header following
- **Refresh header**: Case-insensitive `url=` parsing
- **Meta refresh**: `<meta http-equiv="refresh" content="...;url=...">` detection

All three types share redirect count tracking and loop detection via a seen-URLs set.

### Domain Scoping

- **stay_on_domain**: Only follow links to the same hostname.
- **allow_subdomains**: When enabled with `stay_on_domain`, also follows links to `*.base_host`.
- **include_paths / exclude_paths**: Regex patterns applied to URL paths. Include patterns skip depth-0 (seed URL always processed). Patterns are pre-compiled once before the BFS loop.

### Streaming

`crawl_stream()` wraps the BFS crawl in a tokio task and returns a `ReceiverStream<CrawlEvent>`. Events:
- `CrawlEvent::Page(Box<CrawlPageResult>)` — emitted as each page is processed
- `CrawlEvent::Error { url, error }` — emitted on crawl failure
- `CrawlEvent::Complete { pages_crawled }` — emitted when crawl finishes

Channel size is derived from `max_concurrent * 16`.

## Consequences

### Positive

- **Simple and correct**: Standard library HashSet + VecDeque, no external dependencies for frontier
- **Predictable**: BFS with depth bounds gives consistent, reproducible crawl behavior
- **Configurable**: Depth, page limits, domain scoping, regex filtering all user-controllable
- **Streaming support**: Consumers can process pages incrementally without waiting for full crawl
- **Robust redirects**: Three redirect types handled consistently with shared loop detection

### Negative

- **In-memory frontier**: Not suitable for crawls of millions of URLs. Disk-backed frontier would be needed for web-scale crawling — out of scope for v0.1
- **No rate limiting**: No per-domain request throttling. Relies on `max_concurrent` for overall concurrency control
- **No priority ordering**: All URLs at the same depth are equally prioritized
- **Single-machine**: No distributed crawl coordination

## Notes

Implementation:
- `crates/kreuzcrawl/src/crawl.rs` — BFS crawl loop, redirect handling, regex compilation
- `crates/kreuzcrawl/src/robots.rs` — robots.txt parser with RFC 9309 prefix matching
- `crates/kreuzcrawl/src/sitemap.rs` — Sitemap XML/index/gzip parsing
- `crates/kreuzcrawl/src/normalize.rs` — URL normalization and dedup normalization
- `crates/kreuzcrawl/src/stream.rs` — Streaming crawl wrapper
- `crates/kreuzcrawl/src/types.rs` — CrawlConfig, CrawlResult, CrawlPageResult, CrawlEvent
