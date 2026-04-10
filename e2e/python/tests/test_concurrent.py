"""E2e tests for category: concurrent.
"""
from kreuzcrawl import scrape


def test_concurrent_basic() -> None:
    """Concurrent crawling fetches all pages with max_concurrent workers."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.pages) == 6
    assert len(result.pages) >= 6

def test_concurrent_depth_two_fan_out() -> None:
    """Concurrent depth=2 crawl correctly fans out and deduplicates across levels."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.pages) == 4

def test_concurrent_max_pages_exact() -> None:
    """Concurrent crawling does not exceed max_pages limit even with high concurrency."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.pages) <= 3

def test_concurrent_partial_errors() -> None:
    """Concurrent crawl handles partial failures gracefully."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.pages) >= 2

def test_concurrent_respects_max_pages() -> None:
    """Concurrent crawling respects max_pages limit."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.pages) <= 3

