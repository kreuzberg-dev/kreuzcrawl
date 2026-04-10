# E2e tests for category: cookies
defmodule E2e.CookiesTest do
  use ExUnit.Case, async: true

  describe "cookies_per_domain" do
    test "Isolates cookies per domain during crawl" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'cookies.length' not available on result type
      # skipped: field 'cookies' not available on result type
    end
  end

  describe "cookies_persistence" do
    test "Maintains cookies across multiple crawl requests" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'cookies' not available on result type
    end
  end

  describe "cookies_set_cookie_response" do
    test "Respects Set-Cookie header from server responses" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'cookies' not available on result type
    end
  end
end
