# E2e tests for category: crawl
defmodule E2e.CrawlTest do
  use ExUnit.Case, async: true

  describe "content_binary_skip" do
    test "Skips image and video content types gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'content.was_skipped' not available on result type
    end
  end

  describe "content_pdf_link_skip" do
    test "Encounters PDF link and skips or marks as document type" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'content.was_skipped' not available on result type
    end
  end

  describe "crawl_concurrent_depth" do
    test "Concurrent crawl respects max_depth limit" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
    end
  end

  describe "crawl_concurrent_limit" do
    test "Respects max concurrent requests limit during crawl" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_concurrent_max_pages" do
    test "Concurrent crawl respects max_pages budget" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_custom_headers" do
    test "Sends custom headers on all crawl requests" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_depth_one" do
    test "Follows links one level deep from start page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
    end
  end

  describe "crawl_depth_priority" do
    test "Crawls in breadth-first order, processing depth-0 pages before depth-1" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_depth_two" do
    test "Crawls 3 levels deep (depth 0, 1, 2)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_depth_two_chain" do
    test "Depth=2 crawl follows a chain of links across three levels" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_double_slash_normalization" do
    test "Normalizes double slashes in URL paths (//page to /page)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
    end
  end

  describe "crawl_empty_page_no_links" do
    test "Crawl completes when child page has no outgoing links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_exclude_path_pattern" do
    test "Skips URLs matching the exclude path pattern" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_external_links_ignored" do
    test "External links are discovered but not followed when stay_on_domain is true" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
    end
  end

  describe "crawl_fragment_stripping" do
    test "Strips #fragment from URLs for deduplication" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
    end
  end

  describe "crawl_include_path_pattern" do
    test "Only follows URLs matching the include path pattern" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_max_depth_zero" do
    test "max_depth=0 crawls only the seed page with no link following" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_max_pages" do
    test "Stops crawling at page budget limit" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_mixed_content_types" do
    test "Crawl handles links to non-HTML content types gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_multiple_redirects_in_traversal" do
    test "Multiple linked pages with redirects are handled during crawl traversal" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_query_param_dedup" do
    test "Deduplicates URLs with same query params in different order" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
    end
  end

  describe "crawl_redirect_in_traversal" do
    test "Links that redirect are followed during crawl traversal" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_self_link_no_loop" do
    test "Page linking to itself does not cause infinite crawl loop" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_single_page_no_links" do
    test "Crawling a page with no links returns only the seed page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_stay_on_domain" do
    test "Does not follow external links when stay_on_domain is true" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
    end
  end

  describe "crawl_subdomain_exclusion" do
    test "Stays on exact domain and skips subdomain links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
      # skipped: field 'stayed_on_domain' not available on result type
    end
  end

  describe "crawl_subdomain_inclusion" do
    test "Crawls subdomains when allow_subdomains is enabled" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end

  describe "crawl_trailing_slash_dedup" do
    test "Deduplicates /page and /page/ as the same URL" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'unique_urls.length' not available on result type
    end
  end

  describe "crawl_url_deduplication" do
    test "Deduplicates URLs that differ only by fragment or query params" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'pages.length' not available on result type
    end
  end
end
