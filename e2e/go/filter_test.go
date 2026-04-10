// E2e tests for category: filter
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_FilterBm25CrawlIntegration(t *testing.T) {
	// BM25 filter works during multi-page crawl, keeping relevant pages
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.Filter.RemainingContainKeyword, `rust`) {
		t.Errorf("expected to contain %s, got %q", `rust`, result.Filter.RemainingContainKeyword)
	}
}

func Test_FilterBm25EmptyQuery(t *testing.T) {
	// BM25 filter with empty query passes all pages through
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 2 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
}

func Test_FilterBm25HighThreshold(t *testing.T) {
	// BM25 filter with very high threshold filters out all pages
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Filter.PagesAfterFilter != 0 {
		t.Errorf("equals mismatch: got %q", result.Filter.PagesAfterFilter)
	}
}

func Test_FilterBm25RelevantPages(t *testing.T) {
	// BM25 filter keeps only pages relevant to the query
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.Filter.RemainingContainKeyword, `rust`) {
		t.Errorf("expected to contain %s, got %q", `rust`, result.Filter.RemainingContainKeyword)
	}
}

func Test_FilterBm25ThresholdZero(t *testing.T) {
	// BM25 filter with zero threshold passes all pages
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 2 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
}

func Test_FilterNoopCrawlAllKept(t *testing.T) {
	// NoopFilter keeps all pages during a multi-page crawl
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Filter.PagesAfterFilter != 3 {
		t.Errorf("equals mismatch: got %q", result.Filter.PagesAfterFilter)
	}
}

func Test_FilterNoopPassesAll(t *testing.T) {
	// No content filter passes all crawled pages through
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 3 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
}
