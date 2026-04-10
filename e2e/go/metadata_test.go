// E2e tests for category: metadata
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MetadataArticleTimes(t *testing.T) {
	// Extracts article:published_time, modified_time, author, section, and tags
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if result.Article.PublishedTime != `2024-01-15T10:00:00Z` {
		t.Errorf("equals mismatch: got %q", result.Article.PublishedTime)
	}
	if result.Article.ModifiedTime != `2024-06-20T14:30:00Z` {
		t.Errorf("equals mismatch: got %q", result.Article.ModifiedTime)
	}
	if result.Article.Author != `Jane Developer` {
		t.Errorf("equals mismatch: got %q", result.Article.Author)
	}
	if result.Article.Section != `Technology` {
		t.Errorf("equals mismatch: got %q", result.Article.Section)
	}
	if len(result.Article.Tags) != 3 {
		t.Errorf("equals mismatch: got %q", len(result.Article.Tags))
	}
}

func Test_MetadataFavicons(t *testing.T) {
	// Extracts favicon link tags including apple-touch-icon
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Favicons) != 5 {
		t.Errorf("equals mismatch: got %q", len(result.Favicons))
	}
	if len(result.Favicons[""].AppleTouch) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_MetadataHeadings(t *testing.T) {
	// Extracts heading hierarchy (h1-h6) from HTML page
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Headings.H1) != 1 {
		t.Errorf("equals mismatch: got %q", len(result.Headings.H1))
	}
	if result.Headings.H1["0"].Text != `Primary Heading` {
		t.Errorf("equals mismatch: got %q", result.Headings.H1["0"].Text)
	}
	if len(result.Headings) != 8 {
		t.Errorf("equals mismatch: got %q", len(result.Headings))
	}
}

func Test_MetadataHreflang(t *testing.T) {
	// Extracts hreflang alternate link tags
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Hreflang) != 4 {
		t.Errorf("equals mismatch: got %q", len(result.Hreflang))
	}
	if !strings.Contains(result.Hreflang[""].Lang, `en`) {
		t.Errorf("expected to contain %s, got %q", `en`, result.Hreflang[""].Lang)
	}
}

func Test_MetadataKeywordsAuthor(t *testing.T) {
	// Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadata_title string
	if result.Metadata.Title != nil {
		metadata_title = *result.Metadata.Title
	}
	var metadata_canonical_url string
	if result.Metadata.CanonicalUrl != nil {
		metadata_canonical_url = *result.Metadata.CanonicalUrl
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if metadata_title != `Comprehensive Metadata Test Page` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
	if len(metadata_canonical_url) == 0 {
		t.Errorf("expected non-empty value")
	}
	if result.Metadata != nil {
	if len(result.Metadata.Keywords) == 0 {
		t.Errorf("expected non-empty value")
	}
	}
	if result.Metadata != nil {
	if !strings.Contains(result.Metadata.Keywords, `rust`) {
		t.Errorf("expected to contain %s, got %q", `rust`, result.Metadata.Keywords)
	}
	}
	if result.Metadata != nil {
	if result.Metadata.Author != `Jane Developer` {
		t.Errorf("equals mismatch: got %q", result.Metadata.Author)
	}
	}
	if result.Metadata != nil {
	if len(result.Metadata.Viewport) == 0 {
		t.Errorf("expected non-empty value")
	}
	}
	if result.Metadata != nil {
	if result.Metadata.Generator != `kreuzcrawl/1.0` {
		t.Errorf("equals mismatch: got %q", result.Metadata.Generator)
	}
	}
	if result.Metadata != nil {
	if result.Metadata.ThemeColor != `#ff6600` {
		t.Errorf("equals mismatch: got %q", result.Metadata.ThemeColor)
	}
	}
	if result.Metadata != nil {
	if result.Metadata.Robots != `index, follow` {
		t.Errorf("equals mismatch: got %q", result.Metadata.Robots)
	}
	}
	if result.Metadata != nil {
	if result.Metadata.Lang != `en` {
		t.Errorf("equals mismatch: got %q", result.Metadata.Lang)
	}
	}
	if result.Metadata != nil {
	if result.Metadata.Dir != `ltr` {
		t.Errorf("equals mismatch: got %q", result.Metadata.Dir)
	}
	}
}

func Test_MetadataOgVideoAudio(t *testing.T) {
	// Extracts og:video, og:audio, and og:locale:alternate metadata
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if result.Og.Video != `https://example.com/video.mp4` {
		t.Errorf("equals mismatch: got %q", result.Og.Video)
	}
	if result.Og.Audio != `https://example.com/audio.mp3` {
		t.Errorf("equals mismatch: got %q", result.Og.Audio)
	}
	if len(result.Og.LocaleAlternate) != 2 {
		t.Errorf("equals mismatch: got %q", len(result.Og.LocaleAlternate))
	}
}

func Test_MetadataResponseHeaders(t *testing.T) {
	// Extracts response metadata from HTTP headers (etag, server, content-language)
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.ResponseHeaders.Etag) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(result.ResponseHeaders.LastModified) == 0 {
		t.Errorf("expected non-empty value")
	}
	if !strings.Contains(result.ResponseHeaders.Server, `nginx`) {
		t.Errorf("expected to contain %s, got %q", `nginx`, result.ResponseHeaders.Server)
	}
	if result.ResponseHeaders.ContentLanguage != `en-US` {
		t.Errorf("equals mismatch: got %q", result.ResponseHeaders.ContentLanguage)
	}
}

func Test_MetadataWordCount(t *testing.T) {
	// Computes word count from visible page text
	engine, _ := pkg.CreateEngine(nil)
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if result.Computed.WordCount <= 99 {
		t.Errorf("expected > 99, got %v", result.Computed.WordCount)
	}
	if result.Computed.WordCount >= 301 {
		t.Errorf("expected < 301, got %v", result.Computed.WordCount)
	}
}
