//! E2e tests for category: cookies

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_cookies_per_domain() {
    // Isolates cookies per domain during crawl
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = format!("{}/fixtures/{}", std::env::var("MOCK_SERVER_URL").expect("MOCK_SERVER_URL not set"), "cookies_per_domain");
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'cookies.length' not available on result type
    // skipped: field 'cookies' not available on result type
}

#[tokio::test]
async fn test_cookies_persistence() {
    // Maintains cookies across multiple crawl requests
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = format!("{}/fixtures/{}", std::env::var("MOCK_SERVER_URL").expect("MOCK_SERVER_URL not set"), "cookies_persistence");
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'cookies' not available on result type
}

#[tokio::test]
async fn test_cookies_set_cookie_response() {
    // Respects Set-Cookie header from server responses
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = format!("{}/fixtures/{}", std::env::var("MOCK_SERVER_URL").expect("MOCK_SERVER_URL not set"), "cookies_set_cookie_response");
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'cookies' not available on result type
}
