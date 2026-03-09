# ADR-007: Behavioral Fixture Derivation from Reference Crawlers

**Status**: Accepted (updated 2026-03-09)

**Date**: 2026-03-09

## Context

Our initial E2E fixture corpus needed to cover the behavioral surface area of production crawling libraries. Rather than inventing test scenarios, we analyzed four reference implementations to derive fixtures from real-world capabilities:

- **firecrawl** (TypeScript) — API-driven crawling platform with scrape/crawl/map/extract endpoints
- **spider.rs** (Rust) — Comprehensive crawling library with Chrome rendering, stealth, and AI integration
- **scrapy** (Python) — Mature framework with 200+ settings, middleware pipelines, and auto-throttle
- **colly** (Go) — Lightweight callback-based collector with rate limiting and storage backends

## Decision

Derive 77 behavioral fixtures across 12 categories. Each fixture tests an observable behavior that real-world crawlers must support — not internal implementation details.

### Fixture Breakdown by Category

| Category | Count | Derived From |
|----------|-------|-------------|
| scrape | 10 | firecrawl scrape endpoint, colly basic collectors |
| metadata | 5 | firecrawl metadata extraction, spider.rs page data |
| links | 4 | scrapy link extractors, colly link callbacks |
| crawl | 11 | firecrawl crawl endpoint, scrapy CrawlSpider, spider.rs |
| robots | 9 | scrapy RobotsTxtMiddleware, spider.rs robots module |
| sitemap | 7 | firecrawl sitemap mode, spider.rs sitemap parser |
| error | 10 | scrapy RETRY_EXCEPTIONS/RETRY_HTTP_CODES, firecrawl error handling |
| redirect | 6 | scrapy RedirectMiddleware, colly redirect handler |
| content | 6 | spider.rs content type filtering, colly MaxBodySize |
| cookies | 3 | scrapy CookiesMiddleware, colly cookie jar |
| auth | 3 | scrapy HttpAuthMiddleware, firecrawl auth headers |
| map | 3 | firecrawl map endpoint |
| **Total** | **77** | |

### Key Behavioral Patterns Covered

**Redirect handling** (all four libraries): Chain following, max redirect limits, loop detection, cross-domain redirects, HTML meta-refresh.

**URL filtering** (firecrawl, scrapy, spider.rs): Include/exclude regex patterns, subdomain control, URL deduplication, fragment stripping.

**Content handling** (firecrawl, spider.rs): Binary content skipping, charset detection, body size limits, gzip negotiation.

**Authentication** (all four): Basic auth, bearer tokens, custom auth headers.

**Error recovery** (scrapy, spider.rs): Retry on 503, exponential backoff, connection refused, DNS failure, SSL errors.

**Robots.txt** (scrapy, spider.rs): User-agent specific rules, wildcard paths, allow/disallow precedence, sitemap directive, crawl-delay.

### Schema Extensions

To support the new fixture categories, the generator schema was extended with:

**Config fields**: `include_paths`, `exclude_paths`, `allow_subdomains`, `max_redirects`, `max_body_size`, `max_concurrent`, `custom_headers`, `cookies_enabled`, `auth_basic`, `auth_bearer`, `auth_header`, `retry_count`, `retry_codes`

**Assertion types**: `redirect` (final URL, count, error), `content` (skipped, charset, body size), `cookies` (count, name match), `auth` (header sent, status), `map` (URL count, URL match)

## Consequences

### Positive

- Test corpus covers the behavioral surface area of four production crawlers
- TDD red phase: all 77 tests define expected behavior before engine implementation
- Behavioral (not structural) testing means implementation flexibility
- New categories can be added following the same derivation pattern

### Negative

- Large fixture corpus requires maintenance as engine API evolves
- Some derived behaviors may not be relevant to kreuzcrawl's specific use case
- Reference library analysis is a point-in-time snapshot; new features in those libraries may introduce gaps

## Notes

- Future work: clone firecrawl, spider.rs, scrapy, and colly repos to derive additional fixtures from their test suites and fixture data
- Phase 3 planned: expand corpus with real-world HTML samples and edge cases
