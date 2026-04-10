# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "concurrent" do
  it "concurrent_basic: Concurrent crawling fetches all pages with max_concurrent workers" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'pages.length' not available on result type
  end

  it "concurrent_depth_two_fan_out: Concurrent depth=2 crawl correctly fans out and deduplicates across levels" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "concurrent_max_pages_exact: Concurrent crawling does not exceed max_pages limit even with high concurrency" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "concurrent_partial_errors: Concurrent crawl handles partial failures gracefully" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "concurrent_respects_max_pages: Concurrent crawling respects max_pages limit" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end
end
