"""E2e tests for category: middleware.
"""
from kreuzcrawl import create_engine, scrape


def test_middleware_engine_crawl_with_defaults() -> None:
    """Engine crawl with default middleware chain produces correct multi-page results."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'crawl.min_pages' not available on result type

def test_middleware_noop_no_effect() -> None:
    """Default middleware chain does not affect normal scraping."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.title == "Middleware Test"

