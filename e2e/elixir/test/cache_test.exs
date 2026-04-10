# E2e tests for category: cache
defmodule E2e.CacheTest do
  use ExUnit.Case, async: true

  describe "cache_basic" do
    test "Crawling with disk cache enabled succeeds without errors" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
    end
  end
end
