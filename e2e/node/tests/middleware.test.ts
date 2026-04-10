import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('middleware', () => {
  it('middleware_engine_crawl_with_defaults: Engine crawl with default middleware chain produces correct multi-page results', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(3);
    expect(result.crawl.minPages).toBeGreaterThanOrEqual(3);
  });

  it('middleware_noop_no_effect: Default middleware chain does not affect normal scraping', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.metadata.title).toBe("Middleware Test");
  });
});
