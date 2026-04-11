"""E2e tests for category: robots."""

import os

from kreuzcrawl import create_engine, scrape


def test_robots_allow_all() -> None:
    """Permissive robots.txt allows all paths."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_allow_all"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is True


def test_robots_allow_override() -> None:
    """Allow directive overrides Disallow for specific paths."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_allow_override"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is True


def test_robots_comments_handling() -> None:
    """Correctly parses robots.txt with inline and line comments."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_comments_handling"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is True


def test_robots_crawl_delay() -> None:
    """Respects crawl-delay directive from robots.txt."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_crawl_delay"
    result = scrape(engine=engine, url=url)
    assert result.crawl_delay == 2


def test_robots_disallow_path() -> None:
    """Robots.txt disallows specific paths."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_disallow_path"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is False


def test_robots_meta_nofollow() -> None:
    """Detects nofollow meta robots tag and skips link extraction."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_meta_nofollow"
    result = scrape(engine=engine, url=url)
    assert result.nofollow_detected is True


def test_robots_meta_noindex() -> None:
    """Detects noindex meta robots tag in HTML page."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_meta_noindex"
    result = scrape(engine=engine, url=url)
    assert result.noindex_detected is True


def test_robots_missing_404() -> None:
    """Missing robots.txt (404) allows all crawling."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_missing_404"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is True


def test_robots_multiple_user_agents() -> None:
    """Picks the most specific user-agent block from robots.txt."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_multiple_user_agents"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is True


def test_robots_request_rate() -> None:
    """Parses request-rate directive from robots.txt."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_request_rate"
    result = scrape(engine=engine, url=url)
    assert result.crawl_delay == 5
    assert result.is_allowed is True


def test_robots_sitemap_directive() -> None:
    """Discovers sitemap URL from Sitemap directive in robots.txt."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_sitemap_directive"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is True


def test_robots_user_agent_specific() -> None:
    """Matches user-agent specific rules in robots.txt."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_user_agent_specific"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is False


def test_robots_wildcard_paths() -> None:
    """Handles wildcard Disallow patterns in robots.txt."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_wildcard_paths"
    result = scrape(engine=engine, url=url)
    assert result.is_allowed is False


def test_robots_x_robots_tag() -> None:
    """Respects X-Robots-Tag HTTP header directives."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/robots_x_robots_tag"
    result = scrape(engine=engine, url=url)
    assert result.x_robots_tag.strip() == "noindex, nofollow"
    assert result.noindex_detected is True
    assert result.nofollow_detected is True
