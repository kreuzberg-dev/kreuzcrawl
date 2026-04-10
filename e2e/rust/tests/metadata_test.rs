//! E2e tests for category: metadata

use kreuzcrawl::scrape;

#[test]
fn test_metadata_article_times() {
    // Extracts article:published_time, modified_time, author, section, and tags
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.article.published_time, r#"2024-01-15T10:00:00Z"#, "equals assertion failed");
    assert_eq!(result.article.modified_time, r#"2024-06-20T14:30:00Z"#, "equals assertion failed");
    assert_eq!(result.article.author, r#"Jane Developer"#, "equals assertion failed");
    assert_eq!(result.article.section, r#"Technology"#, "equals assertion failed");
    assert_eq!(result.article.tags.len(), "3", "equals assertion failed");
}

#[test]
fn test_metadata_favicons() {
    // Extracts favicon link tags including apple-touch-icon
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.favicons.len(), "5", "equals assertion failed");
    assert!(!result.favicons.get("").map(|s| s.as_str()).apple_touch.is_empty(), "expected non-empty value");
}

#[test]
fn test_metadata_headings() {
    // Extracts heading hierarchy (h1-h6) from HTML page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.headings.h1.len(), "1", "equals assertion failed");
    assert_eq!(result.headings.h1.get("0").map(|s| s.as_str()).text, r#"Primary Heading"#, "equals assertion failed");
    assert_eq!(result.headings.len(), "8", "equals assertion failed");
}

#[test]
fn test_metadata_hreflang() {
    // Extracts hreflang alternate link tags
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.hreflang.len(), "4", "equals assertion failed");
    assert!(result.hreflang.get("").map(|s| s.as_str()).lang.contains(r#"en"#), "expected to contain: {}", r#"en"#);
}

#[test]
fn test_metadata_keywords_author() {
    // Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
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
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.og.video, r#"https://example.com/video.mp4"#, "equals assertion failed");
    assert_eq!(result.og.audio, r#"https://example.com/audio.mp3"#, "equals assertion failed");
    assert_eq!(result.og.locale_alternate.len(), "2", "equals assertion failed");
}

#[test]
fn test_metadata_response_headers() {
    // Extracts response metadata from HTTP headers (etag, server, content-language)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.response_headers.etag.is_empty(), "expected non-empty value");
    assert!(!result.response_headers.last_modified.is_empty(), "expected non-empty value");
    assert!(result.response_headers.server.contains(r#"nginx"#), "expected to contain: {}", r#"nginx"#);
    assert_eq!(result.response_headers.content_language, r#"en-US"#, "equals assertion failed");
}

#[test]
fn test_metadata_word_count() {
    // Computes word count from visible page text
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.computed.word_count > 99_f64, "expected > 99");
    assert!(result.computed.word_count < 301_f64, "expected < 301");
}

