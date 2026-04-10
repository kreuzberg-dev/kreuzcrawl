// E2e tests for category: strategy
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_StrategyBestFirstSeed(t *testing.T) {
	// BestFirst strategy always processes the seed URL first
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 3 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
	if !strings.Contains(result.Strategy.FirstPageUrlContains, `/`) {
		t.Errorf("expected to contain %s, got %q", `/`, result.Strategy.FirstPageUrlContains)
	}
}

func Test_StrategyBfsDefaultOrder(t *testing.T) {
	// BFS strategy visits pages in breadth-first order
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 5 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
	if result.Strategy.CrawlOrder != `["/","/a","/b","/a/1","/b/1"]` {
		t.Errorf("equals mismatch: got %q", result.Strategy.CrawlOrder)
	}
}

func Test_StrategyDfsDepthFirst(t *testing.T) {
	// DFS strategy visits pages in depth-first order
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Crawl.PagesCrawled != 5 {
		t.Errorf("equals mismatch: got %q", result.Crawl.PagesCrawled)
	}
	if result.Strategy.CrawlOrder != `["/","/b","/b/1","/a","/a/1"]` {
		t.Errorf("equals mismatch: got %q", result.Strategy.CrawlOrder)
	}
}
