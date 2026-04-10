//! E2e tests for category: cache

use kreuzcrawl::scrape;

#[test]
fn test_cache_basic() {
    // Crawling with disk cache enabled succeeds without errors
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

