// E2e tests for category: concurrent
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ConcurrentBasic(t *testing.T) {
	// Concurrent crawling fetches all pages with max_concurrent workers
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 6 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if len(result.Pages) < 6 {
		t.Errorf("expected >= 6, got %v", len(result.Pages))
	}
}

func Test_ConcurrentDepthTwoFanOut(t *testing.T) {
	// Concurrent depth=2 crawl correctly fans out and deduplicates across levels
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_ConcurrentMaxPagesExact(t *testing.T) {
	// Concurrent crawling does not exceed max_pages limit even with high concurrency
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) > 3 {
		t.Errorf("expected <= 3, got %v", len(result.Pages))
	}
}

func Test_ConcurrentPartialErrors(t *testing.T) {
	// Concurrent crawl handles partial failures gracefully
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) < 2 {
		t.Errorf("expected >= 2, got %v", len(result.Pages))
	}
}

func Test_ConcurrentRespectsMaxPages(t *testing.T) {
	// Concurrent crawling respects max_pages limit
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) > 3 {
		t.Errorf("expected <= 3, got %v", len(result.Pages))
	}
}
