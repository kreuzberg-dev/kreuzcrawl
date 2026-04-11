// E2e tests for category: scrape
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ScrapeAssetDedup(t *testing.T) {
	// Same asset linked twice results in one download with one unique hash
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Assets) != 2 {
		t.Errorf("equals mismatch: got %v", len(result.Assets))
	}
	if len(result.Assets[0].ContentHash) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_ScrapeAssetMaxSize(t *testing.T) {
	// Skips assets exceeding max_asset_size limit
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Assets) != 2 {
		t.Errorf("equals mismatch: got %v", len(result.Assets))
	}
}

func Test_ScrapeAssetTypeFilter(t *testing.T) {
	// Only downloads image assets when asset_types filter is set
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Assets) != 1 {
		t.Errorf("equals mismatch: got %v", len(result.Assets))
	}
	if !strings.Contains(string(result.Assets[0].AssetCategory), `image`) {
		t.Errorf("expected to contain %s, got %v", `image`, result.Assets[0].AssetCategory)
	}
}

func Test_ScrapeBasicHtmlPage(t *testing.T) {
	// Scrapes a simple HTML page and extracts title, description, and links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataTitle string
	if result.Metadata.Title != nil {
		metadataTitle = *result.Metadata.Title
	}
	var metadataDescription string
	if result.Metadata.Description != nil {
		metadataDescription = *result.Metadata.Description
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if strings.TrimSpace(result.ContentType) != `text/html` {
		t.Errorf("equals mismatch: got %v", result.ContentType)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if strings.TrimSpace(metadataTitle) != `Example Domain` {
		t.Errorf("equals mismatch: got %v", metadataTitle)
	}
	if !strings.Contains(string(metadataDescription), `illustrative examples`) {
		t.Errorf("expected to contain %s, got %v", `illustrative examples`, metadataDescription)
	}
	if result.Metadata.CanonicalUrl == nil || len(*result.Metadata.CanonicalUrl) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(result.Links) < 1 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
	if !strings.Contains(string(result.Links[0].LinkType), `external`) {
		t.Errorf("expected to contain %s", `external`)
	}
	if len(result.Images) != 0 {
		t.Errorf("equals mismatch: got %v", len(result.Images))
	}
	if result.Metadata.OgTitle != nil && len(*result.Metadata.OgTitle) != 0 {
		t.Errorf("expected empty value, got %v", result.Metadata.OgTitle)
	}
}

func Test_ScrapeComplexLinks(t *testing.T) {
	// Classifies links by type: internal, external, anchor, document, image
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Links) < 10 {
		t.Errorf("expected > 9, got %v", len(result.Links))
	}
	if !strings.Contains(string(result.Links[0].LinkType), `internal`) {
		t.Errorf("expected to contain %s", `internal`)
	}
	if !strings.Contains(string(result.Links[0].LinkType), `external`) {
		t.Errorf("expected to contain %s", `external`)
	}
	if !strings.Contains(string(result.Links[0].LinkType), `anchor`) {
		t.Errorf("expected to contain %s", `anchor`)
	}
	if !strings.Contains(string(result.Links[0].LinkType), `document`) {
		t.Errorf("expected to contain %s", `document`)
	}
}

func Test_ScrapeDownloadAssets(t *testing.T) {
	// Downloads CSS, JS, and image assets from page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Assets) < 3 {
		t.Errorf("expected > 2, got %v", len(result.Assets))
	}
}

func Test_ScrapeDublinCore(t *testing.T) {
	// Extracts Dublin Core metadata from a page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataDcTitle string
	if result.Metadata.DcTitle != nil {
		metadataDcTitle = *result.Metadata.DcTitle
	}
	var metadataDcCreator string
	if result.Metadata.DcCreator != nil {
		metadataDcCreator = *result.Metadata.DcCreator
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(metadataDcTitle) == 0 {
		t.Errorf("expected non-empty value")
	}
	if strings.TrimSpace(metadataDcTitle) != `Effects of Climate Change on Marine Biodiversity` {
		t.Errorf("equals mismatch: got %v", metadataDcTitle)
	}
	if strings.TrimSpace(metadataDcCreator) != `Dr. Jane Smith` {
		t.Errorf("equals mismatch: got %v", metadataDcCreator)
	}
}

func Test_ScrapeEmptyPage(t *testing.T) {
	// Handles an empty HTML document without errors
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Links) <= -1 {
		t.Errorf("expected > -1, got %v", len(result.Links))
	}
	if len(result.Images) != 0 {
		t.Errorf("equals mismatch: got %v", len(result.Images))
	}
}

func Test_ScrapeFeedDiscovery(t *testing.T) {
	// Discovers RSS, Atom, and JSON feed links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Feeds) < 3 {
		t.Errorf("expected >= 3, got %v", len(result.Feeds))
	}
}

func Test_ScrapeImageSources(t *testing.T) {
	// Extracts images from img, picture, og:image, twitter:image
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataOgImage string
	if result.Metadata.OgImage != nil {
		metadataOgImage = *result.Metadata.OgImage
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Images) < 5 {
		t.Errorf("expected > 4, got %v", len(result.Images))
	}
	if strings.TrimSpace(metadataOgImage) != `https://example.com/images/og-hero.jpg` {
		t.Errorf("equals mismatch: got %v", metadataOgImage)
	}
}

func Test_ScrapeJsHeavySpa(t *testing.T) {
	// Handles SPA page with JavaScript-only content (no server-rendered HTML)
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

func Test_ScrapeJsonLd(t *testing.T) {
	// Extracts JSON-LD structured data from a page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var jsonLdName string
	if result.JsonLd[0].Name != nil {
		jsonLdName = *result.JsonLd[0].Name
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.JsonLd) == 0 {
		t.Errorf("expected non-empty value")
	}
	if strings.TrimSpace(result.JsonLd[0].SchemaType) != `Recipe` {
		t.Errorf("equals mismatch: got %v", result.JsonLd[0].SchemaType)
	}
	if strings.TrimSpace(jsonLdName) != `Best Chocolate Cake` {
		t.Errorf("equals mismatch: got %v", jsonLdName)
	}
}

func Test_ScrapeMalformedHtml(t *testing.T) {
	// Gracefully handles broken HTML without crashing
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataDescription string
	if result.Metadata.Description != nil {
		metadataDescription = *result.Metadata.Description
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if !strings.Contains(string(metadataDescription), `broken HTML`) {
		t.Errorf("expected to contain %s, got %v", `broken HTML`, metadataDescription)
	}
}

func Test_ScrapeOgMetadata(t *testing.T) {
	// Extracts full Open Graph metadata from a page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataOgTitle string
	if result.Metadata.OgTitle != nil {
		metadataOgTitle = *result.Metadata.OgTitle
	}
	var metadataOgType string
	if result.Metadata.OgType != nil {
		metadataOgType = *result.Metadata.OgType
	}
	var metadataOgImage string
	if result.Metadata.OgImage != nil {
		metadataOgImage = *result.Metadata.OgImage
	}
	var metadataTitle string
	if result.Metadata.Title != nil {
		metadataTitle = *result.Metadata.Title
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if len(metadataOgTitle) == 0 {
		t.Errorf("expected non-empty value")
	}
	if strings.TrimSpace(metadataOgTitle) != `Article Title` {
		t.Errorf("equals mismatch: got %v", metadataOgTitle)
	}
	if strings.TrimSpace(metadataOgType) != `article` {
		t.Errorf("equals mismatch: got %v", metadataOgType)
	}
	if strings.TrimSpace(metadataOgImage) != `https://example.com/images/article-hero.jpg` {
		t.Errorf("equals mismatch: got %v", metadataOgImage)
	}
	if result.Metadata.OgDescription == nil || len(*result.Metadata.OgDescription) == 0 {
		t.Errorf("expected non-empty value")
	}
	if strings.TrimSpace(metadataTitle) != `Article Title - Example Blog` {
		t.Errorf("equals mismatch: got %v", metadataTitle)
	}
}

func Test_ScrapeTwitterCard(t *testing.T) {
	// Extracts Twitter Card metadata from a page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadataTwitterCard string
	if result.Metadata.TwitterCard != nil {
		metadataTwitterCard = *result.Metadata.TwitterCard
	}
	var metadataTwitterTitle string
	if result.Metadata.TwitterTitle != nil {
		metadataTwitterTitle = *result.Metadata.TwitterTitle
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
	if result.Metadata.TwitterCard == nil || len(*result.Metadata.TwitterCard) == 0 {
		t.Errorf("expected non-empty value")
	}
	if strings.TrimSpace(metadataTwitterCard) != `summary_large_image` {
		t.Errorf("equals mismatch: got %v", metadataTwitterCard)
	}
	if strings.TrimSpace(metadataTwitterTitle) != `New Product Launch` {
		t.Errorf("equals mismatch: got %v", metadataTwitterTitle)
	}
}
