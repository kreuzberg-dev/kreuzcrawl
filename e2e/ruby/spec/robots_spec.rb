# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "robots" do
  it "robots_allow_all: Permissive robots.txt allows all paths" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(true)
  end

  it "robots_allow_override: Allow directive overrides Disallow for specific paths" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(true)
  end

  it "robots_comments_handling: Correctly parses robots.txt with inline and line comments" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(true)
  end

  it "robots_crawl_delay: Respects crawl-delay directive from robots.txt" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.crawl_delay).to eq(2)
  end

  it "robots_disallow_path: Robots.txt disallows specific paths" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(false)
  end

  it "robots_meta_nofollow: Detects nofollow meta robots tag and skips link extraction" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.nofollow_detected).to eq(true)
  end

  it "robots_meta_noindex: Detects noindex meta robots tag in HTML page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.noindex_detected).to eq(true)
  end

  it "robots_missing_404: Missing robots.txt (404) allows all crawling" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(true)
  end

  it "robots_multiple_user_agents: Picks the most specific user-agent block from robots.txt" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(true)
  end

  it "robots_request_rate: Parses request-rate directive from robots.txt" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.crawl_delay).to eq(5)
    expect(result.is_allowed).to eq(true)
  end

  it "robots_sitemap_directive: Discovers sitemap URL from Sitemap directive in robots.txt" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(true)
  end

  it "robots_user_agent_specific: Matches user-agent specific rules in robots.txt" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(false)
  end

  it "robots_wildcard_paths: Handles wildcard Disallow patterns in robots.txt" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_allowed).to eq(false)
  end

  it "robots_x_robots_tag: Respects X-Robots-Tag HTTP header directives" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.x_robots_tag).to eq("noindex, nofollow")
    expect(result.noindex_detected).to eq(true)
    expect(result.nofollow_detected).to eq(true)
  end
end
