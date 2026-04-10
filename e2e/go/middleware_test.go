// E2e tests for category: middleware
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MiddlewareEngineCrawlWithDefaults(t *testing.T) {
	// Engine crawl with default middleware chain produces correct multi-page results
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

func Test_MiddlewareNoopNoEffect(t *testing.T) {
	// Default middleware chain does not affect normal scraping
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
	if metadata_title != `Middleware Test` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
}
