// E2e tests for category: stream
package e2e_test

import (
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_CrawlStreamEvents(t *testing.T) {
	// Crawl stream produces page and complete events
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/crawl_stream_events"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.event_count_min' not available on result type
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
}

func Test_StreamDepthCrawl(t *testing.T) {
	// Stream produces events for multi-depth crawl with link following
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/stream_depth_crawl"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.event_count_min' not available on result type
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
}

func Test_StreamWithErrorEvent(t *testing.T) {
	// Stream emits page and complete events even when some pages fail
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/stream_with_error_event"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
	// skipped: field 'stream.event_count_min' not available on result type
}
