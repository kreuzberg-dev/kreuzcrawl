//! Crawl middleware implementations.

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
