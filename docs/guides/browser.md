# Browser Automation

Kreuzcrawl includes browser-backed rendering for JavaScript-heavy pages. The `browser` feature enables the Chromiumoxide CDP backend; `browser-native` enables the in-process native backend with `BrowserExtras` and network-event capture.

## Browser modes

The `BrowserMode` enum controls when the headless browser is used instead of a plain HTTP fetch.

| Mode             | Behaviour                                                                                                                                                     |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Auto` (default) | Kreuzcrawl first tries an HTTP fetch. If the response looks like it needs JS rendering (e.g. WAF challenge page), it automatically falls back to the browser. |
| `Always`         | Every request goes through the headless browser. Useful for single-page applications or sites that rely entirely on client-side rendering.                    |
| `Never`          | The browser is never launched. Only plain HTTP fetches are performed.                                                                                         |
| `Stealth`        | Every request goes through the browser tier with stealth surfaces enabled.                                                                                    |

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

## Browser backends <span class="version-badge">v0.3</span>

Choose the backend with `BrowserConfig::backend`:

| Backend | Feature | Behavior |
| ------- | ------- | -------- |
| `BrowserBackend::Chromiumoxide` | `browser` | Controls Chrome/Chromium through CDP. Supports external `endpoint` connections and compositor screenshots. |
| `BrowserBackend::Native` | `browser-native` | Uses the in-process native backend. Supports `block_url_patterns`, `eval_script` scrape results, `robots_user_agent`, `capture_network_events`, and `BrowserExtras`. |

```rust
use kreuzcrawl::{BrowserBackend, BrowserConfig, BrowserMode, CrawlConfig};

let config = CrawlConfig {
    browser: BrowserConfig {
        backend: BrowserBackend::Native,
        mode: BrowserMode::Always,
        capture_network_events: true,
        ..Default::default()
    },
    ..Default::default()
};
```

`BrowserExtras` is populated on `ScrapeResult.browser` only when the native backend handled the request. It can contain the `eval_script` return value, captured network events, and cookies from the browser session.

## Browser pooling

The Chromiumoxide backend keeps a Chrome instance alive across requests; tabs are handed out lazily, the pool auto-recovers if Chrome crashes, and concurrent tabs are bounded by `CrawlConfig::max_concurrent`. No additional configuration is required.

## Connecting to an external browser

Point the Chromiumoxide backend at an already-running Chrome via its CDP WebSocket endpoint instead of launching one locally:

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

This is the recommended pattern when running Chrome in a sidecar container or a remote debugging session. `endpoint` is rejected when `BrowserBackend::Native` is selected.

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

Kreuzcrawl detects WAF and bot-mitigation signals with a built-in TOML fingerprint classifier. When a fingerprint matches, the error path includes `CrawlError::WafBlocked { vendor, .. }`; generic or unrecognized blocks may report `unknown` or `generic`. In `Auto` browser mode, those signals can trigger automatic browser escalation. This is not a guarantee that a challenge can be bypassed.

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
