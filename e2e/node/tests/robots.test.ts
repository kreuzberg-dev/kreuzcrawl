import { describe, it, expect } from "vitest";
import { scrape, createEngine } from "@kreuzberg/kreuzcrawl";

describe("robots", () => {
	it("robots_allow_all: Permissive robots.txt allows all paths", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_allow_all`;
		const result = await scrape(engine, url);
		expect(result.isAllowed).toBe(true);
	});

	it("robots_allow_override: Allow directive overrides Disallow for specific paths", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_allow_override`;
		const result = await scrape(engine, url);
		expect(result.isAllowed).toBe(true);
	});

	it("robots_comments_handling: Correctly parses robots.txt with inline and line comments", async () => {
		const engineConfig = { respect_robots_txt: true, user_agent: "kreuzcrawl" };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_comments_handling`;
		const result = await scrape(engine, url);
		expect(result.isAllowed).toBe(true);
	});

	it("robots_meta_nofollow: Detects nofollow meta robots tag and skips link extraction", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_meta_nofollow`;
		const result = await scrape(engine, url);
		expect(result.nofollowDetected).toBe(true);
	});

	it("robots_meta_noindex: Detects noindex meta robots tag in HTML page", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_meta_noindex`;
		const result = await scrape(engine, url);
		expect(result.noindexDetected).toBe(true);
	});

	it("robots_missing_404: Missing robots.txt (404) allows all crawling", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_missing_404`;
		const result = await scrape(engine, url);
		expect(result.isAllowed).toBe(true);
	});

	it("robots_multiple_user_agents: Picks the most specific user-agent block from robots.txt", async () => {
		const engineConfig = { respect_robots_txt: true, user_agent: "SpecificBot" };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_multiple_user_agents`;
		const result = await scrape(engine, url);
		expect(result.isAllowed).toBe(true);
	});

	it("robots_sitemap_directive: Discovers sitemap URL from Sitemap directive in robots.txt", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_sitemap_directive`;
		const result = await scrape(engine, url);
		expect(result.isAllowed).toBe(true);
	});

	it("robots_x_robots_tag: Respects X-Robots-Tag HTTP header directives", async () => {
		const engineConfig = { respect_robots_txt: true };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/robots_x_robots_tag`;
		const result = await scrape(engine, url);
		expect(result.xRobotsTag.trim()).toBe("noindex, nofollow");
		expect(result.noindexDetected).toBe(true);
		expect(result.nofollowDetected).toBe(true);
	});
});
