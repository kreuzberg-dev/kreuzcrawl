# ADR-001: Engine Architecture — HTTP Fetching with TLS Fingerprint Spoofing

**Status**: Accepted

**Date**: 2026-03-09

## Context

Modern websites employ increasingly sophisticated anti-bot protection systems: Cloudflare Bot Management, AWS WAF, Akamai Bot Manager, DataDome, and PerimeterX. These systems detect automated traffic primarily through TLS fingerprinting (JA3/JA4 hashes), HTTP/2 connection characteristics, and header ordering patterns.

Standard HTTP clients like reqwest produce distinctive TLS fingerprints that don't match any real browser, making them trivially detectable. Headless browsers (Chrome via CDP, Playwright) solve this but are heavyweight — each page costs 50-200MB of memory and 2-5 seconds of startup time. For a crawling engine that needs to process thousands of pages efficiently, this tradeoff is unacceptable for the common case.

We need a fetching strategy that:
1. Bypasses TLS fingerprinting on the majority of protected sites
2. Maintains high throughput (hundreds of requests per second)
3. Uses minimal resources per request
4. Falls back gracefully when the fast path fails

## Decision

### Waterfall Fetch Strategy

Implement a two-tier engine waterfall:

1. **Primary: reqwest-impersonate** — HTTP client that spoofs browser TLS fingerprints (Chrome, Firefox, Safari, Edge). Uses BoringSSL with patched ClientHello to produce authentic JA3 hashes. Handles HTTP/2 with browser-like SETTINGS frames and header ordering.

2. **Fallback: reqwest standard** — Plain reqwest with rustls. Used when impersonation is unnecessary (APIs, internal services, sites without bot detection) or explicitly requested.

### Why reqwest-impersonate over alternatives

| Alternative | Rejection Reason |
|-------------|-----------------|
| Headless Chrome (chromiumoxide) | 50-200MB per tab, 2-5s startup. Will be added as feature-gated option in later phase for JS-heavy sites |
| curl-cffi / libcurl | C dependency, harder to integrate with async Rust, license concerns |
| Raw BoringSSL + hyper | Massive engineering effort to replicate browser fingerprints correctly |
| Playwright/Puppeteer | External process, not embeddable in Rust library |

### Why not headless Chrome for v0.1

Chrome/CDP integration (via chromiumoxide crate) is planned for a later phase behind a `chrome` feature flag. v0.1 focuses on the fast path that handles ~80% of websites. The engine trait is designed so Chrome can be added as a third tier without architectural changes.

### Engine Trait Design

```rust
#[async_trait]
pub trait FetchEngine: Send + Sync {
    async fn fetch(&self, request: &FetchRequest) -> Result<FetchResponse>;
    fn name(&self) -> &str;
}
```

The waterfall executor tries engines in order, falling through on specific failure types (TLS errors, 403 with bot-detection signatures, connection resets).

## Consequences

### Positive

- **High throughput**: reqwest-impersonate adds ~0ms overhead vs standard reqwest for TLS negotiation
- **Low resource usage**: No browser process, no DOM, no JavaScript engine — pure HTTP
- **Broad bypass coverage**: JA3 spoofing defeats the majority of passive fingerprinting systems
- **Clean abstraction**: Engine trait allows adding Chrome, custom clients, or proxy-based engines later
- **No external dependencies**: Pure Rust (BoringSSL is vendored by reqwest-impersonate)

### Negative

- **reqwest-impersonate maintenance risk**: Depends on upstream keeping browser fingerprints current as browsers update their TLS stacks
- **BoringSSL build complexity**: reqwest-impersonate bundles BoringSSL which requires cmake and a C++ compiler at build time
- **No JavaScript execution**: Sites requiring JS rendering (SPAs, dynamically loaded content) won't work until Chrome engine is added
- **Active fingerprinting not addressed**: Browser behavior fingerprinting (mouse movements, canvas, WebGL) is out of scope for HTTP-level spoofing

## Notes

Implementation:
- `crates/kreuzcrawl/src/engine/mod.rs` — FetchEngine trait + waterfall executor
- `crates/kreuzcrawl/src/engine/http.rs` — reqwest-impersonate and reqwest implementations
