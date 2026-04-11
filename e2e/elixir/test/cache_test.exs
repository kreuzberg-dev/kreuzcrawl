# E2e tests for category: cache
defmodule E2e.CacheTest do
  use ExUnit.Case, async: true

  describe "cache_basic" do
    test "Crawling with disk cache enabled succeeds without errors" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/cache_basic"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.status_code) == 200
    end
  end
end
