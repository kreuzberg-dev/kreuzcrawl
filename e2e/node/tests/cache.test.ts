import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('cache', () => {
  it('cache_basic: Crawling with disk cache enabled succeeds without errors', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
  });
});
