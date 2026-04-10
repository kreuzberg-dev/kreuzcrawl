# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "redirect" do
  it "redirect_301_permanent: Follows 301 permanent redirect and returns final page content" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/target")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_302_found: Follows 302 Found redirect correctly" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/found-target")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_303_see_other: Follows 303 See Other redirect (method changes to GET)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/see-other")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_307_temporary: Follows 307 Temporary Redirect (preserves method)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/temp-target")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_308_permanent: Follows 308 Permanent Redirect (preserves method)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/perm-target")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_chain: Follows a chain of redirects (301 -> 302 -> 200)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/step2")
    expect(result.redirect_count).to eq(2)
  end

  it "redirect_cross_domain: Reports cross-domain redirect target without following to external domain" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/external-redirect")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_loop: Detects redirect loop (A -> B -> A) and returns error" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_error).to eq(true)
  end

  it "redirect_max_exceeded: Aborts when redirect count exceeds max_redirects limit" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.is_error).to eq(true)
  end

  it "redirect_meta_refresh: Follows HTML meta-refresh redirect to target page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/target")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_refresh_header: Handles HTTP Refresh header redirect" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/refreshed")
    expect(result.redirect_count).to eq(1)
  end

  it "redirect_to_404: Redirect target returns 404 Not Found" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.final_url).to include("/gone")
    expect(result.redirect_count).to eq(1)
    expect(result.is_error).to eq(true)
  end
end
