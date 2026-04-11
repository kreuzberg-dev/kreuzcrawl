"""E2e tests for category: content."""

import os

from kreuzcrawl import create_engine, scrape


def test_content_204_no_content() -> None:
    """Handles 204 No Content response gracefully."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_204_no_content"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 204
    assert not result.html


def test_content_charset_iso8859() -> None:
    """Handles ISO-8859-1 encoded page correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_charset_iso8859"
    result = scrape(engine=engine, url=url)
    assert result.detected_charset.strip() == "iso-8859-1"


def test_content_empty_body() -> None:
    """Handles 200 response with empty body gracefully."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_empty_body"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200


def test_content_gzip_compressed() -> None:
    """Handles response with Accept-Encoding gzip negotiation."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_gzip_compressed"
    result = scrape(engine=engine, url=url)
    assert result.html
    assert result.status_code == 200


def test_content_large_page_limit() -> None:
    """Respects max body size limit and truncates or skips oversized pages."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_large_page_limit"
    result = scrape(engine=engine, url=url)
    assert result.body_size < 1025


def test_content_main_only() -> None:
    """Extracts only main content area, excluding nav, sidebar, footer."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_main_only"
    result = scrape(engine=engine, url=url)
    assert result.main_content_only is True


def test_content_pdf_no_extension() -> None:
    """Detects PDF content by Content-Type header when URL has no .pdf extension."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_pdf_no_extension"
    result = scrape(engine=engine, url=url)
    assert result.is_pdf is True


def test_content_remove_tags() -> None:
    """Removes specified HTML elements by CSS selector before processing."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_remove_tags"
    result = scrape(engine=engine, url=url)
    assert result.html


def test_content_utf8_bom() -> None:
    """Handles UTF-8 content with BOM marker correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/content_utf8_bom"
    result = scrape(engine=engine, url=url)
    assert result.detected_charset.strip() == "utf-8"
    assert result.html
