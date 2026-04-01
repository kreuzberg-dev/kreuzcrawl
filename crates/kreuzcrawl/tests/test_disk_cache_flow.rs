//! Integration tests for DiskCache: verifying cache prevents re-fetching across crawl runs.

use kreuzcrawl::{CrawlConfig, CrawlEngine, DiskCache, NoopRateLimiter};
use tempfile::tempdir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_disk_cache_prevents_refetch() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Cached Content</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let dir = tempdir().unwrap();
    let cache = DiskCache::new(dir.path(), 3600, 1000).unwrap();

    let config = CrawlConfig {
        max_depth: Some(0),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config.clone())
        .cache(cache)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    // First crawl -- hits server.
    let r1 = engine.crawl(&mock.uri()).await.unwrap();
    assert!(!r1.pages.is_empty());
    assert!(r1.pages[0].html.contains("Cached Content"));

    let requests_after_first = mock.received_requests().await.unwrap().len();

    // Second crawl with fresh engine but same cache dir -- should serve from cache.
    let cache2 = DiskCache::new(dir.path(), 3600, 1000).unwrap();
    let engine2 = CrawlEngine::builder()
        .config(config)
        .cache(cache2)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let r2 = engine2.crawl(&mock.uri()).await.unwrap();
    assert!(!r2.pages.is_empty());
    assert!(r2.pages[0].html.contains("Cached Content"));

    let requests_after_second = mock.received_requests().await.unwrap().len();
    let second_crawl_requests = requests_after_second - requests_after_first;

    // The second crawl should make fewer new requests than the first crawl
    // because the page fetch is served from cache.
    assert!(
        second_crawl_requests < requests_after_first,
        "second crawl should make fewer requests ({}) than first ({}), proving the cache avoided a re-fetch",
        second_crawl_requests,
        requests_after_first
    );
}

#[tokio::test]
async fn test_disk_cache_in_crawl() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/p\">Link</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/p"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Page</body></html>")
                .append_header("content-type", "text/html"),
        )
        .expect(1) // Only 1 request.
        .mount(&mock)
        .await;

    let dir = tempdir().unwrap();
    let cache = DiskCache::new(dir.path(), 3600, 1000).unwrap();

    let config = CrawlConfig {
        max_depth: Some(1),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config.clone())
        .cache(cache)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    // First crawl.
    engine.crawl(&mock.uri()).await.unwrap();

    // Second crawl -- /p should be cached.
    let cache2 = DiskCache::new(dir.path(), 3600, 1000).unwrap();
    let engine2 = CrawlEngine::builder()
        .config(config)
        .cache(cache2)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    engine2.crawl(&mock.uri()).await.unwrap();
    // wiremock's expect(1) on /p verifies second crawl used cache.
}
