# E2e tests for category: content
defmodule E2e.ContentTest do
  use ExUnit.Case, async: true

  describe "content_204_no_content" do
    test "Handles 204 No Content response gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 204
      assert String.trim(result.html) == ""
    end
  end

  describe "content_charset_iso8859" do
    test "Handles ISO-8859-1 encoded page correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.detected_charset) == "iso-8859-1"
    end
  end

  describe "content_empty_body" do
    test "Handles 200 response with empty body gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
    end
  end

  describe "content_gzip_compressed" do
    test "Handles response with Accept-Encoding gzip negotiation" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert result.html != ""
      assert String.trim(result.status_code) == 200
    end
  end

  describe "content_large_page_limit" do
    test "Respects max body size limit and truncates or skips oversized pages" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert result.body_size < 1025
    end
  end

  describe "content_main_only" do
    test "Extracts only main content area, excluding nav, sidebar, footer" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.main_content_only) == true
    end
  end

  describe "content_pdf_no_extension" do
    test "Detects PDF content by Content-Type header when URL has no .pdf extension" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_pdf) == true
    end
  end

  describe "content_remove_tags" do
    test "Removes specified HTML elements by CSS selector before processing" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert result.html != ""
    end
  end

  describe "content_utf8_bom" do
    test "Handles UTF-8 content with BOM marker correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.detected_charset) == "utf-8"
      assert result.html != ""
    end
  end
end
