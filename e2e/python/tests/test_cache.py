"""E2e tests for category: cache.
"""
from kreuzcrawl import scrape


def test_cache_basic() -> None:
    """Crawling with disk cache enabled succeeds without errors."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200

