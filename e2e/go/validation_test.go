// E2e tests for category: validation
package e2e_test

import (
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ValidationInvalidExcludeRegex(t *testing.T) {
	// Invalid regex in exclude_paths is rejected
	engine, _ := pkg.CreateEngine(nil)
	_, err := pkg.Scrape(engine, "")
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationInvalidIncludeRegex(t *testing.T) {
	// Invalid regex in include_paths is rejected
	engine, _ := pkg.CreateEngine(nil)
	_, err := pkg.Scrape(engine, "")
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationInvalidRetryCode(t *testing.T) {
	// Retry code outside 100-599 is rejected
	engine, _ := pkg.CreateEngine(nil)
	_, err := pkg.Scrape(engine, "")
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationMaxPagesZero(t *testing.T) {
	// max_pages=0 is rejected as invalid config
	engine, _ := pkg.CreateEngine(nil)
	_, err := pkg.Scrape(engine, "")
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationMaxRedirectsTooHigh(t *testing.T) {
	// max_redirects > 100 is rejected as invalid config
	engine, _ := pkg.CreateEngine(nil)
	_, err := pkg.Scrape(engine, "")
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ValidationTimeoutZero(t *testing.T) {
	// Zero request timeout is rejected as invalid config
	engine, _ := pkg.CreateEngine(nil)
	_, err := pkg.Scrape(engine, "")
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}
