# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "middleware" do
  it "middleware_engine_crawl_with_defaults: Engine crawl with default middleware chain produces correct multi-page results" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.crawl.pages_crawled).to eq(3)
    expect(result.crawl.min_pages).to be >= 3
  end

  it "middleware_noop_no_effect: Default middleware chain does not affect normal scraping" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq("Middleware Test")
  end
end
