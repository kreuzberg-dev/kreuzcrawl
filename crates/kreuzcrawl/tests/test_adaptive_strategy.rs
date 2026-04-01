//! Integration tests for AdaptiveStrategy: verifying early termination on content saturation.

use kreuzcrawl::{AdaptiveStrategy, CrawlConfig, CrawlEngine, NoopRateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_adaptive_stops_on_similar_content() {
    let mock = MockServer::start().await;

    // Root links to 20 pages.
    let mut links = String::new();
    for i in 0..20 {
        links.push_str(&format!("<a href=\"/page{i}\">P{i}</a> "));
    }
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(format!("<html><body>{links}</body></html>"))
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    // All 20 pages have identical content (saturated).
    for i in 0..20 {
        Mock::given(method("GET"))
            .and(path(format!("/page{i}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(
                        "<html><body>The same repeated content on every single page with identical words</body></html>",
                    )
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }

    let config = CrawlConfig {
        max_depth: Some(1),
        max_pages: Some(20),
        max_concurrent: Some(1),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .strategy(AdaptiveStrategy::new(3, 0.02))
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.crawl(&mock.uri()).await.unwrap();

    // Should stop before visiting all 20 pages due to saturation.
    assert!(
        result.pages.len() < 20,
        "adaptive should stop early, got {} pages",
        result.pages.len()
    );
    assert!(
        result.pages.len() >= 4,
        "should crawl at least a few pages, got {}",
        result.pages.len()
    );
}
