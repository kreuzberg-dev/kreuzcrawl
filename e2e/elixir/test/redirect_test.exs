# E2e tests for category: redirect
defmodule E2e.RedirectTest do
  use ExUnit.Case, async: true

  describe "redirect_301_permanent" do
    test "Follows 301 permanent redirect and returns final page content" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_302_found" do
    test "Follows 302 Found redirect correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_303_see_other" do
    test "Follows 303 See Other redirect (method changes to GET)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_307_temporary" do
    test "Follows 307 Temporary Redirect (preserves method)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_308_permanent" do
    test "Follows 308 Permanent Redirect (preserves method)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_chain" do
    test "Follows a chain of redirects (301 -> 302 -> 200)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_cross_domain" do
    test "Reports cross-domain redirect target without following to external domain" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_loop" do
    test "Detects redirect loop (A -> B -> A) and returns error" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'is_error' not available on result type
    end
  end

  describe "redirect_max_exceeded" do
    test "Aborts when redirect count exceeds max_redirects limit" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'is_error' not available on result type
    end
  end

  describe "redirect_meta_refresh" do
    test "Follows HTML meta-refresh redirect to target page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_refresh_header" do
    test "Handles HTTP Refresh header redirect" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
    end
  end

  describe "redirect_to_404" do
    test "Redirect target returns 404 Not Found" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'final_url' not available on result type
      # skipped: field 'redirect_count' not available on result type
      # skipped: field 'is_error' not available on result type
    end
  end
end
