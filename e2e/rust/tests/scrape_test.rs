//! E2e tests for category: scrape

use kreuzcrawl::scrape;

#[test]
fn test_scrape_asset_dedup() {
    // Same asset linked twice results in one download with one unique hash
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.assets.len(), "2", "equals assertion failed");
    assert_eq!(result.assets.unique_hashes, "2", "equals assertion failed");
}

#[test]
fn test_scrape_asset_max_size() {
    // Skips assets exceeding max_asset_size limit
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.assets.len(), "2", "equals assertion failed");
}

#[test]
fn test_scrape_asset_type_filter() {
    // Only downloads image assets when asset_types filter is set
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.assets.len(), "1", "equals assertion failed");
    assert!(result.assets.get("").map(|s| s.as_str()).category.contains(r#"image"#), "expected to contain: {}", r#"image"#);
}

#[test]
fn test_scrape_basic_html_page() {
    // Scrapes a simple HTML page and extracts title, description, and links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    let metadata_description = result.metadata.description.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.content_type, r#"text/html"#, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert_eq!(metadata_title, r#"Example Domain"#, "equals assertion failed");
    assert!(metadata_description.contains(r#"illustrative examples"#), "expected to contain: {}", r#"illustrative examples"#);
    assert!(result.metadata.canonical_url.is_some(), "expected metadata.canonical_url to be present");
    assert!(result.links.len() > 0_f64, "expected > 0");
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"external"#), "expected to contain: {}", r#"external"#);
    assert_eq!(result.images.len(), "0", "equals assertion failed");
    assert!(result.og.title.is_empty(), "expected empty value");
}

#[test]
fn test_scrape_complex_links() {
    // Classifies links by type: internal, external, anchor, document, image
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.links.len() > 9_f64, "expected > 9");
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"internal"#), "expected to contain: {}", r#"internal"#);
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"external"#), "expected to contain: {}", r#"external"#);
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"anchor"#), "expected to contain: {}", r#"anchor"#);
    assert!(result.links.get("").map(|s| s.as_str()).link_type.contains(r#"document"#), "expected to contain: {}", r#"document"#);
}

#[test]
fn test_scrape_download_assets() {
    // Downloads CSS, JS, and image assets from page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.assets.len() > 2_f64, "expected > 2");
}

#[test]
fn test_scrape_dublin_core() {
    // Extracts Dublin Core metadata from a page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.dublin_core.title.is_empty(), "expected non-empty value");
    assert_eq!(result.dublin_core.title, r#"Effects of Climate Change on Marine Biodiversity"#, "equals assertion failed");
    assert_eq!(result.dublin_core.creator, r#"Dr. Jane Smith"#, "equals assertion failed");
}

#[test]
fn test_scrape_empty_page() {
    // Handles an empty HTML document without errors
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.links.len() > -1_f64, "expected > -1");
    assert_eq!(result.images.len(), "0", "equals assertion failed");
}

#[test]
fn test_scrape_feed_discovery() {
    // Discovers RSS, Atom, and JSON feed links
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.feeds.rss.len(), "1", "equals assertion failed");
    assert_eq!(result.feeds.atom.len(), "1", "equals assertion failed");
    assert_eq!(result.feeds.json_feed.len(), "1", "equals assertion failed");
}

#[test]
fn test_scrape_image_sources() {
    // Extracts images from img, picture, og:image, twitter:image
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(result.images.len() > 4_f64, "expected > 4");
    assert_eq!(result.og.image, r#"https://example.com/images/og-hero.jpg"#, "equals assertion failed");
}

#[test]
fn test_scrape_js_heavy_spa() {
    // Handles SPA page with JavaScript-only content (no server-rendered HTML)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}

#[test]
fn test_scrape_json_ld() {
    // Extracts JSON-LD structured data from a page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.json_ld.is_empty(), "expected non-empty value");
    assert_eq!(result.json_ld.type, r#"Recipe"#, "equals assertion failed");
    assert_eq!(result.json_ld.name, r#"Best Chocolate Cake"#, "equals assertion failed");
}

#[test]
fn test_scrape_malformed_html() {
    // Gracefully handles broken HTML without crashing
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let metadata_description = result.metadata.description.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(metadata_description.contains(r#"broken HTML"#), "expected to contain: {}", r#"broken HTML"#);
}

#[test]
fn test_scrape_og_metadata() {
    // Extracts full Open Graph metadata from a page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.og.title.is_empty(), "expected non-empty value");
    assert_eq!(result.og.title, r#"Article Title"#, "equals assertion failed");
    assert_eq!(result.og.type, r#"article"#, "equals assertion failed");
    assert_eq!(result.og.image, r#"https://example.com/images/article-hero.jpg"#, "equals assertion failed");
    assert!(!result.og.description.is_empty(), "expected non-empty value");
    assert_eq!(metadata_title, r#"Article Title - Example Blog"#, "equals assertion failed");
}

#[test]
fn test_scrape_twitter_card() {
    // Extracts Twitter Card metadata from a page
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert!(!result.twitter.card.is_empty(), "expected non-empty value");
    assert_eq!(result.twitter.card_type, r#"summary_large_image"#, "equals assertion failed");
    assert_eq!(result.twitter.title, r#"New Product Launch"#, "equals assertion failed");
}

