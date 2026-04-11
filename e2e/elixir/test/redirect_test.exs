# E2e tests for category: redirect
defmodule E2e.RedirectTest do
  use ExUnit.Case, async: true

  describe "redirect_301_permanent" do
    test "Follows 301 permanent redirect and returns final page content" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_301_permanent"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_302_found" do
    test "Follows 302 Found redirect correctly" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_302_found"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_303_see_other" do
    test "Follows 303 See Other redirect (method changes to GET)" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_303_see_other"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_307_temporary" do
    test "Follows 307 Temporary Redirect (preserves method)" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_307_temporary"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_308_permanent" do
    test "Follows 308 Permanent Redirect (preserves method)" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_308_permanent"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_chain" do
    test "Follows a chain of redirects (301 -> 302 -> 200)" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_chain"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_cross_domain" do
    test "Reports cross-domain redirect target without following to external domain" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_cross_domain"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_loop" do
    test "Detects redirect loop (A -> B -> A) and returns error" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_loop"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'is_error' not available on result type
    end
  end

  describe "redirect_max_exceeded" do
    test "Aborts when redirect count exceeds max_redirects limit" do
      engine_config = %{"max_redirects" => 2, "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_max_exceeded"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'is_error' not available on result type
    end
  end

  describe "redirect_meta_refresh" do
    test "Follows HTML meta-refresh redirect to target page" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_meta_refresh"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_refresh_header" do
    test "Handles HTTP Refresh header redirect" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_refresh_header"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_to_404" do
    test "Redirect target returns 404 Not Found" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/redirect_to_404"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
      # skipped: field 'is_error' not available on result type
    end
  end
end
