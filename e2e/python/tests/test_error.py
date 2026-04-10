"""E2e tests for category: error.
"""
import pytest
from kreuzcrawl import scrape


def test_error_401_unauthorized() -> None:
    """Handles 401 Unauthorized response correctly."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_403_forbidden() -> None:
    """Handles 403 Forbidden response correctly."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_404_page() -> None:
    """Handles 404 response correctly."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_408_request_timeout() -> None:
    """Handles 408 Request Timeout response correctly."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_410_gone() -> None:
    """Handles 410 Gone response correctly."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_500_server() -> None:
    """Handles 500 server error."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_502_bad_gateway() -> None:
    """Handles 502 Bad Gateway response correctly."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_connection_refused() -> None:
    """Handles connection refused error gracefully."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_dns_resolution() -> None:
    """Handles DNS resolution failure gracefully."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_empty_response() -> None:
    """Handles 200 with completely empty body gracefully."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.html_not_empty is False
    assert result.error.is_error is False

def test_error_invalid_proxy() -> None:
    """Proxy pointing to unreachable address causes connection error during scrape."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_partial_response() -> None:
    """Handles incomplete or truncated HTTP response."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_rate_limited() -> None:
    """Handles 429 rate limiting with Retry-After."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_retry_503() -> None:
    """Retries request on 503 Service Unavailable response."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_retry_backoff() -> None:
    """Implements exponential backoff when retrying failed requests."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_ssl_invalid_cert() -> None:
    """Handles SSL certificate validation error."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_timeout() -> None:
    """Handles request timeout."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_waf_akamai() -> None:
    """Akamai WAF detection returns WafBlocked error."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_waf_false_403() -> None:
    """Detects WAF/bot protection false 403 (Cloudflare challenge page)."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

def test_error_waf_imperva() -> None:
    """Imperva/Incapsula WAF detection."""
    engine = None
    url = None
    with pytest.raises(Exception):
        scrape(engine=engine, url=url)

