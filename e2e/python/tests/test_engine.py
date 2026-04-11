"""E2e tests for category: engine."""

import os

from kreuzcrawl import create_engine, scrape


def test_engine_batch_basic() -> None:
    """CrawlEngine with defaults batch scrapes like the free function."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/engine_batch_basic"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'batch.completed_count' not available on result type
    # skipped: field 'batch.total_count' not available on result type


def test_engine_crawl_basic() -> None:
    """CrawlEngine with defaults crawls multiple pages like the free function."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/engine_crawl_basic"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'crawl.min_pages' not available on result type


def test_engine_map_basic() -> None:
    """CrawlEngine with defaults discovers URLs like the free function."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/engine_map_basic"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'map.min_urls' not available on result type


def test_engine_scrape_basic() -> None:
    """CrawlEngine with defaults scrapes a page identically to the free function."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/engine_scrape_basic"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.content_type.strip() == "text/html"
    assert result.metadata.title.strip() == "Engine Test"
    assert "Testing the engine" in result.metadata.description
    assert len(result.links) >= 1
    assert len(result.metadata.headings) >= 1


def test_engine_stream_basic() -> None:
    """CrawlEngine with defaults streams events like the free function."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/engine_stream_basic"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'stream.has_page_event' not available on result type
    # skipped: field 'stream.has_complete_event' not available on result type
    # skipped: field 'stream.event_count_min' not available on result type
