//! E2e tests for category: stream

use kreuzcrawl::scrape;

#[test]
fn test_crawl_stream_events() {
    // Crawl stream produces page and complete events
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.stream.event_count_min >= 4_f64, "expected >= 4");
    assert_eq!(result.stream.has_page_event, "true", "equals assertion failed");
    assert_eq!(result.stream.has_complete_event, "true", "equals assertion failed");
}

#[test]
fn test_stream_depth_crawl() {
    // Stream produces events for multi-depth crawl with link following
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.stream.event_count_min >= 5_f64, "expected >= 5");
    assert_eq!(result.stream.has_page_event, "true", "equals assertion failed");
    assert_eq!(result.stream.has_complete_event, "true", "equals assertion failed");
}

#[test]
fn test_stream_with_error_event() {
    // Stream emits page and complete events even when some pages fail
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.stream.has_page_event, "true", "equals assertion failed");
    assert_eq!(result.stream.has_complete_event, "true", "equals assertion failed");
    assert!(result.stream.event_count_min >= 2_f64, "expected >= 2");
}

