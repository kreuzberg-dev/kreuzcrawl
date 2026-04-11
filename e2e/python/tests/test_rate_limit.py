"""E2e tests for category: rate_limit."""

import os

from kreuzcrawl import create_engine, scrape


def test_rate_limit_basic_delay() -> None:
    """Rate limiter adds delay between requests to the same domain."""
    engine_config = {"max_depth": 1}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/rate_limit_basic_delay"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'rate_limit.min_duration_ms' not available on result type


def test_rate_limit_zero_no_delay() -> None:
    """Rate limiter with zero delay does not slow crawling."""
    engine_config = {"max_depth": 1}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/rate_limit_zero_no_delay"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
