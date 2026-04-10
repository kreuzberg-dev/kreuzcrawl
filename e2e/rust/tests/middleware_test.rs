//! E2e tests for category: middleware

use kreuzcrawl::scrape;

#[test]
fn test_middleware_engine_crawl_with_defaults() {
    // Engine crawl with default middleware chain produces correct multi-page results
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "3", "equals assertion failed");
    assert!(result.crawl.min_pages >= 3_f64, "expected >= 3");
}

#[test]
fn test_middleware_noop_no_effect() {
    // Default middleware chain does not affect normal scraping
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(metadata_title, r#"Middleware Test"#, "equals assertion failed");
}

