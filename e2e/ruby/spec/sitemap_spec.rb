# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "sitemap" do
  it "sitemap_basic: Parses a standard urlset sitemap" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(4)
    expect(result.has_lastmod).to eq(true)
  end

  it "sitemap_compressed_gzip: Parses a gzip-compressed sitemap file" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(3)
  end

  it "sitemap_empty: Handles empty sitemap gracefully" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(0)
  end

  it "sitemap_from_robots_txt: Discovers sitemap via robots.txt Sitemap directive" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(4)
  end

  it "sitemap_index: Follows sitemap index to discover child sitemaps" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(3)
  end

  it "sitemap_lastmod_filter: Filters sitemap URLs by lastmod date" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(4)
    expect(result.has_lastmod).to eq(true)
  end

  it "sitemap_only_mode: Uses sitemap URLs exclusively without following page links" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(4)
  end

  it "sitemap_xhtml_links: Parses sitemap with XHTML namespace alternate links" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.urls.length).to eq(2)
    expect(result.has_lastmod).to eq(false)
  end
end
