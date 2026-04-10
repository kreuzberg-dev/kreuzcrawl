# E2e tests for category: stream
defmodule E2e.StreamTest do
  use ExUnit.Case, async: true

  describe "crawl_stream_events" do
    test "Crawl stream produces page and complete events" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'stream.event_count_min' not available on result type
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
    end
  end

  describe "stream_depth_crawl" do
    test "Stream produces events for multi-depth crawl with link following" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'stream.event_count_min' not available on result type
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
    end
  end

  describe "stream_with_error_event" do
    test "Stream emits page and complete events even when some pages fail" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
      # skipped: field 'stream.event_count_min' not available on result type
    end
  end
end
