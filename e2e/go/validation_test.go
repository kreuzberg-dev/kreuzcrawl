// E2e tests for category: validation
package e2e_test

import (
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ValidationInvalidExcludeRegex(t *testing.T) {
	// Invalid regex in exclude_paths is rejected
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/validation_invalid_exclude_regex"
	_, err := pkg.Scrape(engine, url)
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationInvalidIncludeRegex(t *testing.T) {
	// Invalid regex in include_paths is rejected
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/validation_invalid_include_regex"
	_, err := pkg.Scrape(engine, url)
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationInvalidRetryCode(t *testing.T) {
	// Retry code outside 100-599 is rejected
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/validation_invalid_retry_code"
	_, err := pkg.Scrape(engine, url)
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationMaxPagesZero(t *testing.T) {
	// max_pages=0 is rejected as invalid config
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/validation_max_pages_zero"
	_, err := pkg.Scrape(engine, url)
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationMaxRedirectsTooHigh(t *testing.T) {
	// max_redirects > 100 is rejected as invalid config
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/validation_max_redirects_too_high"
	_, err := pkg.Scrape(engine, url)
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationTimeoutZero(t *testing.T) {
	// Zero request timeout is rejected as invalid config
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/validation_timeout_zero"
	_, err := pkg.Scrape(engine, url)
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}
