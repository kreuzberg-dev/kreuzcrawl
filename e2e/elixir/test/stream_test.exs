# E2e tests for category: stream
defmodule E2e.StreamTest do
  use ExUnit.Case, async: true

  describe "crawl_stream_events" do
    test "Crawl stream produces page and complete events" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert result.stream.event_count_min >= 4
      assert String.trim(result.stream.has_page_event) == true
      assert String.trim(result.stream.has_complete_event) == true
    end
  end

  describe "stream_depth_crawl" do
    test "Stream produces events for multi-depth crawl with link following" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert result.stream.event_count_min >= 5
      assert String.trim(result.stream.has_page_event) == true
      assert String.trim(result.stream.has_complete_event) == true
    end
  end

  describe "stream_with_error_event" do
    test "Stream emits page and complete events even when some pages fail" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.stream.has_page_event) == true
      assert String.trim(result.stream.has_complete_event) == true
      assert result.stream.event_count_min >= 2
    end
  end
end
