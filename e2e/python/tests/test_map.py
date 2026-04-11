"""E2e tests for category: map."""

import os

from kreuzcrawl import create_engine, scrape


def test_map_discover_urls() -> None:
    """Discovers all URLs on a site without fetching full content."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/map_discover_urls"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_map_exclude_patterns() -> None:
    """Excludes URLs matching patterns from URL map."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/map_exclude_patterns"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_map_include_subdomains() -> None:
    """Includes subdomain URLs in URL map discovery."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/map_include_subdomains"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type
    # skipped: field 'urls' not available on result type


def test_map_large_sitemap() -> None:
    """Handles large sitemap with 100+ URLs."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/map_large_sitemap"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_map_limit_pagination() -> None:
    """Limits map result count to specified maximum."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/map_limit_pagination"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type


def test_map_search_filter() -> None:
    """Filters map results by search keyword."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/map_search_filter"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'urls.length' not available on result type
    # skipped: field 'urls' not available on result type
