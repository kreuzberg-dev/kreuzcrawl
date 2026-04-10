// E2e tests for category: encoding
package e2e_test

import (
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_EncodingDoubleEncoded(t *testing.T) {
	// Handles double-encoded URL characters (%25C3%25B6)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(result.Links) < 1 {
		t.Errorf("expected >= 1, got %v", len(result.Links))
	}
}

func Test_EncodingMixedCharsetPage(t *testing.T) {
	// Handles charset mismatch between HTTP header and HTML meta tag
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_EncodingPercentEncodedPath(t *testing.T) {
	// Handles percent-encoded spaces and characters in URL paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(result.Links) < 2 {
		t.Errorf("expected >= 2, got %v", len(result.Links))
	}
}

func Test_EncodingUnicodeUrl(t *testing.T) {
	// Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
}
