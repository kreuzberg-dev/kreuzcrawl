# ADR-007: Behavioral Fixture Derivation from Reference Crawlers

## Status

Accepted

## Context

Our initial E2E fixture corpus (33 fixtures across 7 categories) covered basic scraping, metadata extraction, link classification, robots.txt, sitemaps, and error handling. To ensure kreuzcrawl achieves behavioral parity with production crawlers, we analyzed four reference implementations:

- **firecrawl** (TypeScript) — API-driven crawling platform with scrape/crawl/map/extract endpoints
- **spider.rs** (Rust) — Comprehensive crawling library with Chrome rendering, stealth, and AI integration
- **scrapy** (Python) — Mature framework with 200+ settings, middleware pipelines, and auto-throttle
- **colly** (Go) — Lightweight callback-based collector with rate limiting and storage backends

## Decision

Derive 44 additional behavioral fixtures from capabilities observed across all four libraries, expanding the corpus to 77 fixtures across 12 categories. Each fixture tests an observable behavior that real-world crawlers must support — not internal implementation details.

### New Categories

- **redirect/** (6) — 301/302 chains, max redirect enforcement, loop detection, cross-domain, meta-refresh
- **content/** (6) — PDF/binary skip, charset detection, gzip decompression, body size limits, empty body
- **cookies/** (3) — Persistence across requests, per-domain isolation, Set-Cookie handling
- **auth/** (3) — Basic HTTP auth, Bearer token, custom header authentication
- **map/** (3) — URL discovery without content fetch, subdomain inclusion, pattern exclusion

### Expanded Categories

- **crawl/** (+8) — Subdomain control, include/exclude path patterns, URL deduplication, breadth-first ordering, concurrency limits, custom headers
- **robots/** (+5) — User-agent specific rules, wildcard paths, allow overrides, sitemap directive, multiple user-agent blocks
- **sitemap/** (+4) — Gzip compressed, lastmod filtering, robots.txt discovery, sitemap-only mode
- **error/** (+6) — Connection refused, DNS failure, SSL errors, partial response, retry on 503, exponential backoff

## Consequences

- Generator schema extended with new config fields (include/exclude paths, auth, cookies, redirects, retry) and 5 new assertion types
- Test corpus covers the behavioral surface area of all four reference crawlers
- TDD red phase: all 77 tests define expected behavior before engine implementation begins
- Future categories (rate_limit, engine, stealth) can be added following the same derivation pattern
