"""E2e tests for category: batch.
"""
from kreuzcrawl import create_engine, scrape


def test_scrape_batch_basic() -> None:
    """Batch scrape of multiple URLs all succeeding."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.batch.completed_count == 3
    assert result.batch.failed_count == 0
    assert result.batch.total_count == 3

def test_scrape_batch_partial_failure() -> None:
    """Batch scrape with one URL failing returns partial results."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.batch.completed_count == 2
    assert result.batch.failed_count == 1
    assert result.batch.total_count == 3

def test_scrape_batch_progress() -> None:
    """Batch scrape results include specific URL."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.batch.total_count == 2
    assert "/target" in result.batch.results

