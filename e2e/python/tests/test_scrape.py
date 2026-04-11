"""E2e tests for category: scrape."""

import os

import pytest
from kreuzcrawl import CrawlConfig, create_engine, scrape


@pytest.mark.asyncio
async def test_scrape_asset_dedup() -> None:
    """Same asset linked twice results in one download with one unique hash."""
    engine_config = CrawlConfig(download_assets=True)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_asset_dedup"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) == 2
    assert result.assets[0].content_hash


@pytest.mark.asyncio
async def test_scrape_asset_max_size() -> None:
    """Skips assets exceeding max_asset_size limit."""
    engine_config = CrawlConfig(download_assets=True, max_asset_size=150)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_asset_max_size"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) == 2


@pytest.mark.asyncio
async def test_scrape_asset_type_filter() -> None:
    """Only downloads image assets when asset_types filter is set."""
    engine_config = CrawlConfig(asset_types=["image"], download_assets=True)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_asset_type_filter"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) == 1
    assert "image" in result.assets[0].asset_category


@pytest.mark.asyncio
async def test_scrape_basic_html_page() -> None:
    """Scrapes a simple HTML page and extracts title, description, and links."""
    engine_config = CrawlConfig(max_depth=0, respect_robots_txt=False)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_basic_html_page"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.content_type.strip() == "text/html"
    assert result.html
    assert result.metadata.title.strip() == "Example Domain"
    assert "illustrative examples" in result.metadata.description
    assert result.metadata.canonical_url
    assert len(result.links) > 0
    assert "external" in result.links[0].link_type
    assert len(result.images) == 0
    assert not result.metadata.og_title


@pytest.mark.asyncio
async def test_scrape_complex_links() -> None:
    """Classifies links by type: internal, external, anchor, document, image."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_complex_links"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.links) > 9
    assert result.links[0].url


@pytest.mark.asyncio
async def test_scrape_download_assets() -> None:
    """Downloads CSS, JS, and image assets from page."""
    engine_config = CrawlConfig(download_assets=True)
    engine = create_engine(engine_config)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_download_assets"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) > 2


@pytest.mark.asyncio
async def test_scrape_dublin_core() -> None:
    """Extracts Dublin Core metadata from a page."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_dublin_core"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.dc_title
    assert result.metadata.dc_title.strip() == "Effects of Climate Change on Marine Biodiversity"
    assert result.metadata.dc_creator.strip() == "Dr. Jane Smith"


@pytest.mark.asyncio
async def test_scrape_empty_page() -> None:
    """Handles an empty HTML document without errors."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_empty_page"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.links) > -1
    assert len(result.images) == 0


@pytest.mark.asyncio
async def test_scrape_feed_discovery() -> None:
    """Discovers RSS, Atom, and JSON feed links."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_feed_discovery"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.feeds) >= 3


@pytest.mark.asyncio
async def test_scrape_image_sources() -> None:
    """Extracts images from img, picture, og:image, twitter:image."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_image_sources"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.images) > 4
    assert result.metadata.og_image.strip() == "https://example.com/images/og-hero.jpg"


@pytest.mark.asyncio
async def test_scrape_js_heavy_spa() -> None:
    """Handles SPA page with JavaScript-only content (no server-rendered HTML)."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_js_heavy_spa"
    result = await scrape(engine=engine, url=url)
    assert result.html


@pytest.mark.asyncio
async def test_scrape_json_ld() -> None:
    """Extracts JSON-LD structured data from a page."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_json_ld"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.json_ld
    assert result.json_ld[0].schema_type.strip() == "Recipe"
    assert result.json_ld[0].name.strip() == "Best Chocolate Cake"


@pytest.mark.asyncio
async def test_scrape_malformed_html() -> None:
    """Gracefully handles broken HTML without crashing."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_malformed_html"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.html
    assert "broken HTML" in result.metadata.description


@pytest.mark.asyncio
async def test_scrape_og_metadata() -> None:
    """Extracts full Open Graph metadata from a page."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_og_metadata"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.og_title
    assert result.metadata.og_title.strip() == "Article Title"
    assert result.metadata.og_type.strip() == "article"
    assert result.metadata.og_image.strip() == "https://example.com/images/article-hero.jpg"
    assert result.metadata.og_description
    assert result.metadata.title.strip() == "Article Title - Example Blog"


@pytest.mark.asyncio
async def test_scrape_twitter_card() -> None:
    """Extracts Twitter Card metadata from a page."""
    engine = create_engine(None)
    url = os.environ["MOCK_SERVER_URL"] + "/fixtures/scrape_twitter_card"
    result = await scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.metadata.twitter_card
    assert result.metadata.twitter_card.strip() == "summary_large_image"
    assert result.metadata.twitter_title.strip() == "New Product Launch"
