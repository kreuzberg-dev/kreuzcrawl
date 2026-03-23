# ADR-008: Hybrid HTTP-first, CDP Browser Fallback

**Status**: Accepted

**Date**: 2026-03-23

## Context

kreuzcrawl fetches all pages via HTTP (reqwest). This works well for server-rendered content but fails for:

1. **SPAs** — React, Vue, Next.js, Nuxt apps that return empty shell HTML requiring JavaScript execution
2. **WAF-protected sites** — Cloudflare and similar services that block non-browser requests
3. **JS-rendered content** — Pages where meaningful content is injected by client-side JavaScript

A headless browser fallback is needed, but it should not penalize the common case where HTTP-only fetching works fine.

## Decision

### Hybrid Approach: HTTP First, Browser Fallback

The default path remains HTTP via reqwest. A headless Chrome browser (via CDP) is used only when:

1. **WAF blocked** — The HTTP fetch returns a `WafBlocked` error (Cloudflare 403 with WAF indicators)
2. **JS rendering detected** — After parsing the HTTP response, detection heuristics identify an empty SPA shell
3. **Explicitly requested** — `browser_mode: Always` forces browser for every request

### Feature Gate

All browser/CDP code is behind a Cargo feature flag:

```toml
[features]
browser = ["dep:chromiumoxide"]
```

Config types (`BrowserMode`, `BrowserWait`) are always compiled to keep `CrawlConfig` stable across feature states. Only the `browser.rs` module (containing chromiumoxide usage) is gated.

### Detection Heuristics

A pure-function module (`browser_detect.rs`, always compiled) analyzes HTML to detect JS-dependent pages:

- **Empty SPA mount points** — `<div id="root"></div>`, `#app`, `#__next`, `#__nuxt` with no text content
- **Noscript warnings** — `<noscript>` tags containing "enable JavaScript" messaging
- **Low text + scripts** — Fewer than 20 words of text content combined with external `<script>` tags

The detection result is exposed as `ScrapeResult.js_render_hint: bool` — useful API signal regardless of whether the browser feature is enabled.

### Browser Lifecycle

Two modes are supported:

- **Managed** (default) — kreuzcrawl launches a Chrome child process via `Browser::launch()` and closes it after the fetch
- **External** — The caller provides a CDP WebSocket endpoint (`browser_endpoint` config) for connecting to an existing Chrome instance (Docker sidecar, shared pool)

### Auto Re-fetch Flow

In `Auto` mode with the `browser` feature enabled:

1. HTTP fetch via reqwest (existing path)
2. Parse HTML, extract metadata, compute word count
3. Run detection heuristics → `js_render_hint`
4. If `js_render_hint` is true: re-fetch with browser, re-run full extraction pipeline
5. Return browser result (replaces HTTP result entirely — no merging)

### HttpResponse as Abstraction Boundary

`browser_fetch()` returns the same `HttpResponse` struct as `http_fetch()`. The entire downstream pipeline (charset detection, metadata extraction, link discovery, asset download) is reused unchanged.

## Configuration

New fields on `CrawlConfig`:

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `browser_mode` | `BrowserMode` | `Auto` | When to use browser: Auto, Always, Never |
| `browser_endpoint` | `Option<String>` | `None` | External CDP WebSocket URL |
| `browser_timeout` | `Duration` | 30s | Browser page load timeout |
| `browser_wait` | `BrowserWait` | `NetworkIdle` | Wait strategy after navigation |
| `browser_wait_selector` | `Option<String>` | `None` | CSS selector for `Selector` wait mode |
| `browser_extra_wait` | `Option<Duration>` | `None` | Extra wait after condition is met |

## Consequences

### Positive

- **Zero cost when unused** — No browser dependency or overhead without the `browser` feature
- **Transparent fallback** — Callers in `Auto` mode get rendered content without code changes
- **Detection as API signal** — `js_render_hint` is useful even without browser feature enabled
- **Reuses extraction pipeline** — No duplication of metadata/link/asset extraction logic
- **Flexible lifecycle** — Managed browser for dev, external endpoint for production

### Negative

- **Double fetch in Auto mode** — SPA pages are fetched twice (HTTP then browser)
- **Chrome dependency** — The `browser` feature requires Chrome/Chromium installed
- **Detection heuristics may false-positive** — Empty pages that are intentionally minimal could trigger unnecessary browser fetches
- **Browser overhead** — ~100-300MB memory per Chrome process, 1-5s per page render

### Deferred

- **JavaScript-based redirects** — Detection of `window.location` redirects
- **Screenshot capture** — Useful for debugging but not in scope for v0.1

## BrowserPool — Persistent Chrome Lifecycle

Added in the same wave to address the per-request Chrome launch overhead.

### Design

`BrowserPool` keeps a single Chrome process alive across multiple scrape/crawl/batch operations. Pages (tabs) are the unit of concurrency, controlled by a tokio `Semaphore`.

```rust
let pool = BrowserPool::new(BrowserPoolConfig {
    max_pages: 8,
    ..Default::default()
});
pool.warm().await?;  // eagerly launch Chrome

let config = CrawlConfig {
    browser_mode: BrowserMode::Always,
    browser_pool: Some(Arc::clone(&pool)),
    ..Default::default()
};
let result = scrape("https://example.com", &config).await?;
pool.shutdown().await;
```

### Key decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Pool in CrawlConfig | `Option<Arc<BrowserPool>>` field | Avoids API proliferation; config already holds runtime-adjacent state |
| Chrome instances | 1 per pool | Chrome handles tabs well; 1 process is simpler |
| Page reuse | Fresh pages, not pooled | Avoids state leakage (cookies, localStorage) |
| Crash detection | `handler_handle.is_finished()` + timeout on `new_page()` | Low overhead in happy path; relaunch on failure |
| Crash recovery | Relaunch on next `acquire_page()` | Transparent to caller; no background health thread |
| Page cleanup | Explicit `close()` + Drop safety net | Rust `Drop` is sync; spawns async cleanup as best-effort |
| Concurrency control | `tokio::sync::Semaphore` | Same pattern as batch.rs and assets.rs |

### Lifecycle

1. `BrowserPool::new()` — no Chrome launched
2. `pool.warm()` — eagerly launch Chrome (optional, fail-fast)
3. `pool.acquire_page()` — semaphore permit → mutex (brief, check/launch) → `browser.new_page()` → return `PooledPage`
4. Caller uses `pooled_page.page()` for navigation
5. `pooled_page.close()` — page closed, permit released
6. `pool.shutdown()` — sets flag, drops Chrome, waits for handler
