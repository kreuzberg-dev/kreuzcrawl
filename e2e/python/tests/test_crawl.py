"""E2e tests for category: crawl.
"""
from kreuzcrawl import create_engine, scrape


def test_content_binary_skip() -> None:
    """Skips image and video content types gracefully."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'content.was_skipped' not available on result type

def test_content_pdf_link_skip() -> None:
    """Encounters PDF link and skips or marks as document type."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'content.was_skipped' not available on result type

def test_crawl_concurrent_depth() -> None:
    """Concurrent crawl respects max_depth limit."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'stayed_on_domain' not available on result type

def test_crawl_concurrent_limit() -> None:
    """Respects max concurrent requests limit during crawl."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_concurrent_max_pages() -> None:
    """Concurrent crawl respects max_pages budget."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_custom_headers() -> None:
    """Sends custom headers on all crawl requests."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_depth_one() -> None:
    """Follows links one level deep from start page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'stayed_on_domain' not available on result type

def test_crawl_depth_priority() -> None:
    """Crawls in breadth-first order, processing depth-0 pages before depth-1."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_depth_two() -> None:
    """Crawls 3 levels deep (depth 0, 1, 2)."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'pages.length' not available on result type

def test_crawl_depth_two_chain() -> None:
    """Depth=2 crawl follows a chain of links across three levels."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_double_slash_normalization() -> None:
    """Normalizes double slashes in URL paths (//page to /page)."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'unique_urls.length' not available on result type

def test_crawl_empty_page_no_links() -> None:
    """Crawl completes when child page has no outgoing links."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_exclude_path_pattern() -> None:
    """Skips URLs matching the exclude path pattern."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_external_links_ignored() -> None:
    """External links are discovered but not followed when stay_on_domain is true."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'stayed_on_domain' not available on result type

def test_crawl_fragment_stripping() -> None:
    """Strips #fragment from URLs for deduplication."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'unique_urls.length' not available on result type

def test_crawl_include_path_pattern() -> None:
    """Only follows URLs matching the include path pattern."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_max_depth_zero() -> None:
    """max_depth=0 crawls only the seed page with no link following."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'pages.length' not available on result type

def test_crawl_max_pages() -> None:
    """Stops crawling at page budget limit."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_mixed_content_types() -> None:
    """Crawl handles links to non-HTML content types gracefully."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_multiple_redirects_in_traversal() -> None:
    """Multiple linked pages with redirects are handled during crawl traversal."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_query_param_dedup() -> None:
    """Deduplicates URLs with same query params in different order."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'unique_urls.length' not available on result type

def test_crawl_redirect_in_traversal() -> None:
    """Links that redirect are followed during crawl traversal."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_self_link_no_loop() -> None:
    """Page linking to itself does not cause infinite crawl loop."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_single_page_no_links() -> None:
    """Crawling a page with no links returns only the seed page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_stay_on_domain() -> None:
    """Does not follow external links when stay_on_domain is true."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'stayed_on_domain' not available on result type

def test_crawl_subdomain_exclusion() -> None:
    """Stays on exact domain and skips subdomain links."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type
    # skipped: field 'stayed_on_domain' not available on result type

def test_crawl_subdomain_inclusion() -> None:
    """Crawls subdomains when allow_subdomains is enabled."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

def test_crawl_trailing_slash_dedup() -> None:
    """Deduplicates /page and /page/ as the same URL."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'unique_urls.length' not available on result type

def test_crawl_url_deduplication() -> None:
    """Deduplicates URLs that differ only by fragment or query params."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'pages.length' not available on result type

