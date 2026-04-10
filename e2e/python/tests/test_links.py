"""E2e tests for category: links.
"""
from kreuzcrawl import scrape


def test_links_anchor_fragment() -> None:
    """Identifies fragment-only links as anchor type."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert "anchor" in result.links.get("").link_type

def test_links_base_tag() -> None:
    """Resolves relative URLs using base tag href."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 2
    assert "example.com" in result.links.get("").url

def test_links_document_types() -> None:
    """Detects PDF, DOCX, XLSX links as document type."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert "document" in result.links.get("").link_type

def test_links_empty_href() -> None:
    """Handles empty href attributes without errors."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 0
    assert "/valid" in result.links.get("").url

def test_links_internal_external_classification() -> None:
    """Correctly classifies internal vs external links by domain."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 4
    assert "internal" in result.links.get("").link_type
    assert "external" in result.links.get("").link_type

def test_links_mailto_javascript_skip() -> None:
    """Skips mailto:, javascript:, and tel: scheme links."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 0
    assert "mailto:" not in result.links.get("").url

def test_links_protocol_relative() -> None:
    """Handles protocol-relative URLs (//example.com) correctly."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 1
    assert result.links.get("").protocol_relative

def test_links_rel_attributes() -> None:
    """Preserves rel=nofollow and rel=canonical attributes."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 0

def test_links_relative_parent() -> None:
    """Resolves ../ and ./ relative parent path links correctly."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 3

