import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('rate_limit', () => {
  it('rate_limit_basic_delay: Rate limiter adds delay between requests to the same domain', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(3);
    expect(result.rateLimit.minDurationMs).toBeGreaterThanOrEqual(150);
  });

  it('rate_limit_zero_no_delay: Rate limiter with zero delay does not slow crawling', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(2);
  });
});
