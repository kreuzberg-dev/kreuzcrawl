# E2e tests for category: strategy
defmodule E2e.StrategyTest do
  use ExUnit.Case, async: true

  describe "strategy_best_first_seed" do
    test "BestFirst strategy always processes the seed URL first" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/strategy_best_first_seed"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'strategy.first_page_url_contains' not available on result type
    end
  end

  describe "strategy_bfs_default_order" do
    test "BFS strategy visits pages in breadth-first order" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/strategy_bfs_default_order"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'strategy.crawl_order' not available on result type
    end
  end

  describe "strategy_dfs_depth_first" do
    test "DFS strategy visits pages in depth-first order" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/strategy_dfs_depth_first"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'strategy.crawl_order' not available on result type
    end
  end
end
