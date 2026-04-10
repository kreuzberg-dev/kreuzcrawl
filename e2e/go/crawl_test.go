// E2e tests for category: crawl
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ContentBinarySkip(t *testing.T) {
	// Skips image and video content types gracefully
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.WasSkipped != true {
		t.Errorf("equals mismatch: got %q", result.Content.WasSkipped)
	}
}

func Test_ContentPdfLinkSkip(t *testing.T) {
	// Encounters PDF link and skips or marks as document type
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Content.WasSkipped != true {
		t.Errorf("equals mismatch: got %q", result.Content.WasSkipped)
	}
}

func Test_CrawlConcurrentDepth(t *testing.T) {
	// Concurrent crawl respects max_depth limit
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if result.StayedOnDomain != true {
		t.Errorf("equals mismatch: got %q", result.StayedOnDomain)
	}
}

func Test_CrawlConcurrentLimit(t *testing.T) {
	// Respects max concurrent requests limit during crawl
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 5 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlConcurrentMaxPages(t *testing.T) {
	// Concurrent crawl respects max_pages budget
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) > 3 {
		t.Errorf("expected <= 3, got %v", len(result.Pages))
	}
}

func Test_CrawlCustomHeaders(t *testing.T) {
	// Sends custom headers on all crawl requests
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlDepthOne(t *testing.T) {
	// Follows links one level deep from start page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if result.StayedOnDomain != true {
		t.Errorf("equals mismatch: got %q", result.StayedOnDomain)
	}
}

func Test_CrawlDepthPriority(t *testing.T) {
	// Crawls in breadth-first order, processing depth-0 pages before depth-1
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlDepthTwo(t *testing.T) {
	// Crawls 3 levels deep (depth 0, 1, 2)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if len(result.Pages) < 3 {
		t.Errorf("expected >= 3, got %v", len(result.Pages))
	}
}

func Test_CrawlDepthTwoChain(t *testing.T) {
	// Depth=2 crawl follows a chain of links across three levels
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlDoubleSlashNormalization(t *testing.T) {
	// Normalizes double slashes in URL paths (//page to /page)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.UniqueUrls) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.UniqueUrls))
	}
}

func Test_CrawlEmptyPageNoLinks(t *testing.T) {
	// Crawl completes when child page has no outgoing links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlExcludePathPattern(t *testing.T) {
	// Skips URLs matching the exclude path pattern
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlExternalLinksIgnored(t *testing.T) {
	// External links are discovered but not followed when stay_on_domain is true
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if result.StayedOnDomain != true {
		t.Errorf("equals mismatch: got %q", result.StayedOnDomain)
	}
}

func Test_CrawlFragmentStripping(t *testing.T) {
	// Strips #fragment from URLs for deduplication
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.UniqueUrls) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.UniqueUrls))
	}
}

func Test_CrawlIncludePathPattern(t *testing.T) {
	// Only follows URLs matching the include path pattern
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlMaxDepthZero(t *testing.T) {
	// max_depth=0 crawls only the seed page with no link following
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if len(result.Pages) > 1 {
		t.Errorf("expected <= 1, got %v", len(result.Pages))
	}
}

func Test_CrawlMaxPages(t *testing.T) {
	// Stops crawling at page budget limit
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) > 3 {
		t.Errorf("expected <= 3, got %v", len(result.Pages))
	}
}

func Test_CrawlMixedContentTypes(t *testing.T) {
	// Crawl handles links to non-HTML content types gracefully
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) < 2 {
		t.Errorf("expected >= 2, got %v", len(result.Pages))
	}
}

func Test_CrawlMultipleRedirectsInTraversal(t *testing.T) {
	// Multiple linked pages with redirects are handled during crawl traversal
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) < 1 {
		t.Errorf("expected >= 1, got %v", len(result.Pages))
	}
}

func Test_CrawlQueryParamDedup(t *testing.T) {
	// Deduplicates URLs with same query params in different order
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.UniqueUrls) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.UniqueUrls))
	}
}

func Test_CrawlRedirectInTraversal(t *testing.T) {
	// Links that redirect are followed during crawl traversal
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) < 1 {
		t.Errorf("expected >= 1, got %v", len(result.Pages))
	}
}

func Test_CrawlSelfLinkNoLoop(t *testing.T) {
	// Page linking to itself does not cause infinite crawl loop
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlSinglePageNoLinks(t *testing.T) {
	// Crawling a page with no links returns only the seed page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
}

func Test_CrawlStayOnDomain(t *testing.T) {
	// Does not follow external links when stay_on_domain is true
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if result.StayedOnDomain != true {
		t.Errorf("equals mismatch: got %q", result.StayedOnDomain)
	}
}

func Test_CrawlSubdomainExclusion(t *testing.T) {
	// Stays on exact domain and skips subdomain links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Pages))
	}
	if result.StayedOnDomain != true {
		t.Errorf("equals mismatch: got %q", result.StayedOnDomain)
	}
}

func Test_CrawlSubdomainInclusion(t *testing.T) {
	// Crawls subdomains when allow_subdomains is enabled
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) < 2 {
		t.Errorf("expected >= 2, got %v", len(result.Pages))
	}
}

func Test_CrawlTrailingSlashDedup(t *testing.T) {
	// Deduplicates /page and /page/ as the same URL
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.UniqueUrls) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.UniqueUrls))
	}
}

func Test_CrawlUrlDeduplication(t *testing.T) {
	// Deduplicates URLs that differ only by fragment or query params
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Pages) > 2 {
		t.Errorf("expected <= 2, got %v", len(result.Pages))
	}
}
