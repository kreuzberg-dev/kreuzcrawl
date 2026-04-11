"""E2e tests for category: strategy."""

import os

from kreuzcrawl import CrawlConfig, create_engine, scrape


def test_strategy_best_first_seed() -> None:
    """BestFirst strategy always processes the seed URL first."""
    engine_config = CrawlConfig(max_concurrent=1, max_depth=1)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/strategy_best_first_seed"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'strategy.first_page_url_contains' not available on result type


def test_strategy_bfs_default_order() -> None:
    """BFS strategy visits pages in breadth-first order."""
    engine_config = CrawlConfig(max_concurrent=1, max_depth=2)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/strategy_bfs_default_order"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'strategy.crawl_order' not available on result type


def test_strategy_dfs_depth_first() -> None:
    """DFS strategy visits pages in depth-first order."""
    engine_config = CrawlConfig(max_concurrent=1, max_depth=2)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/strategy_dfs_depth_first"
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'strategy.crawl_order' not available on result type
