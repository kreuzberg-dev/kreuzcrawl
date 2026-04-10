"""E2e tests for category: metadata.
"""
from kreuzcrawl import create_engine, scrape


def test_metadata_article_times() -> None:
    """Extracts article:published_time, modified_time, author, section, and tags."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.article.published_time == "2024-01-15T10:00:00Z"
    assert result.article.modified_time == "2024-06-20T14:30:00Z"
    assert result.article.author == "Jane Developer"
    assert result.article.section == "Technology"
    assert len(result.article.tags) == 3

def test_metadata_favicons() -> None:
    """Extracts favicon link tags including apple-touch-icon."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.favicons) == 5
    assert result.favicons.get("").apple_touch

def test_metadata_headings() -> None:
    """Extracts heading hierarchy (h1-h6) from HTML page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.headings.h1) == 1
    assert result.headings.h1.get("0").text == "Primary Heading"
    assert len(result.headings) == 8

def test_metadata_hreflang() -> None:
    """Extracts hreflang alternate link tags."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.hreflang) == 4
    assert "en" in result.hreflang.get("").lang

def test_metadata_keywords_author() -> None:
    """Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.title == "Comprehensive Metadata Test Page"
    assert result.metadata.canonical_url
    assert result.metadata.keywords
    assert "rust" in result.metadata.keywords
    assert result.metadata.author == "Jane Developer"
    assert result.metadata.viewport
    assert result.metadata.generator == "kreuzcrawl/1.0"
    assert result.metadata.theme_color == "#ff6600"
    assert result.metadata.robots == "index, follow"
    assert result.metadata.lang == "en"
    assert result.metadata.dir == "ltr"

def test_metadata_og_video_audio() -> None:
    """Extracts og:video, og:audio, and og:locale:alternate metadata."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.og.video == "https://example.com/video.mp4"
    assert result.og.audio == "https://example.com/audio.mp3"
    assert len(result.og.locale_alternate) == 2

def test_metadata_response_headers() -> None:
    """Extracts response metadata from HTTP headers (etag, server, content-language)."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.response_headers.etag
    assert result.response_headers.last_modified
    assert "nginx" in result.response_headers.server
    assert result.response_headers.content_language == "en-US"

def test_metadata_word_count() -> None:
    """Computes word count from visible page text."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.computed.word_count > 99
    assert result.computed.word_count < 301

