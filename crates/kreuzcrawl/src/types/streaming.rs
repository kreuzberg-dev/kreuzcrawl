use serde::{Deserialize, Serialize};

/// Request to begin a single-URL streaming crawl.
///
/// Wraps a single seed URL for delivery through the streaming-adapter binding
/// surface. Required as a struct because alef's streaming adapter requires a
/// named request type — primitives are not supported.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrawlStreamRequest {
    /// The seed URL to crawl.
    pub url: String,
}

/// Request to begin a multi-URL streaming crawl.
///
/// Wraps a set of seed URLs for delivery through the streaming-adapter binding
/// surface. Required as a struct because alef's streaming adapter requires a
/// named request type — primitives are not supported.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchCrawlStreamRequest {
    /// The seed URLs to crawl. Each URL is followed independently up to the
    /// engine's configured depth.
    pub urls: Vec<String>,
}
