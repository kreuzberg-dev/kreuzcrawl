# ADR-003: Crawl Orchestration — Async BFS with URL Frontier

**Status**: Accepted

**Date**: 2026-03-09

## Context

A crawling engine needs to efficiently traverse websites by following links, respecting rate limits, avoiding duplicate work, and honoring robots.txt. The orchestration layer must handle:

- URL deduplication across potentially millions of seen URLs
- Depth-bounded traversal (BFS preferred over DFS for breadth coverage)
- Concurrent fetching with bounded parallelism
- Per-domain rate limiting to avoid overwhelming servers
- robots.txt compliance
- Sitemap-driven URL discovery as an alternative to link following
- Graceful timeout and budget enforcement

## Decision

### URL Frontier

The URL frontier tracks URLs to visit and URLs already seen:

- **Seen set**: `AHashSet<u64>` storing 64-bit hashes of canonicalized URLs. AHash provides ~2x faster hashing than SipHash for our use case (URL strings, no HashDoS concern in a crawler). At 8 bytes per entry, 1M URLs costs ~8MB.
- **Queue**: `VecDeque<(Url, u32)>` for BFS ordering with depth tracking. Simple and sufficient — we don't need priority queuing for v0.1.
- **Canonicalization**: Strip fragments, normalize trailing slashes, lowercase scheme and host. Prevents duplicate visits to `example.com/page` vs `example.com/page/` vs `example.com/page#section`.

### Bounded Concurrency

- **Tokio semaphore** limits concurrent in-flight requests (default: 10)
- Semaphore is acquired before fetch, released after response received
- Configurable via `CrawlConfig::max_concurrent_requests`

### Per-Domain Rate Limiting

- **Token bucket per domain**: Each domain gets an independent rate limiter
- Default: 500ms between requests to the same domain
- Implemented as a `HashMap<String, Instant>` tracking last request time per domain
- Before each fetch, sleep until `last_request + delay` if needed

### robots.txt

- **texting_robots crate**: Full robots.txt spec compliance including wildcards, Allow/Disallow precedence, crawl-delay directives
- Fetched once per domain on first visit, cached for the duration of the crawl
- Crawl-delay directive respected (overrides per-domain rate limit if longer)
- If robots.txt returns 4xx, assume all paths allowed (standard practice)
- If robots.txt returns 5xx, skip the domain entirely (conservative approach)

### Sitemap Parsing

- **quick-xml crate**: Fast, zero-copy XML parsing for sitemap.xml and sitemap index files
- Supports: `<urlset>` (plain sitemaps), `<sitemapindex>` (sitemap of sitemaps), gzip-compressed sitemaps
- Sitemap URLs fed into the frontier as depth-0 entries
- Used by the `map` command and optionally by `crawl` for URL seeding

### BFS Crawl Loop

```
1. Seed frontier with start URL(s)
2. Fetch robots.txt for seed domain(s)
3. While frontier has URLs AND budget not exceeded AND timeout not reached:
   a. Pop URL from frontier
   b. Check: already seen? robots.txt disallowed? excluded by pattern?
   c. Acquire concurrency semaphore
   d. Wait for per-domain rate limit
   e. Fetch URL via engine waterfall
   f. Extract links from response HTML
   g. For each link: canonicalize, check domain scope, push to frontier at depth+1
   h. Yield ScrapeResult
4. Return CrawlResult with all pages + skipped URLs
```

### Domain Scoping

- **stay_on_domain** (default: true): Only follow links to the same registered domain (using public suffix list via `addr` crate or simple hostname comparison)
- **include_patterns / exclude_patterns**: Glob-style URL filtering applied before enqueueing

## Consequences

### Positive

- **Memory efficient**: Hash-only seen set scales to millions of URLs at ~8MB per million
- **Fair crawling**: Per-domain rate limiting prevents hammering individual servers
- **Spec compliant**: texting_robots handles robots.txt edge cases correctly
- **Predictable**: BFS with depth bounds gives consistent, reproducible crawl behavior
- **Configurable**: Concurrency, rate limits, depth, budget all user-controllable

### Negative

- **In-memory frontier**: Not suitable for crawls exceeding available RAM (millions of URLs). Disk-backed frontier (RocksDB) would be needed for web-scale crawling — out of scope for v0.1
- **No priority ordering**: All URLs at the same depth are equally prioritized. Smarter prioritization (PageRank-like, content-type preference) deferred to later versions
- **Single-machine**: No distributed crawl coordination. Fine for our use case but not web-scale
- **Hash collisions**: 64-bit hashes have theoretical collision probability of ~1 in 10^18 per pair — negligible for our crawl sizes but worth noting

## Notes

Implementation:
- `crates/kreuzcrawl/src/crawl/mod.rs` — Crawler struct with BFS loop
- `crates/kreuzcrawl/src/crawl/frontier.rs` — URL frontier (seen set + queue)
- `crates/kreuzcrawl/src/crawl/robots.rs` — texting_robots integration
- `crates/kreuzcrawl/src/crawl/sitemap.rs` — Sitemap XML parsing
- `crates/kreuzcrawl/src/crawl/rate_limit.rs` — Per-domain rate limiting
