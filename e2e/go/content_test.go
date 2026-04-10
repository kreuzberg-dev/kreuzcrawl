// E2e tests for category: content
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_Content204NoContent(t *testing.T) {
	// Handles 204 No Content response gracefully
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 204 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Html) != 0 {
		t.Errorf("expected empty value, got %q", result.Html)
	}
}

func Test_ContentCharsetIso8859(t *testing.T) {
	// Handles ISO-8859-1 encoded page correctly
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.DetectedCharset != `iso-8859-1` {
		t.Errorf("equals mismatch: got %q", result.Content.DetectedCharset)
	}
}

func Test_ContentEmptyBody(t *testing.T) {
	// Handles 200 response with empty body gracefully
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}

func Test_ContentGzipCompressed(t *testing.T) {
	// Handles response with Accept-Encoding gzip negotiation
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}

func Test_ContentLargePageLimit(t *testing.T) {
	// Respects max body size limit and truncates or skips oversized pages
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.BodySize >= 1025 {
		t.Errorf("expected < 1025, got %v", result.Content.BodySize)
	}
}

func Test_ContentMainOnly(t *testing.T) {
	// Extracts only main content area, excluding nav, sidebar, footer
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.MainContentOnly != true {
		t.Errorf("equals mismatch: got %q", result.Content.MainContentOnly)
	}
}

func Test_ContentPdfNoExtension(t *testing.T) {
	// Detects PDF content by Content-Type header when URL has no .pdf extension
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.IsPdf != true {
		t.Errorf("equals mismatch: got %q", result.Content.IsPdf)
	}
}

func Test_ContentRemoveTags(t *testing.T) {
	// Removes specified HTML elements by CSS selector before processing
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_ContentUtf8Bom(t *testing.T) {
	// Handles UTF-8 content with BOM marker correctly
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.DetectedCharset != `utf-8` {
		t.Errorf("equals mismatch: got %q", result.Content.DetectedCharset)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
}
