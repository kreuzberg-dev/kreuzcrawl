"""E2e tests for category: validation.
"""
import pytest
from kreuzcrawl import scrape


def test_validation_invalid_exclude_regex() -> None:
    """Invalid regex in exclude_paths is rejected."""
    engine = None
    url = None
    with pytest.raises(Exception) as exc_info:
        scrape(engine=engine, url=url)
    assert "exclude_path" in str(exc_info.value)

def test_validation_invalid_include_regex() -> None:
    """Invalid regex in include_paths is rejected."""
    engine = None
    url = None
    with pytest.raises(Exception) as exc_info:
        scrape(engine=engine, url=url)
    assert "include_path" in str(exc_info.value)

def test_validation_invalid_retry_code() -> None:
    """Retry code outside 100-599 is rejected."""
    engine = None
    url = None
    with pytest.raises(Exception) as exc_info:
        scrape(engine=engine, url=url)
    assert "retry code" in str(exc_info.value)

def test_validation_max_pages_zero() -> None:
    """max_pages=0 is rejected as invalid config."""
    engine = None
    url = None
    with pytest.raises(Exception) as exc_info:
        scrape(engine=engine, url=url)
    assert "max_pages" in str(exc_info.value)

def test_validation_max_redirects_too_high() -> None:
    """max_redirects > 100 is rejected as invalid config."""
    engine = None
    url = None
    with pytest.raises(Exception) as exc_info:
        scrape(engine=engine, url=url)
    assert "max_redirects" in str(exc_info.value)

def test_validation_timeout_zero() -> None:
    """Zero request timeout is rejected as invalid config."""
    engine = None
    url = None
    with pytest.raises(Exception) as exc_info:
        scrape(engine=engine, url=url)
    assert "request_timeout" in str(exc_info.value)

