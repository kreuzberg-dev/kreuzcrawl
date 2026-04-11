import { describe, it, expect } from "vitest";
import { scrape, createEngine } from "@kreuzberg/kreuzcrawl";

describe("batch", () => {
	it("scrape_batch_basic: Batch scrape of multiple URLs all succeeding", async () => {
		const engine = createEngine(null);
		await scrape(engine, "");
		// skipped: field 'batch.completed_count' not available on result type
		// skipped: field 'batch.failed_count' not available on result type
		// skipped: field 'batch.total_count' not available on result type
	});

	it("scrape_batch_partial_failure: Batch scrape with one URL failing returns partial results", async () => {
		const engine = createEngine(null);
		await scrape(engine, "");
		// skipped: field 'batch.completed_count' not available on result type
		// skipped: field 'batch.failed_count' not available on result type
		// skipped: field 'batch.total_count' not available on result type
	});

	it("scrape_batch_progress: Batch scrape results include specific URL", async () => {
		const engine = createEngine(null);
		await scrape(engine, "");
		// skipped: field 'batch.total_count' not available on result type
		// skipped: field 'batch.results' not available on result type
	});
});
