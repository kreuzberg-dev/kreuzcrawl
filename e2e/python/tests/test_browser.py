"""E2e tests for category: browser.
"""
from kreuzcrawl import scrape


def test_browser_config_auto_no_feature() -> None:
    """Browser mode 'auto' without browser feature enabled does not use browser."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is False

def test_browser_config_never_mode() -> None:
    """Browser mode 'never' prevents browser fallback even for SPA shell content."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is False

def test_browser_detect_minimal_page() -> None:
    """Does NOT flag a short but real content page as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is False
    assert result.browser.browser_used is False

def test_browser_detect_next_empty() -> None:
    """Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is False

def test_browser_detect_next_rendered() -> None:
    """Does NOT flag Next.js page with full SSR content as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.html_not_empty is True
    assert result.browser.js_render_hint is False
    assert result.browser.browser_used is False

def test_browser_detect_normal_page() -> None:
    """Does NOT flag a normal server-rendered page as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is False
    assert result.browser.browser_used is False

def test_browser_detect_nuxt_shell() -> None:
    """Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is False

def test_browser_detect_react_shell() -> None:
    """Detects React SPA shell with empty #root div as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.html_not_empty is True
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is False

def test_browser_detect_vue_shell() -> None:
    """Detects Vue SPA shell with empty #app div as needing JS rendering."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is False

def test_browser_fallback_spa_render() -> None:
    """Browser auto re-fetches SPA shell when JS rendering is detected."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.browser.js_render_hint is True
    assert result.browser.browser_used is True

def test_browser_fallback_waf_blocked() -> None:
    """Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403)."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.browser.browser_used is True

def test_browser_mode_always() -> None:
    """Browser mode 'always' uses browser even for normal server-rendered pages."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.browser.browser_used is True

