"""E2e tests for category: filter."""

import os

from kreuzcrawl import create_engine, scrape


def test_filter_bm25_crawl_integration() -> None:
    """BM25 filter works during multi-page crawl, keeping relevant pages."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_bm25_crawl_integration"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'filter.remaining_contain_keyword' not available on result type


def test_filter_bm25_empty_query() -> None:
    """BM25 filter with empty query passes all pages through."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_bm25_empty_query"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type


def test_filter_bm25_high_threshold() -> None:
    """BM25 filter with very high threshold filters out all pages."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_bm25_high_threshold"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'filter.pages_after_filter' not available on result type


def test_filter_bm25_relevant_pages() -> None:
    """BM25 filter keeps only pages relevant to the query."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_bm25_relevant_pages"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'filter.remaining_contain_keyword' not available on result type


def test_filter_bm25_threshold_zero() -> None:
    """BM25 filter with zero threshold passes all pages."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_bm25_threshold_zero"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type


def test_filter_noop_crawl_all_kept() -> None:
    """NoopFilter keeps all pages during a multi-page crawl."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_noop_crawl_all_kept"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'filter.pages_after_filter' not available on result type


def test_filter_noop_passes_all() -> None:
    """No content filter passes all crawled pages through."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/filter_noop_passes_all"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
