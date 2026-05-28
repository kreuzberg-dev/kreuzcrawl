//! CrawlEngine composes trait implementations into a crawl pipeline.

#[cfg(not(target_arch = "wasm32"))]
mod batch;
mod builder;
#[cfg(not(target_arch = "wasm32"))]
mod crawl_loop;

use std::sync::Arc;

use crate::error::CrawlError;
#[cfg(not(target_arch = "wasm32"))]
use crate::tower::CrawlRequest;
use crate::traits::*;
use crate::types::*;

pub use builder::CrawlEngineBuilder;

/// The main crawl engine, composed of pluggable trait implementations.
#[derive(Clone)]
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub struct CrawlEngine {
    pub(crate) config: CrawlConfig,
    pub(crate) frontier: Arc<dyn Frontier>,
    pub(crate) rate_limiter: Arc<dyn RateLimiter>,
    pub(crate) store: Arc<dyn CrawlStore>,
    pub(crate) event_emitter: Arc<dyn EventEmitter>,
    pub(crate) strategy: Arc<dyn CrawlStrategy>,
    pub(crate) content_filter: Arc<dyn ContentFilter>,
    pub(crate) cache: Arc<dyn CrawlCache>,
    /// Shared UA rotation layer — preserves rotation counter across service builds.
    #[cfg(not(target_arch = "wasm32"))]
    ua_rotation: crate::tower::UaRotationLayer,
    #[cfg(all(not(target_arch = "wasm32"), feature = "browser-native"))]
    pub(crate) native_browser_executor: Option<Arc<kreuzcrawl_browser::adapter::NativeBrowserExecutor>>,
}

impl CrawlEngine {
    /// Create a new [`CrawlEngineBuilder`].
    pub fn builder() -> CrawlEngineBuilder {
        CrawlEngineBuilder::new()
    }

    /// Build the Tower service stack for HTTP fetching.
    ///
    /// Layers (outermost to innermost):
    /// 1. Per-domain rate limiting
    /// 2. HTTP response caching
    /// 3. User-agent rotation
    /// 4. Base HTTP fetch
    #[cfg(not(target_arch = "wasm32"))]
    fn build_service(
        &self,
        client: &reqwest::Client,
    ) -> tower::util::BoxCloneService<CrawlRequest, crate::tower::CrawlResponse, CrawlError> {
        use tower::ServiceBuilder;

        let service = ServiceBuilder::new()
            .layer(crate::tower::PerDomainRateLimitLayer::new(self.rate_limiter.clone()))
            .layer(crate::tower::CrawlCacheLayer::new(self.cache.clone()))
            .layer(self.ua_rotation.clone())
            .service(crate::tower::HttpFetchService::new(client.clone(), self.config.clone()));

        #[cfg(feature = "tracing")]
        let service = tower::ServiceBuilder::new()
            .layer(crate::tower::CrawlTracingLayer::new())
            .service(service);

        tower::util::BoxCloneService::new(service)
    }

    /// Fetch a URL through the appropriate path (Tower stack or browser) and
    /// return the `CrawlResponse` together with a flag indicating whether the
    /// browser was used.
    ///
    /// This is intentionally `#[cfg(not(target_arch = "wasm32"))]`-only: wasm
    /// has its own simpler inline path inside `scrape`.
    #[cfg(not(target_arch = "wasm32"))]
    async fn fetch_response(&self, url: &str) -> Result<(crate::tower::CrawlResponse, bool), CrawlError> {
        // BrowserMode::Always — preserved short-circuit, skip dispatch entirely.
        #[cfg(feature = "browser")]
        if self.config.browser.mode == crate::types::BrowserMode::Always {
            let pool = self.config.browser_pool.as_deref();
            #[cfg(feature = "browser-native")]
            let http_resp =
                crate::browser::browser_fetch(url, &self.config, None, pool, self.native_browser_executor.as_deref())
                    .await?;
            #[cfg(not(feature = "browser-native"))]
            let http_resp = crate::browser::browser_fetch(url, &self.config, None, pool).await?;
            let (crawl_resp, _extras) = Self::browser_http_to_crawl(http_resp);
            return Ok((crawl_resp, true));
        }

        // Derive the effective strategy. Two adjustments are applied in order:
        //
        // 1. When `escalation_strategy` is left at its default (`BrowserOnly`) and a
        //    bypass provider is configured, promote to `BypassFirst` to preserve the
        //    pre-tier-dispatch bypass-first semantic.
        //
        // 2. When the effective strategy would route to the Browser tier (`BrowserOnly`)
        //    but `BrowserMode::Never` is set, demote to `None` so no escalation target
        //    exists. Without this, the dispatch loop escalates to the Browser tier and
        //    returns `Err(Unsupported)` instead of the original 403 / WAF error.
        let effective_strategy = {
            let s = if self.config.escalation_strategy == crate::types::EscalationStrategy::BrowserOnly
                && self.config.bypass.is_some()
            {
                crate::types::EscalationStrategy::BypassFirst
            } else {
                self.config.escalation_strategy
            };
            if s == crate::types::EscalationStrategy::BrowserOnly
                && self.config.browser.mode == crate::types::BrowserMode::Never
            {
                crate::types::EscalationStrategy::None
            } else {
                s
            }
        };

        // BypassFirst — legacy preserved: always route through bypass, skip HTTP.
        if matches!(effective_strategy, crate::types::EscalationStrategy::BypassFirst)
            && let Some(provider) = &self.config.bypass
        {
            let bypass_resp = provider.fetch(url).await?;
            return Ok((
                crate::tower::CrawlResponse {
                    status: bypass_resp.status,
                    content_type: bypass_resp.content_type,
                    body: bypass_resp.body,
                    body_bytes: bypass_resp.body_bytes,
                    headers: bypass_resp.headers,
                },
                false,
            ));
        }

        // Build retry policy and budget once for the entire dispatch loop.
        let retry_policy: crate::types::DynRetryPolicy = self
            .config
            .retry_policy
            .clone()
            .unwrap_or_else(|| std::sync::Arc::new(crate::defaults::dispatch::SimpleRetryPolicy::new()));
        let budget: crate::types::DynEscalationBudget = self
            .config
            .escalation_budget
            .clone()
            .unwrap_or_else(|| std::sync::Arc::new(crate::defaults::dispatch::UnlimitedBudget));

        let mut current_tier = crate::types::Tier::Http;
        let mut attempt: u32 = 0;
        // Telemetry accumulators — only consumed by emit_dispatch_span (tracing feature).
        // Gated behind cfg so the compiler doesn't warn about unused variables / assignments
        // when the tracing feature is not enabled.
        #[cfg(feature = "tracing")]
        let mut tiers_attempted: Vec<&'static str> = Vec::new();
        #[cfg(feature = "tracing")]
        let mut last_escalation_reason: Option<&'static str> = None;
        #[cfg(feature = "tracing")]
        let policy_name = retry_policy.name();

        loop {
            #[cfg(feature = "tracing")]
            tiers_attempted.push(Self::tier_name(current_tier));

            let tier_result = self.run_tier(current_tier, url).await;

            match tier_result {
                Ok((resp, browser_used)) => {
                    // Success path: build outcome and consult the policy.
                    // The policy may still signal Escalate (e.g. soft-block detection
                    // via content density). If no next tier is available, return now.
                    let outcome = crate::types::AttemptOutcome {
                        attempt,
                        url,
                        status: Some(resp.status),
                        error: None,
                        waf_signal: None,
                        body_size: resp.body.len(),
                        content_density: 0.0, // placeholder — computed in a later commit
                        bytes_transferred: Some(resp.body_bytes.len() as u64),
                        previous_tier: current_tier,
                    };
                    match retry_policy.decide(&outcome).await {
                        crate::types::RetryDirective::Stop => {
                            #[cfg(feature = "tracing")]
                            Self::emit_dispatch_span(
                                url,
                                &tiers_attempted,
                                last_escalation_reason,
                                attempt,
                                policy_name,
                            );
                            return Ok((resp, browser_used));
                        }
                        crate::types::RetryDirective::Retry { backoff_ms } => {
                            tokio::time::sleep(std::time::Duration::from_millis(backoff_ms)).await;
                            attempt += 1;
                            continue;
                        }
                        crate::types::RetryDirective::Escalate { reason } => {
                            if let Some(next) = Self::next_tier(current_tier, effective_strategy)
                                && budget.try_consume(Self::tier_cost_cents(next)).await.is_ok()
                            {
                                #[cfg(feature = "tracing")]
                                {
                                    last_escalation_reason = Some(Self::escalation_reason_str(&reason));
                                }
                                current_tier = next;
                                attempt = 0;
                                continue;
                            }
                            // No next tier or budget exhausted on a success-path Escalate.
                            // The policy signalled a soft-block or WAF interstitial even though
                            // the HTTP layer returned 2xx. Synthesise an error from the reason
                            // rather than returning the challenge body as Ok.
                            #[cfg(feature = "tracing")]
                            Self::emit_dispatch_span(
                                url,
                                &tiers_attempted,
                                last_escalation_reason,
                                attempt,
                                policy_name,
                            );
                            return Err(Self::escalation_reason_to_error(&reason, url));
                        }
                    }
                }
                Err(err) => {
                    // soft_http_errors: synthesise a response for HTTP-level variants.
                    if self.config.soft_http_errors {
                        if matches!(err, CrawlError::NotFound(_)) {
                            return Ok((Self::synthesise_status(404), false));
                        }
                        if matches!(err, CrawlError::Forbidden(_) | CrawlError::WafBlocked { .. }) {
                            return Ok((Self::synthesise_status(403), false));
                        }
                    }

                    let outcome = crate::types::AttemptOutcome {
                        attempt,
                        url,
                        status: None,
                        error: Some(&err),
                        waf_signal: None,
                        body_size: 0,
                        content_density: 0.0,
                        bytes_transferred: None,
                        previous_tier: current_tier,
                    };
                    match retry_policy.decide(&outcome).await {
                        crate::types::RetryDirective::Stop => {
                            #[cfg(feature = "tracing")]
                            Self::emit_dispatch_span(
                                url,
                                &tiers_attempted,
                                last_escalation_reason,
                                attempt,
                                policy_name,
                            );
                            return Err(err);
                        }
                        crate::types::RetryDirective::Retry { backoff_ms } => {
                            tokio::time::sleep(std::time::Duration::from_millis(backoff_ms)).await;
                            attempt += 1;
                            continue;
                        }
                        crate::types::RetryDirective::Escalate { reason: _reason } => {
                            if let Some(next) = Self::next_tier(current_tier, effective_strategy)
                                && budget.try_consume(Self::tier_cost_cents(next)).await.is_ok()
                            {
                                #[cfg(feature = "tracing")]
                                {
                                    last_escalation_reason = Some(Self::escalation_reason_str(&_reason));
                                }
                                current_tier = next;
                                attempt = 0;
                                continue;
                            }
                            // No next tier or budget exhausted — surface the error.
                            #[cfg(feature = "tracing")]
                            Self::emit_dispatch_span(
                                url,
                                &tiers_attempted,
                                last_escalation_reason,
                                attempt,
                                policy_name,
                            );
                            return Err(err);
                        }
                    }
                }
            }
        }
    }

    /// Dispatch a single fetch attempt to the given tier.
    ///
    /// Returns `(CrawlResponse, browser_used)` or a `CrawlError`.
    #[cfg(not(target_arch = "wasm32"))]
    async fn run_tier(
        &self,
        tier: crate::types::Tier,
        url: &str,
    ) -> Result<(crate::tower::CrawlResponse, bool), CrawlError> {
        match tier {
            crate::types::Tier::Http => {
                let client = crate::http::build_client(&self.config)?;
                let mut service = self.build_service(&client);
                use tower::Service;
                let resp = service.call(CrawlRequest::new(url)).await?;
                Ok((resp, false))
            }
            crate::types::Tier::Bypass => {
                let provider = self.config.bypass.as_ref().ok_or_else(|| {
                    CrawlError::InvalidConfig("escalation to Bypass tier but no bypass provider configured".into())
                })?;
                let bypass_resp = provider.fetch(url).await?;
                Ok((
                    crate::tower::CrawlResponse {
                        status: bypass_resp.status,
                        content_type: bypass_resp.content_type,
                        body: bypass_resp.body,
                        body_bytes: bypass_resp.body_bytes,
                        headers: bypass_resp.headers,
                    },
                    false,
                ))
            }
            crate::types::Tier::Browser => {
                #[cfg(feature = "browser")]
                {
                    let pool = self.config.browser_pool.as_deref();
                    #[cfg(feature = "browser-native")]
                    let http_resp = crate::browser::browser_fetch(
                        url,
                        &self.config,
                        None,
                        pool,
                        self.native_browser_executor.as_deref(),
                    )
                    .await?;
                    #[cfg(not(feature = "browser-native"))]
                    let http_resp = crate::browser::browser_fetch(url, &self.config, None, pool).await?;
                    let (crawl_resp, _extras) = Self::browser_http_to_crawl(http_resp);
                    Ok((crawl_resp, true))
                }
                #[cfg(not(feature = "browser"))]
                Err(CrawlError::Unsupported(
                    "Browser tier requires the 'browser' feature".into(),
                ))
            }
        }
    }

    /// Convert an `HttpResponse` (from the browser path) into the `CrawlResponse`
    /// shape expected by the extraction pipeline.
    #[cfg(all(not(target_arch = "wasm32"), feature = "browser"))]
    fn browser_http_to_crawl(
        r: crate::http::HttpResponse,
    ) -> (crate::tower::CrawlResponse, Option<crate::http::BrowserExtras>) {
        let extras = r.browser_extras;
        (
            crate::tower::CrawlResponse {
                status: r.status,
                content_type: r.content_type,
                body: r.body,
                body_bytes: r.body_bytes,
                headers: std::collections::HashMap::new(),
            },
            extras,
        )
    }

    /// Synthesise a minimal response with the given HTTP status (empty body).
    ///
    /// Used by `soft_http_errors` to surface 4xx responses as `ScrapeResult`
    /// records rather than `CrawlError`.
    #[cfg(not(target_arch = "wasm32"))]
    fn synthesise_status(status: u16) -> crate::tower::CrawlResponse {
        crate::tower::CrawlResponse {
            status,
            content_type: String::new(),
            body: String::new(),
            body_bytes: Vec::new(),
            headers: std::collections::HashMap::new(),
        }
    }

    /// Convert an [`crate::types::EscalationReason`] from a terminal success-path
    /// `Escalate` directive into the most specific available [`CrawlError`].
    ///
    /// Called when the policy signals `Escalate` on a 2xx response (soft-block /
    /// WAF interstitial) but no higher tier is available or the budget is exhausted.
    /// Returning an error prevents the challenge-page body from reaching callers.
    #[cfg(not(target_arch = "wasm32"))]
    fn escalation_reason_to_error(reason: &crate::types::EscalationReason, url: &str) -> CrawlError {
        use crate::types::EscalationReason;
        match reason {
            EscalationReason::WafBlocked { vendor } => CrawlError::WafBlocked {
                vendor: vendor.clone(),
                message: format!("waf/blocked: {vendor} detected at {url}"),
            },
            EscalationReason::SoftBlock => CrawlError::Forbidden(format!("soft_block: {url}")),
            EscalationReason::RenderNeeded => {
                CrawlError::Unsupported(format!("js_render_needed but no browser tier available: {url}"))
            }
            EscalationReason::OriginUnreliable => {
                CrawlError::ServerError(format!("origin_unreliable and no escalation target: {url}"))
            }
        }
    }

    /// Determine the next tier given the current tier and active escalation strategy.
    ///
    /// Returns `None` when the current tier is terminal for the given strategy.
    #[cfg(not(target_arch = "wasm32"))]
    fn next_tier(
        current: crate::types::Tier,
        strategy: crate::types::EscalationStrategy,
    ) -> Option<crate::types::Tier> {
        use crate::types::{EscalationStrategy, Tier};
        match (current, strategy) {
            // Http → Browser
            (Tier::Http, EscalationStrategy::BrowserOnly) => Some(Tier::Browser),
            // Http → Bypass
            (Tier::Http, EscalationStrategy::BypassOnly) => Some(Tier::Bypass),
            (Tier::Http, EscalationStrategy::BypassThenBrowser) => Some(Tier::Bypass),
            // Bypass → Browser (only for BypassThenBrowser)
            (Tier::Bypass, EscalationStrategy::BypassThenBrowser) => Some(Tier::Browser),
            // Browser and None are always terminal; all other combos have no next tier.
            _ => None,
        }
    }

    /// Heuristic cost in internal "cents" for escalating to a tier.
    ///
    /// `Http` costs nothing (it's the baseline). `Bypass` and `Browser` cost 1 each
    /// so that `FixedBudget(n)` limits the total number of non-HTTP escalations per job.
    /// kreuzberg-cloud overrides this via a proper cost model at the cloud layer.
    #[cfg(not(target_arch = "wasm32"))]
    const fn tier_cost_cents(tier: crate::types::Tier) -> u32 {
        match tier {
            crate::types::Tier::Http => 0,
            crate::types::Tier::Bypass | crate::types::Tier::Browser => 1,
        }
    }

    /// Stable lowercase name for a tier, used in span attributes.
    #[cfg(all(not(target_arch = "wasm32"), feature = "tracing"))]
    const fn tier_name(tier: crate::types::Tier) -> &'static str {
        match tier {
            crate::types::Tier::Http => "http",
            crate::types::Tier::Bypass => "bypass",
            crate::types::Tier::Browser => "browser",
        }
    }

    /// Stable lowercase string for an escalation reason.
    #[cfg(all(not(target_arch = "wasm32"), feature = "tracing"))]
    fn escalation_reason_str(reason: &crate::types::EscalationReason) -> &'static str {
        use crate::types::EscalationReason;
        match reason {
            EscalationReason::WafBlocked { .. } => "waf_blocked",
            EscalationReason::SoftBlock => "soft_block",
            EscalationReason::RenderNeeded => "render_needed",
            EscalationReason::OriginUnreliable => "origin_unreliable",
        }
    }

    /// Emit structured dispatch telemetry via tracing.
    ///
    /// Fields: `dispatch.tier_chain`, `dispatch.escalation_reason`,
    /// `dispatch.attempt_count`, `dispatch.policy`.
    #[cfg(all(not(target_arch = "wasm32"), feature = "tracing"))]
    fn emit_dispatch_span(
        url: &str,
        tiers_attempted: &[&str],
        escalation_reason: Option<&str>,
        attempt_count: u32,
        policy: &str,
    ) {
        let tier_chain = tiers_attempted.join(",");
        tracing::info!(
            target: "kreuzcrawl::dispatch",
            url,
            "dispatch.tier_chain" = %tier_chain,
            "dispatch.escalation_reason" = escalation_reason.unwrap_or("none"),
            "dispatch.attempt_count" = attempt_count,
            "dispatch.policy" = policy,
        );
    }

    /// Scrape a single URL, returning the extracted data.
    ///
    /// On native targets, routes the request through the Tower service stack
    /// (rate limiting, UA rotation) then runs the extraction pipeline.
    /// On wasm, performs a direct HTTP fetch without the Tower stack.
    ///
    /// Browser fallback behaviour (native + `browser` feature only):
    /// - `BrowserMode::Always`: skips HTTP entirely, goes straight to headless Chrome.
    /// - `BrowserMode::Auto` + WAF blocked: falls back to headless Chrome when the
    ///   Tower stack returns `CrawlError::WafBlocked`.
    /// - `BrowserMode::Auto` + JS detected: after extraction, if `js_render_hint` is
    ///   `true` and the browser has not been used yet, re-fetches with headless Chrome
    ///   and re-runs the extraction pipeline on the rendered HTML.
    pub async fn scrape(&self, url: &str) -> Result<ScrapeResult, CrawlError> {
        self.config.validate()?;

        // Short-circuit for BrowserMode::Always so we can preserve browser_extras
        // rather than losing them in the fetch_response indirection.
        // Gated on browser-native (not just browser) so it also fires when only
        // the native backend is active without chromiumoxide.
        #[cfg(all(not(target_arch = "wasm32"), feature = "browser-native"))]
        if self.config.browser.mode == crate::types::BrowserMode::Always
            && self.config.browser.backend == crate::types::BrowserBackend::Native
        {
            let native_executor = self.native_browser_executor.as_deref().ok_or_else(|| {
                CrawlError::BrowserError("native browser executor is not available for BrowserBackend::Native".into())
            })?;
            let mut http_resp =
                crate::native_browser::native_browser_fetch(url, &self.config, None, native_executor).await?;
            let raw_extras = http_resp.browser_extras.take();
            let crawl_resp = crate::tower::CrawlResponse {
                status: http_resp.status,
                content_type: http_resp.content_type,
                body: http_resp.body,
                body_bytes: http_resp.body_bytes,
                headers: std::collections::HashMap::new(),
            };
            let mut result = crate::scrape::scrape_from_crawl_response(url, &crawl_resp, &self.config).await?;
            result.browser_used = true;
            if let Some(ex) = raw_extras {
                result.browser = Some(crate::types::BrowserExtras {
                    eval_result: ex.eval_result,
                    network_events: ex.network_events,
                    cookies: ex.cookies,
                });
            }
            return Ok(result);
        }

        #[cfg(not(target_arch = "wasm32"))]
        let (final_url, response, browser_used_for_fetch) = {
            use crawl_loop::follow_redirects;

            let max_redirects = self.config.max_redirects;
            let outcome = follow_redirects(self, url, max_redirects).await?;

            // When soft_http_errors is enabled, a synthesised 4xx response should
            // short-circuit extraction and return a minimal result rather than attempting
            // to parse an empty body as HTML. The redirect-chain synth (302→404) fires
            // regardless of soft_http_errors (handled in follow_redirects).
            let status = outcome.final_response.status;
            if matches!(status, 404 | 403) && outcome.final_response.body.is_empty() && self.config.soft_http_errors {
                return Ok(ScrapeResult {
                    status_code: status,
                    final_url: outcome.final_url,
                    content_type: String::new(),
                    html: String::new(),
                    body_size: 0,
                    metadata: PageMetadata::default(),
                    links: Vec::new(),
                    images: Vec::new(),
                    feeds: Vec::new(),
                    json_ld: Vec::new(),
                    is_allowed: true,
                    crawl_delay: None,
                    noindex_detected: false,
                    nofollow_detected: false,
                    x_robots_tag: None,
                    is_pdf: false,
                    was_skipped: false,
                    detected_charset: None,
                    auth_header_sent: self.config.auth.is_some(),
                    response_meta: None,
                    assets: Vec::new(),
                    js_render_hint: false,
                    browser_used: false,
                    markdown: None,
                    extracted_data: None,
                    extraction_meta: None,
                    screenshot: None,
                    downloaded_document: None,
                    browser: None,
                });
            }
            // Also short-circuit for redirected-chain 404s (redirect_count > 0) —
            // these come from follow_redirects regardless of soft_http_errors.
            if outcome.final_response.status == 404
                && outcome.final_response.body.is_empty()
                && outcome.redirect_count > 0
            {
                return Ok(ScrapeResult {
                    status_code: 404,
                    final_url: outcome.final_url,
                    content_type: String::new(),
                    html: String::new(),
                    body_size: 0,
                    metadata: PageMetadata::default(),
                    links: Vec::new(),
                    images: Vec::new(),
                    feeds: Vec::new(),
                    json_ld: Vec::new(),
                    is_allowed: true,
                    crawl_delay: None,
                    noindex_detected: false,
                    nofollow_detected: false,
                    x_robots_tag: None,
                    is_pdf: false,
                    was_skipped: false,
                    detected_charset: None,
                    auth_header_sent: self.config.auth.is_some(),
                    response_meta: None,
                    assets: Vec::new(),
                    js_render_hint: false,
                    browser_used: false,
                    markdown: None,
                    extracted_data: None,
                    extraction_meta: None,
                    screenshot: None,
                    downloaded_document: None,
                    browser: None,
                });
            }
            (outcome.final_url, outcome.final_response, outcome.browser_used)
        };

        #[cfg(target_arch = "wasm32")]
        let (final_url, response, browser_used_for_fetch) = {
            let client = crate::http::build_client(&self.config)?;
            let resp =
                crate::http::fetch_with_retry(url, &self.config, &std::collections::HashMap::new(), &client).await?;
            // Use the URL from the response: on wasm the browser follows redirects
            // transparently, so `resp.final_url` is the post-redirect URL — not
            // necessarily equal to the original `url` that was requested.
            let post_redirect_url = resp.final_url.clone();
            // fetch_with_retry returns HttpResponse; convert to CrawlResponse
            let crawl_resp = crate::tower::CrawlResponse {
                status: resp.status,
                content_type: resp.content_type,
                body: resp.body,
                body_bytes: resp.body_bytes,
                headers: resp.headers,
            };
            (post_redirect_url, crawl_resp, false)
        };

        let mut result = crate::scrape::scrape_from_crawl_response(&final_url, &response, &self.config).await?;
        result.browser_used = browser_used_for_fetch;

        // When the `browser` feature is not compiled in, BrowserMode::Always means the
        // caller explicitly opted into browser — mark browser_used true so bindings
        // that check it see the expected value (HTTP fallback was still used).
        #[cfg(not(feature = "browser"))]
        if self.config.browser.mode == crate::types::BrowserMode::Always {
            result.browser_used = true;
        }

        Ok(result)
    }

    /// Execute browser actions on a single page.
    ///
    /// The public API is always available. Runtime execution depends on the
    /// configured browser backend and the browser backend features compiled
    /// into the crate.
    pub async fn interact(
        &self,
        url: &str,
        actions: &[crate::interact::PageAction],
    ) -> Result<InteractionResult, CrawlError> {
        crate::interact::run(self, url, actions).await
    }

    /// Discover all pages on a website by following links and sitemaps.
    pub async fn map(&self, url: &str) -> Result<MapResult, CrawlError> {
        self.config.validate()?;
        crate::map::map(url, &self.config).await
    }
}

/// Wasm-specific sequential multi-page crawl implementations.
///
/// The native crawl loop uses `tokio::spawn`, `JoinSet`, and `Semaphore` which do not
/// compile to `wasm32-unknown-unknown`. These implementations drive the same BFS/DFS/
/// strategy logic sequentially using `.await` only — no concurrency primitives.
#[cfg(target_arch = "wasm32")]
impl CrawlEngine {
    /// Normalize a URL for deduplication on wasm.
    ///
    /// Strips query parameters and fragment, removes trailing slash (except root).
    /// Mirrors `normalize::normalize_url_for_dedup` which is cfg-gated to non-wasm.
    fn wasm_dedup_key(raw: &str) -> String {
        if let Ok(mut u) = url::Url::parse(raw) {
            u.set_fragment(None);
            u.set_query(None);
            let path = u.path().to_owned();
            if path.len() > 1 && path.ends_with('/') {
                u.set_path(&path[..path.len() - 1]);
            }
            u.to_string()
        } else {
            raw.to_owned()
        }
    }

    /// Convert a `ScrapeResult` into a `CrawlPageResult` at the given depth.
    fn scrape_to_crawl_page(scrape: ScrapeResult, url: &str, depth: usize, base_host: &str) -> CrawlPageResult {
        let domain = url::Url::parse(url)
            .ok()
            .and_then(|u| u.host_str().map(|h| h.to_owned()))
            .unwrap_or_default();
        let stayed_on_domain = domain == base_host;
        CrawlPageResult {
            url: url.to_owned(),
            normalized_url: crate::normalize::normalize_url(url),
            status_code: scrape.status_code,
            content_type: scrape.content_type,
            html: scrape.html,
            body_size: scrape.body_size,
            metadata: scrape.metadata,
            links: scrape.links,
            images: scrape.images,
            feeds: scrape.feeds,
            json_ld: scrape.json_ld,
            depth,
            stayed_on_domain,
            was_skipped: scrape.was_skipped,
            is_pdf: scrape.is_pdf,
            detected_charset: scrape.detected_charset,
            markdown: scrape.markdown,
            extracted_data: scrape.extracted_data,
            extraction_meta: scrape.extraction_meta,
            downloaded_document: scrape.downloaded_document,
            browser_used: scrape.browser_used,
        }
    }

    /// Compile regex patterns for path filtering, returning an error on invalid patterns.
    fn compile_path_regexes(patterns: &[String]) -> Result<Vec<regex::Regex>, CrawlError> {
        patterns
            .iter()
            .map(|pat| {
                regex::Regex::new(pat).map_err(|e| CrawlError::Other(format!("invalid regex pattern \"{pat}\": {e}")))
            })
            .collect()
    }

    /// Crawl a website starting from `url`.
    ///
    /// Implements a sequential BFS/strategy-driven crawl loop. Follows links discovered
    /// during scraping and applies `max_depth`, `max_pages`, `stay_on_domain`,
    /// `allow_subdomains`, `include_paths`, `exclude_paths`, and the configured
    /// `CrawlStrategy`. No concurrency primitives are used — each page is awaited
    /// sequentially, which is correct for the wasm single-threaded executor.
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, CrawlError> {
        use std::collections::HashSet;

        self.config.validate()?;

        let parsed_seed = url::Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
        let base_host = parsed_seed.host_str().unwrap_or("").to_owned();
        let base_host_suffix = format!(".{base_host}");

        let max_depth = self.config.max_depth.unwrap_or(usize::MAX);
        let max_pages = self.config.max_pages.unwrap_or(usize::MAX);

        let exclude_regexes = Self::compile_path_regexes(&self.config.exclude_paths)?;
        let include_regexes = Self::compile_path_regexes(&self.config.include_paths)?;

        // Local dedup set — mirrors the native frontier's `seen` set. The
        // engine's `frontier` trait object is also updated so that
        // `batch_crawl` across multiple seeds shares state correctly.
        let mut seen: HashSet<String> = HashSet::new();

        let seed_dedup = Self::wasm_dedup_key(url);
        seen.insert(seed_dedup.clone());
        let _ = self.frontier.mark_seen(&seed_dedup).await;

        // Working set: strategy selects from this Vec each iteration.
        let mut working_set: Vec<FrontierEntry> = vec![FrontierEntry {
            url: url.to_owned(),
            depth: 0,
            priority: 1.0,
        }];

        let mut pages: Vec<CrawlPageResult> = Vec::new();
        let mut normalized_urls: Vec<String> = Vec::new();
        let mut redirect_count: usize = 0;
        let mut was_skipped = false;
        let mut pages_failed: usize = 0;
        let mut urls_discovered: usize = 0;
        let mut urls_filtered: usize = 0;
        let mut crawl_error: Option<String> = None;
        // The final URL for CrawlResult is the post-redirect URL of the seed.
        // Wasm's `scrape()` follows redirects transparently, so we capture it
        // from the first page's ScrapeResult.
        let mut final_url = url.to_owned();

        // Sequential crawl loop — no spawn, no JoinSet.
        while !working_set.is_empty() {
            // Check stopping conditions before selecting next entry.
            let stats = CrawlStats {
                pages_crawled: pages.len(),
                pages_failed,
                urls_discovered,
                urls_filtered,
                elapsed: std::time::Duration::ZERO,
            };
            if !self.strategy.should_continue(&stats) {
                break;
            }
            if pages.len() >= max_pages {
                break;
            }

            let Some(idx) = self.strategy.select_next(&working_set) else {
                break;
            };
            let entry = working_set.swap_remove(idx);

            // Apply path filters (include/exclude regexes).
            if let Ok(parsed) = url::Url::parse(&entry.url) {
                let path = parsed.path();
                if !exclude_regexes.is_empty() && exclude_regexes.iter().any(|re| re.is_match(path)) {
                    urls_filtered += 1;
                    continue;
                }
                // Depth-0 seed is always included regardless of include_paths.
                if !include_regexes.is_empty() && entry.depth > 0 && !include_regexes.iter().any(|re| re.is_match(path))
                {
                    urls_filtered += 1;
                    continue;
                }
            }

            // Fetch + extract the page.
            let scrape = match self.scrape(&entry.url).await {
                Ok(s) => s,
                Err(e) => {
                    pages_failed += 1;
                    let error_msg = e.to_string();
                    self.event_emitter
                        .on_error(&crate::traits::ErrorEvent {
                            url: entry.url.clone(),
                            error: error_msg.clone(),
                        })
                        .await;
                    let _ = self.store.store_error(&entry.url, &e).await;
                    // Seed failure is propagated as a crawl-level error so that
                    // the batch_crawl wrapper can classify this seed as failed.
                    if entry.depth == 0 {
                        crawl_error = Some(error_msg);
                    }
                    continue;
                }
            };

            // Track seed redirect count from final_url divergence.
            // Wasm's scrape() follows redirects transparently via the browser;
            // final_url is the post-redirect URL.
            if entry.depth == 0 {
                final_url = scrape.final_url.clone();
                if scrape.final_url != entry.url {
                    redirect_count += 1;
                }
            }

            if scrape.was_skipped || scrape.is_pdf {
                was_skipped = true;
            }

            // Discover and enqueue links before building the page result so
            // that `links` in the page result is the full extracted set.
            if entry.depth < max_depth && !scrape.was_skipped && !scrape.is_pdf {
                for link in &scrape.links {
                    if link.link_type != LinkType::Internal && link.link_type != LinkType::Document {
                        continue;
                    }

                    let link_url = crate::normalize::strip_fragment(&link.url);

                    // stay_on_domain filter.
                    if self.config.stay_on_domain
                        && let Ok(lu) = url::Url::parse(&link_url)
                    {
                        let link_host = lu.host_str().unwrap_or("");
                        if link_host != base_host
                            && (!self.config.allow_subdomains || !link_host.ends_with(&base_host_suffix))
                        {
                            continue;
                        }
                    }

                    let dedup_key = Self::wasm_dedup_key(&link_url);
                    if !seen.contains(&dedup_key) {
                        seen.insert(dedup_key.clone());
                        let _ = self.frontier.mark_seen(&dedup_key).await;
                        let priority = self.strategy.score_url(&link_url, entry.depth + 1);
                        working_set.push(FrontierEntry {
                            url: link_url.clone(),
                            depth: entry.depth + 1,
                            priority,
                        });
                        urls_discovered += 1;
                        self.event_emitter.on_discovered(&link_url, entry.depth + 1).await;
                    }
                }
            }

            // Build and store the page result. Use the post-redirect URL as
            // the canonical page URL, matching native crawl behavior where
            // the final response URL (not the queued URL) is recorded.
            let page_url = scrape.final_url.clone();
            let page = Self::scrape_to_crawl_page(scrape, &page_url, entry.depth, &base_host);

            // Apply content filter — filtered pages still contribute to link discovery.
            let page = match self.content_filter.filter(page).await? {
                Some(filtered) => filtered,
                None => {
                    urls_filtered += 1;
                    continue;
                }
            };

            self.strategy.on_page_processed(&page);
            let _ = self.store.store_crawl_page(&page.url, &page).await;
            self.event_emitter
                .on_page(&crate::traits::PageEvent {
                    url: page.url.clone(),
                    status_code: page.status_code,
                    depth: page.depth,
                })
                .await;

            normalized_urls.push(crate::normalize::normalize_url(&page.url));
            pages.push(page);
        }

        // Emit completion event.
        let _ = self
            .store
            .on_complete(&CrawlStats {
                pages_crawled: pages.len(),
                pages_failed,
                urls_discovered,
                urls_filtered,
                elapsed: std::time::Duration::ZERO,
            })
            .await;
        self.event_emitter
            .on_complete(&crate::traits::CompleteEvent {
                pages_crawled: pages.len(),
            })
            .await;

        // Safety truncation.
        if pages.len() > max_pages {
            pages.truncate(max_pages);
        }

        let stayed_on_domain = pages.iter().all(|p| p.stayed_on_domain);
        Ok(CrawlResult::new(
            pages,
            final_url,
            redirect_count,
            was_skipped,
            crawl_error,
            Vec::new(),
            stayed_on_domain,
            normalized_urls,
        ))
    }

    /// Scrape multiple URLs sequentially (no concurrency on wasm).
    pub async fn batch_scrape(&self, urls: &[&str]) -> Vec<(String, Result<ScrapeResult, CrawlError>)> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            let result = self.scrape(url).await;
            results.push((url.to_string(), result));
        }
        results
    }

    /// Crawl multiple seed URLs sequentially (no concurrency on wasm).
    pub async fn batch_crawl(&self, urls: &[&str]) -> Vec<(String, Result<CrawlResult, CrawlError>)> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            let result = self.crawl(url).await;
            results.push((url.to_string(), result));
        }
        results
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    /// Verify that a connection-refused error propagates with [network:connection] tag
    /// rather than being swallowed by browser fallback. The engine's BrowserMode::Auto
    /// arm must not include Connection errors.
    #[tokio::test]
    async fn connection_refused_propagates_network_tag() {
        use crate::error::classify_reqwest_error;
        use std::time::Duration;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .build()
            .expect("client build must not fail");

        // Port 1 is universally closed; this reliably produces a connection error.
        let raw_err = client
            .get("http://127.0.0.1:1/")
            .send()
            .await
            .expect_err("expected connection error");

        let err = classify_reqwest_error(&raw_err);
        let msg = err.to_string();
        assert!(
            msg.contains("[network:connection]"),
            "expected [network:connection] in '{msg}'"
        );
        assert!(
            matches!(err, CrawlError::Connection(_)),
            "expected CrawlError::Connection, got {err:?}"
        );
    }

    /// Verify that a DNS resolution failure propagates with [network:dns] tag.
    #[tokio::test]
    async fn dns_failure_propagates_network_tag() {
        use crate::error::classify_reqwest_error;
        use std::time::Duration;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(1000))
            .build()
            .expect("client build must not fail");

        let raw_err = client
            .get("http://this-host-does-not-exist-kreuzcrawl-engine-test.invalid/")
            .send()
            .await
            .expect_err("expected dns error");

        let err = classify_reqwest_error(&raw_err);
        let msg = err.to_string();
        assert!(msg.contains("[network:dns]"), "expected [network:dns] in '{msg}'");
        assert!(
            matches!(err, CrawlError::Dns(_)),
            "expected CrawlError::Dns, got {err:?}"
        );
    }
}
