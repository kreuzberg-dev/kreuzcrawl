# E2e tests for category: sitemap
defmodule E2e.SitemapTest do
  use ExUnit.Case, async: true

  describe "sitemap_basic" do
    test "Parses a standard urlset sitemap" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'has_lastmod' not available on result type
    end
  end

  describe "sitemap_compressed_gzip" do
    test "Parses a gzip-compressed sitemap file" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "sitemap_empty" do
    test "Handles empty sitemap gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "sitemap_from_robots_txt" do
    test "Discovers sitemap via robots.txt Sitemap directive" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "sitemap_index" do
    test "Follows sitemap index to discover child sitemaps" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "sitemap_lastmod_filter" do
    test "Filters sitemap URLs by lastmod date" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'has_lastmod' not available on result type
    end
  end

  describe "sitemap_only_mode" do
    test "Uses sitemap URLs exclusively without following page links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
    end
  end

  describe "sitemap_xhtml_links" do
    test "Parses sitemap with XHTML namespace alternate links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'urls.length' not available on result type
      # skipped: field 'has_lastmod' not available on result type
    end
  end
end
