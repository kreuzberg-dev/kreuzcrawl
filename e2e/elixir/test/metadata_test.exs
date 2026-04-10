# E2e tests for category: metadata
defmodule E2e.MetadataTest do
  use ExUnit.Case, async: true

  describe "metadata_article_times" do
    test "Extracts article:published_time, modified_time, author, section, and tags" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert result.article.published_time == "2024-01-15T10:00:00Z"
      assert result.article.modified_time == "2024-06-20T14:30:00Z"
      assert result.article.author == "Jane Developer"
      assert result.article.section == "Technology"
      assert length(result.article.tags) == 3
    end
  end

  describe "metadata_favicons" do
    test "Extracts favicon link tags including apple-touch-icon" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert length(result.favicons) == 5
      assert result.favicons.get("").apple_touch != ""
    end
  end

  describe "metadata_headings" do
    test "Extracts heading hierarchy (h1-h6) from HTML page" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert length(result.headings.h1) == 1
      assert result.headings.h1.get("0").text == "Primary Heading"
      assert length(result.headings) == 8
    end
  end

  describe "metadata_hreflang" do
    test "Extracts hreflang alternate link tags" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert length(result.hreflang) == 4
      assert String.contains?(result.hreflang.get("").lang, "en")
    end
  end

  describe "metadata_keywords_author" do
    test "Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert result.metadata.title == "Comprehensive Metadata Test Page"
      assert result.metadata.canonical_url != ""
      assert result.metadata.keywords != ""
      assert String.contains?(result.metadata.keywords, "rust")
      assert result.metadata.author == "Jane Developer"
      assert result.metadata.viewport != ""
      assert result.metadata.generator == "kreuzcrawl/1.0"
      assert result.metadata.theme_color == "\#ff6600"
      assert result.metadata.robots == "index, follow"
      assert result.metadata.lang == "en"
      assert result.metadata.dir == "ltr"
    end
  end

  describe "metadata_og_video_audio" do
    test "Extracts og:video, og:audio, and og:locale:alternate metadata" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert result.og.video == "https://example.com/video.mp4"
      assert result.og.audio == "https://example.com/audio.mp3"
      assert length(result.og.locale_alternate) == 2
    end
  end

  describe "metadata_response_headers" do
    test "Extracts response metadata from HTTP headers (etag, server, content-language)" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert result.response_headers.etag != ""
      assert result.response_headers.last_modified != ""
      assert String.contains?(result.response_headers.server, "nginx")
      assert result.response_headers.content_language == "en-US"
    end
  end

  describe "metadata_word_count" do
    test "Computes word count from visible page text" do
      result = Kreuzcrawl.scrape!()
      assert result.status_code == 200
      assert result.computed.word_count > 99
      assert result.computed.word_count < 301
    end
  end
end
