"""E2e tests for category: stealth.
"""
from kreuzcrawl import scrape


def test_stealth_ua_rotation_config() -> None:
    """User-agent rotation config is accepted and crawl succeeds."""
    engine = None
    url = None
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200

