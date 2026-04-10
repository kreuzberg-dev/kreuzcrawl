"""E2e tests for category: stream.
"""
from kreuzcrawl import scrape


def test_crawl_stream_events() -> None:
    """Crawl stream produces page and complete events."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.stream.event_count_min >= 4
    assert result.stream.has_page_event is True
    assert result.stream.has_complete_event is True

def test_stream_depth_crawl() -> None:
    """Stream produces events for multi-depth crawl with link following."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.stream.event_count_min >= 5
    assert result.stream.has_page_event is True
    assert result.stream.has_complete_event is True

def test_stream_with_error_event() -> None:
    """Stream emits page and complete events even when some pages fail."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.stream.has_page_event is True
    assert result.stream.has_complete_event is True
    assert result.stream.event_count_min >= 2

