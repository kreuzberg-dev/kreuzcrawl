# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "strategy" do
  it "strategy_best_first_seed: BestFirst strategy always processes the seed URL first" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(3)
    expect(result.strategy.first_page_url_contains).to include("/")
  end

  it "strategy_bfs_default_order: BFS strategy visits pages in breadth-first order" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(5)
    expect(result.strategy.crawl_order).to eq(["/", "/a", "/b", "/a/1", "/b/1"])
  end

  it "strategy_dfs_depth_first: DFS strategy visits pages in depth-first order" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(5)
    expect(result.strategy.crawl_order).to eq(["/", "/b", "/b/1", "/a", "/a/1"])
  end
end
