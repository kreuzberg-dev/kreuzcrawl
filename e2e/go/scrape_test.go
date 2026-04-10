// E2e tests for category: scrape
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ScrapeAssetDedup(t *testing.T) {
	// Same asset linked twice results in one download with one unique hash
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Assets) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Assets))
	}
	if result.Assets.UniqueHashes != 2 {
		t.Errorf("equals mismatch: got %q", result.Assets.UniqueHashes)
	}
}

func Test_ScrapeAssetMaxSize(t *testing.T) {
	// Skips assets exceeding max_asset_size limit
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Assets) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Assets))
	}
}

func Test_ScrapeAssetTypeFilter(t *testing.T) {
	// Only downloads image assets when asset_types filter is set
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Assets) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Assets))
	}
	if !strings.Contains(result.Assets[""].Category, `image`) {
		t.Errorf("expected to contain %s, got %q", `image`, result.Assets[""].Category)
	}
}

func Test_ScrapeBasicHtmlPage(t *testing.T) {
	// Scrapes a simple HTML page and extracts title, description, and links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadata_title string
	if result.Metadata.Title != nil {
		metadata_title = *result.Metadata.Title
	}
	var metadata_description string
	if result.Metadata.Description != nil {
		metadata_description = *result.Metadata.Description
	}
	var metadata_canonical_url string
	if result.Metadata.CanonicalUrl != nil {
		metadata_canonical_url = *result.Metadata.CanonicalUrl
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if result.ContentType != `text/html` {
		t.Errorf("equals mismatch: got %q", result.ContentType)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if metadata_title != `Example Domain` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
	if !strings.Contains(metadata_description, `illustrative examples`) {
		t.Errorf("expected to contain %s, got %q", `illustrative examples`, metadata_description)
	}
	if len(metadata_canonical_url) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[""].LinkType, `external`) {
		t.Errorf("expected to contain %s", `external`)
	}
	if len(result.Images) != 0 {
		t.Errorf("equals mismatch: got %q", len(result.Images))
	}
	if len(result.Og.Title) != 0 {
		t.Errorf("expected empty value, got %q", result.Og.Title)
	}
}

func Test_ScrapeComplexLinks(t *testing.T) {
	// Classifies links by type: internal, external, anchor, document, image
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Links) <= 9 {
		t.Errorf("expected > 9, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[""].LinkType, `internal`) {
		t.Errorf("expected to contain %s", `internal`)
	}
	if !strings.Contains(result.Links[""].LinkType, `external`) {
		t.Errorf("expected to contain %s", `external`)
	}
	if !strings.Contains(result.Links[""].LinkType, `anchor`) {
		t.Errorf("expected to contain %s", `anchor`)
	}
	if !strings.Contains(result.Links[""].LinkType, `document`) {
		t.Errorf("expected to contain %s", `document`)
	}
}

func Test_ScrapeDownloadAssets(t *testing.T) {
	// Downloads CSS, JS, and image assets from page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Assets) <= 2 {
		t.Errorf("expected > 2, got %v", len(result.Assets))
	}
}

func Test_ScrapeDublinCore(t *testing.T) {
	// Extracts Dublin Core metadata from a page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.DublinCore.Title) == 0 {
		t.Errorf("expected non-empty value")
	}
	if result.DublinCore.Title != `Effects of Climate Change on Marine Biodiversity` {
		t.Errorf("equals mismatch: got %q", result.DublinCore.Title)
	}
	if result.DublinCore.Creator != `Dr. Jane Smith` {
		t.Errorf("equals mismatch: got %q", result.DublinCore.Creator)
	}
}

func Test_ScrapeEmptyPage(t *testing.T) {
	// Handles an empty HTML document without errors
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Links) <= -1 {
		t.Errorf("expected > -1, got %v", len(result.Links))
	}
	if len(result.Images) != 0 {
		t.Errorf("equals mismatch: got %q", len(result.Images))
	}
}

func Test_ScrapeFeedDiscovery(t *testing.T) {
	// Discovers RSS, Atom, and JSON feed links
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Feeds.Rss) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Feeds.Rss))
	}
	if len(result.Feeds.Atom) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Feeds.Atom))
	}
	if len(result.Feeds.JsonFeed) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Feeds.JsonFeed))
	}
}

func Test_ScrapeImageSources(t *testing.T) {
	// Extracts images from img, picture, og:image, twitter:image
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Images) <= 4 {
		t.Errorf("expected > 4, got %v", len(result.Images))
	}
	if result.Og.Image != `https://example.com/images/og-hero.jpg` {
		t.Errorf("equals mismatch: got %q", result.Og.Image)
	}
}

func Test_ScrapeJsHeavySpa(t *testing.T) {
	// Handles SPA page with JavaScript-only content (no server-rendered HTML)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_ScrapeJsonLd(t *testing.T) {
	// Extracts JSON-LD structured data from a page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.JsonLd) == 0 {
		t.Errorf("expected non-empty value")
	}
	if result.JsonLd.Type != `Recipe` {
		t.Errorf("equals mismatch: got %q", result.JsonLd.Type)
	}
	if result.JsonLd.Name != `Best Chocolate Cake` {
		t.Errorf("equals mismatch: got %q", result.JsonLd.Name)
	}
}

func Test_ScrapeMalformedHtml(t *testing.T) {
	// Gracefully handles broken HTML without crashing
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadata_description string
	if result.Metadata.Description != nil {
		metadata_description = *result.Metadata.Description
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if !strings.Contains(metadata_description, `broken HTML`) {
		t.Errorf("expected to contain %s, got %q", `broken HTML`, metadata_description)
	}
}

func Test_ScrapeOgMetadata(t *testing.T) {
	// Extracts full Open Graph metadata from a page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadata_title string
	if result.Metadata.Title != nil {
		metadata_title = *result.Metadata.Title
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Og.Title) == 0 {
		t.Errorf("expected non-empty value")
	}
	if result.Og.Title != `Article Title` {
		t.Errorf("equals mismatch: got %q", result.Og.Title)
	}
	if result.Og.Type != `article` {
		t.Errorf("equals mismatch: got %q", result.Og.Type)
	}
	if result.Og.Image != `https://example.com/images/article-hero.jpg` {
		t.Errorf("equals mismatch: got %q", result.Og.Image)
	}
	if len(result.Og.Description) == 0 {
		t.Errorf("expected non-empty value")
	}
	if metadata_title != `Article Title - Example Blog` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
}

func Test_ScrapeTwitterCard(t *testing.T) {
	// Extracts Twitter Card metadata from a page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Twitter.Card) == 0 {
		t.Errorf("expected non-empty value")
	}
	if result.Twitter.CardType != `summary_large_image` {
		t.Errorf("equals mismatch: got %q", result.Twitter.CardType)
	}
	if result.Twitter.Title != `New Product Launch` {
		t.Errorf("equals mismatch: got %q", result.Twitter.Title)
	}
}
