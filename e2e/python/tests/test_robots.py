"""E2e tests for category: robots.
"""
from kreuzcrawl import scrape


def test_robots_allow_all() -> None:
    """Permissive robots.txt allows all paths."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is True

def test_robots_allow_override() -> None:
    """Allow directive overrides Disallow for specific paths."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is True

def test_robots_comments_handling() -> None:
    """Correctly parses robots.txt with inline and line comments."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is True

def test_robots_crawl_delay() -> None:
    """Respects crawl-delay directive from robots.txt."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.crawl_delay == 2

def test_robots_disallow_path() -> None:
    """Robots.txt disallows specific paths."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is False

def test_robots_meta_nofollow() -> None:
    """Detects nofollow meta robots tag and skips link extraction."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.nofollow_detected is True

def test_robots_meta_noindex() -> None:
    """Detects noindex meta robots tag in HTML page."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.noindex_detected is True

def test_robots_missing_404() -> None:
    """Missing robots.txt (404) allows all crawling."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is True

def test_robots_multiple_user_agents() -> None:
    """Picks the most specific user-agent block from robots.txt."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is True

def test_robots_request_rate() -> None:
    """Parses request-rate directive from robots.txt."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.crawl_delay == 5
    assert result.robots.is_allowed is True

def test_robots_sitemap_directive() -> None:
    """Discovers sitemap URL from Sitemap directive in robots.txt."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is True

def test_robots_user_agent_specific() -> None:
    """Matches user-agent specific rules in robots.txt."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is False

def test_robots_wildcard_paths() -> None:
    """Handles wildcard Disallow patterns in robots.txt."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.is_allowed is False

def test_robots_x_robots_tag() -> None:
    """Respects X-Robots-Tag HTTP header directives."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.robots.x_robots_tag == "noindex, nofollow"
    assert result.robots.noindex_detected is True
    assert result.robots.nofollow_detected is True

