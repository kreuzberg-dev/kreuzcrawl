"""E2e tests for category: auth."""

import os

from kreuzcrawl import create_engine, scrape


def test_auth_basic_http() -> None:
    """Sends HTTP Basic authentication header."""
    engine_config = {
        "auth": {"password": "testpass", "type": "basic", "username": "testuser"},
        "respect_robots_txt": False,
    }
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/auth_basic_http"
    result = scrape(engine=engine, url=url)
    assert result.auth_header_sent is True
    assert result.status_code == 200


def test_auth_bearer_token() -> None:
    """Sends Bearer token in Authorization header."""
    engine_config = {
        "auth": {"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test", "type": "bearer"},
        "respect_robots_txt": False,
    }
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/auth_bearer_token"
    result = scrape(engine=engine, url=url)
    assert result.auth_header_sent is True
    assert result.status_code == 200


def test_auth_custom_header() -> None:
    """Sends authentication via custom header (X-API-Key)."""
    engine_config = {
        "auth": {"name": "X-API-Key", "type": "header", "value": "sk-test-key-12345"},
        "respect_robots_txt": False,
    }
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/auth_custom_header"
    result = scrape(engine=engine, url=url)
    assert result.auth_header_sent is True
    assert result.status_code == 200
