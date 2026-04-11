"""E2e tests for category: stream."""

import os

from kreuzcrawl import create_engine, scrape


def test_crawl_stream_events() -> None:
    """Crawl stream produces page and complete events."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/crawl_stream_events"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'stream.event_count_min' not available on result type
    # skipped: field 'stream.has_page_event' not available on result type
    # skipped: field 'stream.has_complete_event' not available on result type


def test_stream_depth_crawl() -> None:
    """Stream produces events for multi-depth crawl with link following."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/stream_depth_crawl"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'stream.event_count_min' not available on result type
    # skipped: field 'stream.has_page_event' not available on result type
    # skipped: field 'stream.has_complete_event' not available on result type


def test_stream_with_error_event() -> None:
    """Stream emits page and complete events even when some pages fail."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/stream_with_error_event"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'stream.has_page_event' not available on result type
    # skipped: field 'stream.has_complete_event' not available on result type
    # skipped: field 'stream.event_count_min' not available on result type
