# E2e tests for category: middleware
defmodule E2e.MiddlewareTest do
  use ExUnit.Case, async: true

  describe "middleware_engine_crawl_with_defaults" do
    test "Engine crawl with default middleware chain produces correct multi-page results" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'crawl.min_pages' not available on result type
    end
  end

  describe "middleware_noop_no_effect" do
    test "Default middleware chain does not affect normal scraping" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(result.metadata.title) == "Middleware Test"
    end
  end
end
