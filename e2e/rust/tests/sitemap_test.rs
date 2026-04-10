//! E2e tests for category: sitemap

use kreuzcrawl::scrape;

#[test]
fn test_sitemap_basic() {
    // Parses a standard urlset sitemap
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "4", "equals assertion failed");
    assert_eq!(result.has_lastmod, "true", "equals assertion failed");
}

#[test]
fn test_sitemap_compressed_gzip() {
    // Parses a gzip-compressed sitemap file
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "3", "equals assertion failed");
}

#[test]
fn test_sitemap_empty() {
    // Handles empty sitemap gracefully
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "0", "equals assertion failed");
}

#[test]
fn test_sitemap_from_robots_txt() {
    // Discovers sitemap via robots.txt Sitemap directive
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "4", "equals assertion failed");
}

#[test]
fn test_sitemap_index() {
    // Follows sitemap index to discover child sitemaps
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "3", "equals assertion failed");
}

#[test]
fn test_sitemap_lastmod_filter() {
    // Filters sitemap URLs by lastmod date
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "4", "equals assertion failed");
    assert_eq!(result.has_lastmod, "true", "equals assertion failed");
}

#[test]
fn test_sitemap_only_mode() {
    // Uses sitemap URLs exclusively without following page links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "4", "equals assertion failed");
}

#[test]
fn test_sitemap_xhtml_links() {
    // Parses sitemap with XHTML namespace alternate links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.urls.len(), "2", "equals assertion failed");
    assert_eq!(result.has_lastmod, "false", "equals assertion failed");
}

