# Antibot Strategy & Stealth Surfaces

Kreuzcrawl detects WAF/bot-mitigation systems, routes requests through bypass providers, and applies stealth patches to avoid detection. Customize this stack via the pluggable `AntibotStrategy` trait or use the bundled `DefaultAntibotStrategy`.

## Architecture

Three layers compose the antibot system:

1. **WAF Classifier** — Fingerprints HTTP responses against 35+ known WAF signatures (Cloudflare, DataDome, PerimeterX, Imperva, Akamai, F5, AWS-WAF).
2. **Decision Hook** — `AntibotStrategy` trait pair: `pre_request` (modify outgoing headers, warm tokens) and `post_response` (inspect response, decide next action).
3. **Stealth Surfaces** — JS patches (chromiumoxide), native TLS spoofing, user-agent rotation (enabled via `BrowserMode::Stealth`).

When a WAF signal is detected, the engine consults the strategy to decide: retry with backoff, escalate to browser, rotate proxy, or accept the response.

## The `AntibotStrategy` trait

Implement this trait to intercept every request/response pair:

```rust
#[async_trait]
pub trait AntibotStrategy: Send + Sync {
    /// Called once per attempt, before the tower-stack fetch fires.
    /// Return Err to abort this attempt; the retry policy decides what happens.
    async fn pre_request(&self, url: &str) -> Result<(), AntibotError>;

    /// Called once per successful HTTP response, after WAF classification.
    /// Return a Decision that overrides the retry policy for this attempt.
    async fn post_response(
        &self,
        response: &HttpResponse,
        waf_signal: Option<&WafSignal>,
    ) -> Decision;
}
```

The `Decision` enum controls what happens next:

```rust
pub enum Decision {
    /// Continue normally — hand the response to the retry policy.
    Accept,
    /// Retry the same tier after `backoff` (Duration).
    Retry { backoff: Duration },
    /// Rotate to a different proxy and retry (not yet implemented).
    /// Logs a warn!() and falls through to Accept for now.
    RotateProxy,
    /// Force escalation to the Browser tier, bypassing the retry policy.
    EscalateBrowser,
}
```

Errors during strategy execution are returned as:

```rust
pub enum AntibotError {
    /// The pre_request hook failed.
    PreRequest(String),
    /// The post_response hook failed.
    PostResponse(String),
}
```

## `BrowserMode` variants

The `BrowserConfig.mode` field controls when and how the browser is used. `BrowserMode::Stealth` is the new variant:

| Variant | Escalation | Stealth Surfaces | Use Case |
|---------|-----------|-----------------|----------|
| `Auto` (default) | HTTP first, escalate to browser on WAF/403 | None | Balanced; handles most sites |
| `Always` | Skip HTTP entirely | None | JS-heavy SPAs |
| `Never` | No browser fallback | None | Performance-critical; no WAF expected |
| `Stealth` | Like Always, escalate on demand | JS patches, TLS spoof, UA rotation | Stealth-hardened mode for challenging sites |

`BrowserMode::Stealth` behaves like `Always` for request routing (every page goes through the browser tier) but additionally enables:

- chromiumoxide JS patches (`crate::stealth::apply_stealth_patches`) that spoof `navigator.webdriver`, `navigator.chromeFlags`, and `navigator.plugins`.
- Native-backend TLS fingerprint spoofing (JA3 randomization).
- Stealth-aware default user-agent when no explicit user-agent is set.
- Forced viewport (1920×1080) to avoid detection via unusual screen sizes.

Previously, a `BrowserConfig.stealth: bool` field existed but had a bug: JS patches always ran regardless of the flag. This field has been removed pre-v1. Use `BrowserMode::Stealth` instead.

## Worked example: per-vendor backoff

Wrap `DefaultAntibotStrategy` to inject custom backoff rules per WAF vendor:

```rust
use std::time::Duration;
use async_trait::async_trait;
use kreuzcrawl::{
    AntibotStrategy, Decision, DefaultAntibotStrategy,
    AntibotError, http::HttpResponse, WafSignal,
};

#[derive(Debug)]
struct VendorBackoffStrategy {
    inner: DefaultAntibotStrategy,
}

#[async_trait]
impl AntibotStrategy for VendorBackoffStrategy {
    async fn pre_request(&self, url: &str) -> Result<(), AntibotError> {
        // Delegate to default (no-op)
        self.inner.pre_request(url).await
    }

    async fn post_response(
        &self,
        resp: &HttpResponse,
        signal: Option<&WafSignal>,
    ) -> Decision {
        match signal.map(|s| s.vendor.as_str()) {
            Some("cloudflare") => {
                // Cloudflare: aggressive backoff
                Decision::Retry {
                    backoff: Duration::from_secs(10),
                }
            }
            Some("datadome") => {
                // DataDome: skip to browser
                Decision::EscalateBrowser
            }
            _ => {
                // Everything else: use default logic
                self.inner.post_response(resp, signal).await
            }
        }
    }
}

// Wire it up
let strategy = std::sync::Arc::new(VendorBackoffStrategy {
    inner: DefaultAntibotStrategy::new(),
});

let profile = DispatchProfile::builder()
    .antibot_strategy(strategy)
    .build();
```

## Defaults

Without an attached strategy, the engine uses `DefaultAntibotStrategy` (defined at `crates/kreuzcrawl/src/types/antibot.rs:132-164`):

- `pre_request` is a no-op.
- `post_response` returns `Decision::EscalateBrowser` when a WAF signal is present, `Decision::Accept` otherwise.

This matches the pre-Cluster-5 escalation logic, so existing code continues to work unchanged.

## WAF detection corpus

Kreuzcrawl classifies 35+ WAF fingerprints via `TomlClassifier` at `crates/kreuzcrawl/rules/waf_fingerprints.toml`. Covered vendors:

- Cloudflare (Worker detection, Bot Management)
- DataDome
- PerimeterX (now Human Security)
- Imperva / Sucuri
- Akamai
- F5
- AWS-WAF

Fingerprints are regex patterns that match response headers, body substrings, or status code + header combos. The classifier uses hot-reload via `TomlClassifier::watch()` on the rule file, so updates don't require a restart.

## Bypass providers

For sites that require third-party bypass (e.g., to solve residential proxy puzzles), kreuzcrawl integrates the `kreuzcrawl-bypass` crate. It provides:

- `SimpleHttpProvider` — YAML-configured, pluggable HTTP-based bypass (call an API, inject headers/cookies into the crawl).
- Shipped vendors: Bright Data, Zyte, ScrapingBee — wire them via YAML config.
- Custom providers: implement the `BypassProvider` trait to add your own.

See `crates/kreuzcrawl-bypass/README.md` for configuration and examples.
