"""E2e tests for category: strategy.
"""
from kreuzcrawl import create_engine, scrape


def test_strategy_best_first_seed() -> None:
    """BestFirst strategy always processes the seed URL first."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'strategy.first_page_url_contains' not available on result type

def test_strategy_bfs_default_order() -> None:
    """BFS strategy visits pages in breadth-first order."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'strategy.crawl_order' not available on result type

def test_strategy_dfs_depth_first() -> None:
    """DFS strategy visits pages in depth-first order."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'strategy.crawl_order' not available on result type

