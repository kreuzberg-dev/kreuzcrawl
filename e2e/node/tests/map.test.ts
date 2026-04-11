import { describe, it, expect } from "vitest";
import { scrape, createEngine } from "@kreuzberg/kreuzcrawl";

describe("map", () => {
	it("map_discover_urls: Discovers all URLs on a site without fetching full content", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/map_discover_urls";
		await scrape(engine, url);
		// skipped: field 'urls.length' not available on result type
	});

	it("map_exclude_patterns: Excludes URLs matching patterns from URL map", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/map_exclude_patterns";
		await scrape(engine, url);
		// skipped: field 'urls.length' not available on result type
	});

	it("map_include_subdomains: Includes subdomain URLs in URL map discovery", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/map_include_subdomains";
		await scrape(engine, url);
		// skipped: field 'urls.length' not available on result type
		// skipped: field 'urls' not available on result type
	});

	it("map_large_sitemap: Handles large sitemap with 100+ URLs", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/map_large_sitemap";
		await scrape(engine, url);
		// skipped: field 'urls.length' not available on result type
	});

	it("map_limit_pagination: Limits map result count to specified maximum", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/map_limit_pagination";
		await scrape(engine, url);
		// skipped: field 'urls.length' not available on result type
	});

	it("map_search_filter: Filters map results by search keyword", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/map_search_filter";
		await scrape(engine, url);
		// skipped: field 'urls.length' not available on result type
		// skipped: field 'urls' not available on result type
	});
});
