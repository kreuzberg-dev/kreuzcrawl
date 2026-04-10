// E2e tests for category: stream
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_CrawlStreamEvents(t *testing.T) {
	// Crawl stream produces page and complete events
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Stream.EventCountMin < 4 {
		t.Errorf("expected >= 4, got %v", result.Stream.EventCountMin)
	}
	if result.Stream.HasPageEvent != true {
		t.Errorf("equals mismatch: got %q", result.Stream.HasPageEvent)
	}
	if result.Stream.HasCompleteEvent != true {
		t.Errorf("equals mismatch: got %q", result.Stream.HasCompleteEvent)
	}
}

func Test_StreamDepthCrawl(t *testing.T) {
	// Stream produces events for multi-depth crawl with link following
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Stream.EventCountMin < 5 {
		t.Errorf("expected >= 5, got %v", result.Stream.EventCountMin)
	}
	if result.Stream.HasPageEvent != true {
		t.Errorf("equals mismatch: got %q", result.Stream.HasPageEvent)
	}
	if result.Stream.HasCompleteEvent != true {
		t.Errorf("equals mismatch: got %q", result.Stream.HasCompleteEvent)
	}
}

func Test_StreamWithErrorEvent(t *testing.T) {
	// Stream emits page and complete events even when some pages fail
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Stream.HasPageEvent != true {
		t.Errorf("equals mismatch: got %q", result.Stream.HasPageEvent)
	}
	if result.Stream.HasCompleteEvent != true {
		t.Errorf("equals mismatch: got %q", result.Stream.HasCompleteEvent)
	}
	if result.Stream.EventCountMin < 2 {
		t.Errorf("expected >= 2, got %v", result.Stream.EventCountMin)
	}
}
