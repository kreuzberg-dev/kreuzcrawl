//! E2e tests for category: batch

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_scrape_batch_basic() {
    // Batch scrape of multiple URLs all succeeding
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.batch.completed_count, "3", "equals assertion failed");
    assert_eq!(result.batch.failed_count, "0", "equals assertion failed");
    assert_eq!(result.batch.total_count, "3", "equals assertion failed");
}

#[test]
fn test_scrape_batch_partial_failure() {
    // Batch scrape with one URL failing returns partial results
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.batch.completed_count, "2", "equals assertion failed");
    assert_eq!(result.batch.failed_count, "1", "equals assertion failed");
    assert_eq!(result.batch.total_count, "3", "equals assertion failed");
}

#[test]
fn test_scrape_batch_progress() {
    // Batch scrape results include specific URL
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.batch.total_count, "2", "equals assertion failed");
    assert!(result.batch.results.contains(r#"/target"#), "expected to contain: {}", r#"/target"#);
}

