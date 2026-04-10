# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "metadata" do
  it "metadata_article_times: Extracts article:published_time, modified_time, author, section, and tags" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.article.published_time).to eq("2024-01-15T10:00:00Z")
    expect(result.article.modified_time).to eq("2024-06-20T14:30:00Z")
    expect(result.article.author).to eq("Jane Developer")
    expect(result.article.section).to eq("Technology")
    expect(result.article.tags.length).to eq(3)
  end

  it "metadata_favicons: Extracts favicon link tags including apple-touch-icon" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.favicons.length).to eq(5)
    expect(result.favicons.get("").apple_touch).not_to be_empty
  end

  it "metadata_headings: Extracts heading hierarchy (h1-h6) from HTML page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.headings.h1.length).to eq(1)
    expect(result.headings.h1.get("0").text).to eq("Primary Heading")
    expect(result.headings.length).to eq(8)
  end

  it "metadata_hreflang: Extracts hreflang alternate link tags" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.hreflang.length).to eq(4)
    expect(result.hreflang.get("").lang).to include("en")
  end

  it "metadata_keywords_author: Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq("Comprehensive Metadata Test Page")
    expect(result.metadata.canonical_url).not_to be_empty
    expect(result.metadata.keywords).not_to be_empty
    expect(result.metadata.keywords).to include("rust")
    expect(result.metadata.author).to eq("Jane Developer")
    expect(result.metadata.viewport).not_to be_empty
    expect(result.metadata.generator).to eq("kreuzcrawl/1.0")
    expect(result.metadata.theme_color).to eq("\#ff6600")
    expect(result.metadata.robots).to eq("index, follow")
    expect(result.metadata.lang).to eq("en")
    expect(result.metadata.dir).to eq("ltr")
  end

  it "metadata_og_video_audio: Extracts og:video, og:audio, and og:locale:alternate metadata" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.og.video).to eq("https://example.com/video.mp4")
    expect(result.og.audio).to eq("https://example.com/audio.mp3")
    expect(result.og.locale_alternate.length).to eq(2)
  end

  it "metadata_response_headers: Extracts response metadata from HTTP headers (etag, server, content-language)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.response_headers.etag).not_to be_empty
    expect(result.response_headers.last_modified).not_to be_empty
    expect(result.response_headers.server).to include("nginx")
    expect(result.response_headers.content_language).to eq("en-US")
  end

  it "metadata_word_count: Computes word count from visible page text" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.computed.word_count).to be > 99
    expect(result.computed.word_count).to be < 301
  end
end
