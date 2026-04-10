"""E2e tests for category: middleware.
"""
from kreuzcrawl import create_engine, scrape


def test_middleware_engine_crawl_with_defaults() -> None:
    """Engine crawl with default middleware chain produces correct multi-page results."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 3
    assert result.crawl.min_pages >= 3

def test_middleware_noop_no_effect() -> None:
    """Default middleware chain does not affect normal scraping."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.title == "Middleware Test"

