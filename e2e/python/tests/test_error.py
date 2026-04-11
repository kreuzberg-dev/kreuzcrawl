"""E2e tests for category: error."""

import os

import pytest
from kreuzcrawl import create_engine, scrape


def test_error_401_unauthorized() -> None:
    """Handles 401 Unauthorized response correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_401_unauthorized"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_403_forbidden() -> None:
    """Handles 403 Forbidden response correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_403_forbidden"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_404_page() -> None:
    """Handles 404 response correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_404_page"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_408_request_timeout() -> None:
    """Handles 408 Request Timeout response correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_408_request_timeout"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_410_gone() -> None:
    """Handles 410 Gone response correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_410_gone"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_500_server() -> None:
    """Handles 500 server error."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_500_server"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_502_bad_gateway() -> None:
    """Handles 502 Bad Gateway response correctly."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_502_bad_gateway"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_connection_refused() -> None:
    """Handles connection refused error gracefully."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_connection_refused"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_dns_resolution() -> None:
    """Handles DNS resolution failure gracefully."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_dns_resolution"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_empty_response() -> None:
    """Handles 200 with completely empty body gracefully."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_empty_response"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'html_not_empty' not available on result type
    # skipped: field 'error.is_error' not available on result type


def test_error_invalid_proxy() -> None:
    """Proxy pointing to unreachable address causes connection error during scrape."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_invalid_proxy"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_partial_response() -> None:
    """Handles incomplete or truncated HTTP response."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_partial_response"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_rate_limited() -> None:
    """Handles 429 rate limiting with Retry-After."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_rate_limited"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_retry_503() -> None:
    """Retries request on 503 Service Unavailable response."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_retry_503"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_retry_backoff() -> None:
    """Implements exponential backoff when retrying failed requests."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_retry_backoff"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_ssl_invalid_cert() -> None:
    """Handles SSL certificate validation error."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_ssl_invalid_cert"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_timeout() -> None:
    """Handles request timeout."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_timeout"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_waf_akamai() -> None:
    """Akamai WAF detection returns WafBlocked error."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_waf_akamai"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_waf_false_403() -> None:
    """Detects WAF/bot protection false 403 (Cloudflare challenge page)."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_waf_false_403"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)


def test_error_waf_imperva() -> None:
    """Imperva/Incapsula WAF detection."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/error_waf_imperva"
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)
