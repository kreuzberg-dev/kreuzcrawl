//! E2e tests for category: map

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_map_discover_urls() {
    // Discovers all URLs on a site without fetching full content
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.urls.len() >= 3_f64, "expected >= 3");
}

#[test]
fn test_map_exclude_patterns() {
    // Excludes URLs matching patterns from URL map
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "1", "equals assertion failed");
}

#[test]
fn test_map_include_subdomains() {
    // Includes subdomain URLs in URL map discovery
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.urls.len() >= 2_f64, "expected >= 2");
    assert!(result.urls.contains(r#"blog.example.com"#), "expected to contain: {}", r#"blog.example.com"#);
}

#[test]
fn test_map_large_sitemap() {
    // Handles large sitemap with 100+ URLs
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.urls.len() >= 100_f64, "expected >= 100");
}

#[test]
fn test_map_limit_pagination() {
    // Limits map result count to specified maximum
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.urls.len() <= 5_f64, "expected <= 5");
}

#[test]
fn test_map_search_filter() {
    // Filters map results by search keyword
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert!(result.urls.len() >= 2_f64, "expected >= 2");
    assert!(result.urls.contains(r#"blog"#), "expected to contain: {}", r#"blog"#);
}

