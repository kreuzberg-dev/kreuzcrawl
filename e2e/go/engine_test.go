// E2e tests for category: engine
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_EngineBatchBasic(t *testing.T) {
	// CrawlEngine with defaults batch scrapes like the free function
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'batch.completed_count' not available on result type
	// skipped: field 'batch.total_count' not available on result type
}

func Test_EngineCrawlBasic(t *testing.T) {
	// CrawlEngine with defaults crawls multiple pages like the free function
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'crawl.pages_crawled' not available on result type
	// skipped: field 'crawl.min_pages' not available on result type
}

func Test_EngineMapBasic(t *testing.T) {
	// CrawlEngine with defaults discovers URLs like the free function
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'map.min_urls' not available on result type
}

func Test_EngineScrapeBasic(t *testing.T) {
	// CrawlEngine with defaults scrapes a page identically to the free function
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataTitle string
	if result.Metadata.Title != nil {
		metadataTitle = *result.Metadata.Title
	}
	var metadataDescription string
	if result.Metadata.Description != nil {
		metadataDescription = *result.Metadata.Description
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if strings.TrimSpace(result.ContentType) != `text/html` {
		t.Errorf("equals mismatch: got %v", result.ContentType)
	}
	if strings.TrimSpace(metadataTitle) != `Engine Test` {
		t.Errorf("equals mismatch: got %v", metadataTitle)
	}
	if !strings.Contains(string(metadataDescription), `Testing the engine`) {
		t.Errorf("expected to contain %s, got %v", `Testing the engine`, metadataDescription)
	}
	if len(result.Links) < 1 {
		t.Errorf("expected >= 1, got %v", len(result.Links))
	}
	if result.Metadata.Headings != nil {
		if result.Metadata.Headings != nil {
			if len(*result.Metadata.Headings) < 1 {
				t.Errorf("expected >= 1, got %v", len(*result.Metadata.Headings))
			}
		}
	}
}

func Test_EngineStreamBasic(t *testing.T) {
	// CrawlEngine with defaults streams events like the free function
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
	// skipped: field 'stream.event_count_min' not available on result type
}
