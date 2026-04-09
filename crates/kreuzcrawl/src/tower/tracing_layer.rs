//! Tracing/telemetry layer for the Tower service stack.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tower::{Layer, Service};
use tracing::Instrument;

use super::types::{CrawlRequest, CrawlResponse};
use crate::error::CrawlError;

/// Tower layer that emits `tracing` spans for each crawl request.
pub struct CrawlTracingLayer;

impl CrawlTracingLayer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CrawlTracingLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Clone> Layer<S> for CrawlTracingLayer {
    type Service = CrawlTracingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CrawlTracingService { inner }
    }
}

/// Tower service that wraps each request in a `tracing` span with HTTP metadata.
#[derive(Clone)]
pub struct CrawlTracingService<S> {
    inner: S,
}

impl<S> Service<CrawlRequest> for CrawlTracingService<S>
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
        let url = req.url.clone();

        let span = tracing::info_span!(
            "crawl.fetch",
            otel.kind = "client",
            url.full = %url,
            server.address = %domain,
            http.request.method = "GET",
            http.response.status_code = tracing::field::Empty,
            http.response.body.size = tracing::field::Empty,
        );

        let mut inner = self.inner.clone();
        std::mem::swap(&mut self.inner, &mut inner);

        Box::pin(
            async move {
                let resp = inner.call(req).await?;
                tracing::info!(status = resp.status, body_size = resp.body.len(), "fetch complete");
                Ok(resp)
            }
            .instrument(span),
        )
    }
}
