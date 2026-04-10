# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "cache" do
  it "cache_basic: Crawling with disk cache enabled succeeds without errors" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
  end
end
