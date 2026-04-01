//! HTTP response cache layer for the Tower service stack.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{SystemTime, UNIX_EPOCH};

use tower::{Layer, Service};

use super::types::{CrawlRequest, CrawlResponse};
use crate::error::CrawlError;
use crate::traits::{CachedPage, CrawlCache};

/// Tower layer that caches HTTP responses using a [`CrawlCache`].
pub struct CrawlCacheLayer {
    cache: Arc<dyn CrawlCache>,
}

impl CrawlCacheLayer {
    pub fn new(cache: Arc<dyn CrawlCache>) -> Self {
        Self { cache }
    }
}

impl<S: Clone> Layer<S> for CrawlCacheLayer {
    type Service = CrawlCacheService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CrawlCacheService {
            inner,
            cache: self.cache.clone(),
        }
    }
}

/// Tower service that checks the cache before forwarding requests and stores responses.
#[derive(Clone)]
pub struct CrawlCacheService<S> {
    inner: S,
    cache: Arc<dyn CrawlCache>,
}

impl<S> Service<CrawlRequest> for CrawlCacheService<S>
where
    S: Service<CrawlRequest, Response = CrawlResponse, Error = CrawlError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = CrawlResponse;
    type Error = CrawlError;
    type Future = Pin<Box<dyn Future<Output = Result<CrawlResponse, CrawlError>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: CrawlRequest) -> Self::Future {
        let cache = self.cache.clone();
        let mut inner = self.inner.clone();
        std::mem::swap(&mut self.inner, &mut inner);
        let url = req.url.clone();

        Box::pin(async move {
            // Check cache
            if let Ok(Some(cached)) = cache.get(&url).await {
                return Ok(CrawlResponse {
                    status: cached.status_code,
                    content_type: cached.content_type,
                    body: cached.body,
                    body_bytes: Vec::new(),
                    headers: HashMap::new(),
                });
            }

            // Cache miss -- forward to inner service
            let resp = inner.call(req).await?;

            // Store in cache on success
            if resp.status >= 200 && resp.status < 300 {
                let _ = cache
                    .set(
                        &url,
                        &CachedPage {
                            url: url.clone(),
                            status_code: resp.status,
                            content_type: resp.content_type.clone(),
                            body: resp.body.clone(),
                            etag: resp.headers.get("etag").cloned(),
                            last_modified: resp.headers.get("last-modified").cloned(),
                            cached_at: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                        },
                    )
                    .await;
            }

            Ok(resp)
        })
    }
}
