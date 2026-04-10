// E2e tests for category: links
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_LinksAnchorFragment(t *testing.T) {
	// Identifies fragment-only links as anchor type
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.Links[""].LinkType, `anchor`) {
		t.Errorf("expected to contain %s", `anchor`)
	}
}

func Test_LinksBaseTag(t *testing.T) {
	// Resolves relative URLs using base tag href
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 2 {
		t.Errorf("expected > 2, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[""].Url, `example.com`) {
		t.Errorf("expected to contain %s, got %q", `example.com`, result.Links[""].Url)
	}
}

func Test_LinksDocumentTypes(t *testing.T) {
	// Detects PDF, DOCX, XLSX links as document type
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.Links[""].LinkType, `document`) {
		t.Errorf("expected to contain %s", `document`)
	}
}

func Test_LinksEmptyHref(t *testing.T) {
	// Handles empty href attributes without errors
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[""].Url, `/valid`) {
		t.Errorf("expected to contain %s, got %q", `/valid`, result.Links[""].Url)
	}
}

func Test_LinksInternalExternalClassification(t *testing.T) {
	// Correctly classifies internal vs external links by domain
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 4 {
		t.Errorf("expected > 4, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[""].LinkType, `internal`) {
		t.Errorf("expected to contain %s", `internal`)
	}
	if !strings.Contains(result.Links[""].LinkType, `external`) {
		t.Errorf("expected to contain %s", `external`)
	}
}

func Test_LinksMailtoJavascriptSkip(t *testing.T) {
	// Skips mailto:, javascript:, and tel: scheme links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
	if strings.Contains(result.Links[""].Url, `mailto:`) {
		t.Errorf("expected NOT to contain %s, got %q", `mailto:`, result.Links[""].Url)
	}
}

func Test_LinksProtocolRelative(t *testing.T) {
	// Handles protocol-relative URLs (//example.com) correctly
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 1 {
		t.Errorf("expected > 1, got %v", len(result.Links))
	}
	if len(result.Links[""].ProtocolRelative) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_LinksRelAttributes(t *testing.T) {
	// Preserves rel=nofollow and rel=canonical attributes
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
}

func Test_LinksRelativeParent(t *testing.T) {
	// Resolves ../ and ./ relative parent path links correctly
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 3 {
		t.Errorf("expected > 3, got %v", len(result.Links))
	}
}
