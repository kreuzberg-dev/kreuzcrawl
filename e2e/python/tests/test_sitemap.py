"""E2e tests for category: sitemap.
"""
from kreuzcrawl import scrape


def test_sitemap_basic() -> None:
    """Parses a standard urlset sitemap."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 4
    assert result.has_lastmod is True

def test_sitemap_compressed_gzip() -> None:
    """Parses a gzip-compressed sitemap file."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 3

def test_sitemap_empty() -> None:
    """Handles empty sitemap gracefully."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 0

def test_sitemap_from_robots_txt() -> None:
    """Discovers sitemap via robots.txt Sitemap directive."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 4

def test_sitemap_index() -> None:
    """Follows sitemap index to discover child sitemaps."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 3

def test_sitemap_lastmod_filter() -> None:
    """Filters sitemap URLs by lastmod date."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 4
    assert result.has_lastmod is True

def test_sitemap_only_mode() -> None:
    """Uses sitemap URLs exclusively without following page links."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 4

def test_sitemap_xhtml_links() -> None:
    """Parses sitemap with XHTML namespace alternate links."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert len(result.urls) == 2
    assert result.has_lastmod is False

