# Antibot Strategy & Stealth Surfaces

Kreuzcrawl detects WAF/bot-mitigation signals, classifies them with hot-reloadable rules, and can escalate through the configured dispatch chain. Customize policy with `AntibotStrategy`, `DispatchProfile`, retry policy, domain state, and optional caller-supplied bypass providers.

## Architecture

Three layers compose the antibot system:

1. **WAF Classifier** — Matches HTTP responses against `crates/kreuzcrawl/rules/waf_fingerprints.toml` and returns a `WafSignal` with `vendor`, `fingerprint_id`, and `weight`.
2. **Decision Hook** — `AntibotStrategy` trait pair: `pre_request` (warm external state) and `post_response` (inspect response, decide next action).
3. **Dispatch Policy** — `DispatchProfile` combines escalation strategy, retry policy, classifier, domain state, budget, and optional bypass provider.

When a WAF signal is detected, the engine consults the strategy and retry policy to decide whether to retry, escalate to the browser tier, route through a caller-supplied bypass provider, or stop. `Decision::RotateProxy` is present but not implemented; it logs and falls through to `Accept`.

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

| Variant | Escalation | Stealth surfaces | Use case |
| ------- | ---------- | ---------------- | -------- |
| `Auto` (default) | HTTP first, escalate to browser on WAF/403 | None | Balanced; handles most sites |
| `Always` | Skip HTTP entirely | None | JS-heavy SPAs |
| `Never` | No browser fallback | None | Performance-critical; no WAF expected |
| `Stealth` | Browser tier for every request | JS patches, native TLS spoofing, UA/viewport defaults | Stealth-hardened mode for challenging sites |

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

## WAF detection corpus <span class="version-badge">v0.3</span>

Kreuzcrawl classifies WAF fingerprints via `TomlClassifier` at `crates/kreuzcrawl/rules/waf_fingerprints.toml`. The rules currently cover Cloudflare, DataDome, PerimeterX/HUMAN Security, Imperva, AWS WAF, Akamai, F5, and generic block patterns.

Fingerprints match response headers, body substrings, or status code/header combinations. The classifier supports hot reload via `TomlClassifier::watch()` so rule updates can land without restarting the process.

## Dispatch and EWMA state <span class="version-badge">v0.3</span>

The default retry and dispatch layer can combine WAF signals, transient errors, and `EwmaDomainState` observations. EWMA state lets callers track per-domain outcomes and feed a `LearningRetryPolicy` without requiring a database.

## Bypass providers

Kreuzcrawl exposes the `BypassProvider` trait and `BypassResponse` type for caller-owned integrations. Providers are responsible for authentication, request shaping, response decoding, cost metadata, and error mapping.

Kreuzcrawl does not ship Bright Data, Zyte, ScrapingBee, or other vendor adapters in the core crate.
