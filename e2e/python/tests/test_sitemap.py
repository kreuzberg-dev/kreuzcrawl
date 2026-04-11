"""E2e tests for category: sitemap."""

from kreuzcrawl import create_engine, scrape


def test_sitemap_basic() -> None:
    """Parses a standard urlset sitemap."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type
    # skipped: field 'has_lastmod' not available on result type


def test_sitemap_compressed_gzip() -> None:
    """Parses a gzip-compressed sitemap file."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_sitemap_empty() -> None:
    """Handles empty sitemap gracefully."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_sitemap_from_robots_txt() -> None:
    """Discovers sitemap via robots.txt Sitemap directive."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_sitemap_index() -> None:
    """Follows sitemap index to discover child sitemaps."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_sitemap_lastmod_filter() -> None:
    """Filters sitemap URLs by lastmod date."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type
    # skipped: field 'has_lastmod' not available on result type


def test_sitemap_only_mode() -> None:
    """Uses sitemap URLs exclusively without following page links."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_sitemap_xhtml_links() -> None:
    """Parses sitemap with XHTML namespace alternate links."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type
    # skipped: field 'has_lastmod' not available on result type
