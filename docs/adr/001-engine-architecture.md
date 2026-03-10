# ADR-001: Engine Architecture — HTTP Fetching Strategy

**Status**: Accepted (updated 2026-03-10)

**Date**: 2026-03-09

## Context

kreuzcrawl needs a reliable HTTP fetching layer that handles the realities of web crawling: connection failures, server overload (503), rate limiting, authentication, cookie management, and content negotiation. The fetching strategy must be embeddable in a Rust library without external process dependencies.

We need a fetching strategy that:
1. Handles transient failures with configurable retry
2. Supports authentication (Basic, Bearer, custom headers)
3. Manages cookies across crawl sessions
4. Negotiates content encoding (gzip, brotli)
5. Respects configurable timeouts and body size limits

## Decision

### reqwest with Retry Logic

Use **reqwest** as the sole HTTP client, with a custom retry layer built on top:

- **reqwest** with rustls for TLS, cookie jar support, gzip/brotli decompression
- **Retry logic** in `fetch_with_retry()`: configurable retry count and retryable status codes (default: 429, 500, 502, 503, 504)
- **Custom headers** applied per-request from `CrawlConfig`
- **Authentication** via `BasicAuth`, `AuthHeader`, or bearer token structs

### Why reqwest

| Alternative | Rejection Reason |
|-------------|-----------------|
| Headless Chrome (chromiumoxide) | 50-200MB per tab, not embeddable as library |
| reqwest-impersonate | BoringSSL build complexity, maintenance risk — can be added later as feature flag |
| curl-cffi / libcurl | C dependency, harder async Rust integration |
| hyper directly | Too low-level, would reimplement what reqwest provides |

### HTTP Client Configuration

```rust
fn build_client(config: &CrawlConfig) -> Result<reqwest::Client, CrawlError>
```

The client is built once per crawl/scrape operation with:
- Configurable user-agent (default: `kreuzcrawl/{version}`)
- Optional cookie store (enabled via `cookies_enabled`)
- Configurable request timeout
- gzip + brotli decompression
- Redirect policy set to none (redirects handled manually for chain tracking)

### Fetch Functions

- `http_fetch()` — Single fetch with headers, auth, timeout
- `fetch_with_retry()` — Wraps `http_fetch` with retry on configurable status codes
- `extract_cookies()` — Parses Set-Cookie headers into structured `CookieInfo`
- `extract_response_meta()` — Captures ETag, Last-Modified, Server, etc.

### Future: TLS Fingerprint Spoofing

reqwest-impersonate (TLS fingerprint spoofing for anti-bot bypass) is a planned addition behind a feature flag. The current architecture supports this cleanly — `build_client` would conditionally use reqwest-impersonate when the feature is enabled.

## Consequences

### Positive

- **Simple dependency tree**: reqwest is well-maintained, widely used, pure Rust TLS via rustls
- **Built-in features**: Cookie jar, gzip/brotli, connection pooling, HTTP/2
- **Configurable retry**: Handles transient failures without external middleware
- **No build complexity**: No cmake, C++ compiler, or BoringSSL required
- **Embeddable**: Pure library, no external processes

### Negative

- **No TLS fingerprint spoofing**: Standard reqwest TLS fingerprint is detectable by anti-bot systems — deferred to feature-gated reqwest-impersonate
- **No JavaScript execution**: Sites requiring JS rendering won't work — Chrome/CDP integration deferred
- **Sequential retry**: Retry is per-request, not per-domain with backoff queuing

## Notes

Implementation:
- `crates/kreuzcrawl/src/http.rs` — Client builder, fetch functions, retry logic, cookie/header extraction
- `crates/kreuzcrawl/src/types.rs` — `CrawlConfig`, `BasicAuth`, `AuthHeader`, `CookieInfo`, `ResponseMeta`
