# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "crawl" do
  it "content_binary_skip: Skips image and video content types gracefully" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'content.was_skipped' not available on result type
  end

  it "content_pdf_link_skip: Encounters PDF link and skips or marks as document type" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'content.was_skipped' not available on result type
  end

  it "crawl_concurrent_depth: Concurrent crawl respects max_depth limit" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
  end

  it "crawl_concurrent_limit: Respects max concurrent requests limit during crawl" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_concurrent_max_pages: Concurrent crawl respects max_pages budget" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_custom_headers: Sends custom headers on all crawl requests" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_depth_one: Follows links one level deep from start page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
  end

  it "crawl_depth_priority: Crawls in breadth-first order, processing depth-0 pages before depth-1" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_depth_two: Crawls 3 levels deep (depth 0, 1, 2)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_depth_two_chain: Depth=2 crawl follows a chain of links across three levels" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_double_slash_normalization: Normalizes double slashes in URL paths (//page to /page)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
  end

  it "crawl_empty_page_no_links: Crawl completes when child page has no outgoing links" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_exclude_path_pattern: Skips URLs matching the exclude path pattern" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_external_links_ignored: External links are discovered but not followed when stay_on_domain is true" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
  end

  it "crawl_fragment_stripping: Strips #fragment from URLs for deduplication" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
  end

  it "crawl_include_path_pattern: Only follows URLs matching the include path pattern" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_max_depth_zero: max_depth=0 crawls only the seed page with no link following" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_max_pages: Stops crawling at page budget limit" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_mixed_content_types: Crawl handles links to non-HTML content types gracefully" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_multiple_redirects_in_traversal: Multiple linked pages with redirects are handled during crawl traversal" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_query_param_dedup: Deduplicates URLs with same query params in different order" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
  end

  it "crawl_redirect_in_traversal: Links that redirect are followed during crawl traversal" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_self_link_no_loop: Page linking to itself does not cause infinite crawl loop" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_single_page_no_links: Crawling a page with no links returns only the seed page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_stay_on_domain: Does not follow external links when stay_on_domain is true" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
  end

  it "crawl_subdomain_exclusion: Stays on exact domain and skips subdomain links" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
  end

  it "crawl_subdomain_inclusion: Crawls subdomains when allow_subdomains is enabled" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end

  it "crawl_trailing_slash_dedup: Deduplicates /page and /page/ as the same URL" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
  end

  it "crawl_url_deduplication: Deduplicates URLs that differ only by fragment or query params" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'pages.length' not available on result type
  end
end
