//! Bridge between kreuzcrawl's trait-based engine and polyglot bindings.
//!
//! The core [`CrawlEngine`] uses `Arc<dyn Trait>` for pluggable components,
//! which cannot cross FFI boundaries. This module provides a config-only
//! construction path with default implementations, plus async adapter
//! functions that alef can generate bindings for.

use crate::engine::CrawlEngine;
use crate::error::CrawlError;
#[cfg(not(target_arch = "wasm32"))]
use crate::types::{BatchCrawlStreamRequest, CrawlEvent, CrawlStreamRequest};
use crate::types::{CrawlConfig, CrawlResult, MapResult, ScrapeResult};
#[cfg(not(target_arch = "wasm32"))]
use futures::future::BoxFuture;
#[cfg(not(target_arch = "wasm32"))]
use futures::stream::{BoxStream, StreamExt};
use serde::{Deserialize, Serialize};

/// Opaque handle to a configured crawl engine.
///
/// Constructed via [`create_engine`] with an optional [`CrawlConfig`].
/// Default implementations for all pluggable components are used internally.
#[derive(Clone)]
pub struct CrawlEngineHandle {
    inner: CrawlEngine,
}

#[cfg(not(target_arch = "wasm32"))]
impl CrawlEngineHandle {
    /// Stream a single-URL crawl, yielding [`CrawlEvent`]s as pages are processed.
    ///
    /// Returns an async stream that emits one event per crawled page, plus a
    /// terminal `Complete` event. On per-URL failure during the crawl, emits an
    /// `Error` event followed by `Complete`. The stream item type is wrapped in
    /// a `Result` to surface transport-level errors; today every emit is `Ok`.
    pub fn crawl_stream(
        &self,
        req: CrawlStreamRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<CrawlEvent, CrawlError>>, CrawlError>> {
        let engine = self.inner.clone();
        Box::pin(async move {
            let stream = engine.crawl_stream(&req.url);
            Ok(stream.map(Ok::<CrawlEvent, CrawlError>).boxed())
        })
    }

    /// Stream a multi-URL crawl, yielding [`CrawlEvent`]s across all seeds.
    ///
    /// Returns an async stream that emits one event per crawled page across all
    /// seeds, plus terminal `Complete` and `Error` events as appropriate. The
    /// stream item type is wrapped in a `Result` to surface transport-level
    /// errors; today every emit is `Ok`.
    pub fn batch_crawl_stream(
        &self,
        req: BatchCrawlStreamRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<CrawlEvent, CrawlError>>, CrawlError>> {
        let engine = self.inner.clone();
        Box::pin(async move {
            let url_refs: Vec<&str> = req.urls.iter().map(String::as_str).collect();
            let stream = engine.batch_crawl_stream(&url_refs);
            Ok(stream.map(Ok::<CrawlEvent, CrawlError>).boxed())
        })
    }
}

/// Result from a single URL in a batch scrape operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchScrapeResult {
    /// The URL that was scraped.
    pub url: String,
    /// The scrape result, if successful.
    pub result: Option<ScrapeResult>,
    /// The error message, if the scrape failed.
    pub error: Option<String>,
}

/// Result from a single URL in a batch crawl operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCrawlResult {
    /// The seed URL that was crawled.
    pub url: String,
    /// The crawl result, if successful.
    pub result: Option<CrawlResult>,
    /// The error message, if the crawl failed.
    pub error: Option<String>,
}

/// Create a new crawl engine with the given configuration.
///
/// If `config` is `None`, uses [`CrawlConfig::default()`].
/// Returns an error if the configuration is invalid.
pub fn create_engine(config: Option<CrawlConfig>) -> Result<CrawlEngineHandle, CrawlError> {
    let mut builder = CrawlEngine::builder();
    if let Some(config) = config {
        config.validate()?;
        builder = builder.config(config);
    }
    let engine = builder.build()?;
    Ok(CrawlEngineHandle { inner: engine })
}

/// Scrape a single URL, returning extracted page data.
pub async fn scrape(engine: &CrawlEngineHandle, url: &str) -> Result<ScrapeResult, CrawlError> {
    engine.inner.scrape(url).await
}

/// Crawl a website starting from `url`, following links up to the configured depth.
pub async fn crawl(engine: &CrawlEngineHandle, url: &str) -> Result<CrawlResult, CrawlError> {
    engine.inner.crawl(url).await
}

/// Discover all pages on a website by following links and sitemaps.
pub async fn map_urls(engine: &CrawlEngineHandle, url: &str) -> Result<MapResult, CrawlError> {
    engine.inner.map(url).await
}

/// Scrape multiple URLs concurrently.
pub async fn batch_scrape(engine: &CrawlEngineHandle, urls: Vec<String>) -> Result<Vec<BatchScrapeResult>, CrawlError> {
    if urls.is_empty() {
        return Err(CrawlError::InvalidConfig("batch_urls must not be empty".into()));
    }
    let url_refs: Vec<&str> = urls.iter().map(String::as_str).collect();
    let results = engine.inner.batch_scrape(&url_refs).await;
    Ok(results
        .into_iter()
        .map(|(url, result)| match result {
            Ok(r) => BatchScrapeResult {
                url,
                result: Some(r),
                error: None,
            },
            Err(e) => BatchScrapeResult {
                url,
                result: None,
                error: Some(e.to_string()),
            },
        })
        .collect())
}

/// Crawl multiple seed URLs concurrently, each following links to configured depth.
pub async fn batch_crawl(engine: &CrawlEngineHandle, urls: Vec<String>) -> Result<Vec<BatchCrawlResult>, CrawlError> {
    if urls.is_empty() {
        return Err(CrawlError::InvalidConfig("batch_urls must not be empty".into()));
    }
    let url_refs: Vec<&str> = urls.iter().map(String::as_str).collect();
    let results = engine.inner.batch_crawl(&url_refs).await;
    Ok(results
        .into_iter()
        .map(|(url, result)| match result {
            Ok(r) => BatchCrawlResult {
                url,
                result: Some(r),
                error: None,
            },
            Err(e) => BatchCrawlResult {
                url,
                result: None,
                error: Some(e.to_string()),
            },
        })
        .collect())
}
