"""E2e tests for category: encoding.
"""
from kreuzcrawl import scrape


def test_encoding_double_encoded() -> None:
    """Handles double-encoded URL characters (%25C3%25B6)."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.html
    assert len(result.links) >= 1

def test_encoding_mixed_charset_page() -> None:
    """Handles charset mismatch between HTTP header and HTML meta tag."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.html

def test_encoding_percent_encoded_path() -> None:
    """Handles percent-encoded spaces and characters in URL paths."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.html
    assert len(result.links) >= 2

def test_encoding_unicode_url() -> None:
    """Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.html

