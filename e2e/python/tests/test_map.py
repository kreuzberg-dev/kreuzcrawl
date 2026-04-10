"""E2e tests for category: map.
"""
from kreuzcrawl import create_engine, scrape


def test_map_discover_urls() -> None:
    """Discovers all URLs on a site without fetching full content."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 3

def test_map_exclude_patterns() -> None:
    """Excludes URLs matching patterns from URL map."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 1

def test_map_include_subdomains() -> None:
    """Includes subdomain URLs in URL map discovery."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 2
    assert "blog.example.com" in result.urls

def test_map_large_sitemap() -> None:
    """Handles large sitemap with 100+ URLs."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 100

def test_map_limit_pagination() -> None:
    """Limits map result count to specified maximum."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.urls) <= 5

def test_map_search_filter() -> None:
    """Filters map results by search keyword."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.urls) >= 2
    assert "blog" in result.urls

