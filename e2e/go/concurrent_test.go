// E2e tests for category: concurrent
package e2e_test

import (
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ConcurrentBasic(t *testing.T) {
	// Concurrent crawling fetches all pages with max_concurrent workers
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/concurrent_basic"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'pages.length' not available on result type
}

func Test_ConcurrentDepthTwoFanOut(t *testing.T) {
	// Concurrent depth=2 crawl correctly fans out and deduplicates across levels
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/concurrent_depth_two_fan_out"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_ConcurrentMaxPagesExact(t *testing.T) {
	// Concurrent crawling does not exceed max_pages limit even with high concurrency
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/concurrent_max_pages_exact"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_ConcurrentPartialErrors(t *testing.T) {
	// Concurrent crawl handles partial failures gracefully
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/concurrent_partial_errors"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_ConcurrentRespectsMaxPages(t *testing.T) {
	// Concurrent crawling respects max_pages limit
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/concurrent_respects_max_pages"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}
