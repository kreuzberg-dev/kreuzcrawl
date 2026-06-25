//! Substrate-only end-to-end crawl.
//!
//! Locks in the substrate-vs-operational acceptance criterion: a developer
//! must be able to crawl a small site using **only crawlberg** — no
//! `xberg-enterprise`, no `crawl-traits`, no external trait impls. The engine
//! ships with usable baselines for every trait extension point
//! (`defaults::{InMemoryFrontier, PerDomainThrottle, NoopCache, NoopStore,
//! NoopEmitter, NoopFilter}`, `TomlClassifier` for WAF) so a stock
//! `create_engine(Some(config))` call is sufficient.
//!
//! Additionally, the public substrate parsers (`crawlberg::robots::*`,
//! `crawlberg::sitemap::*`) are exercised directly to confirm they are
//! reachable from out-of-crate code without spinning the engine.

use crawlberg::robots::{is_path_allowed, parse_robots_txt};
use crawlberg::sitemap::{is_sitemap_index, parse_sitemap_xml};
use crawlberg::{CrawlConfig, crawl, create_engine};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ROBOTS_BODY: &str = "User-agent: *\nDisallow: /private\nSitemap: /sitemap.xml";

const SITEMAP_BODY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://example.com/a</loc></url>
  <url><loc>https://example.com/b</loc></url>
  <url><loc>https://example.com/c</loc></url>
</urlset>"#;

#[test]
fn substrate_robots_parser_is_reachable_out_of_crate() {
    let rules = parse_robots_txt(ROBOTS_BODY, "crawlberg");
    assert!(is_path_allowed("/public", &rules));
    assert!(!is_path_allowed("/private/secret", &rules));
    assert_eq!(rules.sitemaps, vec!["/sitemap.xml".to_string()]);
}

#[test]
fn substrate_sitemap_parser_is_reachable_out_of_crate() {
    assert!(!is_sitemap_index(SITEMAP_BODY));
    let urls = parse_sitemap_xml(SITEMAP_BODY);
    assert_eq!(urls.len(), 3);
    assert_eq!(urls[0].url, "https://example.com/a");
}

#[tokio::test]
async fn substrate_only_engine_crawls_seed_and_linked_pages() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><a href="/a">A</a> <a href="/b">B</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    for name in ["a", "b"] {
        Mock::given(method("GET"))
            .and(path(format!("/{name}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(format!("<html><body>{name}</body></html>"))
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }

    let base = CrawlConfig::builder().allow_private_networks(true).build();
    let config = CrawlConfig {
        max_depth: Some(1),
        respect_robots_txt: false,
        ..base
    };
    let handle = create_engine(Some(config)).expect("engine should build with substrate-only defaults");

    let uri = mock.uri();
    let result = crawl(&handle, &uri).await.expect("crawl should succeed");
    let urls: Vec<_> = result.pages.iter().map(|p| p.url.as_str()).collect();
    assert!(urls.contains(&uri.as_str()), "root must be crawled, got {urls:?}");
    assert!(
        urls.iter().any(|u| u.ends_with("/a")),
        "/a must be crawled, got {urls:?}"
    );
    assert!(
        urls.iter().any(|u| u.ends_with("/b")),
        "/b must be crawled, got {urls:?}"
    );
}
