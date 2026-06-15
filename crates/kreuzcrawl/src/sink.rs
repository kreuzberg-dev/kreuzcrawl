//! Pluggable event sink for streaming crawl events.
//!
//! Allows consumers to subscribe to [`CrawlEvent`]s without depending on
//! kreuzcrawl's internal channel types. Implementations should be cheap on the
//! hot path; defer heavy work (message-bus publishes, external API calls) to dedicated tasks.

use async_trait::async_trait;

use crate::types::CrawlEvent;

/// Pluggable consumer of crawl events.
///
/// Implementations should be cheap on the hot path — avoid blocking operations.
/// For expensive work (message-bus publishes, external API calls), spawn
/// a dedicated task and return immediately.
#[async_trait]
pub trait EventSink: Send + Sync + 'static {
    /// Emit a crawl event. Must not block the caller.
    async fn emit(&self, event: CrawlEvent);
}

/// Default sink that logs events via `tracing::info!`.
///
/// Use as a fallback when no external consumer is configured.
#[derive(Default, Clone)]
pub struct TracingEventSink;

#[async_trait]
impl EventSink for TracingEventSink {
    async fn emit(&self, event: CrawlEvent) {
        tracing::info!(target: "kreuzcrawl.event", event = ?event, "crawl_event");
    }
}

/// Fan-out sink that emits to multiple inner sinks concurrently.
///
/// Each emit call fans out to all registered sinks in parallel.
pub struct MultiEventSink {
    sinks: Vec<std::sync::Arc<dyn EventSink>>,
}

impl MultiEventSink {
    /// Create a new multi-sink from a list of sinks.
    pub fn new(sinks: Vec<std::sync::Arc<dyn EventSink>>) -> Self {
        Self { sinks }
    }

    /// Check if this multi-sink has any registered sinks.
    pub fn is_empty(&self) -> bool {
        self.sinks.is_empty()
    }

    /// Return the number of registered sinks.
    pub fn len(&self) -> usize {
        self.sinks.len()
    }
}

#[async_trait]
impl EventSink for MultiEventSink {
    async fn emit(&self, event: CrawlEvent) {
        let futures: Vec<_> = self
            .sinks
            .iter()
            .map(|s| {
                let s = s.clone();
                let e = event.clone();
                async move { s.emit(e).await }
            })
            .collect();
        futures::future::join_all(futures).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct VecSink {
        events: std::sync::Arc<Mutex<Vec<CrawlEvent>>>,
    }

    impl VecSink {
        fn new() -> Self {
            Self {
                events: std::sync::Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn len(&self) -> usize {
            self.events.lock().unwrap().len()
        }
    }

    #[async_trait]
    impl EventSink for VecSink {
        async fn emit(&self, event: CrawlEvent) {
            self.events.lock().unwrap().push(event);
        }
    }

    #[tokio::test]
    async fn vec_sink_collects_events() {
        let sink = VecSink::new();
        let event = CrawlEvent::Complete { pages_crawled: 42 };
        sink.emit(event).await;
        assert_eq!(sink.len(), 1);
    }

    #[tokio::test]
    async fn tracing_sink_does_not_error() {
        let sink = TracingEventSink;
        let event = CrawlEvent::Complete { pages_crawled: 0 };
        sink.emit(event).await;
    }

    #[tokio::test]
    async fn multi_event_sink_fans_out() {
        let a = std::sync::Arc::new(VecSink::new());
        let b = std::sync::Arc::new(VecSink::new());
        let multi = MultiEventSink::new(vec![a.clone(), b.clone()]);

        let event = CrawlEvent::Complete { pages_crawled: 10 };
        multi.emit(event).await;

        assert_eq!(a.len(), 1);
        assert_eq!(b.len(), 1);
    }

    #[tokio::test]
    async fn multi_event_sink_empty() {
        let multi = MultiEventSink::new(vec![]);
        assert!(multi.is_empty());
        assert_eq!(multi.len(), 0);

        let event = CrawlEvent::Complete { pages_crawled: 0 };
        multi.emit(event).await;
    }
}
