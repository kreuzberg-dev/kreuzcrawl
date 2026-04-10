// E2e tests for category: metadata
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MetadataArticleTimes(t *testing.T) {
	// Extracts article:published_time, modified_time, author, section, and tags
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'article.published_time' not available on result type
	// skipped: field 'article.modified_time' not available on result type
	// skipped: field 'article.author' not available on result type
	// skipped: field 'article.section' not available on result type
	// skipped: field 'article.tags.length' not available on result type
}

func Test_MetadataFavicons(t *testing.T) {
	// Extracts favicon link tags including apple-touch-icon
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'favicons.length' not available on result type
	// skipped: field 'favicons[].apple_touch' not available on result type
}

func Test_MetadataHeadings(t *testing.T) {
	// Extracts heading hierarchy (h1-h6) from HTML page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'headings.h1.length' not available on result type
	// skipped: field 'headings.h1[0].text' not available on result type
	// skipped: field 'headings.length' not available on result type
}

func Test_MetadataHreflang(t *testing.T) {
	// Extracts hreflang alternate link tags
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'hreflang.length' not available on result type
	// skipped: field 'hreflang[].lang' not available on result type
}

func Test_MetadataKeywordsAuthor(t *testing.T) {
	// Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
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
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'og.video' not available on result type
	// skipped: field 'og.audio' not available on result type
	// skipped: field 'og.locale_alternate.length' not available on result type
}

func Test_MetadataResponseHeaders(t *testing.T) {
	// Extracts response metadata from HTTP headers (etag, server, content-language)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'response_headers.etag' not available on result type
	// skipped: field 'response_headers.last_modified' not available on result type
	// skipped: field 'response_headers.server' not available on result type
	// skipped: field 'response_headers.content_language' not available on result type
}

func Test_MetadataWordCount(t *testing.T) {
	// Computes word count from visible page text
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	// skipped: field 'computed.word_count' not available on result type
	// skipped: field 'computed.word_count' not available on result type
}
