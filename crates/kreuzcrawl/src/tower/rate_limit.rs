//! Per-domain rate limiting layer for the Tower service stack.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use tower::{Layer, Service};

use super::types::{CrawlRequest, CrawlResponse};
use crate::error::CrawlError;
use crate::traits::RateLimiter;

/// Tower layer that applies per-domain rate limiting using a [`RateLimiter`].
pub struct PerDomainRateLimitLayer {
    rate_limiter: Arc<dyn RateLimiter>,
}

impl PerDomainRateLimitLayer {
    pub fn new(rate_limiter: Arc<dyn RateLimiter>) -> Self {
        Self { rate_limiter }
    }
}

impl<S: Clone> Layer<S> for PerDomainRateLimitLayer {
    type Service = PerDomainRateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PerDomainRateLimitService {
            inner,
            rate_limiter: self.rate_limiter.clone(),
        }
    }
}

/// Tower service that enforces per-domain rate limits before forwarding requests.
#[derive(Clone)]
pub struct PerDomainRateLimitService<S> {
    inner: S,
    rate_limiter: Arc<dyn RateLimiter>,
}

impl<S> Service<CrawlRequest> for PerDomainRateLimitService<S>
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
        let domain = req.domain().unwrap_or_default();
        let rate_limiter = self.rate_limiter.clone();
        let mut inner = self.inner.clone();
        // Swap to preserve readiness (standard Tower pattern)
        std::mem::swap(&mut self.inner, &mut inner);

        Box::pin(async move {
            // Acquire rate limit permit
            if !domain.is_empty() {
                rate_limiter.acquire(&domain).await?;
            }

            // Forward to inner service
            let resp = inner.call(req).await?;

            // Record response for adaptive backoff
            if !domain.is_empty() {
                rate_limiter.record_response(&domain, resp.status).await?;
            }

            Ok(resp)
        })
    }
}
