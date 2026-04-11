# E2e tests for category: content
defmodule E2e.ContentTest do
  use ExUnit.Case, async: true

  describe "content_204_no_content" do
    test "Handles 204 No Content response gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_204_no_content"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.status_code) == 204
      assert String.trim(result.html) == ""
    end
  end

  describe "content_charset_iso8859" do
    test "Handles ISO-8859-1 encoded page correctly" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_charset_iso8859"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.detected_charset) == "iso-8859-1"
    end
  end

  describe "content_empty_body" do
    test "Handles 200 response with empty body gracefully" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_empty_body"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.status_code) == 200
    end
  end

  describe "content_gzip_compressed" do
    test "Handles response with Accept-Encoding gzip negotiation" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_gzip_compressed"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.html != ""
      assert String.trim(result.status_code) == 200
    end
  end

  describe "content_large_page_limit" do
    test "Respects max body size limit and truncates or skips oversized pages" do
      engine_config = %{"max_body_size" => 1024, "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_large_page_limit"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.body_size < 1025
    end
  end

  describe "content_main_only" do
    test "Extracts only main content area, excluding nav, sidebar, footer" do
      engine_config = %{"main_content_only" => true, "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_main_only"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.main_content_only) == true
    end
  end

  describe "content_pdf_no_extension" do
    test "Detects PDF content by Content-Type header when URL has no .pdf extension" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_pdf_no_extension"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.is_pdf) == true
    end
  end

  describe "content_remove_tags" do
    test "Removes specified HTML elements by CSS selector before processing" do
      engine_config = %{"remove_tags" => ["nav", "aside", "footer"], "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_remove_tags"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.html != ""
    end
  end

  describe "content_utf8_bom" do
    test "Handles UTF-8 content with BOM marker correctly" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/content_utf8_bom"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.detected_charset) == "utf-8"
      assert result.html != ""
    end
  end
end
