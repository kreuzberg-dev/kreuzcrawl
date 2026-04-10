# E2e tests for category: cookies
defmodule E2e.CookiesTest do
  use ExUnit.Case, async: true

  describe "cookies_per_domain" do
    test "Isolates cookies per domain during crawl" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(length(result.cookies)) == 1
      assert String.contains?(result.cookies, "domain_cookie")
    end
  end

  describe "cookies_persistence" do
    test "Maintains cookies across multiple crawl requests" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.contains?(result.cookies, "session")
    end
  end

  describe "cookies_set_cookie_response" do
    test "Respects Set-Cookie header from server responses" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.contains?(result.cookies, "tracking")
    end
  end
end
