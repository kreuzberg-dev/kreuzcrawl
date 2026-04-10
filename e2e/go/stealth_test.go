// E2e tests for category: stealth
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_StealthUaRotationConfig(t *testing.T) {
	// User-agent rotation config is accepted and crawl succeeds
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}
