//! E2e tests for category: filter

use kreuzcrawl::scrape;

#[test]
fn test_filter_bm25_crawl_integration() {
    // BM25 filter works during multi-page crawl, keeping relevant pages
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.filter.remaining_contain_keyword.contains(r#"rust"#), "expected to contain: {}", r#"rust"#);
}

#[test]
fn test_filter_bm25_empty_query() {
    // BM25 filter with empty query passes all pages through
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "2", "equals assertion failed");
}

#[test]
fn test_filter_bm25_high_threshold() {
    // BM25 filter with very high threshold filters out all pages
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.filter.pages_after_filter, "0", "equals assertion failed");
}

#[test]
fn test_filter_bm25_relevant_pages() {
    // BM25 filter keeps only pages relevant to the query
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.filter.remaining_contain_keyword.contains(r#"rust"#), "expected to contain: {}", r#"rust"#);
}

#[test]
fn test_filter_bm25_threshold_zero() {
    // BM25 filter with zero threshold passes all pages
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "2", "equals assertion failed");
}

#[test]
fn test_filter_noop_crawl_all_kept() {
    // NoopFilter keeps all pages during a multi-page crawl
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.filter.pages_after_filter, "3", "equals assertion failed");
}

#[test]
fn test_filter_noop_passes_all() {
    // No content filter passes all crawled pages through
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "3", "equals assertion failed");
}

