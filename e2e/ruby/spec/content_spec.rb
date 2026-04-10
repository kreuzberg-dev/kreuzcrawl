# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "content" do
  it "content_204_no_content: Handles 204 No Content response gracefully" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(204)
    expect(result.html).to be_empty
  end

  it "content_charset_iso8859: Handles ISO-8859-1 encoded page correctly" do
    result = Kreuzcrawl.scrape()
    expect(result.content.detected_charset).to eq("iso-8859-1")
  end

  it "content_empty_body: Handles 200 response with empty body gracefully" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
  end

  it "content_gzip_compressed: Handles response with Accept-Encoding gzip negotiation" do
    result = Kreuzcrawl.scrape()
    expect(result.html).not_to be_empty
    expect(result.status_code).to eq(200)
  end

  it "content_large_page_limit: Respects max body size limit and truncates or skips oversized pages" do
    result = Kreuzcrawl.scrape()
    expect(result.content.body_size).to be < 1025
  end

  it "content_main_only: Extracts only main content area, excluding nav, sidebar, footer" do
    result = Kreuzcrawl.scrape()
    expect(result.content.main_content_only).to eq(true)
  end

  it "content_pdf_no_extension: Detects PDF content by Content-Type header when URL has no .pdf extension" do
    result = Kreuzcrawl.scrape()
    expect(result.content.is_pdf).to eq(true)
  end

  it "content_remove_tags: Removes specified HTML elements by CSS selector before processing" do
    result = Kreuzcrawl.scrape()
    expect(result.html).not_to be_empty
  end

  it "content_utf8_bom: Handles UTF-8 content with BOM marker correctly" do
    result = Kreuzcrawl.scrape()
    expect(result.content.detected_charset).to eq("utf-8")
    expect(result.html).not_to be_empty
  end
end
