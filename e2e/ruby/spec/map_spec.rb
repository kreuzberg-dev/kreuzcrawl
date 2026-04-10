# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "map" do
  it "map_discover_urls: Discovers all URLs on a site without fetching full content" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to be >= 3
  end

  it "map_exclude_patterns: Excludes URLs matching patterns from URL map" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(1)
  end

  it "map_include_subdomains: Includes subdomain URLs in URL map discovery" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to be >= 2
    expect(result.urls).to include("blog.example.com")
  end

  it "map_large_sitemap: Handles large sitemap with 100+ URLs" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to be >= 100
  end

  it "map_limit_pagination: Limits map result count to specified maximum" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to be <= 5
  end

  it "map_search_filter: Filters map results by search keyword" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to be >= 2
    expect(result.urls).to include("blog")
  end
end
