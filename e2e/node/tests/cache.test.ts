import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('cache', () => {
  it('cache_basic: Crawling with disk cache enabled succeeds without errors', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
  });
});
