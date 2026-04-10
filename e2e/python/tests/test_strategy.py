"""E2e tests for category: strategy.
"""
from kreuzcrawl import create_engine, scrape


def test_strategy_best_first_seed() -> None:
    """BestFirst strategy always processes the seed URL first."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 3
    assert "/" in result.strategy.first_page_url_contains

def test_strategy_bfs_default_order() -> None:
    """BFS strategy visits pages in breadth-first order."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 5
    assert result.strategy.crawl_order == '["/","/a","/b","/a/1","/b/1"]'

def test_strategy_dfs_depth_first() -> None:
    """DFS strategy visits pages in depth-first order."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.crawl.pages_crawled == 5
    assert result.strategy.crawl_order == '["/","/b","/b/1","/a","/a/1"]'

