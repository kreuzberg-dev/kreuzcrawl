# Browser Automation

Kreuzcrawl includes a headless Chrome/Chromium integration for rendering JavaScript-heavy pages.
The browser subsystem is feature-gated behind `browser` and uses the Chrome DevTools Protocol (CDP)
via the `chromiumoxide` crate.

## Browser modes

The `BrowserMode` enum controls when the headless browser is used instead of a plain HTTP fetch.

| Mode | Behaviour |
|------|-----------|
| `Auto` (default) | Kreuzcrawl first tries an HTTP fetch. If the response looks like it needs JS rendering (e.g. WAF challenge page), it automatically falls back to the browser. |
| `Always` | Every request goes through the headless browser. Useful for single-page applications or sites that rely entirely on client-side rendering. |
| `Never` | The browser is never launched. Only plain HTTP fetches are performed. |

Set the mode in `CrawlConfig`:

```rust
use kreuzcrawl::{CrawlConfig, BrowserMode};

let config = CrawlConfig {
    browser: kreuzcrawl::BrowserConfig {
        mode: BrowserMode::Always,
        ..Default::default()
    },
    ..Default::default()
};
```

## BrowserPool management

When crawling many pages, launching a fresh Chrome process per request is expensive.
`BrowserPool` keeps a single Chrome instance alive and hands out individual pages (tabs),
limiting concurrency with a semaphore.

```rust
use std::sync::Arc;
use kreuzcrawl::BrowserPoolConfig;
use kreuzcrawl::BrowserPool;

let pool = BrowserPool::new(BrowserPoolConfig {
    max_pages: 8,              // up to 8 concurrent tabs
    browser_endpoint: None,    // launch a local Chrome
    chrome_args: Vec::new(),
    launch_timeout: std::time::Duration::from_secs(30),
});

// Optionally warm the pool so the first request doesn't pay startup cost.
pool.warm().await?;

// Acquire a page, use it, then close.
let page = pool.acquire_page().await?;
// ... use page.page() for CDP operations ...
page.close().await;
```

Key behaviours:

- **Lazy start** -- Chrome is not launched until the first `acquire_page()` or `warm()` call.
- **Auto-recovery** -- If Chrome crashes, the next `acquire_page()` call relaunches it automatically.
- **Graceful shutdown** -- `pool.shutdown().await` closes the browser and rejects further requests.
- **Health probe** -- `pool.is_healthy()` is a lock-free atomic read, safe for liveness checks.

Each `PooledPage` holds a semaphore permit. When the page is closed (or dropped), the permit
is released so another caller can open a tab.

## Connecting to an external browser

Instead of launching a local Chrome process, you can connect to an already-running browser
via its CDP WebSocket endpoint:

```rust
use kreuzcrawl::{CrawlConfig, BrowserConfig};

let config = CrawlConfig {
    browser: BrowserConfig {
        endpoint: Some("ws://127.0.0.1:9222/devtools/browser/...".into()),
        ..Default::default()
    },
    ..Default::default()
};
```

Or with `BrowserPoolConfig`:

```rust
use kreuzcrawl::BrowserPoolConfig;

let pool_config = BrowserPoolConfig {
    browser_endpoint: Some("ws://127.0.0.1:9222/devtools/browser/...".into()),
    ..Default::default()
};
```

This is useful when running Chrome in a sidecar container or a remote debugging session.

## Browser profiles (persistent sessions)

`BrowserProfile` lets you persist cookies, localStorage, and other browser state across
crawl sessions by pointing Chrome at a stable `--user-data-dir`.

```rust
use kreuzcrawl::BrowserProfile;

// Create a profile handle (does not touch disk yet).
let profile = BrowserProfile::new("my-session")?;

// Create the directory if it doesn't exist.
if !profile.exists() {
    profile.create()?;
}

// Pass the profile's Chrome args to BrowserPoolConfig.
let pool_config = BrowserPoolConfig {
    chrome_args: profile.chrome_args(),
    ..Default::default()
};
```

Profile names are validated to prevent path-traversal attacks. Only ASCII alphanumerics,
hyphens, underscores, and dots are allowed (max 255 characters). Profiles are stored under
`<data_dir>/kreuzcrawl/profiles/<name>`.

Manage profiles:

```rust
// List all profiles on disk.
let names = BrowserProfile::list_all()?;

// Delete a profile (refuses to follow symlinks).
profile.delete()?;
```

!!! warning "Unix permissions"
    On Unix, `profile.create()` sets the directory to mode `0o700` (owner-only access).
    Never store profiles in world-readable locations.

## WAF detection

Kreuzcrawl detects when a response is blocked by a Web Application Firewall and returns a
`CrawlError::WafBlocked` error with the identified vendor. Detection runs on both HTTP
responses and browser-rendered pages.

Detected vendors:

| Vendor | Detection signal |
|--------|-----------------|
| Cloudflare | `Server: cloudflare`, `cf-browser-verification`, `cf-chl-` body markers |
| Akamai | `Server: AkamaiGHost` |
| Imperva (Incapsula) | `incapsula`, `_incap_ses_` body markers |
| DataDome | `datadome` body marker, `x-datadome` header |
| PerimeterX | `perimeterx`, `px-captcha` body markers, `x-px-*` headers |
| Sucuri | `sucuri` body marker, `x-sucuri-id` header |
| F5 BIG-IP | `Server: big-ip` |
| AWS WAF | `awselb`, `x-amzn-waf` body markers, `x-amzn-waf-action` header |

In `Auto` browser mode, a WAF challenge triggers an automatic browser fallback so that
JavaScript challenges can be solved client-side.

## Wait strategies

After the browser navigates to a URL, it needs to wait for the page to finish rendering.
The `BrowserWait` enum controls this behaviour.

| Strategy | Behaviour | Default wait |
|----------|-----------|-------------|
| `NetworkIdle` (default) | Waits for a 500 ms settle period after initial page load, giving client-side JS time to execute. | 500 ms |
| `Selector` | Waits until a specific CSS selector appears in the DOM. Falls back to 500 ms if no `wait_selector` is configured. | Varies |
| `Fixed` | Waits a fixed 2-second duration after navigation completes. | 2 s |

Configure in `BrowserConfig`:

```rust
use kreuzcrawl::{BrowserConfig, BrowserWait};

let browser = BrowserConfig {
    wait: BrowserWait::Selector,
    wait_selector: Some("#main-content".into()),
    extra_wait: Some(std::time::Duration::from_millis(200)),
    timeout: std::time::Duration::from_secs(30),
    ..Default::default()
};
```

The `extra_wait` field adds additional sleep time *after* the wait condition is met.
The `timeout` field is the hard cap on the entire navigation-plus-wait cycle; if exceeded,
`CrawlError::BrowserTimeout` is returned.

!!! tip "Choosing a strategy"
    Use `NetworkIdle` for most sites. Switch to `Selector` when you know the exact element
    that signals the page is ready (e.g. a data table or main content div). Use `Fixed`
    only as a last resort for unpredictable sites.
