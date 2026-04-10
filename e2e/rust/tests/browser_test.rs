//! E2e tests for category: browser

use kreuzcrawl::scrape;

#[test]
fn test_browser_config_auto_no_feature() {
    // Browser mode 'auto' without browser feature enabled does not use browser
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_config_never_mode() {
    // Browser mode 'never' prevents browser fallback even for SPA shell content
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_minimal_page() {
    // Does NOT flag a short but real content page as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "false", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_next_empty() {
    // Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_next_rendered() {
    // Does NOT flag Next.js page with full SSR content as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.html_not_empty, "true", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "false", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_normal_page() {
    // Does NOT flag a normal server-rendered page as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "false", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_nuxt_shell() {
    // Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_react_shell() {
    // Detects React SPA shell with empty #root div as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.html_not_empty, "true", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_detect_vue_shell() {
    // Detects Vue SPA shell with empty #app div as needing JS rendering
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "false", "equals assertion failed");
}

#[test]
fn test_browser_fallback_spa_render() {
    // Browser auto re-fetches SPA shell when JS rendering is detected
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.browser.js_render_hint, "true", "equals assertion failed");
    assert_eq!(result.browser.browser_used, "true", "equals assertion failed");
}

#[test]
fn test_browser_fallback_waf_blocked() {
    // Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403)
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.browser.browser_used, "true", "equals assertion failed");
}

#[test]
fn test_browser_mode_always() {
    // Browser mode 'always' uses browser even for normal server-rendered pages
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.browser.browser_used, "true", "equals assertion failed");
}

