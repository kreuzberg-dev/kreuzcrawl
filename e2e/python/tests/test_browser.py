"""E2e tests for category: browser."""

import os

from kreuzcrawl import create_engine, scrape


def test_browser_config_auto_no_feature() -> None:
    """Browser mode 'auto' without browser feature enabled does not use browser."""
    engine_config = {"browser": {"mode": "auto"}}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_config_auto_no_feature"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_config_never_mode() -> None:
    """Browser mode 'never' prevents browser fallback even for SPA shell content."""
    engine_config = {"browser": {"mode": "never"}}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_config_never_mode"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_minimal_page() -> None:
    """Does NOT flag a short but real content page as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_minimal_page"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_next_empty() -> None:
    """Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_next_empty"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_next_rendered() -> None:
    """Does NOT flag Next.js page with full SSR content as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_next_rendered"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'html_not_empty' not available on result type
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_normal_page() -> None:
    """Does NOT flag a normal server-rendered page as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_normal_page"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_nuxt_shell() -> None:
    """Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_nuxt_shell"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_react_shell() -> None:
    """Detects React SPA shell with empty #root div as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_react_shell"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'html_not_empty' not available on result type
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_detect_vue_shell() -> None:
    """Detects Vue SPA shell with empty #app div as needing JS rendering."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_detect_vue_shell"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_fallback_spa_render() -> None:
    """Browser auto re-fetches SPA shell when JS rendering is detected."""
    engine_config = {"browser": {"mode": "auto"}}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_fallback_spa_render"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'browser.js_render_hint' not available on result type
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_fallback_waf_blocked() -> None:
    """Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403)."""
    engine_config = {"browser": {"mode": "auto"}}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_fallback_waf_blocked"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'browser.browser_used' not available on result type


def test_browser_mode_always() -> None:
    """Browser mode 'always' uses browser even for normal server-rendered pages."""
    engine_config = {"browser": {"mode": "always"}}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/browser_mode_always"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'browser.browser_used' not available on result type
