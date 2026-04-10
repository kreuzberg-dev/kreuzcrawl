// E2e tests for category: engine
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_EngineBatchBasic(t *testing.T) {
	// CrawlEngine with defaults batch scrapes like the free function
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Batch.CompletedCount != 2 {
		t.Errorf("equals mismatch: got %q", result.Batch.CompletedCount)
	}
	if result.Batch.TotalCount != 2 {
		t.Errorf("equals mismatch: got %q", result.Batch.TotalCount)
	}
}

func Test_EngineCrawlBasic(t *testing.T) {
	// CrawlEngine with defaults crawls multiple pages like the free function
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 3 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
	if result.Crawl.MinPages < 3 {
		t.Errorf("expected >= 3, got %v", result.Crawl.MinPages)
	}
}

func Test_EngineMapBasic(t *testing.T) {
	// CrawlEngine with defaults discovers URLs like the free function
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Map.MinUrls < 2 {
		t.Errorf("expected >= 2, got %v", result.Map.MinUrls)
	}
}

func Test_EngineScrapeBasic(t *testing.T) {
	// CrawlEngine with defaults scrapes a page identically to the free function
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadata_title string
	if result.Metadata.Title != nil {
		metadata_title = *result.Metadata.Title
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if result.ContentType != `text/html` {
		t.Errorf("equals mismatch: got %q", result.ContentType)
	}
	if metadata_title != `Engine Test` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
	if result.Metadata != nil {
	if !strings.Contains(result.Metadata.DescriptionContains, `Testing the engine`) {
		t.Errorf("expected to contain %s, got %q", `Testing the engine`, result.Metadata.DescriptionContains)
	}
	}
	if result.Links.MinCount < 1 {
		t.Errorf("expected >= 1, got %v", result.Links.MinCount)
	}
	if result.Headings.H1Count != 1 {
		t.Errorf("equals mismatch: got %q", result.Headings.H1Count)
	}
	if result.Headings.H1Text != `Hello Engine` {
		t.Errorf("equals mismatch: got %q", result.Headings.H1Text)
	}
}

func Test_EngineStreamBasic(t *testing.T) {
	// CrawlEngine with defaults streams events like the free function
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
	if result.Stream.EventCountMin < 3 {
		t.Errorf("expected >= 3, got %v", result.Stream.EventCountMin)
	}
}
