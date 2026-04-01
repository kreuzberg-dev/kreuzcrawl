//! Integration tests for batch_crawl_stream: streaming events from multiple seed URLs.

use kreuzcrawl::{CrawlConfig, CrawlEngine, CrawlEvent, NoopRateLimiter};
use tokio_stream::StreamExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_batch_crawl_stream_produces_events() {
    let mock = MockServer::start().await;
    for name in ["a", "b"] {
        Mock::given(method("GET"))
            .and(path(format!("/{name}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(format!("<html><body>{name}</body></html>"))
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }

    let config = CrawlConfig {
        max_depth: Some(0),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let urls = vec![format!("{}/a", mock.uri()), format!("{}/b", mock.uri())];
    let url_refs: Vec<&str> = urls.iter().map(|s| s.as_str()).collect();

    let stream = engine.batch_crawl_stream(&url_refs);
    let events: Vec<CrawlEvent> = stream.collect().await;

    let page_events = events
        .iter()
        .filter(|e| matches!(e, CrawlEvent::Page(_)))
        .count();
    let complete_events = events
        .iter()
        .filter(|e| matches!(e, CrawlEvent::Complete { .. }))
        .count();

    assert!(
        page_events >= 2,
        "should have at least 2 page events, got {page_events}"
    );
    assert!(
        complete_events >= 1,
        "should have at least 1 complete event, got {complete_events}"
    );
}
