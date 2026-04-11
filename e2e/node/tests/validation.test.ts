import { describe, it, expect } from "vitest";
import { scrape, createEngine } from "@kreuzberg/kreuzcrawl";

describe("validation", () => {
	it("validation_invalid_exclude_regex: Invalid regex in exclude_paths is rejected", async () => {
		const engineConfig = { exclude_paths: ["(unclosed"] };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/validation_invalid_exclude_regex`;
		await expect(async () => await scrape(engine, url)).rejects.toThrow();
	});

	it("validation_invalid_include_regex: Invalid regex in include_paths is rejected", async () => {
		const engineConfig = { include_paths: ["[invalid"] };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/validation_invalid_include_regex`;
		await expect(async () => await scrape(engine, url)).rejects.toThrow();
	});

	it("validation_invalid_retry_code: Retry code outside 100-599 is rejected", async () => {
		const engineConfig = { retry_codes: [999] };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/validation_invalid_retry_code`;
		await expect(async () => await scrape(engine, url)).rejects.toThrow();
	});

	it("validation_max_pages_zero: max_pages=0 is rejected as invalid config", async () => {
		const engineConfig = { max_pages: 0 };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/validation_max_pages_zero`;
		await expect(async () => await scrape(engine, url)).rejects.toThrow();
	});

	it("validation_max_redirects_too_high: max_redirects > 100 is rejected as invalid config", async () => {
		const engineConfig = { max_redirects: 200 };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/validation_max_redirects_too_high`;
		await expect(async () => await scrape(engine, url)).rejects.toThrow();
	});

	it("validation_timeout_zero: Zero request timeout is rejected as invalid config", async () => {
		const engineConfig = { request_timeout: 0 };
		const engine = createEngine(engineConfig);
		const url = `${process.env.MOCK_SERVER_URL}/fixtures/validation_timeout_zero`;
		await expect(async () => await scrape(engine, url)).rejects.toThrow();
	});
});
