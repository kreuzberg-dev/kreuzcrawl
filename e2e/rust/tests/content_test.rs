//! E2e tests for category: content

use kreuzcrawl::scrape;

#[test]
fn test_content_204_no_content() {
    // Handles 204 No Content response gracefully
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "204", "equals assertion failed");
    assert!(result.html.is_empty(), "expected empty value");
}

#[test]
fn test_content_charset_iso8859() {
    // Handles ISO-8859-1 encoded page correctly
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.content.detected_charset, r#"iso-8859-1"#, "equals assertion failed");
}

#[test]
fn test_content_empty_body() {
    // Handles 200 response with empty body gracefully
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

#[test]
fn test_content_gzip_compressed() {
    // Handles response with Accept-Encoding gzip negotiation
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert_eq!(result.status_code, "200", "equals assertion failed");
}

#[test]
fn test_content_large_page_limit() {
    // Respects max body size limit and truncates or skips oversized pages
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.content.body_size < 1025_f64, "expected < 1025");
}

#[test]
fn test_content_main_only() {
    // Extracts only main content area, excluding nav, sidebar, footer
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.content.main_content_only, "true", "equals assertion failed");
}

#[test]
fn test_content_pdf_no_extension() {
    // Detects PDF content by Content-Type header when URL has no .pdf extension
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.content.is_pdf, "true", "equals assertion failed");
}

#[test]
fn test_content_remove_tags() {
    // Removes specified HTML elements by CSS selector before processing
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}

#[test]
fn test_content_utf8_bom() {
    // Handles UTF-8 content with BOM marker correctly
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.content.detected_charset, r#"utf-8"#, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}

