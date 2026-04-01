//! A no-op event emitter that silently discards all events.

use async_trait::async_trait;

use crate::traits::{CompleteEvent, ErrorEvent, EventEmitter, PageEvent};

/// An event emitter that does nothing -- all events are silently discarded.
#[derive(Debug, Clone, Default)]
pub struct NoopEmitter;

#[async_trait]
impl EventEmitter for NoopEmitter {
    async fn on_page(&self, _event: &PageEvent) {}
    async fn on_error(&self, _event: &ErrorEvent) {}
    async fn on_complete(&self, _event: &CompleteEvent) {}
    async fn on_discovered(&self, _url: &str, _depth: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{CompleteEvent, ErrorEvent, PageEvent};

    #[tokio::test]
    async fn test_noop_emitter_doesnt_panic() {
        let e = NoopEmitter;
        e.on_page(&PageEvent {
            url: "test".into(),
            status_code: 200,
            depth: 0,
        })
        .await;
        e.on_error(&ErrorEvent {
            url: "test".into(),
            error: "err".into(),
        })
        .await;
        e.on_complete(&CompleteEvent { pages_crawled: 5 }).await;
        e.on_discovered("http://test.com", 1).await;
    }
}
