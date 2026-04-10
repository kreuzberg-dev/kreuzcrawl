// E2e tests for category: cache
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_CacheBasic(t *testing.T) {
	// Crawling with disk cache enabled succeeds without errors
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}
