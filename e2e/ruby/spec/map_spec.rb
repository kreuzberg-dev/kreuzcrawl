# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "map" do
  it "map_discover_urls: Discovers all URLs on a site without fetching full content" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'urls.length' not available on result type
  end

  it "map_exclude_patterns: Excludes URLs matching patterns from URL map" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'urls.length' not available on result type
  end

  it "map_include_subdomains: Includes subdomain URLs in URL map discovery" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'urls' not available on result type
  end

  it "map_large_sitemap: Handles large sitemap with 100+ URLs" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'urls.length' not available on result type
  end

  it "map_limit_pagination: Limits map result count to specified maximum" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'urls.length' not available on result type
  end

  it "map_search_filter: Filters map results by search keyword" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'urls' not available on result type
  end
end
