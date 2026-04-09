//! Integration tests for CrawlStore: verifying store callbacks during a crawl.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use kreuzcrawl::{
    CrawlConfig, CrawlEngine, CrawlError, CrawlPageResult, CrawlStats, CrawlStore, NoopRateLimiter, ScrapeResult,
};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Shared state that can be inspected after the crawl completes.
#[derive(Debug, Default, Clone)]
struct SharedState {
    pages: Arc<Mutex<Vec<String>>>,
    errors: Arc<Mutex<Vec<String>>>,
    completed: Arc<Mutex<bool>>,
}

/// A CrawlStore that records calls into shared state.
#[derive(Debug)]
struct RecordingStore {
    state: SharedState,
}

#[async_trait]
impl CrawlStore for RecordingStore {
    async fn store_page(&self, _url: &str, _result: &ScrapeResult) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn store_crawl_page(&self, url: &str, _result: &CrawlPageResult) -> Result<(), CrawlError> {
        self.state.pages.lock().unwrap().push(url.to_owned());
        Ok(())
    }

    async fn store_error(&self, url: &str, _error: &CrawlError) -> Result<(), CrawlError> {
        self.state.errors.lock().unwrap().push(url.to_owned());
        Ok(())
    }

    async fn on_complete(&self, _stats: &CrawlStats) -> Result<(), CrawlError> {
        *self.state.completed.lock().unwrap() = true;
        Ok(())
    }
}

#[tokio::test]
async fn test_store_receives_crawl_pages() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/page1\">Link</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/page1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Page 1</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let state = SharedState::default();
    let store = RecordingStore { state: state.clone() };
    let config = CrawlConfig {
        max_depth: Some(1),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .store(store)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    engine.crawl(&mock.uri()).await.unwrap();

    let pages = state.pages.lock().unwrap();
    assert!(
        pages.len() >= 2,
        "store should receive at least 2 pages, got {}",
        pages.len()
    );
    assert!(*state.completed.lock().unwrap(), "on_complete should be called");
}
