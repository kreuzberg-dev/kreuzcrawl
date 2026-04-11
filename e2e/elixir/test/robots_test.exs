# E2e tests for category: robots
defmodule E2e.RobotsTest do
  use ExUnit.Case, async: true

  describe "robots_allow_all" do
    test "Permissive robots.txt allows all paths" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_allow_override" do
    test "Allow directive overrides Disallow for specific paths" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_comments_handling" do
    test "Correctly parses robots.txt with inline and line comments" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_crawl_delay" do
    test "Respects crawl-delay directive from robots.txt" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.crawl_delay) == 2
    end
  end

  describe "robots_disallow_path" do
    test "Robots.txt disallows specific paths" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == false
    end
  end

  describe "robots_meta_nofollow" do
    test "Detects nofollow meta robots tag and skips link extraction" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.nofollow_detected) == true
    end
  end

  describe "robots_meta_noindex" do
    test "Detects noindex meta robots tag in HTML page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.noindex_detected) == true
    end
  end

  describe "robots_missing_404" do
    test "Missing robots.txt (404) allows all crawling" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_multiple_user_agents" do
    test "Picks the most specific user-agent block from robots.txt" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_request_rate" do
    test "Parses request-rate directive from robots.txt" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.crawl_delay) == 5
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_sitemap_directive" do
    test "Discovers sitemap URL from Sitemap directive in robots.txt" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == true
    end
  end

  describe "robots_user_agent_specific" do
    test "Matches user-agent specific rules in robots.txt" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == false
    end
  end

  describe "robots_wildcard_paths" do
    test "Handles wildcard Disallow patterns in robots.txt" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.is_allowed) == false
    end
  end

  describe "robots_x_robots_tag" do
    test "Respects X-Robots-Tag HTTP header directives" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.x_robots_tag) == "noindex, nofollow"
      assert String.trim(result.noindex_detected) == true
      assert String.trim(result.nofollow_detected) == true
    end
  end
end
