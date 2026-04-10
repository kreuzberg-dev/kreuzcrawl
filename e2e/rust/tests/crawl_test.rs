//! E2e tests for category: crawl

use kreuzcrawl::scrape;

#[test]
fn test_content_binary_skip() {
    // Skips image and video content types gracefully
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.content.was_skipped, "true", "equals assertion failed");
}

#[test]
fn test_content_pdf_link_skip() {
    // Encounters PDF link and skips or marks as document type
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.content.was_skipped, "true", "equals assertion failed");
}

#[test]
fn test_crawl_concurrent_depth() {
    // Concurrent crawl respects max_depth limit
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "3", "equals assertion failed");
    assert_eq!(result.stayed_on_domain, "true", "equals assertion failed");
}

#[test]
fn test_crawl_concurrent_limit() {
    // Respects max concurrent requests limit during crawl
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "5", "equals assertion failed");
}

#[test]
fn test_crawl_concurrent_max_pages() {
    // Concurrent crawl respects max_pages budget
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() <= 3_f64, "expected <= 3");
}

#[test]
fn test_crawl_custom_headers() {
    // Sends custom headers on all crawl requests
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_depth_one() {
    // Follows links one level deep from start page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "3", "equals assertion failed");
    assert_eq!(result.stayed_on_domain, "true", "equals assertion failed");
}

#[test]
fn test_crawl_depth_priority() {
    // Crawls in breadth-first order, processing depth-0 pages before depth-1
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "4", "equals assertion failed");
}

#[test]
fn test_crawl_depth_two() {
    // Crawls 3 levels deep (depth 0, 1, 2)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "3", "equals assertion failed");
    assert!(result.pages.len() >= 3_f64, "expected >= 3");
}

#[test]
fn test_crawl_depth_two_chain() {
    // Depth=2 crawl follows a chain of links across three levels
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "3", "equals assertion failed");
}

#[test]
fn test_crawl_double_slash_normalization() {
    // Normalizes double slashes in URL paths (//page to /page)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.unique_urls.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_empty_page_no_links() {
    // Crawl completes when child page has no outgoing links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_exclude_path_pattern() {
    // Skips URLs matching the exclude path pattern
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_external_links_ignored() {
    // External links are discovered but not followed when stay_on_domain is true
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
    assert_eq!(result.stayed_on_domain, "true", "equals assertion failed");
}

#[test]
fn test_crawl_fragment_stripping() {
    // Strips #fragment from URLs for deduplication
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.unique_urls.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_include_path_pattern() {
    // Only follows URLs matching the include path pattern
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_max_depth_zero() {
    // max_depth=0 crawls only the seed page with no link following
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "1", "equals assertion failed");
    assert!(result.pages.len() <= 1_f64, "expected <= 1");
}

#[test]
fn test_crawl_max_pages() {
    // Stops crawling at page budget limit
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() <= 3_f64, "expected <= 3");
}

#[test]
fn test_crawl_mixed_content_types() {
    // Crawl handles links to non-HTML content types gracefully
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() >= 2_f64, "expected >= 2");
}

#[test]
fn test_crawl_multiple_redirects_in_traversal() {
    // Multiple linked pages with redirects are handled during crawl traversal
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() >= 1_f64, "expected >= 1");
}

#[test]
fn test_crawl_query_param_dedup() {
    // Deduplicates URLs with same query params in different order
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.unique_urls.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_redirect_in_traversal() {
    // Links that redirect are followed during crawl traversal
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() >= 1_f64, "expected >= 1");
}

#[test]
fn test_crawl_self_link_no_loop() {
    // Page linking to itself does not cause infinite crawl loop
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_single_page_no_links() {
    // Crawling a page with no links returns only the seed page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "1", "equals assertion failed");
}

#[test]
fn test_crawl_stay_on_domain() {
    // Does not follow external links when stay_on_domain is true
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
    assert_eq!(result.stayed_on_domain, "true", "equals assertion failed");
}

#[test]
fn test_crawl_subdomain_exclusion() {
    // Stays on exact domain and skips subdomain links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.pages.len(), "2", "equals assertion failed");
    assert_eq!(result.stayed_on_domain, "true", "equals assertion failed");
}

#[test]
fn test_crawl_subdomain_inclusion() {
    // Crawls subdomains when allow_subdomains is enabled
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() >= 2_f64, "expected >= 2");
}

#[test]
fn test_crawl_trailing_slash_dedup() {
    // Deduplicates /page and /page/ as the same URL
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.unique_urls.len(), "2", "equals assertion failed");
}

#[test]
fn test_crawl_url_deduplication() {
    // Deduplicates URLs that differ only by fragment or query params
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.pages.len() <= 2_f64, "expected <= 2");
}

