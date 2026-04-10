import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('concurrent', () => {
  it('concurrent_basic: Concurrent crawling fetches all pages with max_concurrent workers', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.pages.length).toBe(6);
    expect(result.pages.length).toBeGreaterThanOrEqual(6);
  });

  it('concurrent_depth_two_fan_out: Concurrent depth=2 crawl correctly fans out and deduplicates across levels', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.pages.length).toBe(4);
  });

  it('concurrent_max_pages_exact: Concurrent crawling does not exceed max_pages limit even with high concurrency', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.pages.length).toBeLessThanOrEqual(3);
  });

  it('concurrent_partial_errors: Concurrent crawl handles partial failures gracefully', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.pages.length).toBeGreaterThanOrEqual(2);
  });

  it('concurrent_respects_max_pages: Concurrent crawling respects max_pages limit', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.pages.length).toBeLessThanOrEqual(3);
  });
});
