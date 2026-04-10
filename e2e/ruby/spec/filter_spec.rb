# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "filter" do
  it "filter_bm25_crawl_integration: BM25 filter works during multi-page crawl, keeping relevant pages" do
    result = Kreuzcrawl.scrape()
    expect(result.filter.remaining_contain_keyword).to include("rust")
  end

  it "filter_bm25_empty_query: BM25 filter with empty query passes all pages through" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(2)
  end

  it "filter_bm25_high_threshold: BM25 filter with very high threshold filters out all pages" do
    result = Kreuzcrawl.scrape()
    expect(result.filter.pages_after_filter).to eq(0)
  end

  it "filter_bm25_relevant_pages: BM25 filter keeps only pages relevant to the query" do
    result = Kreuzcrawl.scrape()
    expect(result.filter.remaining_contain_keyword).to include("rust")
  end

  it "filter_bm25_threshold_zero: BM25 filter with zero threshold passes all pages" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(2)
  end

  it "filter_noop_crawl_all_kept: NoopFilter keeps all pages during a multi-page crawl" do
    result = Kreuzcrawl.scrape()
    expect(result.filter.pages_after_filter).to eq(3)
  end

  it "filter_noop_passes_all: No content filter passes all crawled pages through" do
    result = Kreuzcrawl.scrape()
    expect(result.crawl.pages_crawled).to eq(3)
  end
end
