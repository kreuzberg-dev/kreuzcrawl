// E2e tests for category: map
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MapDiscoverUrls(t *testing.T) {
	// Discovers all URLs on a site without fetching full content
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) < 3 {
		t.Errorf("expected >= 3, got %v", len(result.Urls))
	}
}

func Test_MapExcludePatterns(t *testing.T) {
	// Excludes URLs matching patterns from URL map
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
}

func Test_MapIncludeSubdomains(t *testing.T) {
	// Includes subdomain URLs in URL map discovery
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) < 2 {
		t.Errorf("expected >= 2, got %v", len(result.Urls))
	}
	if !strings.Contains(result.Urls, `blog.example.com`) {
		t.Errorf("expected to contain %s, got %q", `blog.example.com`, result.Urls)
	}
}

func Test_MapLargeSitemap(t *testing.T) {
	// Handles large sitemap with 100+ URLs
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) < 100 {
		t.Errorf("expected >= 100, got %v", len(result.Urls))
	}
}

func Test_MapLimitPagination(t *testing.T) {
	// Limits map result count to specified maximum
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) > 5 {
		t.Errorf("expected <= 5, got %v", len(result.Urls))
	}
}

func Test_MapSearchFilter(t *testing.T) {
	// Filters map results by search keyword
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) < 2 {
		t.Errorf("expected >= 2, got %v", len(result.Urls))
	}
	if !strings.Contains(result.Urls, `blog`) {
		t.Errorf("expected to contain %s, got %q", `blog`, result.Urls)
	}
}
