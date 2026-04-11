import { describe, it, expect } from "vitest";
import { scrape, createEngine } from "@kreuzberg/kreuzcrawl";

describe("stream", () => {
	it("crawl_stream_events: Crawl stream produces page and complete events", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/crawl_stream_events";
		await scrape(engine, url);
		// skipped: field 'stream.event_count_min' not available on result type
		// skipped: field 'stream.has_page_event' not available on result type
		// skipped: field 'stream.has_complete_event' not available on result type
	});

	it("stream_depth_crawl: Stream produces events for multi-depth crawl with link following", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/stream_depth_crawl";
		await scrape(engine, url);
		// skipped: field 'stream.event_count_min' not available on result type
		// skipped: field 'stream.has_page_event' not available on result type
		// skipped: field 'stream.has_complete_event' not available on result type
	});

	it("stream_with_error_event: Stream emits page and complete events even when some pages fail", async () => {
		const engine = createEngine(null);
		const url = process.env.MOCK_SERVER_URL + "/fixtures/stream_with_error_event";
		await scrape(engine, url);
		// skipped: field 'stream.has_page_event' not available on result type
		// skipped: field 'stream.has_complete_event' not available on result type
		// skipped: field 'stream.event_count_min' not available on result type
	});
});
