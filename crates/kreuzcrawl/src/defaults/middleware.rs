//! Crawl middleware implementations.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;

use crate::error::CrawlError;
use crate::traits::{CrawlMiddleware, RequestContext, ResponseContext};

/// Middleware that does nothing -- passes everything through unchanged.
#[derive(Debug, Clone, Default)]
pub struct NoopMiddleware;

#[async_trait]
impl CrawlMiddleware for NoopMiddleware {
    async fn before_request(&self, _ctx: &mut RequestContext) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn after_response(&self, _ctx: &mut ResponseContext) -> Result<(), CrawlError> {
        Ok(())
    }
}

/// Middleware that rotates user-agent strings from a configured list.
#[derive(Debug)]
pub struct UaRotationMiddleware {
    user_agents: Vec<String>,
    index: AtomicUsize,
}

impl UaRotationMiddleware {
    /// Create a new UA rotation middleware with the given user-agent list.
    pub fn new(user_agents: Vec<String>) -> Self {
        Self {
            user_agents,
            index: AtomicUsize::new(0),
        }
    }

    fn next_ua(&self) -> Option<&str> {
        if self.user_agents.is_empty() {
            return None;
        }
        let idx = self.index.fetch_add(1, Ordering::Relaxed) % self.user_agents.len();
        Some(&self.user_agents[idx])
    }
}

#[async_trait]
impl CrawlMiddleware for UaRotationMiddleware {
    async fn before_request(&self, ctx: &mut RequestContext) -> Result<(), CrawlError> {
        if let Some(ua) = self.next_ua() {
            ctx.headers.insert("user-agent".to_owned(), ua.to_owned());
        }
        Ok(())
    }

    async fn after_response(&self, _ctx: &mut ResponseContext) -> Result<(), CrawlError> {
        Ok(())
    }
}

/// In-memory HTTP response cache using ETag/Last-Modified for conditional requests.
///
/// On cache hit with matching ETag or Last-Modified, avoids re-downloading unchanged pages.
/// Cache is bounded by max_entries (LRU eviction).
#[derive(Debug)]
pub struct CachingMiddleware {
    cache: std::sync::Mutex<CacheStore>,
    max_entries: usize,
}

#[derive(Debug)]
struct CacheStore {
    entries: HashMap<String, CachedResponse>,
    order: Vec<String>, // LRU order (oldest first)
}

#[derive(Debug, Clone)]
struct CachedResponse {
    etag: Option<String>,
    last_modified: Option<String>,
    body: String,
    status: u16,
    content_type: String,
}

impl CachingMiddleware {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: std::sync::Mutex::new(CacheStore {
                entries: HashMap::with_capacity(max_entries),
                order: Vec::with_capacity(max_entries),
            }),
            max_entries,
        }
    }
}

#[async_trait]
impl CrawlMiddleware for CachingMiddleware {
    async fn before_request(&self, ctx: &mut RequestContext) -> Result<(), CrawlError> {
        let cache = self.cache.lock().unwrap();
        if let Some(cached) = cache.entries.get(&ctx.url) {
            if let Some(ref etag) = cached.etag {
                ctx.headers.insert("if-none-match".to_owned(), etag.clone());
            }
            if let Some(ref lm) = cached.last_modified {
                ctx.headers
                    .insert("if-modified-since".to_owned(), lm.clone());
            }
        }
        Ok(())
    }

    async fn after_response(&self, ctx: &mut ResponseContext) -> Result<(), CrawlError> {
        if ctx.status == 304 {
            // Not modified -- use cached body
            let cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.entries.get(&ctx.url) {
                ctx.body = cached.body.clone();
                ctx.status = cached.status;
                ctx.content_type = cached.content_type.clone();
            }
        } else if ctx.status >= 200 && ctx.status < 300 {
            // Cache the response
            let etag = ctx.headers.get("etag").cloned();
            let last_modified = ctx.headers.get("last-modified").cloned();

            if etag.is_some() || last_modified.is_some() {
                let mut cache = self.cache.lock().unwrap();

                // LRU eviction
                if cache.entries.len() >= self.max_entries
                    && !cache.entries.contains_key(&ctx.url)
                    && let Some(oldest) = cache.order.first().cloned()
                {
                    cache.entries.remove(&oldest);
                    cache.order.remove(0);
                }

                // Remove from order if updating existing
                cache.order.retain(|u| u != &ctx.url);
                cache.order.push(ctx.url.clone());

                cache.entries.insert(
                    ctx.url.clone(),
                    CachedResponse {
                        etag,
                        last_modified,
                        body: ctx.body.clone(),
                        status: ctx.status,
                        content_type: ctx.content_type.clone(),
                    },
                );
            }
        }
        Ok(())
    }
}

// NOTE: Proxy rotation was intentionally removed. Rotating proxies via middleware
// leaked credentials as HTTP headers sent to target servers (security vulnerability).
// Proxy rotation should be handled at the infrastructure level (e.g., a proxy gateway)
// or by configuring separate CrawlEngine instances with different `proxy` configs.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::RequestContext;

    #[tokio::test]
    async fn test_ua_rotation_cycles() {
        let mw = UaRotationMiddleware::new(vec![
            "Mozilla/5.0".to_owned(),
            "Chrome/100".to_owned(),
            "Safari/17".to_owned(),
        ]);
        let mut ctx = RequestContext {
            url: "http://a.com".into(),
            headers: Default::default(),
        };
        mw.before_request(&mut ctx).await.unwrap();
        assert_eq!(ctx.headers.get("user-agent").unwrap(), "Mozilla/5.0");

        let mut ctx = RequestContext {
            url: "http://b.com".into(),
            headers: Default::default(),
        };
        mw.before_request(&mut ctx).await.unwrap();
        assert_eq!(ctx.headers.get("user-agent").unwrap(), "Chrome/100");

        let mut ctx = RequestContext {
            url: "http://c.com".into(),
            headers: Default::default(),
        };
        mw.before_request(&mut ctx).await.unwrap();
        assert_eq!(ctx.headers.get("user-agent").unwrap(), "Safari/17");

        // Wraps around
        let mut ctx = RequestContext {
            url: "http://d.com".into(),
            headers: Default::default(),
        };
        mw.before_request(&mut ctx).await.unwrap();
        assert_eq!(ctx.headers.get("user-agent").unwrap(), "Mozilla/5.0");
    }

    #[tokio::test]
    async fn test_ua_rotation_empty_list() {
        let mw = UaRotationMiddleware::new(vec![]);
        let mut ctx = RequestContext {
            url: "http://a.com".into(),
            headers: Default::default(),
        };
        mw.before_request(&mut ctx).await.unwrap();
        assert!(ctx.headers.get("user-agent").is_none());
    }

    #[tokio::test]
    async fn test_ua_rotation_single_agent() {
        let mw = UaRotationMiddleware::new(vec!["Only-Agent".to_owned()]);
        for _ in 0..5 {
            let mut ctx = RequestContext {
                url: "http://a.com".into(),
                headers: Default::default(),
            };
            mw.before_request(&mut ctx).await.unwrap();
            assert_eq!(ctx.headers.get("user-agent").unwrap(), "Only-Agent");
        }
    }

    #[tokio::test]
    async fn test_noop_middleware_passes_through() {
        let mw = NoopMiddleware;
        let mut ctx = RequestContext {
            url: "http://test.com".into(),
            headers: Default::default(),
        };
        mw.before_request(&mut ctx).await.unwrap();
        assert!(ctx.headers.is_empty());
    }

    #[tokio::test]
    async fn test_caching_stores_response_with_etag() {
        let mw = CachingMiddleware::new(10);
        let url = "http://example.com/page".to_owned();

        // Simulate a 200 response with an ETag
        let mut headers = std::collections::HashMap::new();
        headers.insert("etag".to_owned(), "\"abc123\"".to_owned());
        let mut resp_ctx = ResponseContext {
            url: url.clone(),
            status: 200,
            content_type: "text/html".to_owned(),
            body: "<html>hello</html>".to_owned(),
            headers,
        };
        mw.after_response(&mut resp_ctx).await.unwrap();

        // Subsequent request should inject if-none-match
        let mut req_ctx = RequestContext {
            url: url.clone(),
            headers: Default::default(),
        };
        mw.before_request(&mut req_ctx).await.unwrap();
        assert_eq!(req_ctx.headers.get("if-none-match").unwrap(), "\"abc123\"");
    }

    #[tokio::test]
    async fn test_caching_stores_response_with_last_modified() {
        let mw = CachingMiddleware::new(10);
        let url = "http://example.com/page2".to_owned();

        let mut headers = std::collections::HashMap::new();
        headers.insert(
            "last-modified".to_owned(),
            "Wed, 01 Jan 2025 00:00:00 GMT".to_owned(),
        );
        let mut resp_ctx = ResponseContext {
            url: url.clone(),
            status: 200,
            content_type: "text/html".to_owned(),
            body: "body".to_owned(),
            headers,
        };
        mw.after_response(&mut resp_ctx).await.unwrap();

        let mut req_ctx = RequestContext {
            url: url.clone(),
            headers: Default::default(),
        };
        mw.before_request(&mut req_ctx).await.unwrap();
        assert_eq!(
            req_ctx.headers.get("if-modified-since").unwrap(),
            "Wed, 01 Jan 2025 00:00:00 GMT"
        );
    }

    #[tokio::test]
    async fn test_caching_304_restores_cached_body() {
        let mw = CachingMiddleware::new(10);
        let url = "http://example.com/cached".to_owned();

        // First: cache a 200 response
        let mut headers = std::collections::HashMap::new();
        headers.insert("etag".to_owned(), "\"v1\"".to_owned());
        let mut resp_ctx = ResponseContext {
            url: url.clone(),
            status: 200,
            content_type: "text/html".to_owned(),
            body: "original body".to_owned(),
            headers,
        };
        mw.after_response(&mut resp_ctx).await.unwrap();

        // Second: simulate a 304 response
        let mut resp_ctx_304 = ResponseContext {
            url: url.clone(),
            status: 304,
            content_type: String::new(),
            body: String::new(),
            headers: std::collections::HashMap::new(),
        };
        mw.after_response(&mut resp_ctx_304).await.unwrap();

        assert_eq!(resp_ctx_304.body, "original body");
        assert_eq!(resp_ctx_304.status, 200);
        assert_eq!(resp_ctx_304.content_type, "text/html");
    }

    #[tokio::test]
    async fn test_caching_lru_eviction() {
        let mw = CachingMiddleware::new(2);

        // Fill cache with 2 entries
        for i in 0..2 {
            let url = format!("http://example.com/{i}");
            let mut headers = std::collections::HashMap::new();
            headers.insert("etag".to_owned(), format!("\"etag{i}\""));
            let mut resp_ctx = ResponseContext {
                url,
                status: 200,
                content_type: "text/html".to_owned(),
                body: format!("body{i}"),
                headers,
            };
            mw.after_response(&mut resp_ctx).await.unwrap();
        }

        // Add a 3rd entry -- should evict the first (oldest)
        let mut headers = std::collections::HashMap::new();
        headers.insert("etag".to_owned(), "\"etag2\"".to_owned());
        let mut resp_ctx = ResponseContext {
            url: "http://example.com/2".to_owned(),
            status: 200,
            content_type: "text/html".to_owned(),
            body: "body2".to_owned(),
            headers,
        };
        mw.after_response(&mut resp_ctx).await.unwrap();

        // First entry should be evicted
        let mut req_ctx = RequestContext {
            url: "http://example.com/0".to_owned(),
            headers: Default::default(),
        };
        mw.before_request(&mut req_ctx).await.unwrap();
        assert!(req_ctx.headers.get("if-none-match").is_none());

        // Second entry should still be cached
        let mut req_ctx = RequestContext {
            url: "http://example.com/1".to_owned(),
            headers: Default::default(),
        };
        mw.before_request(&mut req_ctx).await.unwrap();
        assert_eq!(req_ctx.headers.get("if-none-match").unwrap(), "\"etag1\"");
    }

    #[tokio::test]
    async fn test_caching_skips_response_without_validators() {
        let mw = CachingMiddleware::new(10);
        let url = "http://example.com/no-validators".to_owned();

        // 200 response with no etag or last-modified
        let mut resp_ctx = ResponseContext {
            url: url.clone(),
            status: 200,
            content_type: "text/html".to_owned(),
            body: "body".to_owned(),
            headers: std::collections::HashMap::new(),
        };
        mw.after_response(&mut resp_ctx).await.unwrap();

        // Should not have cached anything
        let mut req_ctx = RequestContext {
            url,
            headers: Default::default(),
        };
        mw.before_request(&mut req_ctx).await.unwrap();
        assert!(req_ctx.headers.get("if-none-match").is_none());
        assert!(req_ctx.headers.get("if-modified-since").is_none());
    }

    #[tokio::test]
    async fn test_caching_does_not_cache_error_responses() {
        let mw = CachingMiddleware::new(10);
        let url = "http://example.com/error".to_owned();

        let mut headers = std::collections::HashMap::new();
        headers.insert("etag".to_owned(), "\"err\"".to_owned());
        let mut resp_ctx = ResponseContext {
            url: url.clone(),
            status: 500,
            content_type: "text/html".to_owned(),
            body: "error".to_owned(),
            headers,
        };
        mw.after_response(&mut resp_ctx).await.unwrap();

        let mut req_ctx = RequestContext {
            url,
            headers: Default::default(),
        };
        mw.before_request(&mut req_ctx).await.unwrap();
        assert!(req_ctx.headers.get("if-none-match").is_none());
    }

}
