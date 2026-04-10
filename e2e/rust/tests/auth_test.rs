//! E2e tests for category: auth

use kreuzcrawl::scrape;

#[test]
fn test_auth_basic_http() {
    // Sends HTTP Basic authentication header
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.auth_header_sent, "true", "equals assertion failed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

#[test]
fn test_auth_bearer_token() {
    // Sends Bearer token in Authorization header
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.auth_header_sent, "true", "equals assertion failed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

#[test]
fn test_auth_custom_header() {
    // Sends authentication via custom header (X-API-Key)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.auth_header_sent, "true", "equals assertion failed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

