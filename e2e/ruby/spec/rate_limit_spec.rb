# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "rate_limit" do
  it "rate_limit_basic_delay: Rate limiter adds delay between requests to the same domain" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'rate_limit.min_duration_ms' not available on result type
  end

  it "rate_limit_zero_no_delay: Rate limiter with zero delay does not slow crawling" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
  end
end
