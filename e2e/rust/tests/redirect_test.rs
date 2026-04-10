//! E2e tests for category: redirect

use kreuzcrawl::scrape;

#[test]
fn test_redirect_301_permanent() {
    // Follows 301 permanent redirect and returns final page content
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/target"#), "expected to contain: {}", r#"/target"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_302_found() {
    // Follows 302 Found redirect correctly
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/found-target"#), "expected to contain: {}", r#"/found-target"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_303_see_other() {
    // Follows 303 See Other redirect (method changes to GET)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/see-other"#), "expected to contain: {}", r#"/see-other"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_307_temporary() {
    // Follows 307 Temporary Redirect (preserves method)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/temp-target"#), "expected to contain: {}", r#"/temp-target"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_308_permanent() {
    // Follows 308 Permanent Redirect (preserves method)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/perm-target"#), "expected to contain: {}", r#"/perm-target"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_chain() {
    // Follows a chain of redirects (301 -> 302 -> 200)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/step2"#), "expected to contain: {}", r#"/step2"#);
    assert_eq!(result.redirect_count, "2", "equals assertion failed");
}

#[test]
fn test_redirect_cross_domain() {
    // Reports cross-domain redirect target without following to external domain
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/external-redirect"#), "expected to contain: {}", r#"/external-redirect"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_loop() {
    // Detects redirect loop (A -> B -> A) and returns error
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.is_error, "true", "equals assertion failed");
}

#[test]
fn test_redirect_max_exceeded() {
    // Aborts when redirect count exceeds max_redirects limit
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.is_error, "true", "equals assertion failed");
}

#[test]
fn test_redirect_meta_refresh() {
    // Follows HTML meta-refresh redirect to target page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/target"#), "expected to contain: {}", r#"/target"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_refresh_header() {
    // Handles HTTP Refresh header redirect
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/refreshed"#), "expected to contain: {}", r#"/refreshed"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
}

#[test]
fn test_redirect_to_404() {
    // Redirect target returns 404 Not Found
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.final_url.contains(r#"/gone"#), "expected to contain: {}", r#"/gone"#);
    assert_eq!(result.redirect_count, "1", "equals assertion failed");
    assert_eq!(result.is_error, "true", "equals assertion failed");
}

