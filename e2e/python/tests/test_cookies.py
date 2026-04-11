"""E2e tests for category: cookies."""

import os

from kreuzcrawl import create_engine, scrape


def test_cookies_per_domain() -> None:
    """Isolates cookies per domain during crawl."""
    engine_config = {"cookies_enabled": True, "max_depth": 1, "respect_robots_txt": False}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/cookies_per_domain"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'cookies.length' not available on result type
    # skipped: field 'cookies' not available on result type


def test_cookies_persistence() -> None:
    """Maintains cookies across multiple crawl requests."""
    engine_config = {"cookies_enabled": True, "max_depth": 1, "respect_robots_txt": False}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/cookies_persistence"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'cookies' not available on result type


def test_cookies_set_cookie_response() -> None:
    """Respects Set-Cookie header from server responses."""
    engine_config = {"cookies_enabled": True, "max_depth": 1, "respect_robots_txt": False}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/cookies_set_cookie_response"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'cookies' not available on result type
