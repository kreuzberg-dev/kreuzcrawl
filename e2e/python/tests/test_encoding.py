"""E2e tests for category: encoding."""

import os

import pytest
from kreuzcrawl import create_engine, scrape


@pytest.mark.asyncio
async def test_encoding_double_encoded() -> None:
    """Handles double-encoded URL characters (%25C3%25B6)."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/encoding_double_encoded"
    result = await scrape(engine=engine, url=url)
    assert result.html
    assert len(result.links) >= 1


@pytest.mark.asyncio
async def test_encoding_mixed_charset_page() -> None:
    """Handles charset mismatch between HTTP header and HTML meta tag."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/encoding_mixed_charset_page"
    result = await scrape(engine=engine, url=url)
    assert result.html


@pytest.mark.asyncio
async def test_encoding_percent_encoded_path() -> None:
    """Handles percent-encoded spaces and characters in URL paths."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/encoding_percent_encoded_path"
    result = await scrape(engine=engine, url=url)
    assert result.html
    assert len(result.links) >= 2


@pytest.mark.asyncio
async def test_encoding_unicode_url() -> None:
    """Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/encoding_unicode_url"
    result = await scrape(engine=engine, url=url)
    assert result.html
