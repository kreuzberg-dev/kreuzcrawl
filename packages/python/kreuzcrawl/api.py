"""Public API for conversion."""

from __future__ import annotations

from typing import TYPE_CHECKING

import kreuzcrawl._kreuzcrawl as _rust

if TYPE_CHECKING:
    from ._kreuzcrawl import CrawlEngineHandle
    from .options import BrowserConfig, CrawlConfig, ProxyConfig


def _to_rust_browser_config(value: BrowserConfig | None) -> _rust.BrowserConfig | None:
    """Convert Python BrowserConfig to Rust binding type."""
    if value is None:
        return None
    return _rust.BrowserConfig(
        mode=_rust.BrowserMode(value.mode),
        endpoint=value.endpoint,
        timeout=value.timeout,
        wait=_rust.BrowserWait(value.wait),
        wait_selector=value.wait_selector,
        extra_wait=value.extra_wait,
    )


def _to_rust_proxy_config(value: ProxyConfig | None) -> _rust.ProxyConfig | None:
    """Convert Python ProxyConfig to Rust binding type."""
    if value is None:
        return None
    return _rust.ProxyConfig(
        url=value.url,
        username=value.username,
        password=value.password,
    )


def _to_rust_crawl_config(value: CrawlConfig | None) -> _rust.CrawlConfig | None:
    """Convert Python CrawlConfig to Rust binding type.

    Passes simple scalar/list/dict fields directly. Complex nested types
    (auth, browser, proxy) are passed as-is and rely on PyO3 from_py_object
    or are skipped when the Rust binding can't accept them yet.
    """
    if value is None:
        return None

    # Build kwargs, skipping None values and fields the Rust type doesn't accept yet.
    kwargs: dict = {}
    # Simple scalar fields — pass directly.
    for field in (
        "max_depth",
        "max_pages",
        "max_concurrent",
        "respect_robots_txt",
        "user_agent",
        "stay_on_domain",
        "allow_subdomains",
        "include_paths",
        "exclude_paths",
        "custom_headers",
        "request_timeout",
        "max_redirects",
        "retry_count",
        "retry_codes",
        "cookies_enabled",
        "max_body_size",
        "main_content_only",
        "remove_tags",
        "map_limit",
        "map_search",
        "download_assets",
        "max_asset_size",
        "user_agents",
        "capture_screenshot",
        "download_documents",
        "document_max_size",
        "document_mime_types",
        "warc_output",
        "browser_profile",
        "save_browser_profile",
    ):
        val = getattr(value, field, None)
        if val is not None:
            kwargs[field] = val

    # Auth — pass the dict/object directly; PyO3 handles conversion via serde.
    if value.auth is not None:
        kwargs["auth"] = value.auth

    # Asset types — pass as list of strings.
    if value.asset_types:
        kwargs["asset_types"] = list(value.asset_types)

    # Browser config — convert if present.
    if value.browser is not None:
        kwargs["browser"] = _to_rust_browser_config(value.browser)

    # Proxy config — convert if present.
    if value.proxy is not None:
        kwargs["proxy"] = _to_rust_proxy_config(value.proxy)

    return _rust.CrawlConfig(**kwargs)


def create_engine(config: CrawlConfig | None = None) -> _rust.CrawlEngineHandle:
    """Create a new crawl engine with the given configuration."""
    _rust_config = _to_rust_crawl_config(config)
    return _rust.create_engine(_rust_config)


def scrape(engine: CrawlEngineHandle, url: str) -> _rust.ScrapeResult:
    """Scrape a single URL, returning extracted page data."""
    return _rust.scrape(engine, url)


def crawl(engine: CrawlEngineHandle, url: str) -> _rust.CrawlResult:
    """Crawl a website starting from `url`, following links up to the configured depth."""
    return _rust.crawl(engine, url)


def map_urls(engine: CrawlEngineHandle, url: str) -> _rust.MapResult:
    """Discover all pages on a website by following links and sitemaps."""
    return _rust.map_urls(engine, url)


def batch_scrape(engine: CrawlEngineHandle, urls: list[str]) -> list[_rust.BatchScrapeResult]:
    """Scrape multiple URLs concurrently."""
    return _rust.batch_scrape(engine, urls)


def batch_crawl(engine: CrawlEngineHandle, urls: list[str]) -> list[_rust.BatchCrawlResult]:
    """Crawl multiple seed URLs concurrently, each following links to configured depth."""
    return _rust.batch_crawl(engine, urls)
