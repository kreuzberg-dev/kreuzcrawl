// E2e tests for category: rate_limit
package e2e_test

import (
	"encoding/json"
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_RateLimitBasicDelay(t *testing.T) {
	// Rate limiter adds delay between requests to the same domain
	var engineConfig pkg.CrawlConfig
	if err := json.Unmarshal([]byte(`{"max_depth":1}`), &engineConfig); err != nil {
		t.Fatalf("config parse failed: %v", err)
	}
	engine, createErr := pkg.CreateEngine(&engineConfig)
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/rate_limit_basic_delay"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'crawl.pages_crawled' not available on result type
	// skipped: field 'rate_limit.min_duration_ms' not available on result type
}

func Test_RateLimitZeroNoDelay(t *testing.T) {
	// Rate limiter with zero delay does not slow crawling
	var engineConfig pkg.CrawlConfig
	if err := json.Unmarshal([]byte(`{"max_depth":1}`), &engineConfig); err != nil {
		t.Fatalf("config parse failed: %v", err)
	}
	engine, createErr := pkg.CreateEngine(&engineConfig)
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/rate_limit_zero_no_delay"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'crawl.pages_crawled' not available on result type
}
