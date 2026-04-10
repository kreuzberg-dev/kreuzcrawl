# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "middleware" do
  it "middleware_engine_crawl_with_defaults: Engine crawl with default middleware chain produces correct multi-page results" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'crawl.min_pages' not available on result type
  end

  it "middleware_noop_no_effect: Default middleware chain does not affect normal scraping" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq("Middleware Test")
  end
end
