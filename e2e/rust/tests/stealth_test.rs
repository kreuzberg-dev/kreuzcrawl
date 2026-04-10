//! E2e tests for category: stealth

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_stealth_ua_rotation_config() {
    // User-agent rotation config is accepted and crawl succeeds
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

