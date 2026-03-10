# ADR-007: Behavioral Fixture Derivation from Reference Crawlers

**Status**: Accepted (updated 2026-03-10)

**Date**: 2026-03-09

## Context

Our E2E fixture corpus needed to cover the behavioral surface area of production crawling libraries. Rather than inventing test scenarios, we analyzed four reference implementations to derive fixtures from real-world capabilities:

- **firecrawl** (TypeScript) — API-driven crawling platform with scrape/crawl/map/extract endpoints
- **spider.rs** (Rust) — Comprehensive crawling library with Chrome rendering, stealth, and AI integration
- **scrapy** (Python) — Mature framework with 200+ settings, middleware pipelines, and auto-throttle
- **colly** (Go) — Lightweight callback-based collector with rate limiting and storage backends

## Decision

Derive 132 behavioral fixtures across 15 categories. Each fixture tests an observable behavior that real-world crawlers must support — not internal implementation details.

### Fixture Breakdown by Category

| Category | Count | Derived From |
|----------|-------|-------------|
| scrape | 15 | firecrawl scrape endpoint, colly basic collectors |
| metadata | 8 | firecrawl metadata extraction, spider.rs page data |
| links | 9 | scrapy link extractors, colly link callbacks |
| crawl | 18 | firecrawl crawl endpoint, scrapy CrawlSpider, spider.rs |
| robots | 14 | scrapy RobotsTxtMiddleware, spider.rs robots module |
| sitemap | 8 | firecrawl sitemap mode, spider.rs sitemap parser |
| error | 17 | scrapy RETRY_EXCEPTIONS/RETRY_HTTP_CODES, firecrawl error handling |
| redirect | 12 | scrapy RedirectMiddleware, colly redirect handler |
| content | 11 | spider.rs content type filtering, colly MaxBodySize |
| cookies | 3 | scrapy CookiesMiddleware, colly cookie jar |
| auth | 3 | scrapy HttpAuthMiddleware, firecrawl auth headers |
| map | 6 | firecrawl map endpoint |
| batch | 3 | firecrawl batch scraping, concurrent URL processing |
| stream | 1 | firecrawl streaming mode, event-driven crawling |
| encoding | 4 | charset detection, multi-encoding support |
| **Total** | **132** | |

### Key Behavioral Patterns Covered

**Redirect handling** (all four libraries): Chain following, max redirect limits, loop detection, cross-domain redirects, HTML meta-refresh, Refresh header.

**URL filtering** (firecrawl, scrapy, spider.rs): Include/exclude regex patterns, subdomain control, URL deduplication, fragment stripping, normalized URL comparison.

**Content handling** (firecrawl, spider.rs): Binary content skipping, charset detection, body size limits, PDF detection, main content extraction.

**Authentication** (all four): Basic auth, bearer tokens, custom auth headers.

**Error recovery** (scrapy, spider.rs): Retry on 503, configurable retry codes, connection refused, DNS failure, SSL errors, WAF detection.

**Robots.txt** (scrapy, spider.rs): User-agent specific rules with RFC 9309 prefix matching, wildcard paths, allow/disallow precedence, sitemap directive, crawl-delay.

**Metadata extraction**: Open Graph (including video/audio), Twitter Card, Dublin Core, JSON-LD, article metadata, hreflang, favicons, headings, word count, response headers (ETag, Server, etc.).

**Asset handling**: CSS/JS/image discovery from HTML, concurrent downloading with semaphore, SHA-256 content hashing, asset type filtering, size limits.

**Streaming and batch**: Event-driven crawl streaming (Page, Error, Complete events), concurrent multi-URL batch scraping with partial failure handling.

### Schema Extensions

The fixture schema supports 30 assertion types across all categories:

**Config fields**: `include_paths`, `exclude_paths`, `allow_subdomains`, `max_redirects`, `max_body_size`, `max_concurrent`, `custom_headers`, `cookies_enabled`, `auth_basic`, `auth_bearer`, `auth_header`, `retry_count`, `retry_codes`, `remove_tags`, `main_content_only`, `download_assets`, `asset_types`, `max_asset_size`, `map_search`, `map_limit`, `batch_urls`

**Assertion types**: status_code, content_type, html_not_empty, metadata, links, images, og, twitter, dublin_core, json_ld, feeds, robots, sitemap, crawl, error, redirect, content, cookies, auth, map, extended_metadata, article, extended_og, hreflang, favicons, headings, computed, response_meta, assets, stream, batch

## Consequences

### Positive

- Test corpus covers the behavioral surface area of four production crawlers
- All 132 tests pass — engine implementation verified against behavioral specs
- Behavioral (not structural) testing means implementation flexibility
- New categories can be added following the same derivation pattern
- 30 assertion types cover metadata, crawl behavior, errors, streaming, and batch operations

### Negative

- Large fixture corpus requires maintenance as engine API evolves
- Some derived behaviors may not be relevant to kreuzcrawl's specific use case
- Reference library analysis is a point-in-time snapshot; new features in those libraries may introduce gaps

## Notes

- Future work: expand corpus with real-world HTML samples and edge cases
- Generator validates fixtures at load time with proper error messages (no silent fallbacks)
