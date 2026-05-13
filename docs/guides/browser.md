# Browser Automation

Kreuzcrawl includes a headless Chrome/Chromium integration for rendering JavaScript-heavy pages.
The browser subsystem is feature-gated behind `browser` and uses the Chrome DevTools Protocol (CDP)
via the `chromiumoxide` crate.

## Browser modes

The `BrowserMode` enum controls when the headless browser is used instead of a plain HTTP fetch.

| Mode             | Behaviour                                                                                                                                                     |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Auto` (default) | Kreuzcrawl first tries an HTTP fetch. If the response looks like it needs JS rendering (e.g. WAF challenge page), it automatically falls back to the browser. |
| `Always`         | Every request goes through the headless browser. Useful for single-page applications or sites that rely entirely on client-side rendering.                    |
| `Never`          | The browser is never launched. Only plain HTTP fetches are performed.                                                                                         |

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

## Browser pooling

A single Chrome instance is kept alive across requests; tabs are handed out lazily, the pool auto-recovers if Chrome crashes, and concurrent tabs are bounded by `CrawlConfig::max_concurrent`. No additional configuration is required.

## Connecting to an external browser

Point the engine at an already-running Chrome via its CDP WebSocket endpoint instead of launching one locally:

```rust
use kreuzcrawl::{BrowserConfig, CrawlConfig};

let config = CrawlConfig {
    browser: BrowserConfig {
        endpoint: Some("ws://127.0.0.1:9222/devtools/browser/...".into()),
        ..Default::default()
    },
    ..Default::default()
};
```

This is the recommended pattern when running Chrome in a sidecar container or a remote debugging session.

## Browser profiles

Persistent browser profiles retain cookies, localStorage, and other browser state across crawl sessions. Configure them through `CrawlConfig::browser_profile` (named profile to attach) and `CrawlConfig::save_browser_profile` (persist changes on exit):

```rust
use kreuzcrawl::CrawlConfig;

let config = CrawlConfig {
    browser_profile: Some("my-session".into()),
    save_browser_profile: true,
    ..Default::default()
};
```

Profile names are validated against path-traversal — only ASCII alphanumerics, hyphens, underscores, and dots are allowed (max 255 characters). Profiles are stored under `<data_dir>/kreuzcrawl/profiles/<name>` and, on Unix, are created with mode `0o700`.

## WAF detection

Kreuzcrawl detects when a response is blocked by a Web Application Firewall and returns a
`CrawlError::WafBlocked` error with the identified vendor. Detection runs on both HTTP
responses and browser-rendered pages.

Detected vendors:

| Vendor              | Detection signal                                                        |
| ------------------- | ----------------------------------------------------------------------- |
| Cloudflare          | `Server: cloudflare`, `cf-browser-verification`, `cf-chl-` body markers |
| Akamai              | `Server: AkamaiGHost`                                                   |
| Imperva (Incapsula) | `incapsula`, `_incap_ses_` body markers                                 |
| DataDome            | `datadome` body marker, `x-datadome` header                             |
| PerimeterX          | `perimeterx`, `px-captcha` body markers, `x-px-*` headers               |
| Sucuri              | `sucuri` body marker, `x-sucuri-id` header                              |
| F5 BIG-IP           | `Server: big-ip`                                                        |
| AWS WAF             | `awselb`, `x-amzn-waf` body markers, `x-amzn-waf-action` header         |

In `Auto` browser mode, a WAF challenge triggers an automatic browser fallback so that
JavaScript challenges can be solved client-side.

## Wait strategies

After the browser navigates to a URL, it needs to wait for the page to finish rendering.
The `BrowserWait` enum controls this behaviour.

| Strategy                | Behaviour                                                                                                         | Default wait |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------- | ------------ |
| `NetworkIdle` (default) | Waits for a 500 ms settle period after initial page load, giving client-side JS time to execute.                  | 500 ms       |
| `Selector`              | Waits until a specific CSS selector appears in the DOM. Falls back to 500 ms if no `wait_selector` is configured. | Varies       |
| `Fixed`                 | Waits a fixed 2-second duration after navigation completes.                                                       | 2 s          |

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

The `extra_wait` field adds additional sleep time _after_ the wait condition is met.
The `timeout` field is the hard cap on the entire navigation-plus-wait cycle; if exceeded,
`CrawlError::BrowserTimeout` is returned.

!!! tip "Choosing a strategy"
Use `NetworkIdle` for most sites. Switch to `Selector` when you know the exact element
that signals the page is ready (e.g. a data table or main content div). Use `Fixed`
only as a last resort for unpredictable sites.
