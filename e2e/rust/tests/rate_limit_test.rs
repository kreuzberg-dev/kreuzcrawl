//! E2e tests for category: rate_limit

use kreuzcrawl::scrape;

#[test]
fn test_rate_limit_basic_delay() {
    // Rate limiter adds delay between requests to the same domain
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "3", "equals assertion failed");
    assert!(result.rate_limit.min_duration_ms >= 150_f64, "expected >= 150");
}

#[test]
fn test_rate_limit_zero_no_delay() {
    // Rate limiter with zero delay does not slow crawling
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "2", "equals assertion failed");
}

