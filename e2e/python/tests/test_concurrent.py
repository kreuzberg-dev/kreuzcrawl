"""E2e tests for category: concurrent.
"""
from kreuzcrawl import create_engine, scrape


def test_concurrent_basic() -> None:
    """Concurrent crawling fetches all pages with max_concurrent workers."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'pages.length' not available on result type

def test_concurrent_depth_two_fan_out() -> None:
    """Concurrent depth=2 crawl correctly fans out and deduplicates across levels."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_concurrent_max_pages_exact() -> None:
    """Concurrent crawling does not exceed max_pages limit even with high concurrency."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_concurrent_partial_errors() -> None:
    """Concurrent crawl handles partial failures gracefully."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_concurrent_respects_max_pages() -> None:
    """Concurrent crawling respects max_pages limit."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

