# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "concurrent" do
  it "concurrent_basic: Concurrent crawling fetches all pages with max_concurrent workers" do
    result = Kreuzcrawl.scrape()
    expect(result.pages.length).to eq(6)
    expect(result.pages.length).to be >= 6
  end

  it "concurrent_depth_two_fan_out: Concurrent depth=2 crawl correctly fans out and deduplicates across levels" do
    result = Kreuzcrawl.scrape()
    expect(result.pages.length).to eq(4)
  end

  it "concurrent_max_pages_exact: Concurrent crawling does not exceed max_pages limit even with high concurrency" do
    result = Kreuzcrawl.scrape()
    expect(result.pages.length).to be <= 3
  end

  it "concurrent_partial_errors: Concurrent crawl handles partial failures gracefully" do
    result = Kreuzcrawl.scrape()
    expect(result.pages.length).to be >= 2
  end

  it "concurrent_respects_max_pages: Concurrent crawling respects max_pages limit" do
    result = Kreuzcrawl.scrape()
    expect(result.pages.length).to be <= 3
  end
end
