"""E2e tests for category: cache.
"""
from kreuzcrawl import create_engine, scrape


def test_cache_basic() -> None:
    """Crawling with disk cache enabled succeeds without errors."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200

