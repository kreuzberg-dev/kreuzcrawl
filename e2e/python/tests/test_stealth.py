"""E2e tests for category: stealth."""

import os

from kreuzcrawl import create_engine, scrape


def test_stealth_ua_rotation_config() -> None:
    """User-agent rotation config is accepted and crawl succeeds."""
    engine_config = {"user_agents": ["Mozilla/5.0 (Windows NT 10.0)", "Chrome/120.0.0.0"]}
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/stealth_ua_rotation_config"
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
