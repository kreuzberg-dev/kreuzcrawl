//! E2e tests for category: links

use kreuzcrawl::scrape;

#[test]
fn test_links_anchor_fragment() {
    // Identifies fragment-only links as anchor type
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"anchor"#), "expected to contain: {}", r#"anchor"#);
}

#[test]
fn test_links_base_tag() {
    // Resolves relative URLs using base tag href
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 2_f64, "expected > 2");
    assert!(result.links.get("").map(|s| s.as_str()).url.contains(r#"example.com"#), "expected to contain: {}", r#"example.com"#);
}

#[test]
fn test_links_document_types() {
    // Detects PDF, DOCX, XLSX links as document type
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"document"#), "expected to contain: {}", r#"document"#);
}

#[test]
fn test_links_empty_href() {
    // Handles empty href attributes without errors
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 0_f64, "expected > 0");
    assert!(result.links.get("").map(|s| s.as_str()).url.contains(r#"/valid"#), "expected to contain: {}", r#"/valid"#);
}

#[test]
fn test_links_internal_external_classification() {
    // Correctly classifies internal vs external links by domain
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 4_f64, "expected > 4");
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"internal"#), "expected to contain: {}", r#"internal"#);
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"external"#), "expected to contain: {}", r#"external"#);
}

#[test]
fn test_links_mailto_javascript_skip() {
    // Skips mailto:, javascript:, and tel: scheme links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 0_f64, "expected > 0");
    assert!(!result.links.get("").map(|s| s.as_str()).url.contains(r#"mailto:"#), "expected NOT to contain: {}", r#"mailto:"#);
}

#[test]
fn test_links_protocol_relative() {
    // Handles protocol-relative URLs (//example.com) correctly
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 1_f64, "expected > 1");
    assert!(!result.links.get("").map(|s| s.as_str()).protocol_relative.is_empty(), "expected non-empty value");
}

#[test]
fn test_links_rel_attributes() {
    // Preserves rel=nofollow and rel=canonical attributes
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 0_f64, "expected > 0");
}

#[test]
fn test_links_relative_parent() {
    // Resolves ../ and ./ relative parent path links correctly
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(result.links.len() > 3_f64, "expected > 3");
}

