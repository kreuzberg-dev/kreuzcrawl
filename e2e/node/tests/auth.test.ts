import { describe, it, expect } from "vitest";
import { scrape, createEngine } from "@kreuzberg/kreuzcrawl";

describe("auth", () => {
	it("auth_basic_http: Sends HTTP Basic authentication header", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/auth_basic_http";
		const result = await scrape(engine, url);
		expect(result.authHeaderSent).toBe(true);
		expect(result.statusCode).toBe(200);
	});

	it("auth_bearer_token: Sends Bearer token in Authorization header", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/auth_bearer_token";
		const result = await scrape(engine, url);
		expect(result.authHeaderSent).toBe(true);
		expect(result.statusCode).toBe(200);
	});

	it("auth_custom_header: Sends authentication via custom header (X-API-Key)", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/auth_custom_header";
		const result = await scrape(engine, url);
		expect(result.authHeaderSent).toBe(true);
		expect(result.statusCode).toBe(200);
	});
});
