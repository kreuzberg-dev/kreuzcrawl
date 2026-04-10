# E2e tests for category: links
defmodule E2e.LinksTest do
  use ExUnit.Case, async: true

  describe "links_anchor_fragment" do
    test "Identifies fragment-only links as anchor type" do
      result = Kreuzcrawl.scrape!()
      assert String.contains?(result.links.get("").link_type, "anchor")
    end
  end

  describe "links_base_tag" do
    test "Resolves relative URLs using base tag href" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 2
      assert String.contains?(result.links.get("").url, "example.com")
    end
  end

  describe "links_document_types" do
    test "Detects PDF, DOCX, XLSX links as document type" do
      result = Kreuzcrawl.scrape!()
      assert String.contains?(result.links.get("").link_type, "document")
    end
  end

  describe "links_empty_href" do
    test "Handles empty href attributes without errors" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 0
      assert String.contains?(result.links.get("").url, "/valid")
    end
  end

  describe "links_internal_external_classification" do
    test "Correctly classifies internal vs external links by domain" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 4
      assert String.contains?(result.links.get("").link_type, "internal")
      assert String.contains?(result.links.get("").link_type, "external")
    end
  end

  describe "links_mailto_javascript_skip" do
    test "Skips mailto:, javascript:, and tel: scheme links" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 0
      refute String.contains?(result.links.get("").url, "mailto:")
    end
  end

  describe "links_protocol_relative" do
    test "Handles protocol-relative URLs (//example.com) correctly" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 1
      assert result.links.get("").protocol_relative != ""
    end
  end

  describe "links_rel_attributes" do
    test "Preserves rel=nofollow and rel=canonical attributes" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 0
    end
  end

  describe "links_relative_parent" do
    test "Resolves ../ and ./ relative parent path links correctly" do
      result = Kreuzcrawl.scrape!()
      assert length(result.links) > 3
    end
  end
end
