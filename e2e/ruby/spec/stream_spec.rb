# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "stream" do
  it "crawl_stream_events: Crawl stream produces page and complete events" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.stream.event_count_min).to be >= 4
    expect(result.stream.has_page_event).to eq(true)
    expect(result.stream.has_complete_event).to eq(true)
  end

  it "stream_depth_crawl: Stream produces events for multi-depth crawl with link following" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.stream.event_count_min).to be >= 5
    expect(result.stream.has_page_event).to eq(true)
    expect(result.stream.has_complete_event).to eq(true)
  end

  it "stream_with_error_event: Stream emits page and complete events even when some pages fail" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.stream.has_page_event).to eq(true)
    expect(result.stream.has_complete_event).to eq(true)
    expect(result.stream.event_count_min).to be >= 2
  end
end
