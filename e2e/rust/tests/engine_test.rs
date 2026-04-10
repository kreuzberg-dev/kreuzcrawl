//! E2e tests for category: engine

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_engine_batch_basic() {
    // CrawlEngine with defaults batch scrapes like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.batch.completed_count, "2", "equals assertion failed");
    assert_eq!(result.batch.total_count, "2", "equals assertion failed");
}

#[test]
fn test_engine_crawl_basic() {
    // CrawlEngine with defaults crawls multiple pages like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "3", "equals assertion failed");
    assert!(result.crawl.min_pages >= 3_f64, "expected >= 3");
}

#[test]
fn test_engine_map_basic() {
    // CrawlEngine with defaults discovers URLs like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.map.min_urls >= 2_f64, "expected >= 2");
}

#[test]
fn test_engine_scrape_basic() {
    // CrawlEngine with defaults scrapes a page identically to the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.content_type, r#"text/html"#, "equals assertion failed");
    assert_eq!(metadata_title, r#"Engine Test"#, "equals assertion failed");
    assert!(result.metadata.description_contains.contains(r#"Testing the engine"#), "expected to contain: {}", r#"Testing the engine"#);
    assert!(result.links.min_count >= 1_f64, "expected >= 1");
    assert_eq!(result.headings.h1_count, "1", "equals assertion failed");
    assert_eq!(result.headings.h1_text, r#"Hello Engine"#, "equals assertion failed");
}

#[test]
fn test_engine_stream_basic() {
    // CrawlEngine with defaults streams events like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.stream.has_page_event, "true", "equals assertion failed");
    assert_eq!(result.stream.has_complete_event, "true", "equals assertion failed");
    assert!(result.stream.event_count_min >= 3_f64, "expected >= 3");
}

