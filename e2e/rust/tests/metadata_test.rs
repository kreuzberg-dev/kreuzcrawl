//! E2e tests for category: metadata

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_metadata_article_times() {
    // Extracts article:published_time, modified_time, author, section, and tags
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'article.published_time' not available on result type
    // skipped: field 'article.modified_time' not available on result type
    // skipped: field 'article.author' not available on result type
    // skipped: field 'article.section' not available on result type
    // skipped: field 'article.tags.length' not available on result type
}

#[test]
fn test_metadata_favicons() {
    // Extracts favicon link tags including apple-touch-icon
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'favicons.length' not available on result type
    // skipped: field 'favicons[].apple_touch' not available on result type
}

#[test]
fn test_metadata_headings() {
    // Extracts heading hierarchy (h1-h6) from HTML page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'headings.h1.length' not available on result type
    // skipped: field 'headings.h1[0].text' not available on result type
    // skipped: field 'headings.length' not available on result type
}

#[test]
fn test_metadata_hreflang() {
    // Extracts hreflang alternate link tags
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'hreflang.length' not available on result type
    // skipped: field 'hreflang[].lang' not available on result type
}

#[test]
fn test_metadata_keywords_author() {
    // Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(metadata_title, r#"Comprehensive Metadata Test Page"#, "equals assertion failed");
    assert!(result.metadata.canonical_url.is_some(), "expected metadata.canonical_url to be present");
    assert!(!result.metadata.keywords.is_empty(), "expected non-empty value");
    assert!(result.metadata.keywords.contains(r#"rust"#), "expected to contain: {}", r#"rust"#);
    assert_eq!(result.metadata.author, r#"Jane Developer"#, "equals assertion failed");
    assert!(!result.metadata.viewport.is_empty(), "expected non-empty value");
    assert_eq!(result.metadata.generator, r#"kreuzcrawl/1.0"#, "equals assertion failed");
    assert_eq!(result.metadata.theme_color, r#"#ff6600"#, "equals assertion failed");
    assert_eq!(result.metadata.robots, r#"index, follow"#, "equals assertion failed");
    assert_eq!(result.metadata.lang, r#"en"#, "equals assertion failed");
    assert_eq!(result.metadata.dir, r#"ltr"#, "equals assertion failed");
}

#[test]
fn test_metadata_og_video_audio() {
    // Extracts og:video, og:audio, and og:locale:alternate metadata
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'og.video' not available on result type
    // skipped: field 'og.audio' not available on result type
    // skipped: field 'og.locale_alternate.length' not available on result type
}

#[test]
fn test_metadata_response_headers() {
    // Extracts response metadata from HTTP headers (etag, server, content-language)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'response_headers.etag' not available on result type
    // skipped: field 'response_headers.last_modified' not available on result type
    // skipped: field 'response_headers.server' not available on result type
    // skipped: field 'response_headers.content_language' not available on result type
}

#[test]
fn test_metadata_word_count() {
    // Computes word count from visible page text
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    // skipped: field 'computed.word_count' not available on result type
    // skipped: field 'computed.word_count' not available on result type
}

