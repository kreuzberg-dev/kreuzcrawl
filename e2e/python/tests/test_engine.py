"""E2e tests for category: engine.
"""
from kreuzcrawl import scrape


def test_engine_batch_basic() -> None:
    """CrawlEngine with defaults batch scrapes like the free function."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.batch.completed_count == 2
    assert result.batch.total_count == 2

def test_engine_crawl_basic() -> None:
    """CrawlEngine with defaults crawls multiple pages like the free function."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 3
    assert result.crawl.min_pages >= 3

def test_engine_map_basic() -> None:
    """CrawlEngine with defaults discovers URLs like the free function."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.map.min_urls >= 2

def test_engine_scrape_basic() -> None:
    """CrawlEngine with defaults scrapes a page identically to the free function."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.content_type == "text/html"
    assert result.metadata.title == "Engine Test"
    assert "Testing the engine" in result.metadata.description_contains
    assert result.links.min_count >= 1
    assert result.headings.h1_count == 1
    assert result.headings.h1_text == "Hello Engine"

def test_engine_stream_basic() -> None:
    """CrawlEngine with defaults streams events like the free function."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.stream.has_page_event is True
    assert result.stream.has_complete_event is True
    assert result.stream.event_count_min >= 3

