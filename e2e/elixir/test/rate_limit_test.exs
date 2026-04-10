# E2e tests for category: rate_limit
defmodule E2e.RateLimitTest do
  use ExUnit.Case, async: true

  describe "rate_limit_basic_delay" do
    test "Rate limiter adds delay between requests to the same domain" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'rate_limit.min_duration_ms' not available on result type
    end
  end

  describe "rate_limit_zero_no_delay" do
    test "Rate limiter with zero delay does not slow crawling" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
    end
  end
end
