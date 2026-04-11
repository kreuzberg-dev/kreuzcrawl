// E2e tests for category: map
package e2e_test

import (
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MapDiscoverUrls(t *testing.T) {
	// Discovers all URLs on a site without fetching full content
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/map_discover_urls"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_MapExcludePatterns(t *testing.T) {
	// Excludes URLs matching patterns from URL map
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/map_exclude_patterns"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_MapIncludeSubdomains(t *testing.T) {
	// Includes subdomain URLs in URL map discovery
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/map_include_subdomains"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
	// skipped: field 'urls' not available on result type
}

func Test_MapLargeSitemap(t *testing.T) {
	// Handles large sitemap with 100+ URLs
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/map_large_sitemap"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_MapLimitPagination(t *testing.T) {
	// Limits map result count to specified maximum
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/map_limit_pagination"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_MapSearchFilter(t *testing.T) {
	// Filters map results by search keyword
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/map_search_filter"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
	// skipped: field 'urls' not available on result type
}
