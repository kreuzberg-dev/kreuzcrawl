//! Pluggable bypass-provider trait.
//!
//! When set on `CrawlConfig.bypass`, the engine routes URL fetches through
//! this provider instead of the native HTTP / chromiumoxide backends. This
//! is the integration surface for caller-supplied bypass vendors —
//! kreuzcrawl ships no vendor adapters of its own.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;

use crate::error::CrawlError;

/// Response returned by a `BypassProvider::fetch` call.
///
/// Distinct from the internal `HttpResponse` so the bypass surface stays
/// stable as the native HTTP/browser fetchers evolve, and so bypass
/// adapters can populate cost + vendor metadata without surfacing
/// `HttpResponse`'s native/browser-specific fields.
#[derive(Debug, Clone)]
pub struct BypassResponse {
    /// HTTP status code returned by the vendor for the target URL.
    pub status: u16,
    /// Content-Type header (e.g. "text/html; charset=utf-8"). Empty if absent.
    pub content_type: String,
    /// Rendered page body as text. UTF-8 lossy if the vendor returned binary.
    pub body: String,
    /// Raw response bytes — preserved for callers that want to recompute
    /// encoding themselves.
    pub body_bytes: Vec<u8>,
    /// Response headers, normalized to lowercase keys.
    pub headers: HashMap<String, Vec<String>>,
    /// Final URL after vendor-side redirect following, if reported.
    /// Empty when the vendor doesn't surface the resolved URL.
    pub final_url: String,
    /// Per-request cost in USD reported by the vendor, when available.
    ///
    /// `None` means the vendor didn't report cost in the response (the
    /// provider may still have a static fallback configured); the caller
    /// decides whether to bill the request.
    pub cost_usd: Option<f64>,
    /// Vendor-side request identifier for log correlation, when available.
    pub vendor_request_id: Option<String>,
}

/// Caller-supplied bypass backend. Implementations are responsible for
/// vendor authentication, request shaping, response decoding, and mapping
/// vendor errors into `CrawlError`.
#[async_trait]
pub trait BypassProvider: Send + Sync + fmt::Debug {
    /// Fetch the target URL through the provider, returning a rendered
    /// response. The body should be the page HTML as the vendor returns
    /// it — the downstream extraction pipeline expects the same shape as
    /// a native or chromiumoxide fetch.
    async fn fetch(&self, url: &str) -> Result<BypassResponse, CrawlError>;

    /// Stable, lowercase vendor identifier used for span attributes and
    /// metrics labels. Must not change across releases.
    fn vendor_name(&self) -> &'static str;
}

/// Convenience type alias used on `CrawlConfig.bypass`.
pub type DynBypassProvider = Arc<dyn BypassProvider>;
