# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "cookies" do
  it "cookies_per_domain: Isolates cookies per domain during crawl" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.cookies.length).to eq(1)
    expect(result.cookies).to include("domain_cookie")
  end

  it "cookies_persistence: Maintains cookies across multiple crawl requests" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.cookies).to include("session")
  end

  it "cookies_set_cookie_response: Respects Set-Cookie header from server responses" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.cookies).to include("tracking")
  end
end
