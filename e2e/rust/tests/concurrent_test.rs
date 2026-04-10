//! E2e tests for category: concurrent

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_concurrent_basic() {
    // Concurrent crawling fetches all pages with max_concurrent workers
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "6", "equals assertion failed");
    assert!(result.pages.len() >= 6_f64, "expected >= 6");
}

#[test]
fn test_concurrent_depth_two_fan_out() {
    // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "4", "equals assertion failed");
}

#[test]
fn test_concurrent_max_pages_exact() {
    // Concurrent crawling does not exceed max_pages limit even with high concurrency
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.pages.len() <= 3_f64, "expected <= 3");
}

#[test]
fn test_concurrent_partial_errors() {
    // Concurrent crawl handles partial failures gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.pages.len() >= 2_f64, "expected >= 2");
}

#[test]
fn test_concurrent_respects_max_pages() {
    // Concurrent crawling respects max_pages limit
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.pages.len() <= 3_f64, "expected <= 3");
}

