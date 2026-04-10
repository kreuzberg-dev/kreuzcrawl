"""E2e tests for category: cookies.
"""
from kreuzcrawl import create_engine, scrape


def test_cookies_per_domain() -> None:
    """Isolates cookies per domain during crawl."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.cookies) == 1
    assert "domain_cookie" in result.cookies

def test_cookies_persistence() -> None:
    """Maintains cookies across multiple crawl requests."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert "session" in result.cookies

def test_cookies_set_cookie_response() -> None:
    """Respects Set-Cookie header from server responses."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert "tracking" in result.cookies

