//! User-Agent rotation layer for the Tower service stack.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

use tower::{Layer, Service};

use super::types::{CrawlRequest, CrawlResponse};
use crate::error::CrawlError;

/// Tower layer that rotates User-Agent headers across requests.
#[derive(Clone)]
pub struct UaRotationLayer {
    user_agents: Arc<Vec<String>>,
    index: Arc<AtomicUsize>,
}

impl UaRotationLayer {
    pub fn new(user_agents: Vec<String>) -> Self {
        Self {
            user_agents: Arc::new(user_agents),
            index: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<S: Clone> Layer<S> for UaRotationLayer {
    type Service = UaRotationService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        UaRotationService {
            inner,
            user_agents: self.user_agents.clone(),
            index: self.index.clone(),
        }
    }
}

/// Tower service that injects a rotating User-Agent header into each request.
#[derive(Clone)]
pub struct UaRotationService<S> {
    inner: S,
    user_agents: Arc<Vec<String>>,
    index: Arc<AtomicUsize>,
}

impl<S> Service<CrawlRequest> for UaRotationService<S>
where
    S: Service<CrawlRequest, Response = CrawlResponse, Error = CrawlError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = CrawlResponse;
    type Error = CrawlError;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: CrawlRequest) -> Self::Future {
        if !self.user_agents.is_empty() {
            let idx = self.index.fetch_add(1, Ordering::Relaxed) % self.user_agents.len();
            req.headers
                .insert("user-agent".to_owned(), self.user_agents[idx].clone());
        }
        self.inner.call(req)
    }
}
