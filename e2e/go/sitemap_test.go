// E2e tests for category: sitemap
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_SitemapBasic(t *testing.T) {
	// Parses a standard urlset sitemap
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
	if result.HasLastmod != true {
		t.Errorf("equals mismatch: got %q", result.HasLastmod)
	}
}

func Test_SitemapCompressedGzip(t *testing.T) {
	// Parses a gzip-compressed sitemap file
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
}

func Test_SitemapEmpty(t *testing.T) {
	// Handles empty sitemap gracefully
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 0 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
}

func Test_SitemapFromRobotsTxt(t *testing.T) {
	// Discovers sitemap via robots.txt Sitemap directive
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
}

func Test_SitemapIndex(t *testing.T) {
	// Follows sitemap index to discover child sitemaps
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
}

func Test_SitemapLastmodFilter(t *testing.T) {
	// Filters sitemap URLs by lastmod date
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
	if result.HasLastmod != true {
		t.Errorf("equals mismatch: got %q", result.HasLastmod)
	}
}

func Test_SitemapOnlyMode(t *testing.T) {
	// Uses sitemap URLs exclusively without following page links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
}

func Test_SitemapXhtmlLinks(t *testing.T) {
	// Parses sitemap with XHTML namespace alternate links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Urls) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Urls))
	}
	if result.HasLastmod != false {
		t.Errorf("equals mismatch: got %q", result.HasLastmod)
	}
}
