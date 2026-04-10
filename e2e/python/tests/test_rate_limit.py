"""E2e tests for category: rate_limit.
"""
from kreuzcrawl import create_engine, scrape


def test_rate_limit_basic_delay() -> None:
    """Rate limiter adds delay between requests to the same domain."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 3
    assert result.rate_limit.min_duration_ms >= 150

def test_rate_limit_zero_no_delay() -> None:
    """Rate limiter with zero delay does not slow crawling."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 2

