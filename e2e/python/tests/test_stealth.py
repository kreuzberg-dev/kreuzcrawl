"""E2e tests for category: stealth."""

import os

from kreuzcrawl import create_engine, scrape


def test_stealth_ua_rotation_config() -> None:
    """User-agent rotation config is accepted and crawl succeeds."""
    engine = create_engine()
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/stealth_ua_rotation_config"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
