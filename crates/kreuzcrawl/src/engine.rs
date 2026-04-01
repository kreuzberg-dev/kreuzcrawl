//! CrawlEngine composes trait implementations into a crawl pipeline.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use regex::Regex;
use scraper::Html;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio_stream::wrappers::ReceiverStream;
use url::Url;

use std::collections::HashMap;

use tower::Service;

use crate::defaults;
use crate::error::CrawlError;
use crate::helpers::{compile_regexes, fetch_robots_rules, find_ascii_case_insensitive};
use crate::html::{
    HtmlExtraction, detect_charset, detect_meta_refresh, extract_page_data, is_binary_content_type,
    is_binary_url, is_html_content, is_pdf_content, is_pdf_url,
};
use crate::http::{build_client, extract_cookies_from_hashmap};
use crate::normalize::{normalize_url, normalize_url_for_dedup, resolve_redirect, strip_fragment};
use crate::robots::{RobotsRules, is_path_allowed};
use crate::tower::CrawlRequest;
use crate::traits::*;
use crate::types::*;

/// Result of a concurrent fetch task, holding everything needed to process a completed fetch.
struct FetchResult {
    entry: FrontierEntry,
    status_code: u16,
    content_type: String,
    body: String,
    headers: HashMap<String, Vec<String>>,
    extraction: HtmlExtraction,
    is_binary: bool,
    is_pdf: bool,
    detected_charset: Option<String>,
}

/// The main crawl engine, composed of pluggable trait implementations.
#[derive(Clone)]
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
    ua_rotation: crate::tower::UaRotationLayer,
}

/// Builder for [`CrawlEngine`].
///
/// Any field left unset will be filled with a default implementation
/// from [`crate::defaults`].
pub struct CrawlEngineBuilder {
    config: Option<CrawlConfig>,
    frontier: Option<Arc<dyn Frontier>>,
    rate_limiter: Option<Arc<dyn RateLimiter>>,
    store: Option<Arc<dyn CrawlStore>>,
    event_emitter: Option<Arc<dyn EventEmitter>>,
    strategy: Option<Arc<dyn CrawlStrategy>>,
    content_filter: Option<Arc<dyn ContentFilter>>,
    cache: Option<Arc<dyn CrawlCache>>,
}

impl CrawlEngineBuilder {
    /// Create a new builder with no fields set.
    pub fn new() -> Self {
        Self {
            config: None,
            frontier: None,
            rate_limiter: None,
            store: None,
            event_emitter: None,
            strategy: None,
            content_filter: None,
            cache: None,
        }
    }

    /// Set the crawl configuration.
    pub fn config(mut self, config: CrawlConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the frontier implementation.
    pub fn frontier(mut self, frontier: impl Frontier + 'static) -> Self {
        self.frontier = Some(Arc::new(frontier));
        self
    }

    /// Set the rate limiter implementation.
    pub fn rate_limiter(mut self, rate_limiter: impl RateLimiter + 'static) -> Self {
        self.rate_limiter = Some(Arc::new(rate_limiter));
        self
    }

    /// Set the store implementation.
    pub fn store(mut self, store: impl CrawlStore + 'static) -> Self {
        self.store = Some(Arc::new(store));
        self
    }

    /// Set the event emitter implementation.
    pub fn event_emitter(mut self, event_emitter: impl EventEmitter + 'static) -> Self {
        self.event_emitter = Some(Arc::new(event_emitter));
        self
    }

    /// Set the crawl strategy implementation.
    pub fn strategy(mut self, strategy: impl CrawlStrategy + 'static) -> Self {
        self.strategy = Some(Arc::new(strategy));
        self
    }

    /// Set the content filter implementation.
    pub fn content_filter(mut self, content_filter: impl ContentFilter + 'static) -> Self {
        self.content_filter = Some(Arc::new(content_filter));
        self
    }

    /// Set the persistent cache implementation.
    pub fn cache(mut self, cache: impl CrawlCache + 'static) -> Self {
        self.cache = Some(Arc::new(cache));
        self
    }

    /// Build the [`CrawlEngine`], validating the config and filling in defaults.
    ///
    /// Returns an error if the configuration is invalid.
    pub fn build(self) -> Result<CrawlEngine, CrawlError> {
        let config = self.config.unwrap_or_default();
        config.validate()?;
        let ua_rotation = crate::tower::UaRotationLayer::new(config.user_agents.clone());
        Ok(CrawlEngine {
            config,
            frontier: self
                .frontier
                .unwrap_or_else(|| Arc::new(defaults::InMemoryFrontier::new())),
            rate_limiter: self.rate_limiter.unwrap_or_else(|| {
                Arc::new(defaults::PerDomainThrottle::new(
                    std::time::Duration::from_millis(200),
                ))
            }),
            store: self.store.unwrap_or_else(|| Arc::new(defaults::NoopStore)),
            event_emitter: self
                .event_emitter
                .unwrap_or_else(|| Arc::new(defaults::NoopEmitter)),
            strategy: self
                .strategy
                .unwrap_or_else(|| Arc::new(defaults::BfsStrategy)),
            content_filter: self
                .content_filter
                .unwrap_or_else(|| Arc::new(defaults::NoopFilter)),
            cache: self.cache.unwrap_or_else(|| Arc::new(defaults::NoopCache)),
            ua_rotation,
        })
    }
}

impl Default for CrawlEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
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
    fn build_service(
        &self,
        client: &reqwest::Client,
    ) -> tower::util::BoxCloneService<CrawlRequest, crate::tower::CrawlResponse, CrawlError> {
        use tower::ServiceBuilder;

        let service = ServiceBuilder::new()
            .layer(crate::tower::PerDomainRateLimitLayer::new(
                self.rate_limiter.clone(),
            ))
            .layer(crate::tower::CrawlCacheLayer::new(self.cache.clone()))
            .layer(self.ua_rotation.clone())
            .service(crate::tower::HttpFetchService::new(
                client.clone(),
                self.config.clone(),
            ));

        #[cfg(feature = "tracing")]
        let service = tower::ServiceBuilder::new()
            .layer(crate::tower::CrawlTracingLayer::new())
            .service(service);

        tower::util::BoxCloneService::new(service)
    }

    /// Scrape a single URL, returning the extracted data.
    ///
    /// Routes the request through the Tower service stack (rate limiting,
    /// UA rotation) then runs the extraction pipeline on the response.
    pub async fn scrape(&self, url: &str) -> Result<ScrapeResult, CrawlError> {
        let client = build_client(&self.config)?;
        let mut service = self.build_service(&client);

        let response = match service.call(CrawlRequest::new(url)).await {
            Ok(resp) => resp,
            // When robots.txt checking is enabled, a 404 is not a hard error --
            // the page simply doesn't exist, but the robots check result is still
            // meaningful. Return a minimal successful ScrapeResult.
            Err(CrawlError::NotFound(_)) if self.config.respect_robots_txt => {
                return Ok(ScrapeResult {
                    status_code: 404,
                    content_type: String::new(),
                    html: String::new(),
                    body_size: 0,
                    metadata: PageMetadata::default(),
                    links: Vec::new(),
                    images: Vec::new(),
                    feeds: Vec::new(),
                    json_ld: Vec::new(),
                    is_allowed: true, // 404 robots.txt means "allow all"
                    crawl_delay: None,
                    noindex_detected: false,
                    nofollow_detected: false,
                    x_robots_tag: None,
                    is_pdf: false,
                    was_skipped: false,
                    detected_charset: None,
                    main_content_only: self.config.main_content_only,
                    auth_header_sent: self.config.auth.is_some(),
                    response_meta: None,
                    assets: Vec::new(),
                    js_render_hint: false,
                    browser_used: false,
                    markdown: None,
                    extracted_data: None,
                    extraction_meta: None,
                    screenshot: None,
                });
            }
            Err(e) => return Err(e),
        };

        crate::scrape::scrape_from_crawl_response(url, &response, &self.config).await
    }

    /// Crawl a website starting from `url`, following links up to the configured depth.
    ///
    /// Uses the engine's [`CrawlStrategy`] and [`Frontier`] traits to control URL
    /// selection order and deduplication.
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, CrawlError> {
        self.crawl_with_sender(url, None).await
    }

    /// Crawl a website and return a stream of events as pages are processed.
    ///
    /// Uses the engine's trait implementations (strategy, frontier, etc.) for the crawl.
    pub fn crawl_stream(&self, url: &str) -> ReceiverStream<CrawlEvent> {
        let url = url.to_owned();
        let engine = self.clone();

        let channel_size = self.config.max_concurrent.unwrap_or(4) * 16;
        let (tx, rx) = tokio::sync::mpsc::channel(channel_size);

        tokio::spawn(async move {
            match engine.crawl_with_sender(&url, Some(tx.clone())).await {
                Ok(result) => {
                    let _ = tx
                        .send(CrawlEvent::Complete {
                            pages_crawled: result.pages.len(),
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx
                        .send(CrawlEvent::Error {
                            url: url.clone(),
                            error: e.to_string(),
                        })
                        .await;
                    let _ = tx.send(CrawlEvent::Complete { pages_crawled: 0 }).await;
                }
            }
        });

        ReceiverStream::new(rx)
    }

    /// Scrape multiple URLs concurrently.
    ///
    /// Unlike the standalone `batch::batch_scrape`, this method routes each URL
    /// through the engine's middleware chain, rate limiter, and cache.
    pub async fn batch_scrape(
        &self,
        urls: &[&str],
    ) -> Vec<(String, Result<ScrapeResult, CrawlError>)> {
        let max_concurrent = self.config.max_concurrent.unwrap_or(10);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut handles = Vec::with_capacity(urls.len());

        for url in urls {
            let url_owned = url.to_string();
            let engine = self.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            handles.push(tokio::spawn(async move {
                let _permit = permit;
                let result = engine.scrape(&url_owned).await;
                (url_owned, result)
            }));
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push((
                    String::new(),
                    Err(CrawlError::Other(format!("task panicked: {e}"))),
                )),
            }
        }
        results
    }

    /// Discover all pages on a website by following links and sitemaps.
    pub async fn map(&self, url: &str) -> Result<MapResult, CrawlError> {
        // Note: map() currently bypasses engine middleware. The internal map.rs module operates
        // independently and does not have access to self.middlewares. Full support would require
        // refactoring map.rs to accept middleware functions as parameters. For now, map() provides
        // raw site mapping without middleware processing.
        crate::map::map(url, &self.config).await
    }

    /// Crawl multiple seed URLs, each following links to configured depth.
    /// Returns results paired with seed URLs as they complete.
    pub async fn batch_crawl(
        &self,
        urls: &[&str],
    ) -> Vec<(String, Result<CrawlResult, CrawlError>)> {
        let max_concurrent = self.config.max_concurrent.unwrap_or(10);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        let mut handles = Vec::with_capacity(urls.len());

        for url in urls {
            let url_owned = url.to_string();
            let engine = self.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            handles.push(tokio::spawn(async move {
                let _permit = permit;
                let result = engine.crawl(&url_owned).await;
                (url_owned, result)
            }));
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    // Task panicked
                    results.push((
                        String::new(),
                        Err(CrawlError::Other(format!("task panicked: {e}"))),
                    ));
                }
            }
        }
        results
    }

    /// Crawl multiple seed URLs and stream events from all crawls.
    pub fn batch_crawl_stream(&self, urls: &[&str]) -> ReceiverStream<CrawlEvent> {
        let urls: Vec<String> = urls.iter().map(|u| u.to_string()).collect();
        let engine = self.clone();
        let channel_size = self.config.max_concurrent.unwrap_or(10) * 16;
        let (tx, rx) = tokio::sync::mpsc::channel(channel_size);

        tokio::spawn(async move {
            let max_concurrent = engine.config.max_concurrent.unwrap_or(10);
            let semaphore = Arc::new(Semaphore::new(max_concurrent));
            let mut join_set = JoinSet::new();

            for url in urls {
                let engine = engine.clone();
                let tx = tx.clone();
                let permit = semaphore.clone().acquire_owned().await.unwrap();

                join_set.spawn(async move {
                    let _permit = permit;
                    match engine.crawl_with_sender(&url, Some(tx.clone())).await {
                        Ok(result) => {
                            let _ = tx
                                .send(CrawlEvent::Complete {
                                    pages_crawled: result.pages.len(),
                                })
                                .await;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(CrawlEvent::Error {
                                    url: url.clone(),
                                    error: e.to_string(),
                                })
                                .await;
                        }
                    }
                });
            }

            while let Some(_) = join_set.join_next().await {}
        });

        ReceiverStream::new(rx)
    }

    /// Internal crawl implementation that uses the engine's trait objects.
    ///
    /// When `tx` is `Some`, each page is sent through the channel as it is processed
    /// so that callers can consume results incrementally via [`crawl_stream`](Self::crawl_stream).
    pub(crate) async fn crawl_with_sender(
        &self,
        url: &str,
        tx: Option<tokio::sync::mpsc::Sender<CrawlEvent>>,
    ) -> Result<CrawlResult, CrawlError> {
        let parsed_url =
            Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
        let client = build_client(&self.config)?;
        let base_host = parsed_url.host_str().unwrap_or("").to_owned();

        let base_host_suffix = format!(".{base_host}");
        let max_depth = self.config.max_depth.unwrap_or(0);
        let max_pages = self.config.max_pages.unwrap_or(usize::MAX);
        let max_redirects = self.config.max_redirects;

        let capacity = max_pages.min(1024);
        let mut pages: Vec<CrawlPageResult> = Vec::with_capacity(capacity);
        let mut normalized_urls: Vec<String> = Vec::with_capacity(capacity);
        let mut redirect_count: usize = 0;
        let mut error: Option<String> = None;
        let mut was_skipped = false;
        let mut all_cookies: Vec<CookieInfo> = Vec::new();

        let mut pages_failed: usize = 0;
        let mut urls_discovered: usize = 0;
        let mut urls_filtered: usize = 0;

        let start_time = Instant::now();

        // ── Phase 1: resolve initial redirects ──────────────────────────
        // Uses the Tower service stack so headers (e.g. user-agent rotation)
        // are applied to redirect-resolution fetches.
        let mut service = self.build_service(&client);

        let mut current_url = url.to_owned();
        let mut seen_redirects: HashSet<String> = HashSet::with_capacity(max_redirects + 1);
        seen_redirects.insert(current_url.clone());

        loop {
            let resp = match service.call(CrawlRequest::new(&current_url)).await {
                Ok(r) => r,
                Err(e) => {
                    error = Some(format!("{e}"));
                    break;
                }
            };

            if self.config.cookies_enabled {
                all_cookies.extend(extract_cookies_from_hashmap(&resp.headers));
            }

            let status = resp.status;
            let is_redirect = matches!(status, 301 | 302 | 303 | 307 | 308);

            if is_redirect {
                if redirect_count >= max_redirects {
                    error = Some("too many redirects".to_owned());
                    break;
                }
                if let Some(location) = resp.headers.get("location").and_then(|v| v.first()) {
                    let target = resolve_redirect(&current_url, location);
                    if seen_redirects.contains(&target) {
                        error = Some("redirect loop detected".to_owned());
                        break;
                    }
                    seen_redirects.insert(target.clone());
                    redirect_count += 1;
                    current_url = target;
                    continue;
                }
            }

            // Check for Refresh header redirect
            if let Some(refresh) = resp.headers.get("refresh").and_then(|v| v.first())
                && let Some(pos) = find_ascii_case_insensitive(refresh, "url=")
            {
                if redirect_count >= max_redirects {
                    error = Some("too many redirects".to_owned());
                    break;
                }
                let target_path = refresh[pos + 4..].trim();
                let target = resolve_redirect(&current_url, target_path);
                if !seen_redirects.contains(&target) {
                    seen_redirects.insert(target.clone());
                    redirect_count += 1;
                    current_url = target;
                    continue;
                }
            }

            // Check for meta refresh
            if is_html_content(&resp.content_type, &resp.body) {
                let doc = Html::parse_document(&resp.body);
                if let Some(refresh_target) = detect_meta_refresh(&doc) {
                    if redirect_count >= max_redirects {
                        error = Some("too many redirects".to_owned());
                        break;
                    }
                    let target = resolve_redirect(&current_url, &refresh_target);
                    if !seen_redirects.contains(&target) {
                        seen_redirects.insert(target.clone());
                        redirect_count += 1;
                        current_url = target;
                        continue;
                    }
                }
            }

            // Check for error status on final page (after redirect)
            if status >= 400 && redirect_count > 0 {
                error = Some(format!("HTTP {status}"));
                break;
            }

            break;
        }

        let final_url = current_url;

        // If we have an error already (from redirects), return early
        if error.is_some() {
            return Ok(CrawlResult::new(
                pages,
                final_url,
                redirect_count,
                false,
                error,
                all_cookies,
                normalized_urls,
            ));
        }

        // ── Phase 2: prepare filters and robots rules ───────────────────
        let exclude_regexes: Vec<Regex> = compile_regexes(&self.config.exclude_paths)?;
        let include_regexes: Vec<Regex> = compile_regexes(&self.config.include_paths)?;

        let robots_rules: Option<RobotsRules> = if self.config.respect_robots_txt {
            fetch_robots_rules(&final_url, &self.config, &client).await
        } else {
            None
        };

        // Pass robots.txt crawl-delay to RateLimiter
        if let Some(ref rules) = robots_rules
            && let Some(delay) = rules.crawl_delay
            && let Ok(parsed) = Url::parse(&final_url)
            && let Some(domain) = parsed.host_str()
        {
            self.rate_limiter
                .set_crawl_delay(domain, Duration::from_secs(delay))
                .await?;
        }

        // ── Phase 3: seed the working set and mark as seen via Frontier ─
        // We maintain a local working_set (Vec) rather than popping from frontier because:
        // 1. The CrawlStrategy needs random access to all candidates via select_next(&[...])
        // 2. The frontier is shared across potential concurrent batch_crawl operations
        // 3. This design keeps the hot path lock-free (no frontier mutex per iteration)
        // The frontier's push/pop are available for custom implementations that need
        // persistent or distributed URL queues. The is_seen/mark_seen methods provide
        // deduplication regardless of whether push/pop are used.
        let mut working_set: Vec<FrontierEntry> = Vec::new();

        let dedup_key = normalize_url_for_dedup(&final_url);
        self.frontier.mark_seen(&dedup_key).await?;
        working_set.push(FrontierEntry {
            url: final_url.clone(),
            depth: 0,
            priority: 1.0,
        });

        // ── Phase 4: main crawl loop using CrawlStrategy + concurrent fetching ──
        let max_concurrent = self.config.max_concurrent.unwrap_or(10);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut join_set: JoinSet<Result<FetchResult, (FrontierEntry, CrawlError)>> =
            JoinSet::new();

        while !working_set.is_empty() || !join_set.is_empty() {
            // 1. Fill JoinSet from working_set, up to max_concurrent
            while join_set.len() < max_concurrent && !working_set.is_empty() {
                if pages.len() + join_set.len() >= max_pages {
                    break;
                }

                // Check strategy stopping condition
                let stats = CrawlStats {
                    pages_crawled: pages.len(),
                    pages_failed,
                    urls_discovered,
                    urls_filtered,
                    elapsed: start_time.elapsed(),
                };
                if !self.strategy.should_continue(&stats) {
                    break;
                }

                // Let the strategy pick the next entry
                let Some(idx) = self.strategy.select_next(&working_set) else {
                    break;
                };
                let entry = working_set.swap_remove(idx);
                let depth = entry.depth;

                // Parse URL for filtering
                let page_parsed = match Url::parse(&entry.url) {
                    Ok(u) => u,
                    Err(_) => continue,
                };
                let path = page_parsed.path();

                // Check include/exclude paths BEFORE spawning to avoid wasted fetches
                if !exclude_regexes.is_empty() && exclude_regexes.iter().any(|re| re.is_match(path))
                {
                    urls_filtered += 1;
                    continue;
                }
                if !include_regexes.is_empty()
                    && depth > 0
                    && !include_regexes.iter().any(|re| re.is_match(path))
                {
                    urls_filtered += 1;
                    continue;
                }

                // Check robots.txt rules
                if let Some(ref rules) = robots_rules
                    && !is_path_allowed(path, rules)
                {
                    urls_filtered += 1;
                    continue;
                }

                // Acquire semaphore permit before spawning
                let permit = semaphore
                    .clone()
                    .acquire_owned()
                    .await
                    .map_err(|_| CrawlError::Other("semaphore closed".into()))?;

                let mut svc = service.clone();

                join_set.spawn(async move {
                    let _permit = permit;

                    // ONE call — Tower handles rate limit + cache + UA rotation + fetch
                    let resp = svc
                        .call(CrawlRequest::new(&entry.url))
                        .await
                        .map_err(|e| (entry.clone(), e))?;

                    let status_code = resp.status;
                    let content_type = resp.content_type.clone();
                    let headers = resp.headers.clone();
                    let body = resp.body.clone();

                    // Extract in spawn_blocking (Html is !Send)
                    let body_clone = body.clone();
                    let url_for_extract = entry.url.clone();
                    let content_type_clone = content_type.clone();

                    let (extraction, is_binary, is_pdf, detected_charset) =
                        tokio::task::spawn_blocking(move || {
                            let parsed_url = Url::parse(&url_for_extract)
                                .unwrap_or_else(|_| Url::parse("http://invalid").unwrap());
                            let is_binary = is_binary_content_type(&content_type_clone)
                                || is_binary_url(&url_for_extract);
                            let is_pdf = is_pdf_content(&content_type_clone, &body_clone)
                                || is_pdf_url(&url_for_extract);
                            let is_html = is_html_content(&content_type_clone, &body_clone);

                            let doc = Html::parse_document(&body_clone);
                            let extraction = extract_page_data(
                                &doc,
                                &body_clone,
                                &parsed_url,
                                is_html && !is_binary && !is_pdf,
                                false,
                            );
                            let detected_charset = detect_charset(&content_type_clone, &body_clone);
                            (extraction, is_binary, is_pdf, detected_charset)
                        })
                        .await
                        .unwrap();

                    Ok(FetchResult {
                        entry,
                        status_code,
                        content_type,
                        body,
                        headers,
                        extraction,
                        is_binary,
                        is_pdf,
                        detected_charset,
                    })
                });
            }

            // 2. Collect one completed result (or break if nothing in-flight)
            if join_set.is_empty() {
                break;
            }

            let result = join_set.join_next().await;
            let Some(result) = result else {
                break;
            };

            match result {
                Ok(Ok(fetch)) => {
                    let page_url = fetch.entry.url.clone();
                    let depth = fetch.entry.depth;

                    if self.config.cookies_enabled {
                        all_cookies.extend(extract_cookies_from_hashmap(&fetch.headers));
                    }

                    let mut body = fetch.body;

                    // Body truncation
                    if let Some(max_size) = self.config.max_body_size
                        && body.len() > max_size
                    {
                        body.truncate(max_size);
                    }
                    let body_size = body.len();

                    let page_was_skipped = fetch.is_binary || fetch.is_pdf;
                    if page_was_skipped {
                        was_skipped = true;
                    }

                    let page_parsed = Url::parse(&page_url)
                        .unwrap_or_else(|_| Url::parse("http://invalid").unwrap());
                    let domain = page_parsed.host_str().unwrap_or("");
                    let norm_url = normalize_url(&page_url);
                    let stayed_on_domain = domain == base_host;

                    normalized_urls.push(norm_url.clone());

                    // ── Link discovery: add children to working set via Frontier dedup ──
                    if depth < max_depth && !page_was_skipped {
                        for link in &fetch.extraction.links {
                            if link.link_type == LinkType::Internal
                                || link.link_type == LinkType::Document
                            {
                                let link_url = strip_fragment(&link.url);

                                // Check stay_on_domain
                                if self.config.stay_on_domain
                                    && let Ok(lu) = Url::parse(&link_url)
                                {
                                    let link_host = lu.host_str().unwrap_or("");
                                    if link_host != base_host
                                        && (!self.config.allow_subdomains
                                            || !link_host.ends_with(&base_host_suffix))
                                    {
                                        continue;
                                    }
                                }

                                let dedup_key = normalize_url_for_dedup(&link_url);
                                if !self.frontier.is_seen(&dedup_key).await? {
                                    self.frontier.mark_seen(&dedup_key).await?;
                                    let priority = self.strategy.score_url(&link_url, depth + 1);
                                    working_set.push(FrontierEntry {
                                        url: link_url.clone(),
                                        depth: depth + 1,
                                        priority,
                                    });
                                    urls_discovered += 1;
                                    self.event_emitter.on_discovered(&link_url, depth + 1).await;
                                }
                            }
                        }
                    }

                    // Convert HTML to Markdown.
                    let markdown = crate::markdown::convert_to_markdown(&body).await;

                    let page = CrawlPageResult {
                        url: page_url.clone(),
                        normalized_url: norm_url,
                        status_code: fetch.status_code,
                        content_type: fetch.content_type,
                        html: body,
                        body_size,
                        metadata: fetch.extraction.metadata,
                        links: fetch.extraction.links,
                        images: fetch.extraction.images,
                        feeds: fetch.extraction.feeds,
                        json_ld: fetch.extraction.json_ld,
                        depth,
                        stayed_on_domain,
                        was_skipped: page_was_skipped,
                        is_pdf: fetch.is_pdf,
                        detected_charset: fetch.detected_charset,
                        markdown,
                        extracted_data: None,
                        extraction_meta: None,
                    };

                    // Apply content filter -- links are already discovered above,
                    // so filtered-out pages still contribute to link discovery.
                    let page = match self.content_filter.filter(page).await? {
                        Some(filtered_page) => filtered_page,
                        None => {
                            urls_filtered += 1;
                            continue;
                        }
                    };

                    // Notify strategy (e.g. for adaptive saturation tracking)
                    self.strategy.on_page_processed(&page);

                    // Store the crawl page result
                    let _ = self.store.store_crawl_page(&page.url, &page).await;

                    // Emit page event
                    self.event_emitter
                        .on_page(&PageEvent {
                            url: page.url.clone(),
                            status_code: page.status_code,
                            depth: page.depth,
                        })
                        .await;

                    // Send page event through the channel if streaming
                    if let Some(ref sender) = tx
                        && sender
                            .send(CrawlEvent::Page(Box::new(page.clone())))
                            .await
                            .is_err()
                    {
                        // Receiver dropped; stop crawling
                        break;
                    }

                    pages.push(page);

                    if pages.len() >= max_pages {
                        // Abort remaining in-flight tasks and break
                        join_set.abort_all();
                        break;
                    }
                }
                Ok(Err((entry, error))) => {
                    // Fetch error
                    pages_failed += 1;
                    self.event_emitter
                        .on_error(&ErrorEvent {
                            url: entry.url.clone(),
                            error: error.to_string(),
                        })
                        .await;
                    let _ = self.store.store_error(&entry.url, &error).await;
                }
                Err(_join_error) => {
                    // Task panicked -- log and continue
                    pages_failed += 1;
                }
            }

            // 3. Check stopping condition
            let stats = CrawlStats {
                pages_crawled: pages.len(),
                pages_failed,
                urls_discovered,
                urls_filtered,
                elapsed: start_time.elapsed(),
            };
            if !self.strategy.should_continue(&stats) {
                break;
            }
        }

        // Safety: ensure we never return more than max_pages
        if pages.len() > max_pages {
            pages.truncate(max_pages);
        }

        // Build final stats and notify store/emitter
        let stats = CrawlStats {
            pages_crawled: pages.len(),
            pages_failed,
            urls_discovered,
            urls_filtered,
            elapsed: start_time.elapsed(),
        };
        let _ = self.store.on_complete(&stats).await;
        self.event_emitter
            .on_complete(&CompleteEvent {
                pages_crawled: pages.len(),
            })
            .await;

        // Deduplicate cookies by (name, domain, path)
        let mut seen_cookies: HashSet<(String, String, String)> = HashSet::new();
        all_cookies.retain(|c| {
            seen_cookies.insert((
                c.name.clone(),
                c.domain.clone().unwrap_or_default(),
                c.path.clone().unwrap_or_default(),
            ))
        });

        Ok(CrawlResult::new(
            pages,
            final_url,
            redirect_count,
            was_skipped,
            error,
            all_cookies,
            normalized_urls,
        ))
    }
}
