import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('rate_limit', () => {
  it('rate_limit_basic_delay: Rate limiter adds delay between requests to the same domain', async () => {
    const result = await scrape();
    expect(result.crawl.pagesCrawled).toBe(3);
    expect(result.rateLimit.minDurationMs).toBeGreaterThanOrEqual(150);
  });

  it('rate_limit_zero_no_delay: Rate limiter with zero delay does not slow crawling', async () => {
    const result = await scrape();
    expect(result.crawl.pagesCrawled).toBe(2);
  });
});
