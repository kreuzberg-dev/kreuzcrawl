"""E2e tests for category: batch."""

import os

from kreuzcrawl import create_engine, scrape


def test_scrape_batch_basic() -> None:
    """Batch scrape of multiple URLs all succeeding."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_batch_basic"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'batch.completed_count' not available on result type
    # skipped: field 'batch.failed_count' not available on result type
    # skipped: field 'batch.total_count' not available on result type


def test_scrape_batch_partial_failure() -> None:
    """Batch scrape with one URL failing returns partial results."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_batch_partial_failure"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'batch.completed_count' not available on result type
    # skipped: field 'batch.failed_count' not available on result type
    # skipped: field 'batch.total_count' not available on result type


def test_scrape_batch_progress() -> None:
    """Batch scrape results include specific URL."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_batch_progress"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'batch.total_count' not available on result type
    # skipped: field 'batch.results' not available on result type
