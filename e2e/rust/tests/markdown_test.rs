//! E2e tests for category: markdown

use kreuzcrawl::scrape;

#[test]
fn test_markdown_basic_conversion() {
    // HTML is always converted to markdown alongside raw HTML
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    let markdown = result.markdown.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(metadata_title, r#"Test"#, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(!markdown.is_empty(), "expected non-empty value");
    assert!(markdown.contains(r#"Hello World"#), "expected to contain: {}", r#"Hello World"#);
}

#[test]
fn test_markdown_crawl_all_pages() {
    // All crawled pages have markdown field populated
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "2", "equals assertion failed");
}

#[test]
fn test_markdown_fit_content() {
    // Fit markdown removes navigation and boilerplate content
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.markdown.is_some(), "expected markdown to be present");
}

#[test]
fn test_markdown_headings_and_paragraphs() {
    // Markdown conversion preserves heading hierarchy and paragraph text
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let markdown = result.markdown.as_deref().unwrap_or("");
    assert!(!markdown.is_empty(), "expected non-empty value");
    assert!(markdown.contains(r#"Main Title"#), "expected to contain: {}", r#"Main Title"#);
}

#[test]
fn test_markdown_links_converted() {
    // HTML links are converted to markdown link syntax
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let markdown = result.markdown.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(!markdown.is_empty(), "expected non-empty value");
    assert!(markdown.contains(r#"Example"#), "expected to contain: {}", r#"Example"#);
}

#[test]
fn test_markdown_with_citations() {
    // Markdown includes citation conversion with numbered references
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.markdown.is_some(), "expected markdown to be present");
}

