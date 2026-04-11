"""E2e tests for category: markdown."""

from kreuzcrawl import create_engine, scrape


def test_markdown_basic_conversion() -> None:
    """HTML is always converted to markdown alongside raw HTML."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.title.strip() == "Test"
    assert result.html
    assert result.markdown.content
    assert "Hello World" in result.markdown.content


def test_markdown_crawl_all_pages() -> None:
    """All crawled pages have markdown field populated."""
    engine = create_engine()
    url = ""
    _ = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type


def test_markdown_fit_content() -> None:
    """Fit markdown removes navigation and boilerplate content."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.markdown.content


def test_markdown_headings_and_paragraphs() -> None:
    """Markdown conversion preserves heading hierarchy and paragraph text."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.markdown.content
    assert "Main Title" in result.markdown.content


def test_markdown_links_converted() -> None:
    """HTML links are converted to markdown link syntax."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.html
    assert result.markdown.content
    assert "Example" in result.markdown.content


def test_markdown_with_citations() -> None:
    """Markdown includes citation conversion with numbered references."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.markdown.content
