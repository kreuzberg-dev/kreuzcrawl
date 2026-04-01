//! Integration tests for EventEmitter: verifying lifecycle events during a crawl.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use kreuzcrawl::{
    CompleteEvent, CrawlConfig, CrawlEngine, ErrorEvent, EventEmitter, NoopRateLimiter, PageEvent,
};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Shared state that can be inspected after the crawl completes.
#[derive(Debug, Default, Clone)]
struct SharedState {
    pages: Arc<Mutex<Vec<String>>>,
    discovered: Arc<Mutex<Vec<String>>>,
    completed: Arc<Mutex<bool>>,
}

/// An EventEmitter that records events into shared state.
#[derive(Debug)]
struct RecordingEmitter {
    state: SharedState,
}

#[async_trait]
impl EventEmitter for RecordingEmitter {
    async fn on_page(&self, event: &PageEvent) {
        self.state.pages.lock().unwrap().push(event.url.clone());
    }

    async fn on_error(&self, _event: &ErrorEvent) {}

    async fn on_complete(&self, _event: &CompleteEvent) {
        *self.state.completed.lock().unwrap() = true;
    }

    async fn on_discovered(&self, url: &str, _depth: usize) {
        self.state.discovered.lock().unwrap().push(url.to_owned());
    }
}

#[tokio::test]
async fn test_emitter_receives_events() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/p1\">Link</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/p1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>P1</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let state = SharedState::default();
    let emitter = RecordingEmitter {
        state: state.clone(),
    };
    let config = CrawlConfig {
        max_depth: Some(1),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .event_emitter(emitter)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    engine.crawl(&mock.uri()).await.unwrap();

    let pages = state.pages.lock().unwrap();
    assert!(
        pages.len() >= 2,
        "should emit at least 2 page events, got {}",
        pages.len()
    );
    let discovered = state.discovered.lock().unwrap();
    assert!(!discovered.is_empty(), "should emit discovered events");
    assert!(*state.completed.lock().unwrap(), "on_complete should fire");
}
