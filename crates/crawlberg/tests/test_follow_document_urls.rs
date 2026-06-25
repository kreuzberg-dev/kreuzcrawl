//! Integration tests for `CrawlConfig.follow_document_urls` and
//! `CrawlConfig.document_url_depth`.
//!
//! Test harness: `wiremock` (same pattern as `test_document_download.rs`).
//!
//! `.txt` URLs are used because:
//!
//! - `classify_link` maps them to `LinkType::Document` (extension in `DOCUMENT_EXTENSIONS`)
//! - `is_pdf_url` / `is_binary_url` return `false`, so `page_was_skipped=false` with
//!   `text/html` content-type — links ARE extracted from their HTML bodies.
//!
//! This exercises `entry.doc_depth` propagation and the `follow_document_urls` gate without
//! requiring actual binary document parsing.

use crawlberg::{CrawlConfig, batch_crawl, create_engine};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ---------------------------------------------------------------------------
// Scenario 1 — follow_document_urls=false (default)
//
// Layout:
//   /index.html  ->  /paper.txt   (LinkType::Document, enqueued, doc_depth=1)
//   /paper.txt   ->  /page2.html  (Internal link from within document context)
//
// Expected: /paper.txt is fetched (it was discovered from HTML).  Discovery
// does NOT run on /paper.txt (doc_depth=1, follow_document_urls=false), so
// /page2.html is NEVER crawled.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn follow_document_urls_false_does_not_crawl_links_from_document_page() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/index.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/paper.txt">paper</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    // The document page contains an internal link — must NOT be followed when
    // follow_document_urls is false.
    Mock::given(method("GET"))
        .and(path("/paper.txt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/page2.html">next</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/page2.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>page 2</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(3),
        max_pages: Some(20),
        download_documents: true,
        follow_document_urls: false,
        ..Default::default()
    };
    let handle = create_engine(Some(config)).unwrap();

    let results = batch_crawl(&handle, vec![format!("{}/index.html", mock.uri())])
        .await
        .expect("batch_crawl should succeed");

    let crawl = results.results[0].result.as_ref().expect("seed crawl should succeed");

    let urls: Vec<&str> = crawl.pages.iter().map(|p| p.url.as_str()).collect();

    assert!(
        urls.iter().any(|u| u.ends_with("/paper.txt")),
        "paper.txt must be fetched (it was discovered as a Document link from an HTML page)"
    );
    assert!(
        !urls.iter().any(|u| u.ends_with("/page2.html")),
        "page2.html must NOT be crawled: discovery is blocked for doc_depth>0 pages when follow_document_urls=false"
    );
}

// ---------------------------------------------------------------------------
// Scenario 2 — follow_document_urls=true, document_url_depth=None, max_depth=3
//
// Layout (chain via .txt links):
//   /index.html  ->  /doc1.txt  (doc_depth=1, depth=1)
//   /doc1.txt    ->  /doc2.txt  (doc_depth=2, depth=2)
//   /doc2.txt    ->  /doc3.txt  (doc_depth=3, depth=3)
//
// Expected: all three documents are crawled (max_depth=3 permits depth=3,
// no document_url_depth cap).
// ---------------------------------------------------------------------------
#[tokio::test]
async fn follow_document_urls_true_no_depth_cap_traverses_until_max_depth() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/index.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/doc1.txt">d1</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/doc1.txt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/doc2.txt">d2</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/doc2.txt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/doc3.txt">d3</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/doc3.txt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>leaf document</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(3),
        max_pages: Some(20),
        download_documents: true,
        follow_document_urls: true,
        document_url_depth: None,
        ..Default::default()
    };
    let handle = create_engine(Some(config)).unwrap();

    let results = batch_crawl(&handle, vec![format!("{}/index.html", mock.uri())])
        .await
        .expect("batch_crawl should succeed");

    let crawl = results.results[0].result.as_ref().expect("seed crawl should succeed");

    let urls: Vec<&str> = crawl.pages.iter().map(|p| p.url.as_str()).collect();

    for suffix in ["/doc1.txt", "/doc2.txt", "/doc3.txt"] {
        assert!(
            urls.iter().any(|u| u.ends_with(suffix)),
            "{suffix} must be crawled when follow_document_urls=true with no depth cap"
        );
    }
}

// ---------------------------------------------------------------------------
// Scenario 3 — follow_document_urls=true, document_url_depth=Some(1), max_depth=5
//
// Layout:
//   /index.html  ->  /doc1.txt  (doc_depth=1, allowed)
//   /doc1.txt    ->  /doc2.txt  (doc_depth=2, BLOCKED by document_url_depth=Some(1))
//
// Expected: /doc1.txt is crawled; /doc2.txt is NOT, even though max_depth=5 permits it.
// ---------------------------------------------------------------------------
#[tokio::test]
async fn follow_document_urls_document_depth_cap_stops_at_configured_limit() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/index.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/doc1.txt">d1</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/doc1.txt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/doc2.txt">d2</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    // This endpoint must NOT be reached.
    Mock::given(method("GET"))
        .and(path("/doc2.txt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>should not be reached</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(5),
        max_pages: Some(20),
        download_documents: true,
        follow_document_urls: true,
        document_url_depth: Some(1),
        ..Default::default()
    };
    let handle = create_engine(Some(config)).unwrap();

    let results = batch_crawl(&handle, vec![format!("{}/index.html", mock.uri())])
        .await
        .expect("batch_crawl should succeed");

    let crawl = results.results[0].result.as_ref().expect("seed crawl should succeed");

    let urls: Vec<&str> = crawl.pages.iter().map(|p| p.url.as_str()).collect();

    assert!(
        urls.iter().any(|u| u.ends_with("/doc1.txt")),
        "doc1.txt (doc_depth=1) must be crawled"
    );
    assert!(
        !urls.iter().any(|u| u.ends_with("/doc2.txt")),
        "doc2.txt (doc_depth=2) must NOT be crawled when document_url_depth=Some(1)"
    );
}
