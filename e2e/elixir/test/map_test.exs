# E2e tests for category: map
defmodule E2e.MapTest do
  use ExUnit.Case, async: true

  describe "map_discover_urls" do
    test "Discovers all URLs on a site without fetching full content" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/map_discover_urls"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "map_exclude_patterns" do
    test "Excludes URLs matching patterns from URL map" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/map_exclude_patterns"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "map_include_subdomains" do
    test "Includes subdomain URLs in URL map discovery" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/map_include_subdomains"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'urls' not available on result type
    end
  end

  describe "map_large_sitemap" do
    test "Handles large sitemap with 100+ URLs" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/map_large_sitemap"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "map_limit_pagination" do
    test "Limits map result count to specified maximum" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/map_limit_pagination"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "map_search_filter" do
    test "Filters map results by search keyword" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/map_search_filter"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'urls' not available on result type
    end
  end
end
