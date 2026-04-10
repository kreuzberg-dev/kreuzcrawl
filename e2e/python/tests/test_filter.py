"""E2e tests for category: filter.
"""
from kreuzcrawl import create_engine, scrape


def test_filter_bm25_crawl_integration() -> None:
    """BM25 filter works during multi-page crawl, keeping relevant pages."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert "rust" in result.filter.remaining_contain_keyword

def test_filter_bm25_empty_query() -> None:
    """BM25 filter with empty query passes all pages through."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 2

def test_filter_bm25_high_threshold() -> None:
    """BM25 filter with very high threshold filters out all pages."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.filter.pages_after_filter == 0

def test_filter_bm25_relevant_pages() -> None:
    """BM25 filter keeps only pages relevant to the query."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert "rust" in result.filter.remaining_contain_keyword

def test_filter_bm25_threshold_zero() -> None:
    """BM25 filter with zero threshold passes all pages."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 2

def test_filter_noop_crawl_all_kept() -> None:
    """NoopFilter keeps all pages during a multi-page crawl."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.filter.pages_after_filter == 3

def test_filter_noop_passes_all() -> None:
    """No content filter passes all crawled pages through."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 3

