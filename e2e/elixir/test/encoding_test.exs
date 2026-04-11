# E2e tests for category: encoding
defmodule E2e.EncodingTest do
  use ExUnit.Case, async: true

  describe "encoding_double_encoded" do
    test "Handles double-encoded URL characters (%25C3%25B6)" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/encoding_double_encoded"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.html != ""
      assert length(result.links) >= 1
    end
  end

  describe "encoding_mixed_charset_page" do
    test "Handles charset mismatch between HTTP header and HTML meta tag" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/encoding_mixed_charset_page"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.html != ""
    end
  end

  describe "encoding_percent_encoded_path" do
    test "Handles percent-encoded spaces and characters in URL paths" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/encoding_percent_encoded_path"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.html != ""
      assert length(result.links) >= 2
    end
  end

  describe "encoding_unicode_url" do
    test "Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/encoding_unicode_url"
      result = Kreuzcrawl.scrape!(engine, url)
      assert result.html != ""
    end
  end
end
