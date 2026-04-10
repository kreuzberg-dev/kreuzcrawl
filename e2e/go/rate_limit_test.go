// E2e tests for category: rate_limit
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_RateLimitBasicDelay(t *testing.T) {
	// Rate limiter adds delay between requests to the same domain
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 3 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
	if result.RateLimit.MinDurationMs < 150 {
		t.Errorf("expected >= 150, got %v", result.RateLimit.MinDurationMs)
	}
}

func Test_RateLimitZeroNoDelay(t *testing.T) {
	// Rate limiter with zero delay does not slow crawling
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 2 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
}
