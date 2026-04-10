# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "engine" do
  it "engine_batch_basic: CrawlEngine with defaults batch scrapes like the free function" do
    result = Kreuzcrawl.scrape()
    expect(result.batch.completed_count).to eq(2)
    expect(result.batch.total_count).to eq(2)
  end

  it "engine_crawl_basic: CrawlEngine with defaults crawls multiple pages like the free function" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(3)
    expect(result.crawl.min_pages).to be >= 3
  end

  it "engine_map_basic: CrawlEngine with defaults discovers URLs like the free function" do
    result = Kreuzcrawl.scrape()
    expect(result.map.min_urls).to be >= 2
  end

  it "engine_scrape_basic: CrawlEngine with defaults scrapes a page identically to the free function" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.content_type).to eq("text/html")
    expect(result.metadata.title).to eq("Engine Test")
    expect(result.metadata.description_contains).to include("Testing the engine")
    expect(result.links.min_count).to be >= 1
    expect(result.headings.h1_count).to eq(1)
    expect(result.headings.h1_text).to eq("Hello Engine")
  end

  it "engine_stream_basic: CrawlEngine with defaults streams events like the free function" do
    result = Kreuzcrawl.scrape()
    expect(result.stream.has_page_event).to eq(true)
    expect(result.stream.has_complete_event).to eq(true)
    expect(result.stream.event_count_min).to be >= 3
  end
end
