"""E2e tests for category: map.
"""
from kreuzcrawl import scrape


def test_map_discover_urls() -> None:
    """Discovers all URLs on a site without fetching full content."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 3

def test_map_exclude_patterns() -> None:
    """Excludes URLs matching patterns from URL map."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 1

def test_map_include_subdomains() -> None:
    """Includes subdomain URLs in URL map discovery."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 2
    assert "blog.example.com" in result.urls

def test_map_large_sitemap() -> None:
    """Handles large sitemap with 100+ URLs."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 100

def test_map_limit_pagination() -> None:
    """Limits map result count to specified maximum."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) <= 5

def test_map_search_filter() -> None:
    """Filters map results by search keyword."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 2
    assert "blog" in result.urls

