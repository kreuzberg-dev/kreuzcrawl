# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "stream" do
  it "crawl_stream_events: Crawl stream produces page and complete events" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'stream.event_count_min' not available on result type
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
  end

  it "stream_depth_crawl: Stream produces events for multi-depth crawl with link following" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'stream.event_count_min' not available on result type
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
  end

  it "stream_with_error_event: Stream emits page and complete events even when some pages fail" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
      # skipped: field 'stream.event_count_min' not available on result type
  end
end
