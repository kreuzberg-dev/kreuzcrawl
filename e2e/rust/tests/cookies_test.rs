//! E2e tests for category: cookies

use kreuzcrawl::scrape;

#[test]
fn test_cookies_per_domain() {
    // Isolates cookies per domain during crawl
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.cookies.len(), "1", "equals assertion failed");
    assert!(result.cookies.contains(r#"domain_cookie"#), "expected to contain: {}", r#"domain_cookie"#);
}

#[test]
fn test_cookies_persistence() {
    // Maintains cookies across multiple crawl requests
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.cookies.contains(r#"session"#), "expected to contain: {}", r#"session"#);
}

#[test]
fn test_cookies_set_cookie_response() {
    // Respects Set-Cookie header from server responses
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.cookies.contains(r#"tracking"#), "expected to contain: {}", r#"tracking"#);
}

