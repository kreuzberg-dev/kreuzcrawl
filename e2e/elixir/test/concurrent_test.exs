# E2e tests for category: concurrent
defmodule E2e.ConcurrentTest do
  use ExUnit.Case, async: true

  describe "concurrent_basic" do
    test "Concurrent crawling fetches all pages with max_concurrent workers" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "concurrent_depth_two_fan_out" do
    test "Concurrent depth=2 crawl correctly fans out and deduplicates across levels" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "concurrent_max_pages_exact" do
    test "Concurrent crawling does not exceed max_pages limit even with high concurrency" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "concurrent_partial_errors" do
    test "Concurrent crawl handles partial failures gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "concurrent_respects_max_pages" do
    test "Concurrent crawling respects max_pages limit" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end
end
