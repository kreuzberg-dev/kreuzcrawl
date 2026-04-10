# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "rate_limit" do
  it "rate_limit_basic_delay: Rate limiter adds delay between requests to the same domain" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.crawl.pages_crawled).to eq(3)
    expect(result.rate_limit.min_duration_ms).to be >= 150
  end

  it "rate_limit_zero_no_delay: Rate limiter with zero delay does not slow crawling" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.crawl.pages_crawled).to eq(2)
  end
end
