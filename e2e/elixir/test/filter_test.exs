# E2e tests for category: filter
defmodule E2e.FilterTest do
  use ExUnit.Case, async: true

  describe "filter_bm25_crawl_integration" do
    test "BM25 filter works during multi-page crawl, keeping relevant pages" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'filter.remaining_contain_keyword' not available on result type
    end
  end

  describe "filter_bm25_empty_query" do
    test "BM25 filter with empty query passes all pages through" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
    end
  end

  describe "filter_bm25_high_threshold" do
    test "BM25 filter with very high threshold filters out all pages" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'filter.pages_after_filter' not available on result type
    end
  end

  describe "filter_bm25_relevant_pages" do
    test "BM25 filter keeps only pages relevant to the query" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'filter.remaining_contain_keyword' not available on result type
    end
  end

  describe "filter_bm25_threshold_zero" do
    test "BM25 filter with zero threshold passes all pages" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
    end
  end

  describe "filter_noop_crawl_all_kept" do
    test "NoopFilter keeps all pages during a multi-page crawl" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'filter.pages_after_filter' not available on result type
    end
  end

  describe "filter_noop_passes_all" do
    test "No content filter passes all crawled pages through" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
    end
  end
end
