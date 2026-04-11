"""E2e tests for category: redirect."""

from kreuzcrawl import create_engine, scrape


def test_redirect_301_permanent() -> None:
    """Follows 301 permanent redirect and returns final page content."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_302_found() -> None:
    """Follows 302 Found redirect correctly."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_303_see_other() -> None:
    """Follows 303 See Other redirect (method changes to GET)."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_307_temporary() -> None:
    """Follows 307 Temporary Redirect (preserves method)."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_308_permanent() -> None:
    """Follows 308 Permanent Redirect (preserves method)."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_chain() -> None:
    """Follows a chain of redirects (301 -> 302 -> 200)."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_cross_domain() -> None:
    """Reports cross-domain redirect target without following to external domain."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_loop() -> None:
    """Detects redirect loop (A -> B -> A) and returns error."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'is_error' not available on result type


def test_redirect_max_exceeded() -> None:
    """Aborts when redirect count exceeds max_redirects limit."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'is_error' not available on result type


def test_redirect_meta_refresh() -> None:
    """Follows HTML meta-refresh redirect to target page."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_refresh_header() -> None:
    """Handles HTTP Refresh header redirect."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type


def test_redirect_to_404() -> None:
    """Redirect target returns 404 Not Found."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'final_url' not available on result type
    # skipped: field 'redirect_count' not available on result type
    # skipped: field 'is_error' not available on result type
