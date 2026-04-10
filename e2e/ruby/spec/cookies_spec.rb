# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "cookies" do
  it "cookies_per_domain: Isolates cookies per domain during crawl" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'cookies.length' not available on result type
      # skipped: field 'cookies' not available on result type
  end

  it "cookies_persistence: Maintains cookies across multiple crawl requests" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'cookies' not available on result type
  end

  it "cookies_set_cookie_response: Respects Set-Cookie header from server responses" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'cookies' not available on result type
  end
end
