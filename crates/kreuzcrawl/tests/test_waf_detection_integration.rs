#![allow(clippy::unwrap_used, clippy::panic)]
//! Integration tests for WAF/bot-protection detection via actual HTTP responses.

use kreuzcrawl::{BrowserMode, CrawlConfig, CrawlError, create_engine, scrape};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn no_browser_config() -> CrawlConfig {
    let mut config = CrawlConfig::default();
    config.browser.mode = BrowserMode::Never;
    config
}

async fn assert_waf_blocked(body: &str, headers: Vec<(&str, &str)>) {
    let mock = MockServer::start().await;

    let mut resp = ResponseTemplate::new(403);
    for (k, v) in headers {
        resp = resp.append_header(k, v);
    }
    resp = resp.set_body_string(body);

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(resp)
        .mount(&mock)
        .await;

    let handle = create_engine(Some(no_browser_config())).unwrap();
    let result = scrape(&handle, &mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::WafBlocked(_))),
        "expected WafBlocked, got: {:?}",
        result
    );
}

#[tokio::test]
async fn test_cloudflare_detection() {
    assert_waf_blocked(
        "<html>cf-browser-verification challenge-form</html>",
        vec![("content-type", "text/html"), ("server", "cloudflare")],
    )
    .await;
}

#[tokio::test]
async fn test_imperva_detection() {
    assert_waf_blocked(
        "<html>Powered by Incapsula _incap_ses_</html>",
        vec![("content-type", "text/html")],
    )
    .await;
}

#[tokio::test]
async fn test_datadome_detection() {
    assert_waf_blocked(
        "<html><script src=\"dd.js\"></script>datadome</html>",
        vec![("content-type", "text/html")],
    )
    .await;
}

#[tokio::test]
async fn test_akamai_detection() {
    assert_waf_blocked(
        "<html>Access Denied</html>",
        vec![("content-type", "text/html"), ("server", "AkamaiGHost")],
    )
    .await;
}

/// Cloudflare's "Just a moment..." JS challenge ships with **200 OK** on
/// modern accounts, not 403. The body contains `cf-chl-` / `__cf_chl_`
/// fingerprints. The detector must classify this as `WafBlocked` so the
/// engine's `BrowserMode::Auto` fallback can retry via headless Chrome —
/// otherwise the challenge HTML is fed downstream as if it were content.
#[tokio::test]
async fn test_cloudflare_2xx_interstitial_detected() {
    let mock = MockServer::start().await;
    let challenge = r#"
        <!DOCTYPE html>
        <html><head><title>Just a moment...</title></head>
        <body>
            <noscript>This site requires JavaScript</noscript>
            <script src="/cdn-cgi/challenge-platform/h/g/orchestrate/chl_page/v1?ray=abc"></script>
            <div id="cf-chl-widget-abc"></div>
        </body></html>
    "#;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(challenge)
                .append_header("content-type", "text/html")
                .append_header("server", "cloudflare"),
        )
        .mount(&mock)
        .await;

    let handle = create_engine(Some(no_browser_config())).unwrap();
    let result = scrape(&handle, &mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::WafBlocked(_))),
        "2xx Cloudflare interstitial must be WafBlocked, got: {:?}",
        result
    );
}

/// DataDome JS challenge serves with 200 OK and an `x-datadome` header.
/// The header alone is enough to classify regardless of body content.
#[tokio::test]
async fn test_datadome_2xx_header_detected() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><script src=\"https://js.datadome.co/tags.js\"></script></html>")
                .append_header("content-type", "text/html")
                .append_header("x-datadome", "blocked"),
        )
        .mount(&mock)
        .await;

    let handle = create_engine(Some(no_browser_config())).unwrap();
    let result = scrape(&handle, &mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::WafBlocked(_))),
        "2xx with x-datadome header must be WafBlocked, got: {:?}",
        result
    );
}

/// PerimeterX challenge serves with 200 OK and `x-px-*` response headers.
#[tokio::test]
async fn test_perimeterx_2xx_header_detected() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html>px-captcha</html>")
                .append_header("content-type", "text/html")
                .append_header("x-px-block", "1"),
        )
        .mount(&mock)
        .await;

    let handle = create_engine(Some(no_browser_config())).unwrap();
    let result = scrape(&handle, &mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::WafBlocked(_))),
        "2xx with x-px-* header must be WafBlocked, got: {:?}",
        result
    );
}

/// A legitimate page mentioning a WAF vendor in prose (e.g. a blog post
/// about Cloudflare) MUST NOT be classified as blocked. The page is well
/// over CHALLENGE_BODY_LIMIT and contains no JS challenge fingerprint.
#[tokio::test]
async fn test_2xx_legitimate_long_page_not_flagged() {
    let mock = MockServer::start().await;
    // Build a body well over 100 KB that mentions vendor names in prose
    // but contains no JS challenge markers.
    let mut body = String::from("<html><body><h1>How Cloudflare and DataDome work</h1>");
    body.push_str(&"<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. </p>".repeat(4000));
    body.push_str("</body></html>");
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(&body)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let handle = create_engine(Some(no_browser_config())).unwrap();
    let result = scrape(&handle, &mock.uri()).await;
    assert!(
        result.is_ok(),
        "legitimate long page mentioning vendors must NOT be blocked, got: {:?}",
        result
    );
}

#[tokio::test]
async fn test_plain_403_is_not_waf() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_string("<html>Forbidden</html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let handle = create_engine(Some(no_browser_config())).unwrap();
    let result = scrape(&handle, &mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::Forbidden(_))),
        "plain 403 should be Forbidden, not WafBlocked: {:?}",
        result
    );
}
