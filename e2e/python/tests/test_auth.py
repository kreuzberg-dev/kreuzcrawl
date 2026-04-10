"""E2e tests for category: auth.
"""
from kreuzcrawl import create_engine, scrape


def test_auth_basic_http() -> None:
    """Sends HTTP Basic authentication header."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.auth_header_sent is True
    assert result.status_code == 200

def test_auth_bearer_token() -> None:
    """Sends Bearer token in Authorization header."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.auth_header_sent is True
    assert result.status_code == 200

def test_auth_custom_header() -> None:
    """Sends authentication via custom header (X-API-Key)."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.auth_header_sent is True
    assert result.status_code == 200

